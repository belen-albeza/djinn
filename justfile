clean:
    cargo clean --manifest-path djinn-rs/Cargo.toml
    rm -rf packages/djinn-dev-wasm
    rm -rf djinn-ide/build

build-wasm:
    wasm-pack build djinn-rs/djinn-dev-wasm --out-dir ../../packages/djinn-dev-wasm

lint-wasm:
    cargo clippy --manifest-path djinn-rs/Cargo.toml

test-wasm: lint-wasm
    cargo test --manifest-path djinn-rs/Cargo.toml

dev: build-wasm
    bun --cwd djinn-ide dev

build: build-wasm
    bun --cwd djinn-ide build

test: test-wasm
    bun --cwd djinn-ide test

