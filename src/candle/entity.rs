use crate::schema::candle;
use diesel::{AsChangeset, Insertable, Queryable};
use chrono::{NaiveDateTime, Utc};
use crate::tinkoff::proto::HistoricCandle;

#[derive(Debug, Queryable)]
pub struct Candle {
    pub id: i32,
    pub share_id: i32,
    pub open: f32,
    pub close: f32,
    pub high: f32,
    pub low: f32,
    pub volume: i32,
    pub time: NaiveDateTime
}

#[derive(AsChangeset, Debug, Insertable)]
#[table_name = "candle"]
pub struct NewCandle {
    pub share_id: i32,
    pub open: f32,
    pub close: f32,
    pub high: f32,
    pub low: f32,
    pub volume: i32,
    pub time: NaiveDateTime
}

// impl NewCandle {
//     pub fn from_request(share_id: i32, candle: HistoricCandle) -> Self {
//         let time = candle.time.unwrap();
//
//         Self {
//             share_id,
//             open: f32,
//             close: f32,
//             high: f32,
//             low: f32,
//             volume: i32,
//             time: NaiveDateTime::from_timestamp(time.seconds, time.nanos as u32),
//         }
//     }
// }