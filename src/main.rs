mod database_service;
mod types;
use axum::{
    http,
    routing::{get, post},
    Router,
};
use sqlx::mysql::MySqlPoolOptions;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

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
        .route("/posts", get(database_service::posts))
        .route("/posts/:id_or_title", get(database_service::get_post_by_id_or_title))
        .route("/posts", post(database_service::upload_post))
        .route("/posts/delete/:id", post(database_service::delete_post))
        .route("/comments", post(database_service::upload_comment))
        .layer(ServiceBuilder::new().layer(cors))
        .with_state(pool);

    let address = "localhost:8080";
    println!("Listening on {}", address);
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
