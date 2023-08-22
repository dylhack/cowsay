use std::sync::Arc;

use crate::config::get_server_port;
use crate::{
    database,
    proto::cowfiles::{
        cowfiles_manager_server::{CowfilesManager, CowfilesManagerServer},
        Cowfile, Cowfiles, GetCowfilesRequest, SaveCowfileRequest,
    },
};
use anyhow::Result;
use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug)]
pub struct CowManager {
    pool: Arc<Pool<Postgres>>,
}

impl CowManager {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CowfilesManager for CowManager {
    async fn save_cowfile(
        &self,
        request: Request<SaveCowfileRequest>,
    ) -> Result<Response<Cowfile>, Status> {
        let msg: &SaveCowfileRequest = request.get_ref();
        match database::save_cowfile(
            &self.pool,
            &msg.name,
            &msg.server_id,
            &msg.uploader_id,
            msg.author.clone(),
            &msg.data,
        )
        .await
        {
            Ok(reply) => Ok(Response::new(reply)),
            Err(msg) => Err(Status::internal(msg.to_string())),
        }
    }

    async fn get_cowfiles(
        &self,
        request: Request<GetCowfilesRequest>,
    ) -> Result<Response<Cowfiles>, Status> {
        let msg = request.get_ref();
        match database::get_cowfiles(&self.pool, msg.server_id.clone()).await {
            Ok(reply) => Ok(Response::new(Cowfiles { cowfiles: reply })),
            Err(msg) => Err(Status::internal(msg.to_string())),
        }
    }
}

pub async fn start_server(pool: Arc<Pool<Postgres>>) {
    let addr = format!("0.0.0.0:{}", get_server_port()).parse().unwrap();
    let cowfiles = CowManager::new(pool);

    println!("listening on {}", addr);

    Server::builder()
        .add_service(CowfilesManagerServer::new(cowfiles))
        .serve(addr)
        .await
        .expect("Failed to start gRPC server.");
}
