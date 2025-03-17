use crate::config::Config;
use crate::models::rune_pool::{Interval, Meta, RunePoolResponse};
use serde_json;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use std::error::Error;
use std::sync::Arc;

pub struct SurrealDBClient {
    db: Arc<Surreal<surrealdb::engine::remote::ws::Client>>,
}

impl SurrealDBClient {
    /// Initializes a new SurrealDB instance with the given config.
    pub async fn new(config: &Config) -> Result<Self, Box<dyn Error>> {
        // Connect to the database
        let db = Surreal::new::<Ws>(config.surrealdb_url.clone()).await?;
        
        // Sign in as root (or use namespace/database)
        db.use_ns("runepool").use_db("runepool").await?;
        
        // Create tables if they don't exist
        let _ = db.query("DEFINE TABLE meta SCHEMAFULL").await?;
        let _ = db.query("DEFINE TABLE intervals SCHEMAFULL").await?;
        
        Ok(SurrealDBClient { 
            db: Arc::new(db)
        })
    }

    /// Updates the database with a RunePoolResponse.
    pub async fn update_rune_pool(&self, response: &RunePoolResponse) -> Result<(), Box<dyn Error>> {
        // First, clear existing data
        let _ = self.db.query("DELETE FROM meta").await?;
        let _ = self.db.query("DELETE FROM intervals").await?;
        
        // Insert meta data with specific ID - Clone to own the data
        let meta_id = "meta:main";
        let meta_clone = response.meta.clone();
        let _: Option<Meta> = self.db
            .create(("meta", "main"))
            .content(meta_clone)
            .await?;
        
        // Insert interval data
        for (index, interval) in response.intervals.iter().enumerate() {
            // Create interval data as owned values
            let interval_data = serde_json::json!({
                "index": index,
                "meta_id": meta_id,
                "start_time": interval.start_time,
                "end_time": interval.end_time,
                "count": interval.count,
                "units": interval.units
            });
            
            let _: Option<serde_json::Value> = self.db
                .create("intervals")
                .content(interval_data)
                .await?;
        }
        
        Ok(())
    }

    /// Retrieves the stored RunePoolResponse from the database.
    pub async fn get_rune_pool(&self) -> Result<RunePoolResponse, Box<dyn Error>> {
        // Get meta data
        let mut result = self.db
            .query("SELECT * FROM meta LIMIT 1")
            .await?;
            
        let meta_vec: Vec<Meta> = result.take(0)?;
        let meta = meta_vec.get(0).ok_or("Meta not found")?.clone();
        
        // Get intervals ordered by their index
        let mut result = self.db
            .query("SELECT start_time, end_time, count, units FROM intervals ORDER BY index")
            .await?;
            
        let intervals: Vec<Interval> = result.take(0)?;
        
        Ok(RunePoolResponse { meta, intervals })
    }
    
    /// Benchmark-optimized method to retrieve only meta
    pub async fn get_meta(&self) -> Result<Meta, Box<dyn Error>> {
        let mut result = self.db
            .query("SELECT * FROM meta LIMIT 1")
            .await?;
            
        let meta_vec: Vec<Meta> = result.take(0)?;
        meta_vec.get(0).cloned().ok_or_else(|| "Meta not found".into())
    }
    
    /// Benchmark-optimized method to retrieve intervals count
    pub async fn get_intervals_count(&self) -> Result<usize, Box<dyn Error>> {
        let mut result = self.db
            .query("SELECT count() as count FROM intervals")
            .await?;
            
        #[derive(serde::Deserialize)]
        struct CountResult {
            count: i64,
        }
        
        let counts: Vec<CountResult> = result.take(0)?;
        let count = counts.get(0).map(|c| c.count).unwrap_or(0);
        
        Ok(count as usize)
    }
}