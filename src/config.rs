use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, dotenvy::Error> {
        // Load .env if present (ignore error if file is missing, as variables might be in system env)
        let _ = dotenvy::dotenv();

        let database_url = env::var("DATABASE_URL").unwrap();
        
        let host = env::var("HOST")
            .unwrap_or_else(|_| "0.0.0.0".to_string());

        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .unwrap_or(8080);

        Ok(Config {
            database_url,
            host,
            port,
        })
    }
}
