use crate::models::Comment;
use sqlx::mysql::{MySqlPool, MySqlQueryResult};

pub async fn upload_comment(pool: &MySqlPool, comment: &Comment) -> Result<MySqlQueryResult, sqlx::Error> {
    sqlx::query("INSERT INTO comments (post_id, email, name, comment) values (?, ?, ?, ?)")
        .bind(&comment.post_id)
        .bind(&comment.email)
        .bind(&comment.name)
        .bind(&comment.comment)
        .execute(pool)
        .await
}

pub async fn get_comments_by_post_id(
    pool: MySqlPool,
    post_id: &i32,
) -> Result<Vec<Comment>, sqlx::Error> {
    sqlx::query_as::<_, Comment>("Select * from comments where post_id = ?")
        .bind(post_id)
        .fetch_all(&pool)
        .await
}
