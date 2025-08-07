from fastapi.testclient import TestClient
from app.main import app

client = TestClient(app)

def test_execute_query():
    response = client.post("/execute/", json={"query": "SELECT COUNT(*) AS total FROM vendas"})
    assert response.status_code == 200
    assert "columns" in response.json()
    assert "rows" in response.json()
