use super::CommentInput;
use crate::database_service;
use crate::models::Comment;
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use sqlx::mysql::MySqlPool;
use validation::validate_comment;
mod validation;

pub async fn upload_comment(
    State(pool): State<MySqlPool>,
    Json(comment): Json<CommentInput>,
) -> Result<axum::Json<Comment>, (StatusCode, String)> {
    let comment = Comment::from(comment);
    match validate_comment(&comment, &pool).await {
        Ok(_) => (),
        Err(e) => return Err((StatusCode::BAD_REQUEST, e)),
    };

    match database_service::comments::upload_comment(&pool, &comment).await {
        Ok(_) => Ok(axum::Json(comment)),
        Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to upload comment".to_string())),
    }
}

pub async fn get_comments_by_post_id(
    State(pool): State<MySqlPool>,
    Path(post_id): Path<i32>,
) -> Result<axum::Json<Vec<Comment>>, (StatusCode, String)> {
    match database_service::comments::get_comments_by_post_id(pool, &post_id).await {
        Ok(comments) => Ok(axum::Json(comments)),
        Err(_) => Ok(axum::Json(vec![])),
    }
}
