A simple table example with just local data stored as `Vec<Book>` Uses the Bootstrap class provider.

If you don't have it installed already, install [Trunk](https://trunkrs.dev/)
as well as the wasm32-unknown-unknown target:

```bash
cargo install trunk
rustup target add wasm32-unknown-unknown
```

Then, to run this example, execute in a terminal:

```bash
trunk serve --open
```