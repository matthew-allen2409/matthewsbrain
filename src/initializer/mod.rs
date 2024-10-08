use crate::handler;
use axum::{
    http,
    routing::{get, post},
    Router,
};
use sqlx::mysql;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

pub struct DBConfig {
    pub max_connections: u32,
    pub user: String,
    pub password: String,
    pub host: String,
    pub database: String,
}

pub async fn initialize_router(config: DBConfig) -> Router {
    let pool = initialize_database(config)
        .await
        .expect("Cannot connect to database");

    let cors = initialize_cors();

    Router::new()
        .route("/posts", get(handler::post::posts))
        .route("/posts/:id_or_title", get(handler::post::get_post_by_id))
        .route("/posts", post(handler::post::upload_post))
        .route("/posts/delete/:id", post(handler::post::delete_post))
        .route("/comments", post(handler::comment::upload_comment))
        .route("/comments/:post_id", get(handler::comment::get_comments_by_post_id))
        .layer(ServiceBuilder::new().layer(cors))
        .with_state(pool)
}

async fn initialize_database(config: DBConfig) -> Result<sqlx::Pool<sqlx::MySql>, sqlx::Error> {
    let pool = mysql::MySqlPoolOptions::new()
        .max_connections(config.max_connections)
        .connect(format!(
            "mysql://{}:{}@{}/{}",
            config.user,
            config.password,
            config.host,
            config.database
        ).as_str())
        .await;
    
    pool
}

fn initialize_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([http::Method::GET, http::Method::POST])
        .allow_origin(Any)
        .allow_headers([http::header::CONTENT_TYPE])
}

pub async fn initialize_listener(address: String) -> tokio::net::TcpListener {
    println!("Listening on {address}");

    tokio::net::TcpListener::bind(&address).await
        .expect(&format!("Cannot bind to {address}"))
}

