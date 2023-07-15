In this example we use cargo-leptos to create a server function that runs on the client and server at the same time.

If you don't have it installed already, install [Cargo-Leptos](https://leptos.dev/)
as well as the nightly toolchain for Rust and the wasm32-unknown-unknown target:

## Install dev dependencies

```
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos
```

## How to run

```
cargo leptos watch
```