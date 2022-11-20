mod hello;

use hello::hello::hello_service_client::HelloServiceClient;
use hello::hello::HelloRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = HelloServiceClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Brasil".into(),
    });

    let response = client.hello_world(request).await?;
    println!("RESPONSE={:?}", response.into_inner().message);
    Ok(())
}
