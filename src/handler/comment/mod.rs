use super::CommentInput;
use crate::database_service;
use crate::models::Comment;
use axum::extract::{Json, Path, State};
use sqlx::mysql::MySqlPool;
use validation::validate_comment;
mod validation;

pub async fn upload_comment(
    State(pool): State<MySqlPool>,
    Json(comment): Json<CommentInput>,
) -> axum::Json<Comment> {
    let comment = Comment::from(comment);
    validate_comment(&comment, &pool).await.unwrap();

    database_service::comments::upload_comment(&pool, &comment).await;
    axum::Json(comment)
}

pub async fn get_comments_by_post_id(
    State(pool): State<MySqlPool>,
    Path(post_id): Path<i32>,
) -> axum::Json<Vec<Comment>> {
    let comments = database_service::comments::get_comments_by_post_id(pool, &post_id).await;

    axum::Json(comments)
}
