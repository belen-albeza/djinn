# 🧞 Djinn

Djinn is a fantasy console to make lo-fi games, inspired by PICO-8, Varvara and DIV Games Studio.

## Requirements

- [Bun](https://bun.com/) 1.3.14
- [Rust](https://rustup.rs/) 1.96
- [`wasm-pack`](https://wasm-bindgen.github.io/wasm-pack/installer/) 0.15.0
- [`just`](https://github.com/casey/just) 1.52.0

## Build

Install dependencies with:

```zsh
bun install
cd djinn-ide && bun install
```

Then `just build` to build the project. For development builds with a watcher, run `just dev`


## License

© 2026 Belén Albeza.

Djinn is released under the Mozilla Public License 2.0. See [LICENSE](./LICENSE) for details.
