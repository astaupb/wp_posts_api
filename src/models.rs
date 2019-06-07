use super::schema::*;
use chrono::naive::NaiveDateTime;

/// Leaving out the GMT versions in this as they tend to have bogus values
/// on unreleased posts
#[derive(Identifiable, Queryable, Debug, Serialize)]
#[table_name = "wp_posts"]
#[primary_key("ID")]
pub struct Post {
    pub id: u32,
    post_author: u32,
    pub post_date: NaiveDateTime,
    //post_date_gmt: Option<NaiveDateTime>,
    pub post_content: String,
    pub post_title: String,
    post_excerpt: String,
    post_status: String,
    comment_status: String,
    ping_status: String,
    post_password: String,
    pub post_name: String,
    to_ping: String,
    pinged: String,
    pub post_modified: NaiveDateTime,
    //post_modified_gmt: Option<NaiveDateTime>,
    post_content_filtered: String,
    post_parent: u32,
    guid: String,
    menu_order: i32,
    post_type: String,
    post_mime_type: String,
    comment_count: i32,
}
