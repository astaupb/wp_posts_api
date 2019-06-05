use diesel::prelude::*;
use rocket::State;
use rocket_contrib::json::Json;
use crate::{
    pool::ConnectionPool,
    schema::wp_posts,
    models::{
        Post, PostResponse, 
    },
};

#[get("/")]
pub fn get_posts(state: State<ConnectionPool>) -> Json<Vec<PostResponse>> {
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
pub fn get_post(state: State<ConnectionPool>, post_id: u32) -> Option<Json<PostResponse>> {
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
