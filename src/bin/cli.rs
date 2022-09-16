#[macro_use]
extern crate diesel;

use dotenv::dotenv;
use diesel::prelude::*;
use std::env;
use std::error::Error;
use chrono::NaiveDateTime;
use tonic::{metadata::MetadataValue, transport::Channel, service::Interceptor, Status};
use tonic::codegen::InterceptedService;
use tonic::transport::Endpoint;
use quote_api::db_connection;
use quote_api::model::{ Share, NewShare };
use quote_api::schema::share;
use quote_api::tinkoff::proto::{InstrumentsRequest, SharesResponse};
use quote_api::tinkoff::TinkoffService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let connection = db_connection();
    let response = fetch_shares().await?;

    let shares = Share::find_by(&connection);

    for share in shares {
        println!("{:?}", share);
    }

    // for instrument in response.into_inner().instruments {
    //     Share::insert_or_update(
    //         NewShare::from_response(instrument),
    //         &connection
    //     );
    // }

    Ok(())
}

async fn fetch_shares() -> Result<tonic::Response<SharesResponse>, Box<dyn Error>>  {
    let service = TinkoffService::new(
        env::var("TINKOFF_API_HOST").unwrap(),
        env::var("TINKOFF_API_TOKEN").unwrap()
    );

    let channel = service.create_channel().await?;
    let mut instruments = service.instruments(channel).await?;

    let request = tonic::Request::new(
        InstrumentsRequest {
            instrument_status: 1
        }
    );

    let response = instruments.shares(request).await?;

    Ok(response)
}


//
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
