use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub ai_service_url: String,
    pub upload_dir: String,
    pub max_file_size: u64,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set"),
            ai_service_url: env::var("AI_SERVICE_URL")
                .unwrap_or_else(|_| "http://localhost:8001".to_string()),
            upload_dir: env::var("UPLOAD_DIR")
                .unwrap_or_else(|_| "./uploads".to_string()),
            max_file_size: env::var("MAX_FILE_SIZE")
                .unwrap_or_else(|_| "10485760".to_string())
                .parse()
                .expect("MAX_FILE_SIZE must be a valid number"),
        }
    }
}
