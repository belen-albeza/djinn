clean:
    cargo clean --manifest-path djinn-rs/Cargo.toml
    rm -rf packages/djinn-dev-wasm
    rm -rf djinn-ide/build

build-wasm:
    wasm-pack build djinn-rs/djinn-dev-wasm --out-dir ../../packages/djinn-dev-wasm

dev: build-wasm
    bun --cwd djinn-ide dev

build: build-wasm
    bun --cwd djinn-ide build

test:
    cargo test --manifest-path djinn-rs/Cargo.toml
    bun --cwd djinn-ide test

