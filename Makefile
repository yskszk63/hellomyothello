debug: cargo-debug debug/hello.js debug/index.html
release: cargo-release docs/hello.js docs/index.html

debug/hello.js: target/wasm32-unknown-emscripten/debug/hellomyothello.js
	mkdir -p debug
	cp $< $@
	cp target/wasm32-unknown-emscripten/debug/deps/*.wasm debug/hello.wasm

debug/index.html: static/index.html
	mkdir -p debug
	cp $< $@

cargo-debug: assets/FiraMono-Regular.ttf
	env EMMAKEN_CFLAGS='-s USE_SDL=2 -O3' cargo build --target=wasm32-unknown-emscripten


docs/hello.js: target/wasm32-unknown-emscripten/release/hellomyothello.js
	mkdir -p docs
	cp $< $@
	cp target/wasm32-unknown-emscripten/release/deps/*.wasm docs/hello.wasm

docs/index.html: static/index.html
	mkdir -p docs
	cp $< $@

cargo-release: assets/FiraMono-Regular.ttf
	env EMMAKEN_CFLAGS='-s USE_SDL=2 -O3' cargo build --target=wasm32-unknown-emscripten --release


assets/FiraMono-Regular.ttf:
	mkdir -p assets
	curl -o $@ 'https://mozilla.github.io/Fira/ttf/FiraMono-Regular.ttf'

.PHONY: cargo-debug cargo-release all
