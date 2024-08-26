### Example that shows how to use pagination with a table.

To make this example work, you must download / fork the whole repo because this is in the Cargo.toml: `leptos-struct-table = { path = "../.." }`.

If you don't have it installed already, install [Trunk](https://trunkrs.dev/) 
as well as the wasm32-unknown-unknown target:

```bash
cargo install trunk
npm install -D tailwindcss
rustup target add wasm32-unknown-unknown
```

Then, open two terminals. In the first one, run:

```
npx tailwindcss -i ./input.css -o ./style/output.css --watch
```

In the second one, run:

```bash
trunk serve --open
```