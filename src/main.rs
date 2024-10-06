use blog_controller::initializer;
use blog_controller::initializer::DBConfig;

#[tokio::main]
async fn main() {
    let config = DBConfig {
        max_connections: 5,
        user: String::from("app"),
        password: String::from("xauY5.B;<E*X~+y%"),
        host: String::from("localhost"),
        database: String::from("blog"),
    };
    let router = initializer::initialize_router(config).await;

    let host = "localhost";
    let port = "8080";
    let address = format!("{host}:{port}");
    println!("Listening on {address}");
    
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
