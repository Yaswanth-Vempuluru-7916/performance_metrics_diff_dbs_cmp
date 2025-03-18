use crate::config::Config;
use crate::db::leveldb::LevelDBClient;
use crate::db::mongodb::MongoDBClient;
use crate::db::psql::PsqlClient;
use crate::db::rocksdb::RocksDBClient;
use crate::db::surrealdb::SurrealDBClient;
use crate::models::rune_pool::{ApiRunePoolResponse, DbRunePoolResponse};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use reqwest::Client as HttpClient;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    config: Config,
    leveldb: Arc<LevelDBClient>,
    rocksdb: Arc<RocksDBClient>,
    surrealdb: Arc<Mutex<SurrealDBClient>>,
    psql: Arc<Mutex<PsqlClient>>,
    mongodb: Arc<Mutex<MongoDBClient>>,
    http_client: HttpClient,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Self, Box<dyn Error>> {
        let leveldb = Arc::new(LevelDBClient::new(&config)?);
        let rocksdb = Arc::new(RocksDBClient::new(&config)?);
        let surrealdb = Arc::new(Mutex::new(SurrealDBClient::new(&config).await?));
        let psql = Arc::new(Mutex::new(PsqlClient::new(&config).await?));
        let mongodb = Arc::new(Mutex::new(MongoDBClient::new(&config).await?));
        let http_client = HttpClient::new();

        Ok(AppState {
            config,
            leveldb,
            rocksdb,
            surrealdb,
            psql,
            mongodb,
            http_client,
        })
    }
}

// Handler to update RunePoolResponse across all DBs
pub async fn update_rune_pool(
    State(state): State<AppState>,
    Json(payload): Json<ApiRunePoolResponse>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let db_response: DbRunePoolResponse = payload.clone().into();

    state.leveldb.update_rune_pool(&db_response).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    state.rocksdb.update_rune_pool(&db_response).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    state.surrealdb.lock().await.update_rune_pool(&db_response).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    state.psql.lock().await.update_rune_pool(&db_response).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    state.mongodb.lock().await.update_rune_pool(&db_response).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::OK, Json(payload)))
}

// Handler to retrieve RunePoolResponse from a specified DB
pub async fn get_rune_pool(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let db = params.get("db").ok_or((
        StatusCode::BAD_REQUEST,
        "Missing 'db' query parameter".to_string(),
    ))?;

    let retrieved_api: ApiRunePoolResponse = match db.as_str() {
        "leveldb" => {
            let retrieved_db = state
                .leveldb
                .get_rune_pool()
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            retrieved_db.into()
        }
        "rocksdb" => {
            let retrieved_db = state
                .rocksdb
                .get_rune_pool()
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            retrieved_db.into()
        }
        "surrealdb" => {
            let retrieved_db = state
                .surrealdb
                .lock()
                .await
                .get_rune_pool()
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            retrieved_db.into()
        }
        "psql" => {
            let retrieved_db = state
                .psql
                .lock()
                .await
                .get_rune_pool()
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            retrieved_db.into()
        }
        "mongodb" => {
            let retrieved_db = state
                .mongodb
                .lock()
                .await
                .get_rune_pool()
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            retrieved_db.into()
        }
        _ => return Err((StatusCode::BAD_REQUEST, format!("Unknown database: {}", db))),
    };

    Ok((StatusCode::OK, Json(retrieved_api)))
}

// Handler to fetch from Midgard API and update all DBs
pub async fn fetch_and_update_rune_pool(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let url = format!(
        "{}?interval={}&from={}&count=400",
        state.config.api_url,
        state.config.interval,
        state.config.initial_from
    );

    let response = state
        .http_client
        .get(&url)
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to fetch from Midgard: {}", e)))?
        .json::<ApiRunePoolResponse>()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to parse Midgard response: {}", e)))?;

    let db_response: DbRunePoolResponse = response.clone().into();

    state.leveldb.update_rune_pool(&db_response).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    state.rocksdb.update_rune_pool(&db_response).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    state.surrealdb.lock().await.update_rune_pool(&db_response).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    state.psql.lock().await.update_rune_pool(&db_response).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    state.mongodb.lock().await.update_rune_pool(&db_response).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::OK, Json(response)))
}

// Handler to clear all databases
pub async fn clear_databases(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    state.leveldb.clear().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    state.rocksdb.clear().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    state.surrealdb.lock().await.clear().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    state.psql.lock().await.clear().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    state.mongodb.lock().await.clear().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok((StatusCode::NO_CONTENT, "".to_string())) // 204 No Content
}