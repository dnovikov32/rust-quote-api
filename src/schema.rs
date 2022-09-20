// @generated automatically by Diesel CLI.

diesel::table! {
    candle (id) {
        id -> Int4,
        share_id -> Int4,
        open -> Float8,
        close -> Float8,
        high -> Float8,
        low -> Float8,
        volume -> Float8,
        date_time -> Timestamp,
    }
}

diesel::table! {
    share (id) {
        id -> Int4,
        figi -> Varchar,
        ticker -> Varchar,
        isin -> Varchar,
        lot -> Int4,
        currency -> Varchar,
        name -> Varchar,
        first_1min_candle_at -> Timestamp,
        first_1day_candle_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(candle -> share (share_id));

diesel::allow_tables_to_appear_in_same_query!(
    candle,
    share,
);
