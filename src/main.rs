use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::Json as JsonResponse,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tower_http::cors::CorsLayer;
use tracing::{info, warn};
use hyper::Server;

mod storage;

#[derive(Clone)]
struct AppState {
    db_path: String,
}

#[derive(Debug, Deserialize)]
struct QueryRequest {
    query: String,
}

#[derive(Debug, Serialize)]
struct QueryResponse {
    success: bool,
    data: Option<Vec<HashMap<String, serde_json::Value>>>,
    error: Option<String>,
    metadata: QueryMetadata,
}

#[derive(Debug, Serialize)]
struct QueryMetadata {
    row_count: usize,
    columns: Vec<String>,
    execution_time_ms: u64,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("Starting Askadb Query Engine...");

    let db_path = "data/askadb.db".to_string();

    // Initialize DB (tables + sample data)
    if let Err(e) = storage::init_db(&db_path) {
        panic!("Failed to initialize database: {}", e);
    }

    let state = AppState { db_path };

    let app = Router::new()
        .route("/execute", post(execute_query))
        .route("/health", post(health_check))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = "0.0.0.0:8002".parse().unwrap();
    info!("Query Engine listening on http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn execute_query(
    State(state): State<AppState>,
    Json(payload): Json<QueryRequest>,
) -> Result<JsonResponse<QueryResponse>, StatusCode> {
    let start_time = std::time::Instant::now();

    info!("Executing query: {}", payload.query);

    match storage::execute_query(&state.db_path, &payload.query) {
        Ok((rows, columns)) => {
            let execution_time = start_time.elapsed().as_millis() as u64;

            let metadata = QueryMetadata {
                row_count: rows.len(),
                columns,
                execution_time_ms: execution_time,
            };

            let response = QueryResponse {
                success: true,
                data: Some(rows),
                error: None,
                metadata,
            };

            Ok(JsonResponse(response))
        }
        Err(e) => {
            warn!("Query execution failed: {}", e);

            let response = QueryResponse {
                success: false,
                data: None,
                error: Some(e.to_string()),
                metadata: QueryMetadata {
                    row_count: 0,
                    columns: vec![],
                    execution_time_ms: 0,
                },
            };

            Ok(JsonResponse(response))
        }
    }
}

async fn health_check() -> JsonResponse<HashMap<&'static str, &'static str>> {
    let mut response = HashMap::new();
    response.insert("status", "healthy");
    response.insert("service", "askadb-query-engine");
    JsonResponse(response)
}
