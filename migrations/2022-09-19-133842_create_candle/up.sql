CREATE TABLE candle (
    id SERIAL CONSTRAINT candle_id_pk PRIMARY KEY,
    share_id INT NOT NULL,
    open FLOAT NOT NULL,
    close FLOAT NOT NULL,
    high FLOAT NOT NULL,
    low FLOAT NOT NULL,
    volume FLOAT NOT NULL,
    date_time TIMESTAMP(0) WITHOUT TIME ZONE NOT NULL
);

ALTER TABLE candle ADD CONSTRAINT candle_share_id_fk
    FOREIGN KEY (share_id) REFERENCES share (id)
    ON DELETE CASCADE NOT DEFERRABLE INITIALLY IMMEDIATE;

CREATE INDEX candle_share_id_date_time_idx ON candle (share_id, date_time);