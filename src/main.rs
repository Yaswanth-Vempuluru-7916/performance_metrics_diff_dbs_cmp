mod config;
mod db;
mod models;

use tokio::runtime::Runtime;
use config::Config;
use db::leveldb::LevelDBClient;
use models::rune_pool::RunePoolResponse;
use db::surrealdb::SurrealDBClient;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use std::error::Error;
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let config = Config::load();
//     // let client = LevelDBClient::new(&config)?;
//     let client = SurrealDBClient::new(&config).await?;

//     // Simulate raw API JSON with strings
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

//     // Deserialize JSON into RunePoolResponse (strings -> u64)
//     let response: RunePoolResponse = serde_json::from_str(json)?;
//     println!("Deserialized: {:?}\n", response);

//     // Update and retrieve
//     client.update_rune_pool(&response).await?;
//     let retrieved = client.get_rune_pool().await?;
//     println!("Retrieved: {:?}", retrieved);
//     Ok(())
// }


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create tokio runtime for async operations
    let rt = Runtime::new()?;
    
    // Run the async code using the runtime
    rt.block_on(async {
        let config = Config::load();
        let client = SurrealDBClient::new(&config).await?;

        // Simulate raw API JSON with strings (same sample data as original)
        let json = r#"
        {
            "meta": {
                "startTime": "1728802800",
                "endTime": "1728813600",
                "startCount": "1",
                "endCount": "362",
                "startUnits": "364510161922082",
                "endUnits": "364460711492685"
            },
            "intervals": [
                {
                    "startTime": "1728802800",
                    "endTime": "1728806400",
                    "count": "1",
                    "units": "364510161922082"
                },
                {
                    "startTime": "1728806400",
                    "endTime": "1728810000",
                    "count": "362",
                    "units": "364460711492685"
                }
            ]
        }"#;

        // Deserialize JSON into RunePoolResponse (strings -> u64)
        let response: RunePoolResponse = serde_json::from_str(json)?;
        println!("Deserialized: {:?}\n", response);

        // Update SurrealDB with the data
        client.update_rune_pool(&response).await?;
        println!("Data successfully stored in SurrealDB");
        
        // Retrieve the data from SurrealDB
        let retrieved = client.get_rune_pool().await?;
        println!("Retrieved from SurrealDB: {:?}", retrieved);
        
        // Show additional metrics for benchmarking purposes
        let meta = client.get_meta().await?;
        let intervals_count = client.get_intervals_count().await?;
        println!("\nPerformance metrics:");
        println!("- Meta data retrieved: {:?}", meta);
        println!("- Number of intervals: {}", intervals_count);
        
        Ok::<(), Box<dyn std::error::Error>>(())
    })?;
    
    Ok(())
}