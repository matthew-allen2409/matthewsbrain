use crate::database_service;
use crate::models::Post;
use axum::extract::{Json, Path, State};
use sqlx::mysql::MySqlPool;

#[derive(serde::Deserialize)]
pub struct CreatePostRequest {
    pub post: Post,
    pub auth_token: String,
}

pub async fn posts(State(pool): State<MySqlPool>) -> axum::Json<Vec<Post>> {
    let result = database_service::posts::posts(pool).await;

    axum::Json(result)
}

pub async fn get_post_by_id(
    State(pool): State<MySqlPool>,
    Path(post_id): Path<i32>,
) -> axum::Json<Post> {
    let result = database_service::posts::get_post_by_id(&pool, post_id).await.unwrap();

    axum::Json(result)
}

pub async fn upload_post(State(pool): State<MySqlPool>, request: Json<CreatePostRequest>) {
    if !validate_auth_token(&request.auth_token) {
        panic!("Invalid auth token");
    }

    database_service::posts::upload_post(pool, &request.post).await;
}

pub async fn delete_post(
    State(pool): State<MySqlPool>,
    Path(id): Path<u32>,
    Json(auth_token): Json<String>,
) {
    if !validate_auth_token(&auth_token) {
        panic!("Invalid auth token");
    }

    database_service::posts::delete_post(pool, id).await;
}

fn validate_auth_token(auth_token: &str) -> bool {
    auth_token == "bFIooII561RZeUgntImunCTFVYirieSjmMARdDmWOSFFFAeTGFw5aAvhSKOET6h58weJ3y0O96mDKmpNDfHqinxWaUEImtLNIr5m2scQ6HdJ7NB1lzGcuLt4fOUXxa5a"
}
