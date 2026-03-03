# Agent Guidelines

## After modifying Rust code

Run the following commands from the workspace root:

```bash
cargo clippy --all --all-features
cargo +nightly fmt --all
```

## After modifying the Node.js SDK (`nodejs/`)

Build the native `.node` binary from the `nodejs/` directory:

```bash
npm run build:debug
```
