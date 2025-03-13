mod config;
mod db;
mod models;

use config::Config;
use db::leveldb::LevelDBClient;
use models::rune_pool::RunePoolResponse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load();
    let client = LevelDBClient::new(&config)?;

    // Simulate raw API JSON with strings
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

    // Update and retrieve
    client.update_rune_pool(&response)?;
    let retrieved = client.get_rune_pool()?;
    println!("Retrieved: {:?}", retrieved);
    Ok(())
}
