use common::diesel::prelude::*;
use common::model::*;
use common::schema::quote::dsl::*;

fn main() {
    let connection = common::db_connection();
    let posts = quote
        .order(id.asc())
        .load::<Quote>(&connection)
        .expect("Error loading quotes");

    println!("Displaying {} posts", posts.len());

    for post in posts {
        println!("{} - {}", post.id, post.title);
    }
}