use axum::extract::{ Json, Path, State };
use crate::types::{ CreatePostRequest, Post };
use sqlx::mysql::MySqlPool;

pub async fn posts(State(pool): State<MySqlPool>) -> axum::Json<Vec<Post>> {
    let result = sqlx::query_as::<_, Post>("Select * from posts")
        .fetch_all(&pool)
        .await
        .expect("Cannot fetch posts");

    axum::Json(result)
}

pub async fn get_post_by_id_or_title(
    State(pool): State<MySqlPool>,
    Path(id_or_title): Path<String>,
) -> axum::Json<Post> {
    let result = match id_or_title.parse::<u32>() {
        Ok(id) => sqlx::query_as::<_, Post>("Select * from posts where post_id = ?")
            .bind(id)
            .fetch_one(&pool)
            .await
            .expect("Cannot fetch post by id"),
        Err(_) => sqlx::query_as::<_, Post>("Select * from posts where title = ?")
            .bind(id_or_title)
            .fetch_one(&pool)
            .await
            .expect("Cannot fetch post by title"),
    };

    axum::Json(result)
}

pub async fn upload_post(
    State(pool): State<MySqlPool>,
    Json(create_post_request): Json<CreatePostRequest>,
) {
    if !validate_auth_token(create_post_request.auth_token) {
        return;
    }
    let post = create_post_request.post;
    sqlx::query("INSERT INTO posts (title, content) values (?, ?)")
        .bind(post.title)
        .bind(post.content)
        .execute(&pool)
        .await
        .expect("Cannot insert post");
}

pub async fn delete_post(
    State(pool): State<MySqlPool>,
    Path(id): Path<u32>,
    Json(auth_token): Json<String>,
) {
    if !validate_auth_token(auth_token) {
        return;
    }
    sqlx::query("DELETE FROM posts WHERE post_id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .expect("Cannot delete post");
}

fn validate_auth_token(auth_token: String) -> bool {
    auth_token == "bFIooII561RZeUgntImunCTFVYirieSjmMARdDmWOSFFFAeTGFw5aAvhSKOET6h58weJ3y0O96mDKmpNDfHqinxWaUEImtLNIr5m2scQ6HdJ7NB1lzGcuLt4fOUXxa5a"
}
