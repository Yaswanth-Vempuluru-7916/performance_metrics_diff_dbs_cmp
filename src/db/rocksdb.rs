use crate::config::Config;
use crate::models::rune_pool::{Interval, Meta, RunePoolResponse};
use rocksdb::{Options, DB};
use serde::de::value;
use serde_json;
use std::error::Error;

pub struct RocksDBClient {
    db: DB,
}

impl RocksDBClient {
    pub fn new(config: &Config) -> Result<Self, Box<dyn Error>> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, &config.rocksdb_path)?; // '?' is for error propagation
        Ok(RocksDBClient { db })
    }

    pub fn update_rune_pool(&self, response: &RunePoolResponse) -> Result<(), Box<dyn Error>> {
        let meta_key = "meta".as_bytes();
        //rocksdb stores data as byte array &[u8]

        let meta_value = serde_json::to_vec(&response.meta)?;
        self.db.put(meta_key, meta_value)?;

        for (index, interval) in response.intervals.iter().enumerate() {
            let key = format!("interval_{}", index).into_bytes();
            let value = serde_json::to_vec(interval)?;
            self.db.put(&key, value)?;
        }
        Ok(())
    }

    pub fn get_rune_pool(&self) -> Result<RunePoolResponse, Box<dyn Error>> {
        let meta_key = "meta".as_bytes();
        let meta_value = self.db.get(meta_key)?.ok_or("Meta not found")?;
        let meta: Meta = serde_json::from_slice(&meta_value)?;

        let mut intervals = Vec::new();
        let mut index = 0;
        loop {
            let key = format!("interval_{}", index).into_bytes();
            match self.db.get(&key)? {
                Some(value) => {
                    let interval: Interval = serde_json::from_slice(&value)?;
                    intervals.push(interval);
                    index += 1;
                }
                None => break,
            }
        }
        Ok(RunePoolResponse { meta, intervals })
    }
}
