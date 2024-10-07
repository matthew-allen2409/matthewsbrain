use crate::{models::Comment, schema::CommentInput};
use axum::extract::{Json, Path, State};
use sqlx::mysql::MySqlPool;

pub async fn upload_comment(
    State(pool): State<MySqlPool>,
    Json(comment): Json<CommentInput>,
) -> axum::Json<Comment> {
    sqlx::query("INSERT INTO comments (post_id, email, name, comment) values (?, ?, ?, ?)")
        .bind(&comment.post_id)
        .bind(&comment.email)
        .bind(&comment.name)
        .bind(&comment.comment)
        .execute(&pool)
        .await
        .expect("Cannot insert comment");

    axum::Json(Comment::from(comment))
}

pub async fn get_comments_by_post_id(
    State(pool): State<MySqlPool>,
    Path(post_id): Path<u32>,
) -> axum::Json<Vec<Comment>> {
    let result = sqlx::query_as::<_, Comment>("Select * from comments where post_id = ?")
        .bind(post_id)
        .fetch_all(&pool)
        .await
        .expect("Cannot fetch comments");

    axum::Json(result)
}
