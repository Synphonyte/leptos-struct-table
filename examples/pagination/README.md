Example that shows how to use pagination with a table.

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