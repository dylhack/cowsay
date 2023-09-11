use tonic::{Request, Status};

use super::Interceptor;

impl Interceptor {
    // #[cfg(debug_assertions)]
    pub fn auth_check(&self, req: Request<()>) -> Result<Request<()>, Status> {
        Ok(req)
    }

    // TODO(dylhack): I would like to implement this.
    //                see https://github.com/hyperium/tonic/issues/1307
    // #[cfg(not(debug_assertions))]
    // pub fn auth_check(&self, req: Request<()>) -> Result<Request<()>, Status> {
    //     use crate::database::clients::get_client_creds;

    //     let req_token = req
    //         .metadata()
    //         .get("authorization")
    //         .and_then(|s| Some(s.to_str().unwrap_or("")))
    //         .and_then(|s| Some(s.to_string()))
    //         .unwrap_or("".to_string());
    //     let token = req_token.split(" ").last().unwrap_or("").to_string();

    //     if token.is_empty() {
    //         return Err(Status::unauthenticated("Not Authenticated"));
    //     }

    //     match get_client_creds(&self.pool, &token).await {
    //         Ok(_) => Ok(req),
    //         Err(_) => Err(Status::unauthenticated("Not Authenticated")),
    //     }
    // }
}
