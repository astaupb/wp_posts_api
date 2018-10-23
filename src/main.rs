#![feature(plugin, custom_derive, custom_attribute)]
#![feature(type_ascription)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_cors;

extern crate asta_jobboerse_api;
extern crate diesel;

use asta_jobboerse_api::models::*;
use asta_jobboerse_api::schema::*;
use asta_jobboerse_api::*;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use rocket::{http::Method, State};
use rocket_contrib::Json;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

type ConnectionPool = Pool<ConnectionManager<MysqlConnection>>;

fn main() {
    let pool = init_connection_pool();
    rocket::ignite()
        .mount("/", routes![get_posts, get_post])
        .manage(pool)
        .attach(cors())
        .launch();
}

fn cors() -> rocket_cors::Cors {
    rocket_cors::Cors {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![Method::Get, Method::Post, Method::Delete, Method::Put]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "X-Api-Key",
            "Content-Type",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
}

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
