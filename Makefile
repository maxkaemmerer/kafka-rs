build-rust-consumer:
	cd rust-consumer && cargo build --release --bin rust-consumer
	cd rust-consumer && cp target/release/rust-consumer .

build-go-consumer:
	cd go-consumer && go build

build-node-consumer:
	cd node-consumer && ./node_modules/.bin/tsc src/consume.ts

build-node-producer:
	cd node-producer && ./node_modules/.bin/tsc src/produce.ts

build: build-go-consumer build-rust-consumer build-node-consumer build-node-producer