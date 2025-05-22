use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn socket_addr(&self) -> SocketAddr {
        format!("{}:{}", self.host, self.port)
            .parse()
            .expect("Invalid server address")
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let _env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".into());
        
        let config = config::Config::builder()
            // Start with default settings
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 3000)?
            .set_default("database.url", "postgres://postgres:postgres@localhost:5432/rprt_db")?
            .set_default("database.max_connections", 5)?
            // Add in settings from environment variables (with a prefix of APP and '__' as separator)
            // E.g. `APP_SERVER__PORT=5000` would set `server.port`
            .add_source(
                config::Environment::with_prefix("APP")
                    .prefix_separator("_")
                    .separator("__")
                    .try_parsing(true),
            )
            .build()?;

        config.try_deserialize()
    }
}
