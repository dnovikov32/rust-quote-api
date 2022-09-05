#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use quote_api::*;
use quote_api::model::*;
use quote_api::schema::quote::dsl::*;

fn main() {
    let connection = db_connection();
    let posts = quote
        .order(id.asc())
        .load::<Quote>(&connection)
        .expect("Error loading quotes");

    println!("Displaying {} posts", posts.len());

    for post in posts {
        println!("{} - {}", post.id, post.title);
    }
}