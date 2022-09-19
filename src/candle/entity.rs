use crate::schema::candle;
use diesel::Insertable;
use chrono::{NaiveDateTime, Utc};

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