PHONY=cargo all

all: cargo js/hello.js js/hello.wasm

js/hello.js: target/wasm32-unknown-emscripten/debug/*.js
	cp $< $@

js/hello.wasm: target/wasm32-unknown-emscripten/debug/deps/*.wasm
	cp $< $@

cargo:
	cargo build --target=wasm32-unknown-emscripten
