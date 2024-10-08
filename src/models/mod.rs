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
    pub post_id: i32,
    pub email: String,
    pub name: String,
    pub comment: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
