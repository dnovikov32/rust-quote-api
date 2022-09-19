table! {
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
