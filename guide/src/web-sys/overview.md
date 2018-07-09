# `web-sys` Overview

The `web-sys` crate has this file and directory layout:

```text
.
├── build.rs
├── Cargo.toml
├── README.md
├── src
│   └── lib.rs
└── webidls
    ├── available
    │   └── ...
    └── enabled
        └── ...
```

### `webidls/available/*.webidl`

These are all the different WebIDL definitions we intend to support, but don't
yet. At the time of writing, these are the majority of `.webidl`s.

### `webidls/enabled/*.webidl`

These are the WebIDL interfaces that we will actually generate bindings for (or
at least bindings for *some* of the things defined in these files).

### `build.rs`

The `build.rs` invokes `wasm-bindgen`'s WebIDL frontend on all the WebIDL files
in `webidls/enabled`. It writes the resulting bindings into the cargo build's
out directory.

### `src/lib.rs`

The only thing `src/lib.rs` does is include the bindings generated at compile
time in `build.rs`. Here is the whole `src/lib.rs` file:

```rust
{{#include ../../../crates/web-sys/src/lib.rs}}
```
