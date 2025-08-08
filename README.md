# askadb - Query Engine

Core do banco de dados Askadb em Rust. Executa queries SQL usando SQLite com dados de exemplo.

## Executar localmente

```bash
cargo run
```

## Endpoints
- POST `/execute` → body `{"query": "string"}` → retorna `QueryResponse`
- POST `/health` → retorna status do serviço

## Dados
- Dados de exemplo são carregados automaticamente na tabela `sales`
- Arquivos SQLite são armazenados em `data/`

## Makefile útil
```bash
make install    # cargo build
make run        # cargo run
make test       # cargo test
make clean      # cargo clean
make docker-build && make docker-run
```

## Estrutura
- `src/main.rs` - Servidor Axum
- `src/storage.rs` - Engine de storage SQLite
- `src/query.rs` - Executor de queries

