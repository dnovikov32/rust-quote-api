#[macro_use]
extern crate diesel;

use dotenv::dotenv;
use diesel::prelude::*;
use std::env;

use quote_api::*;
use quote_api::model::*;
use quote_api::schema::quote::dsl::*;
use quote_api::tinkoff::instruments_service_client::InstrumentsServiceClient;
use quote_api::tinkoff::InstrumentsRequest;

use tonic::{metadata::MetadataValue, transport::Channel, service::Interceptor, Status};
use tonic::codegen::InterceptedService;
use tonic::transport::Endpoint;


//
// #[derive(Debug)]
// pub struct DefaultInterceptor {
//     token: String,
// }
//
//
// impl Interceptor for DefaultInterceptor {
//     fn call(&mut self, request: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
//         let mut req = request;
//         req.metadata_mut().append(
//             "authorization",
//             format!("bearer {}", self.token).parse().unwrap(),
//         );
//         // req.metadata_mut().append(
//         //     "x-tracking-id",
//         //     uuid::Uuid::new_v4().to_string().parse().unwrap(),
//         // );
//         // req.metadata_mut()
//         //     .append("x-app-name", "ovr.tinkoffInvestRust".parse().unwrap());
//
//         Ok(req)
//     }
// }


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let host = env::var("TINKOFF_API_HOST").unwrap();
    let secret = env::var("TINKOFF_API_TOKEN").unwrap();
    // let tls = ClientTlsConfig::new();

    let channel = Channel::from_static("https://invest-public-api.tinkoff.ru:443/")
        .connect()
        .await?;

    let mut client = InstrumentsServiceClient::with_interceptor(
        channel,
        intercept,
    );

    let request = tonic::Request::new(
        InstrumentsRequest {
            instrument_status: 1
        }
    );

    let response = client.shares(request).await?;
    // let response = client.shares(request).await.unwrap().into_inner();

    // println!("RESPONSE 1111 = {:?}", response.into_inner().instruments);
    for instrument in response.into_inner().instruments {
        println!("{}", instrument.figi);
    }

    Ok(())

}

fn intercept(req: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
    println!("Intercepting request: {:?}", req);

    let mut request = req;
    request.metadata_mut().append(
        "authorization",
        format!("Bearer {}", "t.XrALhgBIXiOEvvEhnKp0neSZqxaykaKaBNsIWsjo9Q6Ec0MJ6vdcnr97xPf0W0mCSoZx302VZc67CNkcKfMIAw").parse().unwrap(),
    );

    println!("Intercepting request 2: {:?}", request);

    Ok(request)
}

// fn intercept(req: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
//     println!("Intercepting request: {:?}", req);
//
//     let mut request = req;
//     request.metadata_mut().append(
//         "Authorization",
//         format!("Bearer {}", "t.XrALhgBIXiOEvvEhnKp0neSZqxaykaKaBNsIWsjo9Q6Ec0MJ6vdcnr97xPf0W0mCSoZx302VZc67CNkcKfMIAw").parse().unwrap(),
//     );
//
//     println!("Intercepting request {:?}", request);
//
//     Ok(request)
// }


// struct MyInterceptor;
//
// impl Interceptor for MyInterceptor {
//     fn call(&mut self, request: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
//         Ok(request)
//     }
// }
//
// #[allow(dead_code, unused_variables)]
// async fn using_named_interceptor() -> Result<(), Box<dyn std::error::Error>> {
//     let channel = Endpoint::from_static("http://[::1]:50051")
//         .connect()
//         .await?;
//
//     let client: GreeterClient<InterceptedService<Channel, MyInterceptor>> =
//         GreeterClient::with_interceptor(channel, MyInterceptor);
//
//     Ok(())
// }



// async fn test_grps() -> Result<(), Box<dyn std::error::Error>> {
//     dotenv().ok();
//
//     let host = env::var("TINKOFF_API_HOST").unwrap();
//     let secret = env::var("TINKOFF_API_TOKEN").unwrap();
//
//     let channel = Channel::from_static("https://invest-public-api.tinkoff.ru").connect().await;
//
//     let token: MetadataValue<_> = "Bearer t.XrALhgBIXiOEvvEhnKp0neSZqxaykaKaBNsIWsjo9Q6Ec0MJ6vdcnr97xPf0W0mCSoZx302VZc67CNkcKfMIAw".parse()?;
//
//     // let mut client = InstrumentsServiceClient::connect(url).await?;
//
//     let mut client = InstrumentsServiceClient::with_interceptor(channel, move |mut req: tonic::Request<()> | {
//         req.metadata_mut().insert("authorization", token.clone());
//         Ok(req)
//     });
//
//     let request = tonic::Request::new(
//         InstrumentsRequest {
//             instrument_status: 1
//         }
//     );
//
//     let response = client.shares(request).await?;
//     println!("RESPONSE = {:?}", response);
//     for instrument in response.into_inner().instruments {
//         println!("{}", instrument.figi);
//     }
//
//     Ok(())
// }


// let connection = db_connection();
// let posts = quote
//     .order(id.asc())
//     .load::<Quote>(&connection)
//     .expect("Error loading quotes");
//
// println!("Displaying {} posts", posts.len());
//
// for post in posts {
//     println!("{} - {}", post.id, post.title);
// }

// let runtime = tokio::runtime::Runtime::new().unwrap();
//
// match runtime.block_on(test_grps()) {
//     Ok(x) => x,
//     Err(_) => println!("Error on call async funciton")
// }

// test_grps().await?;

// Ok(())
