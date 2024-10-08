.PHONY: all
all:
	cargo fmt
	cargo test
	wasm-pack build --target web
