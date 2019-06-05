#![feature(proc_macro_hygiene, decl_macro, type_ascription)]
#[macro_use]
extern crate rocket;

extern crate asta_jobboerse_api;

use asta_jobboerse_api::{
    pool::pool,
    cors::cors,
    get::*,
};

fn main() {
    rocket::ignite()
        .mount("/", routes![get_posts, get_post])
        .manage(pool())
        .attach(cors())
        .launch();
}
