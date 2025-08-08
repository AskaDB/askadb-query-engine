CARGO ?= cargo
PORT ?= 8002

install:
	$(CARGO) build

run:
	$(CARGO) run

test:
	$(CARGO) test

clean:
	$(CARGO) clean

docker-build:
	docker build -t askadb/query-engine:local .

docker-run:
	docker run --rm -p $(PORT):$(PORT) -v $(PWD)/data:/app/data askadb/query-engine:local

