use crate::database_service;
use crate::models::Comment;
use sqlx::mysql::MySqlPool;

pub async fn validate_comment(comment: &Comment, pool: &MySqlPool) -> Result<(), String> {
    if !database_service::posts::get_post_by_id(&pool, comment.post_id).await.is_ok() {
        return Err(format!("Post not found with id {}", comment.post_id));
    }

    if comment.name.is_empty() {
        return Err("Name is required".to_string());
    }

    if comment.comment.is_empty() {
        return Err("Comment is required".to_string());
    }

    Ok(())
}

