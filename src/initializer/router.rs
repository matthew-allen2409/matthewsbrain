use super::middleware;
use crate::handler;
use axum::{
    routing::{get, post},
    Router,
};
use tower::ServiceBuilder;

pub fn initialize_router(pool: sqlx::mysql::MySqlPool) -> Router {
    let cors = middleware::initialize_cors();

    Router::new()
        .route("/posts", get(handler::post::posts))
        .route("/posts/:id_or_title", get(handler::post::get_post_by_id))
        .route("/posts", post(handler::post::upload_post))
        .route("/posts/delete/:id", post(handler::post::delete_post))
        .route("/comments", post(handler::comment::upload_comment))
        .route("/comments/:post_id", get(handler::comment::get_comments_by_post_id))
        .route("/rss", get(handler::rss::handle_rss_feed))
        .layer(ServiceBuilder::new().layer(cors))
        .with_state(pool)
}
