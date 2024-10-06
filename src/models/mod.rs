use crate::schema::CommentInput;

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct Post {
    #[serde(default)]
    pub post_id: Option<i32>,
    pub title: String,
    pub content: String,
    #[serde(default)]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Comment {
    pub name: String,
    pub comment: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Comment {
    pub fn from(input: CommentInput) -> Self {
        Self {
            name: input.name,
            comment: input.comment,
            created_at: chrono::Utc::now(),
        }
    }
}
