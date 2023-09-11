mod auth;
mod log;
use crate::database::Client;
use std::sync::Arc;
use tonic::{service::Interceptor as TonicInterceptor, Request, Status};

pub struct Interceptor {
    pub pool: Arc<Client>,
}

impl Clone for Interceptor {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
        }
    }
}

impl TonicInterceptor for Interceptor {
    fn call(&mut self, req: Request<()>) -> Result<Request<()>, Status> {
        let req = self.auth_check(req)?;
        let req = self.log(req)?;
        Ok(req)
    }
}
