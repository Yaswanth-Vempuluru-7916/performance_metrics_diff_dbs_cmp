mod models;
mod config;
mod db;
use config::Config;
use db::rocksdb::RocksDBClient;
use models::rune_pool::{Interval, Meta, RunePoolResponse};

fn main()-> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load();

    let client = RocksDBClient::new(&config)?;

    let response = RunePoolResponse {
        meta: Meta {
            start_time: 1728802800,
            end_time: 1728813600,
            start_count: 1,
            end_count: 362,
            start_units: 364510161922082,
            end_units: 364460711492685,
        },
        intervals: vec![
            Interval {
                start_time: 1728802800,
                end_time: 1728806400,
                count: 1,
                units: 364510161922082,
            },
            Interval {
                start_time: 1728806400,
                end_time: 1728810000,
                count: 362,
                units: 364460711492685,
            },
        ],
    };

    client.update_rune_pool(&response)?;
    let retrieved = client.get_rune_pool()?;
    println!("Retrieved: {:?}", retrieved);
    Ok(())
    
}