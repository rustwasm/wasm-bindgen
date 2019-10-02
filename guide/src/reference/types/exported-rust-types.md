# Exported `struct Whatever` Rust Types

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value | JavaScript representation |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| No | Yes | Yes | No | No | No | Inner instances of a `wasm-bindgen`-generated JavaScript `class Whatever { ... }` |

See also [`WasmType<T>`](wrapped-rust-types.html).

## Example Rust Usage

```rust
{{#include ../../../../examples/guide-supported-types-examples/src/exported_types.rs}}
```

## Example JavaScript Usage

```js
{{#include ../../../../examples/guide-supported-types-examples/exported_types.js}}
```
