mod config;
mod db;
mod models;

use config::Config;
use db::{leveldb::LevelDBClient, rocksdb::RocksDBClient, surrealdb::SurrealDBClient};
use crate::models::rune_pool::{DbRunePoolResponse, ApiRunePoolResponse};

use serde_json;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load();
    println!("LevelDB Path: {}", config.leveldb_path);
    println!("RocksDB Path: {}", config.rocksdb_path);
    println!("SurrealDB URL: {}", config.surrealdb_url);

    let json = r#"
    {
        "meta": {
            "startTime": "1728802820",
            "endTime": "1728813620",
            "startCount": "1",
            "endCount": "362",
            "startUnits": "364510161922082",
            "endUnits": "364460711492685"
        },
        "intervals": [
            {
                "startTime": "1728802820",
                "endTime": "1728806400",
                "count": "1",
                "units": "364510161922082"
            },
            {
                "startTime": "1728806420",
                "endTime": "1728810020",
                "count": "362",
                "units": "364460711492685"
            }
        ]
    }"#;

    let api_response: ApiRunePoolResponse = serde_json::from_str(json)?;
    println!("Deserialized: {:?}\n", api_response);

    let db_response: DbRunePoolResponse = api_response.clone().into();

    // Test LevelDB
    let leveldb_client = LevelDBClient::new(&config)?;
    leveldb_client.update_rune_pool(&db_response)?;
    let retrieved_db: DbRunePoolResponse = leveldb_client.get_rune_pool()?;
    let retrieved_api: ApiRunePoolResponse = retrieved_db.into();
    println!("Retrieved (LevelDB): {:?}", retrieved_api);

    // Test RocksDB
    let rocksdb_client = RocksDBClient::new(&config)?;
    rocksdb_client.update_rune_pool(&db_response)?;
    let retrieved_db: DbRunePoolResponse = rocksdb_client.get_rune_pool()?;
    let retrieved_api: ApiRunePoolResponse = retrieved_db.into();
    println!("Retrieved (RocksDB): {:?}", retrieved_api);

    let client = SurrealDBClient::new(&config).await?;
    let db_response: DbRunePoolResponse = api_response.clone().into();
    client.update_rune_pool(&db_response).await?;
    let retrieved_db: DbRunePoolResponse = client.get_rune_pool().await?;
    let retrieved_api: ApiRunePoolResponse = retrieved_db.into();
    println!("Retrieved: {:?}", retrieved_api);

    Ok(())
}


// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let config = Config::load();
//     println!("LevelDB Path: {}", config.leveldb_path);
//     println!("RocksDB Path: {}", config.rocksdb_path);

//     let json = r#"
//     {
//         "meta": {
//             "startTime": "1728802800",
//             "endTime": "1728813600",
//             "startCount": "1",
//             "endCount": "362",
//             "startUnits": "364510161922082",
//             "endUnits": "364460711492685"
//         },
//         "intervals": [
//             {
//                 "startTime": "1728802800",
//                 "endTime": "1728806400",
//                 "count": "1",
//                 "units": "364510161922082"
//             },
//             {
//                 "startTime": "1728806400",
//                 "endTime": "1728810000",
//                 "count": "362",
//                 "units": "364460711492685"
//             }
//         ]
//     }"#;

//     let api_response: ApiRunePoolResponse = serde_json::from_str(json)?;
//     println!("Deserialized: {:?}\n", api_response);

//     let db_response: DbRunePoolResponse = api_response.clone().into();

//     // Test LevelDB
//     let leveldb_client = LevelDBClient::new(&config)?;
//     leveldb_client.update_rune_pool(&db_response)?;
//     let retrieved_db: DbRunePoolResponse = leveldb_client.get_rune_pool()?;
//     let retrieved_api: ApiRunePoolResponse = retrieved_db.into();
//     println!("Retrieved (LevelDB): {:?}", retrieved_api);

//     // Test RocksDB
//     let rocksdb_client = RocksDBClient::new(&config)?;
//     rocksdb_client.update_rune_pool(&db_response)?;
//     let retrieved_db: DbRunePoolResponse = rocksdb_client.get_rune_pool()?;
//     let retrieved_api: ApiRunePoolResponse = retrieved_db.into();
//     println!("Retrieved (RocksDB): {:?}", retrieved_api);

//     Ok(())
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let config = Config::load();
//     println!("Config URL: {}", config.surrealdb_url);

//     let json = r#"
//     {
//         "meta": {
//             "startTime": "1728802800",
//             "endTime": "1728813600",
//             "startCount": "1",
//             "endCount": "362",
//             "startUnits": "364510161922082",
//             "endUnits": "364460711492685"
//         },
//         "intervals": [
//             {
//                 "startTime": "1728802800",
//                 "endTime": "1728806400",
//                 "count": "1",
//                 "units": "364510161922082"
//             },
//             {
//                 "startTime": "1728806400",
//                 "endTime": "1728810000",
//                 "count": "362",
//                 "units": "364460711492685"
//             }
//         ]
//     }"#;

//     let api_response: ApiRunePoolResponse = serde_json::from_str(json)?;
//     println!("Deserialized: {:?}\n", api_response);

//     let client = SurrealDBClient::new(&config).await?;
//     let db_response: DbRunePoolResponse = api_response.clone().into();
//     client.update_rune_pool(&db_response).await?;
//     let retrieved_db: DbRunePoolResponse = client.get_rune_pool().await?;
//     let retrieved_api: ApiRunePoolResponse = retrieved_db.into();
//     println!("Retrieved: {:?}", retrieved_api);

//     Ok(())
// }