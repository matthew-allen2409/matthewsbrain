pub mod database;
mod middleware;
pub mod router;

pub struct ApplicationConfig {
    pub address: String,
    pub db_config: database::DBConfig,
}

pub async fn initialize(config: ApplicationConfig) {
    let pool = database::initialize_database(config.db_config)
        .await
        .expect("Cannot connect to database");

    let router = router::initialize_router(pool);

    let listener = tokio::net::TcpListener::bind(&config.address)
        .await
        .expect(&format!("Cannot bind to {}", &config.address));

    println!("Listening on {}", &config.address);
    axum::serve(listener, router).await.unwrap();
}
