pub mod cowfiles {
    tonic::include_proto!("cowfiles");
}
use self::cowfiles::{CowfileDescriptor, GetCowfileRequest};
use crate::{client::cowfiles::GetCowfilesRequest, config};
use anyhow::{anyhow, Result};
use base64::Engine;
use cowfiles::cowfiles_manager_client::CowfilesManagerClient;
pub type CowClient = CowfilesManagerClient<tonic::transport::Channel>;

impl serenity::prelude::TypeMapKey for CowClient {
    type Value = CowClient;
}

fn decode_base64(data: &str) -> Result<String> {
    let decoded = base64::engine::general_purpose::STANDARD.decode(data)?;
    let decoded = String::from_utf8(decoded)?;
    Ok(decoded)
}

pub async fn connect() -> Result<CowClient> {
    let client = CowfilesManagerClient::connect(format!("{}", config::get_server_url())).await?;
    Ok(client)
}

pub async fn get_cowfiles(server_id: Option<String>) -> Result<Vec<CowfileDescriptor>> {
    let mut client = connect().await?;
    let resp = client
        .get_cowfiles(GetCowfilesRequest { server_id })
        .await?;
    Ok(resp.into_inner().cowfiles)
}

pub async fn get_cowfile(id: &str) -> Result<String> {
    let mut client = connect().await?;
    client
        .get_cowfile(GetCowfileRequest { id: id.to_string() })
        .await
        .and_then(|res| Ok(decode_base64(&res.get_ref().data)))?
}

pub async fn get_cowfile_by_name(server_id: Option<String>, name: &str) -> Result<String> {
    let cowfiles = get_cowfiles(server_id).await?;
    let cowfile = cowfiles
        .into_iter()
        .find(|cowfile| cowfile.name == name)
        .ok_or(anyhow!("Character not found"))?;

    get_cowfile(&cowfile.id).await
}
