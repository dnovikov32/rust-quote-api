use std::env;
use std::error::Error;
use std::hash::Hash;
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, Timelike, Utc};
use dotenv::dotenv;
use quote_api::db_connection;
use clap::Parser;
use prost_types::{Timestamp, TimestampError};
use tonic::Response;
use quote_api::share::repository::ShareRepository;
use quote_api::tinkoff::proto::{CandleInterval, GetCandlesRequest, GetCandlesResponse};
use quote_api::tinkoff::TinkoffService;

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
    let period_start = add_time(share.first_1min_candle_at, 0, 0, 0);
    let period_end = add_time(Utc::now().naive_utc(), 23, 23, 59);
    let mut day_start = period_start;

    println!("Share {}", share.figi);
    println!("Period {} - {}", period_start, period_end);

    while day_start <= period_end {
        let mut day_end = add_time(day_start, 23, 23, 59);

        let candles = fetch_candle(share.figi, day_start, day_end)
            .await?
            .into_inner()
            .candles;

        println!("    Day {} - {} [{}]", day_start, day_end, candles.len());

        for candle in candles {
            println!("{:?}", candle.close);

            break;
        }

        break;

        day_start = day_start + Duration::days(1);
    }


    Ok(())
}

fn add_time(dt: NaiveDateTime, h: u32, m: u32, s: u32) -> NaiveDateTime {
    NaiveDate::from_ymd(dt.year(), dt.month(), dt.day())
        .and_hms(h, m, s)
}

fn timestamp(dt: NaiveDateTime) -> Result<Timestamp, TimestampError> {
    Timestamp::date_time(
        dt.year() as i64,
        dt.month() as u8,
        dt.day() as u8,
        dt.hour() as u8,
        dt.minute() as u8,
        dt.second() as u8
    )
}

async fn fetch_candle(
    figi: String,
    from: NaiveDateTime,
    to: NaiveDateTime
) -> Result<Response<GetCandlesResponse>, Box<dyn Error>> {
    let service = TinkoffService::new(
        env::var("TINKOFF_API_HOST").unwrap(),
        env::var("TINKOFF_API_TOKEN").unwrap()
    );

    let channel = service.create_channel().await?;
    let mut market_data = service.market_data(channel).await?;

    let request = tonic::Request::new(
        GetCandlesRequest {
            figi: figi,
            from: Some(timestamp(from).unwrap()),
            to: Some(timestamp(to).unwrap()),
            interval: CandleInterval::CandleInterval1Min as i32
        }
    );

    let response = market_data.get_candles(request).await?;

    Ok(response)
}