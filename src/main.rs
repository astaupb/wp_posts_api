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
        .mount("/", routes![get_posts, get_post])
        .manage(pool)
        .launch();
}

type ConnectionPool = Pool<ConnectionManager<MysqlConnection>>;

#[get("/")]
fn get_posts(state: State<ConnectionPool>) -> Json<Vec<PostResponse>> {
    Json(
        wp_posts::table
            .select(wp_posts::all_columns)
            .filter(wp_posts::post_type.eq("job_listing"))
            .filter(wp_posts::post_status.eq("publish"))
            .order(wp_posts::post_date.desc())
            .load(&state.get().unwrap())
            .unwrap()
            .iter()
            .map(|post: &Post| PostResponse::from(post: &Post))
            .collect(),
    )
}

#[get("/<post_id>")]
fn get_post(state: State<ConnectionPool>, post_id: u32) -> Option<Json<PostResponse>> {
        wp_posts::table
            .select(wp_posts::all_columns)
            .filter(wp_posts::post_type.eq("job_listing"))
            .filter(wp_posts::post_status.eq("publish"))
            .filter(wp_posts::ID.eq(post_id))
            .first(&state.get().unwrap())
            .optional()
            .unwrap()
            .map(|post: Post| Json(PostResponse::from(&post)))
}
