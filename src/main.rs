use axum::{
    extract::Json,
    http::StatusCode,
    response::Json as JsonResponse,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tower_http::cors::CorsLayer;
use tracing::{info, warn};

mod storage;
mod query;

use storage::StorageEngine;
use query::{QueryExecutor, QueryResult};

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
    // Initialize logging
    tracing_subscriber::fmt::init();
    info!("Starting Askadb Query Engine...");

    // Initialize storage engine
    let storage = StorageEngine::new("data/askadb.db").await;
    
    // Initialize query executor
    let query_executor = QueryExecutor::new(storage);

    // Create router
    let app = Router::new()
        .route("/execute", post(execute_query))
        .route("/health", post(health_check))
        .layer(CorsLayer::permissive())
        .with_state(query_executor);

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8002").await.unwrap();
    info!("Query Engine listening on http://0.0.0.0:8002");
    
    axum::serve(listener, app).await.unwrap();
}

async fn execute_query(
    Json(payload): Json<QueryRequest>,
    axum::extract::State(query_executor): axum::extract::State<QueryExecutor>,
) -> Result<JsonResponse<QueryResponse>, StatusCode> {
    let start_time = std::time::Instant::now();
    
    info!("Executing query: {}", payload.query);
    
    match query_executor.execute(&payload.query).await {
        Ok(result) => {
            let execution_time = start_time.elapsed().as_millis() as u64;
            
            let metadata = QueryMetadata {
                row_count: result.data.len(),
                columns: result.columns,
                execution_time_ms: execution_time,
            };
            
            let response = QueryResponse {
                success: true,
                data: Some(result.data),
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
