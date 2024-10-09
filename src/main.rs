use blog_controller::initializer;
use blog_controller::initializer::database::DBConfig;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_config = DBConfig {
        max_connections: env::var("MAX_CONNECTIONS")
            .expect("MAX_CONNECTIONS must be set")
            .parse()
            .unwrap(),
        user: env::var("DB_USER").expect("DB_USER must be set"),
        password: env::var("DB_PASSWORD").expect("DB_PASSWORD must be set"),
        host: env::var("DB_HOST").expect("DB_HOST must be set"),
        database: env::var("DB_NAME").expect("DB_NAME must be set"),
    };

    let address = env::var("ADDRESS").expect("ADDRESS must be set");
    let config = initializer::ApplicationConfig { address, db_config };

    initializer::initialize(config).await;
}
