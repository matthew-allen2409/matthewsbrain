use crate::models::Comment;
use sqlx::mysql::MySqlPool;

pub async fn upload_comment(pool: &MySqlPool, comment: &Comment) {
    sqlx::query("INSERT INTO comments (post_id, email, name, comment) values (?, ?, ?, ?)")
        .bind(&comment.post_id)
        .bind(&comment.email)
        .bind(&comment.name)
        .bind(&comment.comment)
        .execute(pool)
        .await
        .expect("Cannot insert comment");
}

pub async fn get_comments_by_post_id(
    pool: MySqlPool,
    post_id: &i32,
) -> Vec<Comment> {
    sqlx::query_as::<_, Comment>("Select * from comments where post_id = ?")
        .bind(post_id)
        .fetch_all(&pool)
        .await
        .expect("Cannot fetch comments")
}
