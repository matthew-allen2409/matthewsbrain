use axum::{
    extract,
    extract::{Path, State},
    http,
    routing::{get, post},
    Router,
};
use chrono::DateTime;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize)]
struct Post {
    #[serde(default)]
    post_id: Option<i32>,
    title: String,
    content: String,
    #[serde(default)]
    created_at: Option<DateTime<chrono::Utc>>,
}

#[derive(serde::Deserialize)]
struct CreatePostRequest {
    post: Post,
    auth_token: String,
}

#[tokio::main]
async fn main() {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://app:xauY5.B;<E*X~+y%@localhost/blog")
        .await
        .expect("Cannot connect to database");

    let cors = CorsLayer::new()
        .allow_methods([http::Method::GET])
        .allow_origin(Any);

    let app = Router::new()
        .route("/posts", get(posts))
        .route("/posts/:id_or_title", get(get_post_by_id_or_title))
        .route("/posts", post(upload_post))
        .layer(ServiceBuilder::new().layer(cors))
        .with_state(pool);

    let address = "localhost:8080";
    println!("Listening on {}", address);
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn posts(State(pool): State<MySqlPool>) -> axum::Json<Vec<Post>> {
    let result = sqlx::query_as::<_, Post>("Select * from posts")
        .fetch_all(&pool)
        .await
        .expect("Cannot fetch posts");

    axum::Json(result)
}

async fn get_post_by_id_or_title(
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

async fn upload_post(
    State(pool): State<MySqlPool>,
    extract::Json(create_post_request): extract::Json<CreatePostRequest>,
) {
    if !validate_auth_token(create_post_request.auth_token) {
        return;
    }
    let post = create_post_request.post;
    sqlx::query_as::<_, Post>("INSERT INTO posts (title, content) values (?, ?)")
        .bind(post.title)
        .bind(post.content)
        .fetch_one(&pool)
        .await
        .expect("Cannot insert post");
}

fn validate_auth_token(auth_token: String) -> bool {
    auth_token == "bFIooII561RZeUgntImunCTFVYirieSjmMARdDmWOSFFFAeTGFw5aAvhSKOET6h58weJ3y0O96mDKmpNDfHqinxWaUEImtLNIr5m2scQ6HdJ7NB1lzGcuLt4fOUXxa5a"
}
