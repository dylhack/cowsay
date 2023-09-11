use tonic::{Request, Status};

use super::Interceptor;

impl Interceptor {
    pub fn log(&self, req: Request<()>) -> Result<Request<()>, Status> {
        println!("{:?}", req);
        Ok(req)
    }
}
