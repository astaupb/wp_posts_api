use diesel::prelude::*;
use rocket::State;
use rocket_contrib::json::Json;
use crate::{
    pool::ConnectionPool,
    models::Post,
    schema::*,
    response::{
        FullPostResponse, ShortPostResponse,
    },
};

#[get("/")]
pub fn get_posts(state: State<ConnectionPool>) -> Json<Vec<ShortPostResponse>> {
    Json(
        wp_posts::table
        .select((wp_posts::ID, wp_posts::post_date, wp_posts::post_modified, wp_posts::post_title, wp_posts::post_name))
        .load(&state.get().unwrap())
        .unwrap()
        .iter()
        .map(ShortPostResponse::from)
        .collect()
    )
}

#[get("/<id>")]
pub fn get_full_post(state: State<ConnectionPool>, id: u32) -> Option<Json<FullPostResponse>> {
    wp_posts::table
        .select(wp_posts::all_columns)
        .filter(wp_posts::ID.eq(id))
        .first(&state.get().unwrap())
        .optional()
        .unwrap()
        .map(|post: Post| Json(FullPostResponse::from(post)))
}
