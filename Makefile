debug: cargo-debug debug/hello.js debug/hello.wasm debug/index.html
release: cargo-release release/hello.js release/hello.wasm release/index.html

debug/hello.js: target/wasm32-unknown-emscripten/debug/*.js
	mkdir -p debug
	cp $< $@

debug/hello.wasm: target/wasm32-unknown-emscripten/debug/deps/*.wasm
	mkdir -p debug
	cp $< $@

debug/index.html: static/index.html
	mkdir -p debug
	cp $< $@

cargo-debug:
	env EMMAKEN_CFLAGS='-s USE_SDL=2 -O3' cargo build --target=wasm32-unknown-emscripten


release/hello.js: target/wasm32-unknown-emscripten/release/*.js
	mkdir -p release
	cp $< $@

release/hello.wasm: target/wasm32-unknown-emscripten/release/deps/*.wasm
	mkdir -p release
	cp $< $@

release/index.html: static/index.html
	mkdir -p release
	cp $< $@

cargo-release:
	env EMMAKEN_CFLAGS='-s USE_SDL=2 -O3' cargo build --target=wasm32-unknown-emscripten --release


.PHONY: cargo-debug cargo-release all
