use crate::schema::share;
use crate::tinkoff::proto::Share as TinkoffShare;
use chrono::{ NaiveDateTime, Utc };
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::{ AsChangeset, Insertable, Queryable };

#[derive(Debug, Queryable)]
pub struct Share {
    pub id: i32,
    pub figi: String,
    pub ticker: String,
    pub isin: String,
    pub lot: i32,
    pub currency: String,
    pub name: String,
    pub first_1min_candle_at: NaiveDateTime,
    pub first_1day_candle_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Share {
    pub fn insert_or_update(new_share: NewShare, connection: &PgConnection) -> bool {
        diesel::insert_into(share::table)
            .values(&new_share)
            .on_conflict(share::figi)
            .do_update()
            .set(&new_share)
            .execute(connection)
            .is_ok()
    }

    pub fn find_all(connection: &PgConnection) -> Vec<Share> {
        share::table
            .order(share::name.asc())
            .load::<Share>(connection)
            .expect("Error loading shares")
    }
}

#[derive(AsChangeset, Debug, Insertable)]
#[table_name = "share"]
pub struct NewShare {
    pub figi: String,
    pub ticker: String,
    pub isin: String,
    pub lot: i32,
    pub currency: String,
    pub name: String,
    pub first_1min_candle_at: NaiveDateTime,
    pub first_1day_candle_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl NewShare {
    pub fn from_response(share: TinkoffShare) -> Self {
        let min_date = share.first_1min_candle_date.unwrap();
        let day_date = share.first_1day_candle_date.unwrap();

        Self {
            figi: share.figi,
            ticker: share.ticker,
            isin: share.isin,
            lot: share.lot,
            currency: share.currency,
            name: share.name,
            first_1min_candle_at: NaiveDateTime::from_timestamp(min_date.seconds, min_date.nanos as u32),
            first_1day_candle_at: NaiveDateTime::from_timestamp(day_date.seconds, day_date.nanos as u32),
            updated_at: Utc::now().naive_utc()
        }
    }
}
