use crate::database_service;
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

    let cors = CorsLayer::new()
        .allow_methods([http::Method::GET, http::Method::POST])
        .allow_origin(Any)
        .allow_headers([http::header::CONTENT_TYPE]);

    Router::new()
        .route("/analytics:post_id", post(database_service::increment_view_count))
        .route("/posts", get(database_service::posts))
        .route("/posts/:id_or_title", get(database_service::get_post_by_id_or_title))
        .route("/posts", post(database_service::upload_post))
        .route("/posts/delete/:id", post(database_service::delete_post))
        .route("/comments", post(database_service::upload_comment))
        .route("/comments/:post_id", get(database_service::get_comments_by_post_id))
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

