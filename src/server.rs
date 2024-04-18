use tonic::{transport::Server, Request, Response, Status};
use full_service::full_server::{Full, FullServer};
use full_service::{FullReply, FullRequest};
use uuid::Uuid;


pub mod full_service {
    tonic::include_proto!("serving");
}

#[derive(Debug, Default)]
pub struct MyFull {}

#[tonic::async_trait]
impl Full for MyFull {
    async fn request_full(
        &self,
        request: Request<FullRequest>,
    ) -> Result<Response<FullReply>, Status> {

        let data = request.into_inner();

        println!("Got a full request from client: {:?}", data.client);


        let reply = full_service::FullReply {
            client:  data.client,
            execution: Uuid::new_v4().to_string(),
            mask: data.mask,
            features: data.features
        };


        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let server = MyFull::default();

    Server::builder()
        .add_service(FullServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}