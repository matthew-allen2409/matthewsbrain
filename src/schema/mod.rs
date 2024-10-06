use crate::models;

#[derive(serde::Deserialize)]
pub struct CreatePostRequest {
    pub post: models::Post,
    pub auth_token: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct CommentInput {
    pub post_id: i32,
    pub email: String,
    pub name: String,
    pub comment: String,
}
