### A simple table example with just local data stored as `Vec<Book>` Uses the Tailwind class provider.

To make this example work, you must download / fork the whole repo because this is in the Cargo.toml: `leptos-struct-table = { path = "../.." }`.

The way Tailwind works, is to scan the classes in the code. Due to this it is
recommended to copy the file `src/class_providers/tailwind.rs` into your project as done in this example.

If you don't have it installed already, install [Trunk](https://trunkrs.dev/) and [Tailwind](https://tailwindcss.com/docs/installation)
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