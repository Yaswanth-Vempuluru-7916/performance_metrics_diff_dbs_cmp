### `Performance Metrics Across Different Databases`
```markdown

This project compares the performance of five databases—`LevelDB`, `RocksDB`, `SurrealDB`, `PostgreSQL`, and `MongoDB`—for storing and retrieving `RunePoolResponse` data. It uses an Axum-based web server to expose endpoints for updating, fetching, retrieving, and clearing data across all databases simultaneously. The goal is to measure write, read, and clear operation times to determine which database is most efficient for this workload.

## Features
- **Endpoints**: RESTful API for updating (`/update`), fetching and updating (`/fetch-and-update`), retrieving (`/get`), and clearing (`/clear`) data.
- **Databases**: Embedded (`LevelDB`, `RocksDB`) and networked (`SurrealDB`, `PostgreSQL`, `MongoDB`).
- **Performance Metrics**: Timings (in milliseconds) are returned in API responses for each database operation.


## Setup

### Prerequisites
- **Rust**: Install via `rustup` (`rustup install stable`).
- **Docker**: For running `PostgreSQL` and `MongoDB`.
- **SurrealDB**: Install the CLI (`cargo install surrealdb` or download from [SurrealDB](https://surrealdb.com)).

### Dependencies
Add to `Cargo.toml`:
```toml
[dependencies]
axum = "0.8.1"
chrono = "0.4.40"
dotenvy = "0.15.7"
futures-util = "0.3.31"
leveldb = "0.8.6"
mongodb = "3.2.2"
reqwest = { version = "0.12.14", features = ["json"] }
rocksdb = {version = "0.23.0", default-features = false}
serde = { version = "1.0.219", features = ["derive"] }
serde_json = "1.0.140"
serde_with = "3.12.0"
sqlx = { version = "0.8.3", features = ["runtime-tokio-rustls", "postgres","derive"] }
surrealdb = "2.2.1"
tokio = { version = "1.44.0", features = ["full"] }
tokio-postgres = "0.7.13"
tokio-tungstenite = "0.26.2"

```

### Running Locally
1. **Start Databases**:
   ```bash
   surreal start --user root --pass root 
   ```

2. **Set Environment Variables**:
   ```bash
   set API_URL=https://midgard.ninerealms.com/v2/history/runepool
   set INTERVAL=hour
   set ROCKSDB_PATH=./my_rocksdb
   set LEVELDB_PATH=./data/leveldb
   set SURREALDB_URL=127.0.0.1:8000
   set PSQL_CONN=postgres://user:password@localhost:5432/runepool
   set MONGODB_URI=mongodb://localhost:27017/runepool
   set DB_NAME=runepool
   set HOST=0.0.0.0
   set PORT=3000

   mkdir my_rocksdb
   mkdir data\leveldb
   ```

3. **Run the Server**:
   ```bash
   cargo run
   ```



##  API Endpoints

- **Clear Databases**:
  - **Method**: `DELETE`
  - **URL**: `http://localhost:3000/clear`

- **Fetch and Update (Bulk Write)**:
  - **Method**: `POST`
  - **URL**: `http://localhost:3000/fetch-and-update`

- **Update (Small Write)**:
  - **Method**: `POST`
  - **URL**: `http://localhost:3000/update`
  - **Example**:
    ```bash
    curl -X POST https://your-app.com/update -H "Content-Type: application/json" -d '{"meta":{"startTime":"1728802800","endTime":"1728813600","startCount":"1","endCount":"362","startUnits":"364510161922082","endUnits":"364460711492685"},"intervals":[{"startTime":"1728802800","endTime":"1728806400","count":"1","units":"364510161922082"},{"startTime":"1728806400","endTime":"1728810000","count":"362","units":"364460711492685"}]}'
    ```

- **Get Data**:
  - **Method**: `GET`
  - **URL**: `http://localhost:3000/get?db=<database>`
  - **Databases**: `leveldb`, `rocksdb`, `surrealdb`, `psql`, `mongodb`
  



## Performance Metrics
Below are the measured timings (in milliseconds) for key operations across the databases. The bulk write data is from a `/fetch-and-update` call with 400 intervals. Other values are placeholders to be updated later.

| Operation         | LevelDB | RocksDB | SurrealDB | PostgreSQL | MongoDB |
|-------------------|---------|---------|-----------|------------|---------|
| Small Write (ms)  |   0      |    0     |   163        |   9         |    318     |
| Bulk Write (ms)   | 9       | 25      | 6168      | 562        | 2128    |
| Read (ms)         |   1      |     2    |     66      |     16       |   2546      |
| Clear (ms)        |  3       |     16    |      46     |    136        |     467    |

### Observations
- **LevelDB**: Fastest for bulk writes (9 ms), leveraging its embedded, lightweight design.
- **RocksDB**: Close second (25 ms), with potential for better scaling.
- **SurrealDB**: Slowest (6168 ms), likely due to network latency and per-interval queries.
- **PostgreSQL**: Moderate (562 ms), balancing relational features with performance.
- **MongoDB**: Slower than expected (2128 ms).


```
