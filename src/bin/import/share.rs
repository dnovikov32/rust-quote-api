#[macro_use]
extern crate diesel;

use dotenv::dotenv;
use std::env;
use std::error::Error;
use indicatif::ProgressBar;
use quote_api::db_connection;
use quote_api::schema::share;
use quote_api::share::entity::{NewShare, Share};
use quote_api::share::repository::ShareRepository;
use quote_api::tinkoff::proto::{InstrumentsRequest, SharesResponse};
use quote_api::tinkoff::TinkoffService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let instruments = fetch_shares()
        .await?
        .into_inner()
        .instruments;

    let db_connection = db_connection();
    let repository = ShareRepository::new(&db_connection);
    let progress_bar = ProgressBar::new(instruments.len().try_into().unwrap());

    for instrument in instruments {
        let new_share = NewShare::from_response(instrument);

        repository.insert_or_update(new_share);
        progress_bar.inc(1);
    }

    progress_bar.finish_with_message("Done");

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
