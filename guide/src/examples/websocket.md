# WebSocket

[View full source code][code] or [view the compiled example online][online]

[online]: https://rustwasm.github.io/wasm-bindgen/exbuild/websocket/
[code]: https://github.com/rustwasm/wasm-bindgen/tree/master/examples/websocket/

This example implements simple websocket client using WebSocket API.

## `Cargo.toml`

The `Cargo.toml` enables the web-sys::WebSocket and js-sys::Funcition.

```toml
{{#include ../../../examples/websocket/Cargo.toml}}
```

## `src/lib.rs`

The Rust code implements the WebSocketFactory struct with send and close methods.

```rust
{{#include ../../../examples/websocket/src/lib.rs}}
```

## `index.js`

The JavaScript code adds event listeners to html elements, implementing basic websocket client page.

```js
{{#include ../../../examples/websocket/index.js}}
```
