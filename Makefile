build-rust-consumer:
	cd rust-consumer && cargo build --release --bin rust-consumer
	cd rust-consumer && cp target/release/rust-consumer .

build-go-consumer:
	cd go-consumer && go build

build-node-consumer:
	cd node-consumer && ./node_modules/.bin/tsc src/consume.ts

build-node-producer:
	cd node-producer && ./node_modules/.bin/tsc src/produce.ts

down:
	docker-compose down --remove-orphans

up:
	docker-compose up -d

build: down build-go-consumer build-rust-consumer build-node-consumer build-node-producer

install:
	npm install -g pnpm
	cd node-producer && pnpm install
	cd node-consumer && pnpm install