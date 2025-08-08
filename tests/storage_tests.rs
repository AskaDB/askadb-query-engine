use askadb_query_engine::storage;
use std::fs;

#[test]
fn init_and_basic_query() {
    let db_path = "data/test_cov.db";
    // cleanup
    let _ = fs::remove_file(db_path);

    storage::init_db(db_path).expect("init db");

    let (rows, cols) = storage::execute_query(db_path, "SELECT 1 as x").expect("query");
    assert_eq!(cols, vec!["x".to_string()]);
    assert_eq!(rows.len(), 1);
}
