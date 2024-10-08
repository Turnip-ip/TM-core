PKG_NAME:=$(shell cargo metadata --no-deps --format-version 1 | head -c 31 | tail -c 9)

.PHONY: all
all: doc release

.PHONY: test
test:
	cargo fmt
	cargo test

.PHONY: doc
doc:
	cargo doc

.PHONY: release
release: test
	wasm-pack build --target web --release --out-dir $(PKG_NAME)

.PHONY: debug
debug: test
	wasm-pack build --target web --dev --out-dir $(PKG_NAME)

