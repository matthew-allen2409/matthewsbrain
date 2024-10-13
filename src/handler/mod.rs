pub mod post;
pub mod comment;
pub mod rss;

#[derive(serde::Deserialize, Debug)]
pub struct CommentInput {
    pub post_id: i32,
    pub email: String,
    pub name: String,
    pub comment: String,
}
