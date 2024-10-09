use axum::http;
use tower_http::cors::{Any, CorsLayer};

pub fn initialize_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([http::Method::GET, http::Method::POST])
        .allow_origin(Any)
        .allow_headers([http::header::CONTENT_TYPE])
}
