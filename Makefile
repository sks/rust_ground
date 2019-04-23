.DEFAULT_GOAL = build

run:
	cargo run

build:
	cargo build --release

dockerize:
	docker build -t wasm:latest .