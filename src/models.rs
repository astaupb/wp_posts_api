use super::schema::*;
use chrono::naive::NaiveDateTime;

#[derive(Identifiable, Queryable, Debug, Serialize)]
#[table_name = "wp_posts"]
#[primary_key("ID")]

pub struct Post {
    id: u64,
    post_author: u64,
    post_date: NaiveDateTime,
    post_date_gmt: NaiveDateTime,
    post_content: String,
    post_title: String,
    post_excerpt: String,
    post_status: String,
    comment_status: String,
    ping_status: String,
    post_password: String,
    post_name: String,
    to_ping: String,
    pinged: String,
    post_modified: NaiveDateTime,
    post_modified_gmt: NaiveDateTime,
    post_content_filtered: String,
    post_parent: u64,
    guid: String,
    menu_order: i32,
    post_type: String,
    post_mime_type: String,
    comment_count: i64,
}
