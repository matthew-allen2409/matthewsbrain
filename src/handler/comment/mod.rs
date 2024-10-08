use crate::database_service;
use crate::models::Comment;
use axum::extract::{Json, State, Path};
use sqlx::mysql::MySqlPool;

#[derive(serde::Deserialize, Debug)]
pub struct CommentInput {
    pub post_id: i32,
    pub email: String,
    pub name: String,
    pub comment: String,
}

impl Comment {
    fn from(input: CommentInput) -> Comment {
        Comment {
            post_id: input.post_id,
            email: input.email,
            name: input.name,
            comment: input.comment,
            created_at: chrono::Utc::now(),
        }
    }
}

pub async fn upload_comment(
    State(pool): State<MySqlPool>,
    Json(comment): Json<CommentInput>,
) -> axum::Json<Comment> {
    let comment = Comment::from(comment);
    database_service::comments::upload_comment(pool, &comment).await;

    axum::Json(comment)
}

pub async fn get_comments_by_post_id(
    State(pool): State<MySqlPool>,
    Path(post_id): Path<i32>,
) -> axum::Json<Vec<Comment>> {
    let comments = database_service::comments::get_comments_by_post_id(pool, &post_id).await;

    axum::Json(comments)
}
