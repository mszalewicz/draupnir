.PHONY: run

build_wasm:
	cargo build --profile wasm-release --target wasm32-unknown-unknown
	cd target/wasm32-unknown-unknown/wasm-release/ && wasm-opt -Oz --output optimized.wasm draupnir.wasm
	cd target/wasm32-unknown-unknown/wasm-release/ && mv optimized.wasm draupnir.wasm
	cd target/wasm32-unknown-unknown/wasm-release/ && rm -f draupnir.wasm.br
	cd target/wasm32-unknown-unknown/wasm-release/ && brotli --best draupnir.wasm

build_fast:
	cargo build --profile fast --target aarch64-apple-darwin

build_release:
	cargo build --profile desktop_release --target aarch64-apple-darwin