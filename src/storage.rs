use anyhow::Result;
use rusqlite::{Connection, params};
use serde_json::Value;
use std::collections::HashMap;
use tracing::info;

pub struct StorageEngine {
    conn: Connection,
}

impl StorageEngine {
    pub async fn new(db_path: &str) -> Self {
        info!("Initializing storage engine at: {}", db_path);
        
        // Ensure data directory exists
        if let Some(parent) = std::path::Path::new(db_path).parent() {
            std::fs::create_dir_all(parent).ok();
        }
        
        let conn = Connection::open(db_path).expect("Failed to open database");
        
        // Initialize tables
        Self::init_tables(&conn).expect("Failed to initialize tables");
        
        // Load sample data if tables are empty
        Self::load_sample_data(&conn).expect("Failed to load sample data");
        
        Self { conn }
    }
    
    fn init_tables(conn: &Connection) -> Result<()> {
        // Create sales table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS sales (
                id INTEGER PRIMARY KEY,
                region TEXT NOT NULL,
                product TEXT NOT NULL,
                month TEXT NOT NULL,
                sales_amount REAL NOT NULL,
                quantity INTEGER NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;
        
        info!("Tables initialized successfully");
        Ok(())
    }
    
    fn load_sample_data(conn: &Connection) -> Result<()> {
        // Check if data already exists
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM sales", [], |row| row.get(0))?;
        
        if count > 0 {
            info!("Sample data already loaded, skipping...");
            return Ok(());
        }
        
        // Insert sample sales data
        let sample_data = vec![
            ("North", "Product A", "January", 15000.0, 100),
            ("North", "Product B", "January", 12000.0, 80),
            ("South", "Product A", "January", 18000.0, 120),
            ("South", "Product B", "January", 14000.0, 90),
            ("East", "Product A", "January", 16000.0, 110),
            ("East", "Product B", "January", 13000.0, 85),
            ("West", "Product A", "January", 17000.0, 115),
            ("West", "Product B", "January", 11000.0, 75),
            ("North", "Product A", "February", 16000.0, 105),
            ("North", "Product B", "February", 13000.0, 85),
            ("South", "Product A", "February", 19000.0, 125),
            ("South", "Product B", "February", 15000.0, 95),
            ("East", "Product A", "February", 17000.0, 115),
            ("East", "Product B", "February", 14000.0, 90),
            ("West", "Product A", "February", 18000.0, 120),
            ("West", "Product B", "February", 12000.0, 80),
            ("North", "Product A", "March", 17000.0, 110),
            ("North", "Product B", "March", 14000.0, 90),
            ("South", "Product A", "March", 20000.0, 130),
            ("South", "Product B", "March", 16000.0, 100),
            ("East", "Product A", "March", 18000.0, 120),
            ("East", "Product B", "March", 15000.0, 95),
            ("West", "Product A", "March", 19000.0, 125),
            ("West", "Product B", "March", 13000.0, 85),
            ("North", "Product A", "April", 18000.0, 115),
            ("North", "Product B", "April", 15000.0, 95),
            ("South", "Product A", "April", 21000.0, 135),
            ("South", "Product B", "April", 17000.0, 105),
            ("East", "Product A", "April", 19000.0, 125),
            ("East", "Product B", "April", 16000.0, 100),
            ("West", "Product A", "April", 20000.0, 130),
            ("West", "Product B", "April", 14000.0, 90),
            ("North", "Product A", "May", 19000.0, 120),
            ("North", "Product B", "May", 16000.0, 100),
            ("South", "Product A", "May", 22000.0, 140),
            ("South", "Product B", "May", 18000.0, 110),
            ("East", "Product A", "May", 20000.0, 130),
            ("East", "Product B", "May", 17000.0, 105),
            ("West", "Product A", "May", 21000.0, 135),
            ("West", "Product B", "May", 15000.0, 95),
        ];
        
        for (region, product, month, sales_amount, quantity) in sample_data {
            conn.execute(
                "INSERT INTO sales (region, product, month, sales_amount, quantity) VALUES (?, ?, ?, ?, ?)",
                params![region, product, month, sales_amount, quantity],
            )?;
        }
        
        info!("Sample data loaded successfully");
        Ok(())
    }
    
    pub fn execute_query(&self, sql: &str) -> Result<Vec<HashMap<String, Value>>> {
        let mut stmt = self.conn.prepare(sql)?;
        let rows = stmt.query_map([], |row| {
            let mut map = HashMap::new();
            for (i, name) in stmt.column_names().iter().enumerate() {
                let value: Value = match row.get(i) {
                    Ok(v) => serde_json::to_value(v).unwrap_or(Value::Null),
                    Err(_) => Value::Null,
                };
                map.insert(name.clone(), value);
            }
            Ok(map)
        })?;
        
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        
        Ok(results)
    }
    
    pub fn get_schema(&self) -> Result<Vec<ColumnInfo>> {
        let mut stmt = self.conn.prepare(
            "SELECT name, type FROM pragma_table_info('sales') ORDER BY cid"
        )?;
        
        let rows = stmt.query_map([], |row| {
            Ok(ColumnInfo {
                name: row.get(0)?,
                data_type: row.get(1)?,
            })
        })?;
        
        let mut columns = Vec::new();
        for row in rows {
            columns.push(row?);
        }
        
        Ok(columns)
    }
}

#[derive(Debug)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
}
