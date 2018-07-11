.PHONY: clean
clean: clean_web_app
	cargo clean
	rm -rf target/*

.PHONY: web_app
web_app: clean_web_app
	cargo +stable web build --release --target wasm32-unknown-emscripten --package yew-callback-test
	cargo +stable web deploy --release --target wasm32-unknown-emscripten --package yew-callback-test
	mkdir --parents ./site
	cp target/deploy/index.html target/deploy/get-payload-script.js target/deploy/yew-callback-test.js target/deploy/yew_callback_test.wasm ./site

# Lame but for some reason it's necessary for the build/deploy to function
.PHONY: clean_web_app
clean_web_app:
	rm --recursive --force target/wasm32-unknown-emscripten target/deploy
	rm --force --recursive site

.PHONY: serve
serve:
	cd site && rackup ../serve.ru --port 8080 --server puma --host 127.0.0.1
