use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use crate::schema::share;
use crate::share::entity::{NewShare, Share};

pub struct ShareRepository<'a> {
    connection: &'a PgConnection
}

impl<'a> ShareRepository<'_> {
    pub fn new(connection: &PgConnection) -> ShareRepository {
        ShareRepository { connection }
    }

    pub fn insert_or_update(&self, new_share: NewShare) -> usize {
        diesel::insert_into(share::table)
            .values(&new_share)
            .on_conflict(share::figi)
            .do_update()
            .set(&new_share)
            .execute(self.connection)
            .expect("Failed on insert or update share")
    }

    pub fn find_all(&self) -> Vec<Share> {
        share::table
            .order(share::name.asc())
            .load::<Share>(self.connection)
            .expect("Failed on find all shares")
    }

    pub fn find_by_ticker(&self, ticker: String) -> Share {
        share::table
            .filter(share::ticker.eq(ticker))
            .first::<Share>(self.connection)
            .expect("Failed find share by ticker")
    }

}