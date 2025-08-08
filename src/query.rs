use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use tracing::info;

use crate::storage::StorageEngine;

pub struct QueryExecutor {
    storage: StorageEngine,
}

#[derive(Debug)]
pub struct QueryResult {
    pub data: Vec<HashMap<String, Value>>,
    pub columns: Vec<String>,
}

impl QueryExecutor {
    pub fn new(storage: StorageEngine) -> Self {
        Self { storage }
    }
    
    pub async fn execute(&self, sql: &str) -> Result<QueryResult> {
        info!("Executing SQL query: {}", sql);
        
        // Execute the query
        let data = self.storage.execute_query(sql)?;
        
        // Extract column names from the first row
        let columns = if let Some(first_row) = data.first() {
            first_row.keys().cloned().collect()
        } else {
            vec![]
        };
        
        let result = QueryResult { data, columns };
        
        info!("Query executed successfully, returned {} rows", result.data.len());
        Ok(result)
    }
    
    pub fn get_schema(&self) -> Result<Vec<String>> {
        let schema = self.storage.get_schema()?;
        Ok(schema.into_iter().map(|col| col.name).collect())
    }
}
