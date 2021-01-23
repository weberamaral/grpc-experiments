use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloResponse, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("grpc.experiments.hello"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello( &self, request: Request<HelloRequest>,)
                                -> Result<Response<HelloResponse>, Status> {
        println!("Got a request: {:?}", request);

        let resp = hello_world::HelloResponse {
            msg: format!("Hello {}!", request.into_inner().name).into(),
        };
        Ok(Response::new(resp))
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:6000".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}