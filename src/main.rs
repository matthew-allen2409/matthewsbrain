use blog_controller::initializer;
use blog_controller::initializer::DBConfig;

#[tokio::main]
async fn main() {
    let config = DBConfig {
        max_connections: 50,
        user: String::from("app"),
        password: String::from("xauY5.B;<E*X~+y%"),
        host: String::from("localhost"),
        database: String::from("blog"),
    };
    let router = initializer::initialize_router(config).await;

    let host = "localhost";
    let port = "8080";
    let address = format!("{host}:{port}");
    let listener = initializer::initialize_listener(address).await;
    axum::serve(listener, router).await.unwrap();
}
