use crate::config::Config;
use crate::models::rune_pool::{Meta, Interval, RunePoolResponse};
use leveldb::database::Database;
use leveldb::options::{Options, WriteOptions, ReadOptions};
use leveldb::kv::KV;
use serde_json;
use std::error::Error;
use std::path::Path;

pub struct LevelDBClient {
    db: Database<i32>, // Use i32 as the key type for simplicity
}

impl LevelDBClient {
    /// Initializes a new LevelDB instance with the given config.
    pub fn new(config: &Config) -> Result<Self, Box<dyn Error>> {
        let mut opts = Options::new();
        opts.create_if_missing = true; // Create the DB if it doesn’t exist
        let db = Database::open(Path::new(&config.leveldb_path), opts)?;
        Ok(LevelDBClient { db })
    }

    /// Updates the database with a RunePoolResponse.
    pub fn update_rune_pool(&self, response: &RunePoolResponse) -> Result<(), Box<dyn Error>> {
        let write_opts = WriteOptions::new();

        // Serialize and store meta
        let meta_key = 0; // Use a fixed key for meta
        let meta_value = serde_json::to_vec(&response.meta)?;
        self.db.put(write_opts, meta_key, &meta_value)?;

        // Serialize and store each interval with a unique key
        let write_opts = WriteOptions::new(); // Recreate for each call or scope
        for (index, interval) in response.intervals.iter().enumerate() {
            let key = index as i32 + 1; // Start keys from 1 to avoid clashing with meta
            let value = serde_json::to_vec(interval)?;
            self.db.put(write_opts, key, &value)?;
        }

        Ok(())
    }

    /// Retrieves the stored RunePoolResponse from the database.
    pub fn get_rune_pool(&self) -> Result<RunePoolResponse, Box<dyn Error>> {
        let read_opts = ReadOptions::new();

        // Retrieve meta
        let meta_key = 0;
        let meta_value = self.db.get(read_opts, meta_key)?.ok_or("Meta not found")?;
        let meta: Meta = serde_json::from_slice(&meta_value)?;

        // Retrieve intervals
        let mut intervals = Vec::new();
        let mut index = 1; // Start from 1 since meta uses 0
        loop {
            let read_opts = ReadOptions::new(); // Recreate for each call
            let key = index as i32;
            match self.db.get(read_opts, key)? {
                Some(value) => {
                    let interval: Interval = serde_json::from_slice(&value)?;
                    intervals.push(interval);
                    index += 1;
                }
                None => break, // Stop when no more intervals are found
            }
        }

        Ok(RunePoolResponse { meta, intervals })
    }
}