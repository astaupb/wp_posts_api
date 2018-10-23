#![feature(custom_attribute)]

#[macro_use]
extern crate diesel;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate chrono;

pub mod models;
pub mod schema;

use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

use std::env;

pub fn establish_connection() -> MysqlConnection {
    let database_url = env::var("WP_DATABASE_URL").expect("reading db URL from environment");

    MysqlConnection::establish(&database_url).expect(&format!("connecting to {}", database_url))
}

type ConnectionPool = Pool<ConnectionManager<MysqlConnection>>;

pub fn init_connection_pool() -> ConnectionPool {
    let database_url = env::var("WP_DATABASE_URL").expect("reading db URL from environment");

    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    Pool::new(manager).expect("creating connection pool")
}
