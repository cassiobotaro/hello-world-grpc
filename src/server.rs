mod hello;
use hello::hello::{hello_service_server::HelloService, HelloRequest, HelloResponse};
use tonic::{transport::Server, Request, Response, Status};

use crate::hello::hello::hello_service_server::HelloServiceServer;

#[derive(Debug, Default)]
struct MyHelloServer;

#[tonic::async_trait]
impl HelloService for MyHelloServer {
    async fn hello_world(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        let params = request.into_inner();
        let message = format!("Hello World! {}", params.name);
        Ok(Response::new(HelloResponse { message }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let hello_server = MyHelloServer::default();

    println!("MyHelloServer listening on {}", addr);

    Server::builder()
        .add_service(HelloServiceServer::new(hello_server))
        .serve(addr)
        .await?;

    Ok(())
}
