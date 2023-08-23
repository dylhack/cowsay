pub mod cowfiles {
    tonic::include_proto!("cowfiles");
}
#[cfg(not(feature = "cowserve"))]
mod builtin;
#[cfg(feature = "cowserve")]
use crate::config;
use anyhow::{anyhow, Result};
use base64::Engine;
use cowfiles::CowfileDescriptor;
#[cfg(feature = "cowserve")]
use cowfiles::{cowfiles_manager_client::CowfilesManagerClient, GetCowfileRequest, GetCowfilesRequest};
#[cfg(feature = "cowserve")]
pub type CowClient = CowfilesManagerClient<tonic::transport::Channel>;

fn decode_base64(data: &str) -> Result<String> {
    let decoded = base64::engine::general_purpose::STANDARD.decode(data)?;
    let decoded = String::from_utf8(decoded)?;
    Ok(decoded)
}

#[cfg(feature = "cowserve")]
async fn connect() -> Result<CowClient> {
    let client = CowfilesManagerClient::connect(format!("{}", config::get_server_url())).await?;
    Ok(client)
}

#[cfg(feature = "cowserve")]
pub async fn get_cowfiles(server_id: Option<String>) -> Result<Vec<CowfileDescriptor>> {
    let mut client = connect().await?;
    let resp = client
        .get_cowfiles(GetCowfilesRequest { server_id })
        .await?;
    Ok(resp.into_inner().cowfiles)
}

#[cfg(feature = "cowserve")]
pub async fn get_cowfile(id: &str) -> Result<String> {
    let mut client = connect().await?;
    client
        .get_cowfile(GetCowfileRequest { id: id.to_string() })
        .await
        .and_then(|res| Ok(decode_base64(&res.get_ref().data)))?
}

#[cfg(feature = "cowserve")]
pub async fn get_cowfile_by_name(server_id: Option<String>, name: &str) -> Result<String> {
    let cowfiles = get_cowfiles(server_id).await?;
    let cowfile = cowfiles
        .into_iter()
        .find(|cowfile| cowfile.name == name)
        .ok_or(anyhow!("Character not found"))?;

    get_cowfile(&cowfile.id).await
}

#[cfg(not(feature = "cowserve"))]
pub async fn get_cowfiles(_sid: Option<String>) -> Result<Vec<CowfileDescriptor>> {
    let mut result = vec![];

    builtin::get_builtin().iter().for_each(|chara| {
        result.push(CowfileDescriptor {
            id: chara.id.clone(),
            server_id: None,
            name: chara.name.clone(),
            author: chara.author.clone(),
            uploader_id: chara.uploader_id.clone(),
        })
    });

    Ok(result)
}

#[cfg(not(feature = "cowserve"))]
pub async fn get_cowfile(id: &str) -> Result<String> {
    let cowfiles = builtin::get_builtin();
    let data = cowfiles
        .into_iter()
        .find(|cowfile| cowfile.id == id)
        .and_then(|res| Some(decode_base64(&res.data).unwrap()))
        .ok_or(anyhow!("Character not found"))?;

    Ok(data)
}

#[cfg(not(feature = "cowserve"))]
pub async fn get_cowfile_by_name(server_id: Option<String>, name: &str) -> Result<String> {
    let cowfiles = get_cowfiles(server_id).await?;
    let cowfile = cowfiles
        .into_iter()
        .find(|cowfile| cowfile.name == name)
        .ok_or(anyhow!("Character not found"))?;

    get_cowfile(&cowfile.id).await
}
