PKG_NAME:=$(shell cargo metadata --no-deps --format-version 1 | head -c 31 | tail -c 9)

.PHONY: all
all: release

.PHONY: test
test:
	# cargo test
	cargo llvm-cov

.PHONY: lint
lint:
	cargo clippy --all-targets --all-features
	cargo fmt

.PHONY: doc
doc:
	RUSTDOCFLAGS="--enable-index-page -Zunstable-options" cargo +nightly doc --document-private-items

.PHONY: release
release: lint test
	wasm-pack build --target web --release --out-dir $(PKG_NAME)

.PHONY: debug
debug:
	wasm-pack build --target web --dev --out-dir $(PKG_NAME)
	cp -r $(PKG_NAME) tests/website
	cd tests/website; python -m http.server

