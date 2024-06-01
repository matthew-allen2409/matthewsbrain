#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct Post {
    #[serde(default)]
    pub post_id: Option<i32>,
    pub title: String,
    pub content: String,
    #[serde(default)]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(serde::Deserialize)]
pub struct CreatePostRequest {
    pub post: Post,
    pub auth_token: String,
}

#[derive(serde::Deserialize)]
pub struct CommentInput {
    pub post_id: i32,
    pub email: String,
    pub name: String,
    pub comment: String,
}
