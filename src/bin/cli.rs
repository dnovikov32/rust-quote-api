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
use quote_api::model::{ NewShare };
use quote_api::schema::share;
use quote_api::tinkoff::proto::{InstrumentsRequest, SharesResponse};
use quote_api::tinkoff::TinkoffService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let response = fetch_shares().await?;

    for instrument in response.into_inner().instruments {
        insert_share(
            NewShare::from_response(instrument)
        );
    }

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

fn insert_share(new_share: NewShare) -> bool {
    let connection = db_connection();

    diesel::insert_into(share::table)
        .values(&new_share)
        .on_conflict(share::figi)
        .do_update()
        .set(&new_share)
        .execute(&connection)
        .is_ok()
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
