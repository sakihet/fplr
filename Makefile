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
setup:
	git config core.hooksPath .githooks
site-build:
	cd site && npm run build
site-dev:
	cd site && npm run dev
site-reference: build
	site/scripts/gen-reference.sh
test:
	cargo test
