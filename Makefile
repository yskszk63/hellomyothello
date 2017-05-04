all: cargo js/hello.js js/hello.wasm

js/hello.js: target/wasm32-unknown-emscripten/debug/*.js
	cp $< $@

js/hello.wasm: target/wasm32-unknown-emscripten/debug/deps/*.wasm
	cp $< $@

cargo:
	#cargo build --target=wasm32-unknown-emscripten
	env EMMAKEN_CFLAGS='-s USE_SDL=2 -O3' cargo build --target=wasm32-unknown-emscripten

.PHONY: cargo all

#env EMMAKEN_CFLAGS='-s USE_SDL=2 -O3' cargo build --target=asmjs-unknown-emscripten