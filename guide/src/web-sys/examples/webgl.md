# 2D Canvas

This example draws a triangle to the screen using the WebGL API.

[See the full source at
`wasm-bindgen/examples/webgl`.](https://github.com/rustwasm/wasm-bindgen/tree/master/examples/webgl)

## `Cargo.toml`

The `Cargo.toml` enables features necessary to obtain and use a WebGL
rendering context.

```toml
{{#include ../../../../examples/webgl/Cargo.toml}}
```

## `src/lib.rs`

This source file handles all of the necessary logic to obtain a rendering
context, compile shaders, fill a buffer with vertex coordinates, and draw a
triangle to the screen.

```rust
{{#include ../../../../examples/webgl/src/lib.rs}}
```
