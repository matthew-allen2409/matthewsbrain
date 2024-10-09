use blog_controller::initializer;
use blog_controller::initializer::database::DBConfig;

#[tokio::main]
async fn main() {
    let db_config = DBConfig {
        max_connections: 50,
        user: String::from("app"),
        password: String::from("xauY5.B;<E*X~+y%"),
        host: String::from("localhost"),
        database: String::from("blog"),
    };

    let host = "localhost";
    let port = "8080";
    let address = format!("{host}:{port}");
    let config = initializer::ApplicationConfig { address, db_config };

    initializer::initialize(config).await;
}
