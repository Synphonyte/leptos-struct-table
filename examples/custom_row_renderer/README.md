A simple table example with just local data stored as `Vec<Book>` demonstrating a custom row renderer and skipping header titles.

If you don't have it installed already, install [Trunk](https://trunkrs.dev/) 
as well as the nightly toolchain for Rust and the wasm32-unknown-unknown target:

```bash
cargo install trunk
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown
```

Then, to run this example, execute in a terminal:

```bash
trunk serve --open
```