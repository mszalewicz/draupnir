.PHONY: run run_wasm build_wasm build_fast build_release

run_wasm_wgpu: build_wasm_wgpu
	@echo "Building server"
	@cd server && go build -ldflags="-s -w" .
	@echo "Running server"
	@cd server && ./server


build_wasm:
	@echo "Building WASM"
	@BACKEND=webgpu cargo build --profile wasm_wgpu-release --target wasm32-unknown-unknown
	@echo "Binding WASM"
	@cd target/wasm32-unknown-unknown/wasm_wgpu-release/ && wasm-bindgen --out-name draupnir --out-dir . --target web draupnir.wasm
	@echo "Optimizing WASM size"
	@cd target/wasm32-unknown-unknown/wasm_wgpu-release/ && wasm-opt -Oz --output optimized.wasm draupnir_bg.wasm
	@cd target/wasm32-unknown-unknown/wasm_wgpu-release/ && mv optimized.wasm draupnir_bg.wasm
	@cd target/wasm32-unknown-unknown/wasm_wgpu-release/ && rm -f draupnir_bg.wasm.br
	@echo "Minimizing WASM"
	@cd target/wasm32-unknown-unknown/wasm_wgpu-release/ && brotli --best draupnir_bg.wasm
	@cp target/wasm32-unknown-unknown/wasm_wgpu-release/draupnir_bg.wasm server/static
	@cp target/wasm32-unknown-unknown/wasm_wgpu-release/draupnir_bg.wasm.br server/static
	@cp target/wasm32-unknown-unknown/wasm_wgpu-release/draupnir.js server/static

run_wasm_webgl: build_wasm_webgl
	@echo "Building server"
	@cd server && go build -ldflags="-s -w" .
	@echo "Running server"
	@cd server && ./server


build_wasm_webgl:
	@echo "Building WASM"
	@BACKEND=webgl2 cargo build --profile wasm_webgl2-release --target wasm32-unknown-unknown
	@echo "Binding WASM"
	@cd target/wasm32-unknown-unknown/wasm_webgl2-release/ && wasm-bindgen --out-name draupnir --out-dir . --target web draupnir.wasm
	@echo "Optimizing WASM size"
	@cd target/wasm32-unknown-unknown/wasm_webgl2-release/ && wasm-opt -Oz --output optimized.wasm draupnir_bg.wasm
	@cd target/wasm32-unknown-unknown/wasm_webgl2-release/ && mv optimized.wasm draupnir_bg.wasm
	@cd target/wasm32-unknown-unknown/wasm_webgl2-release/ && rm -f draupnir_bg.wasm.br
	@echo "Minimizing WASM"
	@cd target/wasm32-unknown-unknown/wasm_webgl2-release/ && brotli --best draupnir_bg.wasm
	@cp target/wasm32-unknown-unknown/wasm_webgl2-release/draupnir_bg.wasm server/static
	@cp target/wasm32-unknown-unknown/wasm_webgl2-release/draupnir_bg.wasm.br server/static
	@cp target/wasm32-unknown-unknown/wasm_webgl2-release/draupnir.js server/static


build_fast:
	cargo build --profile fast --target aarch64-apple-darwin

build_release:
	cargo build --profile desktop_release --target aarch64-apple-darwin