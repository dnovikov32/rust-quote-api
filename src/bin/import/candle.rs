use std::error::Error;
use chrono::{Datelike, NaiveDate, NaiveDateTime, Utc};
use dotenv::dotenv;
use quote_api::db_connection;
use clap::Parser;
use quote_api::share::repository::ShareRepository;

#[derive(Debug, Parser)]
struct Cli {
    ticker: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let args = Cli::parse();
    let db_connection = db_connection();
    let share_repository = ShareRepository::new(&db_connection);

    let share = share_repository.find_by_ticker(args.ticker.to_uppercase());

    // let candles = fetch_candles()

    println!("{:?}", share);

    let start = get_date_time(share.first_1min_candle_at, 0, 0, 0);
    let end = get_date_time(Utc::now().naive_utc(), 23, 23, 59);

    println!("start {:?}", start);
    println!("end {:?}", end);



    Ok(())
}

fn get_date_time(dt: NaiveDateTime, h: u32, m: u32, s: u32) -> NaiveDateTime {
    NaiveDate::from_ymd(dt.year(), dt.month(), dt.day()).and_hms(h, m, s)
}