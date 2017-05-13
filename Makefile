debug: cargo-debug debug/hello.js debug/hello.wasm debug/index.html
release: cargo-release docs/hello.js docs/hello.wasm docs/index.html

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


docs/hello.js: target/wasm32-unknown-emscripten/release/*.js
	mkdir -p docs
	cp $< $@

docs/hello.wasm: target/wasm32-unknown-emscripten/release/deps/*.wasm
	mkdir -p docs
	cp $< $@

docs/index.html: static/index.html
	mkdir -p docs
	cp $< $@

cargo-release:
	env EMMAKEN_CFLAGS='-s USE_SDL=2 -O3' cargo build --target=wasm32-unknown-emscripten --release


.PHONY: cargo-debug cargo-release all
