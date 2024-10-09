use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

pub struct DBConfig {
    pub max_connections: u32,
    pub user: String,
    pub password: String,
    pub host: String,
    pub database: String,
}

pub async fn initialize_database(config: DBConfig) -> Result<MySqlPool, sqlx::Error> {
    let pool = MySqlPoolOptions::new()
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

