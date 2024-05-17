use axum::{
    extract::State,
}
use chrono::DateTime;
use sqlx::mysql::MySqlPool;

pub mod database_service {
    #[derive(sqlx::FromRow, serde::Serialize)]
    struct Post {
        post_id: i32,
        title: String,
        content: String,
        created_at: DateTime<chrono::Utc>,
    }

    pub async fn posts(State(pool): State<MySqlPool>) -> axum::Json<Vec<Post>> {
        let result = sqlx::query_as::<_, Post>(
            "Select * from posts"
        ).fetch_all(&pool).await.expect("Cannot fetch posts");

        axum::Json(result)
    }
}

