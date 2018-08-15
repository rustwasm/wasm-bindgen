# `str`

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value | JavaScript representation |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| No | Yes | No | No | No | No | JavaScript string value |

Copies the string's contents back and forth between the JavaScript
garbage-collected heap and the Wasm linear memory with `TextDecoder` and
`TextEncoder`. If you don't want to perform this copy, and would rather work
with handles to JavaScript string values, use the `js_sys::JsString` type.

## Example Rust Usage

```rust
{{#include ../../../../examples/guide-supported-types-examples/src/str.rs}}
```

## Example JavaScript Usage

```js
{{#include ../../../../examples/guide-supported-types-examples/str.js}}
```
