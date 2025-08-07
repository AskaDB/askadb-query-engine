import duckdb
import os

DATA_PATH = os.path.join(os.path.dirname(__file__), "..", "..", "data", "vendas.parquet")

def run_query(query: str):
    con = duckdb.connect()
    con.execute(f"CREATE VIEW vendas AS SELECT * FROM '{DATA_PATH}'")
    
    result = con.execute(query)
    columns = [desc[0] for desc in result.description]
    rows = [dict(zip(columns, row)) for row in result.fetchall()]
    
    return {
        "columns": columns,
        "rows": rows
    }
