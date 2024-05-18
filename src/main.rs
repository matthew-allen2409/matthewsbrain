mod types;
mod database_service;
use axum::{
    http,
    routing::{get, post},
    Router,
};
use sqlx::mysql::MySqlPoolOptions;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use crate::types::types::{ CreatePostRequest, Post };
use crate::database_service::{ get_post_by_id_or_title, posts, upload_post };

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
