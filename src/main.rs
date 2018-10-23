#![feature(plugin, custom_derive, custom_attribute)]
#![feature(type_ascription)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

extern crate asta_jobboerse_api;
extern crate diesel;

use asta_jobboerse_api::models::*;
use asta_jobboerse_api::schema::*;
use asta_jobboerse_api::*;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use rocket::State;
use rocket_contrib::Json;


fn main() {
    let pool = init_connection_pool();
    rocket::ignite()
        .mount("/", routes![get_posts])
        .manage(pool)
        .launch();
}

#[get("/")]
fn get_posts(state: State<Pool<ConnectionManager<MysqlConnection>>>) -> Json<Vec<PostResponse>> {
    Json(
        wp_posts::table
            .select(wp_posts::all_columns)
            .filter(wp_posts::post_type.eq("job_listing"))
            .load(&state.get().unwrap())
            .unwrap()
            .iter()
            .map(|post: &Post| PostResponse::from(post: &Post))
            .collect()
    )
}
