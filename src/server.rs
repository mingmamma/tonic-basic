#![allow(dead_code, unused)]
// use tonic::{transport::Server, Request, Response, Status};
use tonic::transport::Server;

// use basic::greeter_server::{Greeter, GreeterServer};
// use basic::{TestReply, TestRequest};

pub mod test_mod {
    tonic::include_proto!("basic");
}

// #[derive(Debug, Default)]
// pub struct MyGreeter {}

// #[tonic::async_trait]
// impl Greeter for MyGreeter {
//     async fn say_hello(
//         &self,
//         request: Request<TestRequest>,
//     ) -> Result<Response<TestReply>, Status> {
//         println!("Got a request: {:?}", request);

//         let reply = TestReply {
//             message: format!("got request {}!", request.into_inner().name), // We must use .into_inner() as the fields of gRPC requests and responses are private
//         };

//         Ok(Response::new(reply)) // Send back our formatted greeting
//     }
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let addr = "[::1]:50051".parse().unwrap();
    // let greeter = MyGreeter::default();
    // println!("GreeterServer listening on {}", addr);

    // Server::builder()
    //     .add_service(GreeterServer::new(greeter))
    //     .serve(addr)
    //     .await?;

    Ok(())
}