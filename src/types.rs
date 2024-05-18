pub mod types {
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
}
