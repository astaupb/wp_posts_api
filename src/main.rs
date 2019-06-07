#![feature(proc_macro_hygiene, decl_macro, type_ascription)]
#[macro_use]
extern crate rocket;

extern crate wp_posts_api;

use wp_posts_api::{
    pool::pool,
    cors::cors,
    get::*,
};

fn main() {
    rocket::ignite()
        .mount("/", routes![get_posts, get_full_post])
        .manage(pool())
        .attach(cors())
        .launch();
}
