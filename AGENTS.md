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

`nodejs/index.d.ts` and `nodejs/index.js` are **auto-generated** by
`npm run build:debug` — never edit them by hand.

## After modifying the C SDK (`c/`)

`c/csrc/include/longbridge.h` is **auto-generated** by `cbindgen` during the
build — never edit it by hand. Rebuild the C crate to update it:

```bash
cargo build -p longbridge-c
```
