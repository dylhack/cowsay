pub mod cowfiles {
    tonic::include_proto!("cowfiles");
}
use anyhow::Result;
use cowfiles::cowfiles_manager_client::CowfilesManagerClient;

use crate::config;

pub type CowClient = CowfilesManagerClient<tonic::transport::Channel>;

impl serenity::prelude::TypeMapKey for CowClient {
    type Value = CowClient;
}

pub async fn connect() -> Result<CowClient> {
    let server_url = config::get_server_url();
    let client = CowfilesManagerClient::connect(format!("{}", server_url)).await?;
    Ok(client)
}
