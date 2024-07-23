#![allow(dead_code, unused)]

use basic::greeter_client::GreeterClient;
use basic::TestRequest;

pub mod basic {
    tonic::include_proto!("basic");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let mut client = GreeterClient::connect("http://[::1]:50051").await?;

//     let request = tonic::Request::new(TestRequest {
//         name: "Tonic".into(),
//     });

//     let response = client.run_sth(request).await?;

//     println!("RESPONSE={:?}", response);

    Ok(())
}