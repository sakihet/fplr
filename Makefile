build:
	cargo build --release
fmt:
	cargo fmt
install:
	cargo install --path .
lint:
	cargo clippy -- -D warnings
lint-fix:
	cargo clippy --fix --allow-dirty --allow-staged -- -D warnings
run:
	cargo run
test:
	cargo test
