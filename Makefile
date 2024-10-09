PKG_NAME:=$(shell cargo metadata --no-deps --format-version 1 | head -c 31 | tail -c 9)

.PHONY: all
all: doc release

.PHONY: test
test:
	cargo test

.PHONY: lint
lint:
	cargo clippy
	cargo fmt

.PHONY: doc
doc:
	cargo doc

.PHONY: release
release: test
	wasm-pack build --target web --release --out-dir $(PKG_NAME)

.PHONY: debug
debug: test
	wasm-pack build --target web --dev --out-dir $(PKG_NAME)
	cp -r $(PKG_NAME) tests/website
	cd tests/website; python -m http.server

