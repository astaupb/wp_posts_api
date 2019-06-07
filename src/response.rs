use chrono::NaiveDateTime;
use models::Post;

#[derive(Debug, Serialize)]
pub struct FullPostResponse {
    id: u32,
    date: NaiveDateTime,
    modified: NaiveDateTime,
    content: String,
    title: String,
    name: String,
}

impl From<Post> for FullPostResponse
{
    fn from(post: Post) -> FullPostResponse
    {
        FullPostResponse {
            id: post.id,
            date: post.post_date,
            modified: post.post_modified,
            content: post.post_content,
            title: post.post_title,
            name: post.post_name,
        } 
    }
}

#[derive(Debug, Serialize)]
pub struct ShortPostResponse {
    id: u32,
    date: NaiveDateTime,
    modified: NaiveDateTime,
    title: String,
    name: String,
}

impl<'a> From<&'a (u32, NaiveDateTime, NaiveDateTime, String, String)> for ShortPostResponse
{
    fn from(post: &(u32, NaiveDateTime, NaiveDateTime, String, String)) -> ShortPostResponse
    {
        ShortPostResponse {
            id: post.0,
            date: post.1,
            modified: post.2.clone(),
            title: post.3.clone(),
            name: post.4.clone(),
        } 
    }
}
