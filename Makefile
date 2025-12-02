build:
	cargo build --release
fmt:
	cargo fmt
lint:
	cargo clippy -- -D warnings
test:
	cargo test
run:
	cargo run
