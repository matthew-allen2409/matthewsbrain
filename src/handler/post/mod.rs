use crate::database_service;
use crate::models::Post;
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use sqlx::mysql::MySqlPool;

#[derive(serde::Deserialize)]
pub struct CreatePostRequest {
    pub post: Post,
    pub auth_token: String,
}

pub async fn posts(
    State(pool): State<MySqlPool>,
) -> Result<axum::Json<Vec<Post>>, (StatusCode, String)> {
    let result = database_service::posts::posts(pool).await;
    match result {
        Ok(posts) => Ok(axum::Json(posts)),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch posts".to_string())),
    }
}

pub async fn get_post_by_id(
    State(pool): State<MySqlPool>,
    Path(post_id): Path<i32>,
) -> Result<axum::Json<Post>, (StatusCode, String)> {
    match database_service::posts::get_post_by_id(&pool, post_id).await {
        Ok(post) => Ok(axum::Json(post)),
        Err(_) => Err((StatusCode::NOT_FOUND, format!("Failed to fetch post with id {}", post_id))),
    }
}

pub async fn upload_post(
    State(pool): State<MySqlPool>,
    request: Json<CreatePostRequest>,
) -> Result<(), (StatusCode, String)> {
    if !validate_auth_token(&request.auth_token) {
        return Err((StatusCode::OK, "Invalid auth token".to_string()));
    }

    match database_service::posts::upload_post(pool, &request.post).await {
        Ok(_) => Ok(()),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to upload post".to_string())),
    }
}

pub async fn delete_post(
    State(pool): State<MySqlPool>,
    Path(id): Path<u32>,
    Json(auth_token): Json<String>,
) -> Result<(), (StatusCode, String)> {
    if !validate_auth_token(&auth_token) {
        return Err((StatusCode::UNAUTHORIZED, "Invalid auth token".to_string()));
    }

    match database_service::posts::delete_post(pool, id).await {
        Ok(_) => Ok(()),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to delete post with id {}", id))),
    }
}

fn validate_auth_token(auth_token: &str) -> bool {
    auth_token == "bFIooII561RZeUgntImunCTFVYirieSjmMARdDmWOSFFFAeTGFw5aAvhSKOET6h58weJ3y0O96mDKmpNDfHqinxWaUEImtLNIr5m2scQ6HdJ7NB1lzGcuLt4fOUXxa5a"
}
