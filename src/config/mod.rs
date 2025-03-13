use std::env;
use chrono::{Duration,Utc};

pub struct Config{
    pub api_url : String,
    pub interval  :String,
    pub initial_from : u64,
    pub rocksdb_path : String,
    pub leveldb_path : String,
    pub surrealdb_url : String,
    pub psql_conn :String,
    pub mongodb_uri : String
}

impl Config{
    pub fn load() -> Self{
        dotenvy::dotenv().ok();

        let api_url = env::var("API_URL")
        .unwrap_or_else(|_| "https://midgard.ninerealms.com/v2/history/runepool".to_string());
    let interval = env::var("INTERVAL").unwrap_or_else(|_| "hour".to_string());
        
        // Calculate initial `from` timestamp (6 months back from now)
        let six_months_ago = Utc::now() - Duration::days(6 * 30); // Approx 6 months
        let initial_from = six_months_ago.timestamp() as u64;

        // Database settings (defaults can be overridden via .env)
        let rocksdb_path = env::var("ROCKSDB_PATH").unwrap_or_else(|_| "./data/rocksdb".to_string());
        let leveldb_path = env::var("LEVELDB_PATH").unwrap_or_else(|_| "./data/leveldb".to_string());
        let surrealdb_url = env::var("SURREALDB_URL")
            .unwrap_or_else(|_| "ws://localhost:8000".to_string());
        let psql_conn = env::var("PSQL_CONN")
            .unwrap_or_else(|_| "postgres://user:password@localhost:5432/runepool".to_string());
        let mongodb_uri = env::var("MONGODB_URI")
            .unwrap_or_else(|_| "mongodb://localhost:27017/runepool".to_string());

        Config {
            api_url,
            interval,
            initial_from,
            rocksdb_path,
            leveldb_path,
            surrealdb_url,
            psql_conn,
            mongodb_uri,
        }

    }
}