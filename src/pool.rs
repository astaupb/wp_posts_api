use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

use std::env;

pub type ConnectionPool = Pool<ConnectionManager<MysqlConnection>>;

pub fn pool() -> ConnectionPool {
    let database_url = env::var("WP_DATABASE_URL").expect("reading db URL from environment");

    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    Pool::new(manager).expect("creating connection pool")
}
