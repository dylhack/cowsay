mod interceptor;
use crate::{
    config::get_server_port,
    cowfiles::{
        cowfiles_manager_server::{CowfilesManager, CowfilesManagerServer},
        CowfileData, CowfileDescriptor, Cowfiles, CowsayData, GetCowfileRequest,
        GetCowfilesRequest, GetCowsayRequest, Preview, SaveCowfileRequest,
    },
    database::{cowdata::get_cowdata, get_cowfile, get_cowfiles, save_cowfile, Client},
    server::interceptor::Interceptor,
    services::{cowsay::cowsay, previews::get_preview},
};
use anyhow::Result;
use async_trait::async_trait;
use celery::Celery;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tonic::{transport::Server, Request, Response, Status};

pub struct CowManager {
    pool: Arc<Client>,
    queue: Arc<Celery>,
}

impl CowManager {
    pub fn new(pool: Arc<Client>, queue: Arc<Celery>) -> Self {
        Self { pool, queue }
    }
}

#[async_trait]
impl CowfilesManager for CowManager {
    async fn save_cowfile(
        &self,
        request: Request<SaveCowfileRequest>,
    ) -> Result<Response<CowfileDescriptor>, Status> {
        let msg: &SaveCowfileRequest = request.get_ref();
        println!("{:?}", request);
        match save_cowfile(
            &self.pool,
            &msg.name,
            &msg.server_id,
            &msg.uploader_id,
            &msg.author,
            &msg.data,
        )
        .await
        {
            Ok(reply) => {
                if let Err(why) = self
                    .queue
                    .send_task(crate::jobs::previews::gen_previews::new())
                    .await
                {
                    println!("Error sending task: {:?}", why);
                }
                Ok(Response::new(reply))
            }
            Err(msg) => Err(Status::internal(msg.to_string())),
        }
    }

    async fn get_cowfiles(
        &self,
        request: Request<GetCowfilesRequest>,
    ) -> Result<Response<Cowfiles>, Status> {
        let msg = request.get_ref();
        match get_cowfiles(&self.pool, msg.server_id.clone()).await {
            Ok(reply) => Ok(Response::new(Cowfiles { cowfiles: reply })),
            Err(msg) => Err(Status::internal(msg.to_string())),
        }
    }

    async fn get_cowfile(
        &self,
        request: Request<GetCowfileRequest>,
    ) -> Result<Response<CowfileDescriptor>, Status> {
        let msg = request.get_ref();
        match get_cowfile(&self.pool, &msg.id).await {
            Ok(reply) => Ok(Response::new(reply)),
            Err(msg) => Err(Status::internal(msg.to_string())),
        }
    }

    async fn get_preview(
        &self,
        request: Request<GetCowfileRequest>,
    ) -> Result<Response<Preview>, Status> {
        let msg = request.get_ref();
        match get_preview(&self.pool, &msg.id).await {
            Ok(data) => Ok(Response::new(Preview { data })),
            Err(msg) => Err(Status::internal(msg.to_string())),
        }
    }

    async fn get_cowdata(
        &self,
        request: Request<GetCowfileRequest>,
    ) -> Result<Response<CowfileData>, Status> {
        let msg = request.get_ref();
        match get_cowdata(&self.pool, &msg.id).await {
            Ok(data) => Ok(Response::new(CowfileData { data })),
            Err(msg) => Err(Status::internal(msg.to_string())),
        }
    }

    async fn cowsay(
        &self,
        request: Request<GetCowsayRequest>,
    ) -> Result<Response<CowsayData>, Status> {
        let msg = request.get_ref();
        match cowsay(&self.pool, msg.clone()).await {
            Ok(data) => Ok(Response::new(data)),
            Err(msg) => Err(Status::internal(msg.to_string())),
        }
    }
}

pub async fn start_server(pool: Arc<Pool<Postgres>>, queue: Arc<Celery>) {
    let addr = format!("0.0.0.0:{}", get_server_port()).parse().unwrap();
    let interceptor = Interceptor {
        pool: Arc::clone(&pool),
    };
    let cowfiles = CowfilesManagerServer::with_interceptor(
        CowManager::new(Arc::clone(&pool), queue),
        interceptor,
    );
    println!("Listening on {}", addr);

    Server::builder()
        .add_service(cowfiles)
        .serve(addr)
        .await
        .expect("Failed to start gRPC server.");
}
