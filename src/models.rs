use super::schema::*;
use chrono::naive::NaiveDateTime;

/// Leaving out the GMT versions in this as they tend to have bogus values
/// on unreleased posts
#[derive(Identifiable, Queryable, Debug, Serialize)]
#[table_name = "wp_posts"]
#[primary_key("ID")]
pub struct Post {
    id: u32,
    post_author: u32,
    post_date: NaiveDateTime,
    //post_date_gmt: Option<NaiveDateTime>,
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
    //post_modified_gmt: Option<NaiveDateTime>,
    post_content_filtered: String,
    post_parent: u32,
    guid: String,
    menu_order: i32,
    post_type: String,
    post_mime_type: String,
    comment_count: i32,
}

#[derive(Debug, Serialize)]
pub struct PostResponse {
    id: u32,
    date: NaiveDateTime,
    modified: NaiveDateTime,
    content: String,
    title: String,
    excerpt: String,
    name: String,
}

impl<'a> From<&'a Post> for PostResponse {
    fn from(post: &'a Post) -> PostResponse {
        PostResponse {
            id: post.id,
            date: post.post_date,
            modified: post.post_modified,
            content: post.post_content.clone(),
            title: post.post_title.clone(),
            excerpt: post.post_excerpt.clone(),
            name: post.post_name.clone(),
        }
    }
}
