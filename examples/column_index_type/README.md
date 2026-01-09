### A table example with a generated enum as column index type.
Two custom renders are also used to show where the generated enum type needs to fill in and how it can be used.

To make this example work, you must download / fork the whole repo because this is in the Cargo.toml: `leptos-struct-table = { path = "../.." }`.

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