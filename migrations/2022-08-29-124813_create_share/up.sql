CREATE TABLE share (
    id SERIAL CONSTRAINT share_id_pk PRIMARY KEY,
    figi VARCHAR(12) NOT NULL,
    ticker VARCHAR(12) NOT NULL,
    isin VARCHAR(12) NOT NULL,
    lot INT NOT NULL,
    currency VARCHAR(3) NOT NULL,
    name VARCHAR(255) NOT NULL,
    first_1min_candle_at TIMESTAMP(0) WITHOUT TIME ZONE NOT NULL,
    first_1day_candle_at TIMESTAMP(0) WITHOUT TIME ZONE NOT NULL,
    updated_at TIMESTAMP(0) WITHOUT TIME ZONE NOT NULL
);

CREATE UNIQUE INDEX share_figi_unq ON share (figi);
CREATE UNIQUE INDEX share_ticker_unq ON share (ticker);
CREATE UNIQUE INDEX share_isin_unq ON share (isin);