# askadb - Query Engine

Executa queries SQL usando DuckDB localmente sobre dados em Parquet/CSV.

## Executar localmente

```bash
uvicorn app.main:app --reload
```

## Endpoints
- POST `/execute/` → body `QueryInput { query: string }` → retorna `QueryOutput`

## Dados
- Coloque arquivos `.parquet`/`.csv` em `data/`

## Makefile útil
```bash
make install
make run PORT=8002
make test
make docker-build && make docker-run
```

