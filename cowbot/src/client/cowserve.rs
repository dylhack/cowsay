use super::cowfiles::{
    cowfiles_manager_client::CowfilesManagerClient, CowfileDescriptor, GetCowfilesRequest,
    GetCowsayRequest,
};
use crate::config;
use anyhow::Result;

pub type CowClient = CowfilesManagerClient<tonic::transport::Channel>;

const FONT: &[u8; 2063520] =
    include_bytes!("../../../assets/font/JetBrainsMonoNerdFont-Regular.ttf");
const FONT_BOLD: &[u8; 2067288] =
    include_bytes!("../../../assets/font/JetBrainsMonoNerdFont-Bold.ttf");

async fn connect() -> Result<CowClient> {
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

pub async fn cowsay(cid: &str, msg: &str) -> Result<Vec<u8>> {
    let mut client = connect().await?;
    let cowsay = client
        .get_cowsay(GetCowsayRequest {
            chara_id: cid.to_string(),
            message: msg.to_string(),
            font: FONT.to_vec(),
            bubble_font: Some(FONT_BOLD.to_vec()),
        })
        .await?;

    Ok(cowsay.into_inner().data)
}
