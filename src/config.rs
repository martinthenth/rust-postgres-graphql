use std::env;
use std::io::Error;
use std::sync::OnceLock;

/// Define and implement the configuration.
pub struct Config {
    pub database_url: String,
    pub endpoint_url: String,
}

impl Config {
    /// Load the configuration from the environment variables.
    fn load_from_env() -> Result<Config, Error> {
        Ok(Config {
            database_url: env::var("DATABASE_URL").unwrap(),
            endpoint_url: env::var("ENDPOINT_URL").unwrap(),
        })
    }
}

/// Get the configuration.
pub fn get_config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env().unwrap_or_else(|e| panic!("Failed to load config: {:?}", e))
    })
}
