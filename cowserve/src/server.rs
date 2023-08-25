use std::sync::Arc;

use crate::{
    config::get_server_port,
    database::{get_cowfile, get_cowfiles, save_cowfile},
    proto::cowfiles::{
        cowfiles_manager_server::{CowfilesManager, CowfilesManagerServer},
        Cowfile, Cowfiles, GetCowfileRequest, GetCowfilesRequest, SaveCowfileRequest, GetPreviewRequest, Preview
    }, services::previews::get_preview,
};
use anyhow::Result;
use async_trait::async_trait;
use celery::Celery;
use sqlx::{Pool, Postgres};
use tonic::{transport::Server, Request, Response, Status};

pub struct CowManager {
    pool: Arc<Pool<Postgres>>,
    queue: Arc<Celery>,
}

impl CowManager {
    pub fn new(pool: Arc<Pool<Postgres>>, queue: Arc<Celery>) -> Self {
        Self { pool, queue }
    }
}

#[async_trait]
impl CowfilesManager for CowManager {
    async fn save_cowfile(
        &self,
        request: Request<SaveCowfileRequest>,
    ) -> Result<Response<Cowfile>, Status> {
        let msg: &SaveCowfileRequest = request.get_ref();
        println!("{:?}", request);
        match save_cowfile(
            &self.pool,
            &msg.name,
            &msg.server_id,
            &msg.uploader_id,
            msg.author.clone(),
            &msg.data,
        )
        .await
        {
            Ok(reply) => {
                if let Err(why) = self.queue.send_task(crate::jobs::previews::gen_previews::new()).await {
                    println!("Error sending task: {:?}", why);
                }
                Ok(Response::new(reply))
            },
            Err(msg) => Err(Status::internal(msg.to_string())),
        }
    }

    async fn get_cowfiles(
        &self,
        request: Request<GetCowfilesRequest>,
    ) -> Result<Response<Cowfiles>, Status> {
        let msg = request.get_ref();
        println!("{:?}", request);
        match get_cowfiles(&self.pool, msg.server_id.clone()).await {
            Ok(reply) => Ok(Response::new(Cowfiles { cowfiles: reply })),
            Err(msg) => Err(Status::internal(msg.to_string())),
        }
    }

    async fn get_cowfile(
        &self,
        request: Request<GetCowfileRequest>,
    ) -> Result<Response<Cowfile>, Status> {
        let msg = request.get_ref();
        println!("{:?}", request);
        match get_cowfile(&self.pool, &msg.id).await {
            Ok(reply) => Ok(Response::new(reply)),
            Err(msg) => Err(Status::internal(msg.to_string())),
        }
    }

    async fn get_preview(&self, request: Request<GetPreviewRequest>) -> Result<Response<Preview>, Status> {
        let msg = request.get_ref();
        println!("{:?}", request);
        match get_preview(&self.pool, &msg.id).await {
            Ok(data) => Ok(Response::new(Preview { data })),
            Err(msg) => Err(Status::internal(msg.to_string())),
        }
    }
}

pub async fn start_server(pool: Arc<Pool<Postgres>>, queue: Arc<Celery>) {
    let addr = format!("0.0.0.0:{}", get_server_port()).parse().unwrap();
    let cowfiles = CowManager::new(pool, queue);

    println!("listening on {}", addr);

    Server::builder()
        .add_service(CowfilesManagerServer::new(cowfiles))
        .serve(addr)
        .await
        .expect("Failed to start gRPC server.");
}
