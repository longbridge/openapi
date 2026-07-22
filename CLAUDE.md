# Agent Guidelines

## After modifying Rust code

Run the following commands from the workspace root:

```bash
cargo clippy --all --all-features
cargo +nightly fmt --all
```

> **Note:** `cargo +nightly fmt` may reflow doc comments (e.g. `/// @param …`
> lines). Do **not** revert those changes — they are intentional formatting
> output and should be committed as-is.

## After modifying the Node.js SDK (`nodejs/`)

Build the native `.node` binary from the `nodejs/` directory:

```bash
npm run build:debug
```

`nodejs/index.d.ts` and `nodejs/index.js` are **auto-generated** by
`npm run build:debug` — never edit them by hand.

## After updating the proto submodule (`rust/crates/proto/openapi-protobufs/`)

Run the following command from the workspace root to regenerate the Rust proto source files
(e.g. `rust/crates/proto/src/longbridge.control.v1.rs`,
`rust/crates/proto/src/longbridge.quote.v1.rs`,
`rust/crates/proto/src/longbridge.trade.v1.rs`):

```bash
cargo make protoc
```

The generated `*.rs` files under `rust/crates/proto/src/` are **auto-generated** — never edit
them by hand.

## After modifying the Python SDK API (`python/`)

`python/pysrc/longbridge/openapi.pyi` is a **manually maintained** type-stub
file that provides type hints and docstrings for the native Rust/PyO3 extension
module. IDEs and type checkers (mypy/pyright) rely on it for autocompletion and
static analysis.

When you add, remove, or change any `#[pyclass]`/`#[pymethods]` definitions in
`python/src/`, you **must** update `openapi.pyi` accordingly — keeping
signatures, type annotations, and docstrings in sync with the Rust
implementation.

To build and install the Python SDK locally (from the `python/` directory):

```bash
maturin develop
```

`python/pyproject.toml` uses `dynamic = ["version"]` — maturin reads the
version from `python/Cargo.toml` automatically. Do **not** add a hardcoded
`version` field to `pyproject.toml`.

## After modifying the C SDK (`c/`)

`c/csrc/include/longbridge.h` is **auto-generated** by `cbindgen` during the
build — never edit it by hand. Rebuild the C crate to update it:

```bash
cargo build -p longbridge-c
```

## After any change

Update `CHANGELOG.md` in the workspace root to document notable changes. The
format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/). Add an
entry under the `[Unreleased]` section in the appropriate subsection (`Added`,
`Changed`, `Fixed`, `Breaking changes`, etc.). If the `[Unreleased]` section
does not yet exist, create it at the top of the changelog (above the latest
versioned block).
