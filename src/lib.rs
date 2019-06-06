#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::models::{Todo,NewTodo};

pub mod schema;
pub mod models;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_todo<'a>(conn: &PgConnection, name: &'a str) -> Todo {
    use schema::todos;
    let  new_todo = NewTodo {
        name,
    };

    diesel::insert_into(todos::table)
        .values(&new_todo)
        .get_result(conn)
        .expect("Error saving new todo in table")

}