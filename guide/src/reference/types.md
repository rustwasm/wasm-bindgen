# Supported Rust Types and their JavaScript Representations

This section provides an overview of all the types that `wasm-bindgen` can send
and receive across the WebAssembly ABI boundary, and how they translate into
JavaScript.

## Imported `extern Whatever;` JavaScript Types

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value | JavaScript representation |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Yes | Yes | No | Yes | Yes | Yes | Instances of the extant `Whatever` JavaScript class / prototype constructor |

### Example Rust Usage

```rust
{{#include ../../../examples/guide-supported-types-examples/src/imported_types.rs}}
```

### Example JavaScript Usage

```js
{{#include ../../../examples/guide-supported-types-examples/imported_types.js}}
```

## Exported `struct Whatever` Rust Types

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value | JavaScript representation |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Yes | Yes | Yes | Yes | No | No | Instances of a `wasm-bindgen`-generated JavaScript `class Whatever { ... }` |

### Example Rust Usage

```rust
{{#include ../../../examples/guide-supported-types-examples/src/exported_types.rs}}
```

### Example JavaScript Usage

```js
{{#include ../../../examples/guide-supported-types-examples/exported_types.js}}
```

## `str`

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value | JavaScript representation |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| No | Yes | No | No | No | No | JavaScript string value |

Copies the string's contents back and forth between the JavaScript
garbage-collected heap and the Wasm linear memory with `TextDecoder` and
`TextEncoder`. If you don't want to perform this copy, and would rather work
with handles to JavaScript string values, use the `js_sys::JsString` type.

### Example Rust Usage

```rust
{{#include ../../../examples/guide-supported-types-examples/src/str.rs}}
```

### Example JavaScript Usage

```js
{{#include ../../../examples/guide-supported-types-examples/str.js}}
```

## `String`

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value | JavaScript representation |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Yes | No | No | Yes | Yes | Yes | JavaScript string value |

Copies the string's contents back and forth between the JavaScript
garbage-collected heap and the Wasm linear memory with `TextDecoder` and
`TextEncoder`

### Example Rust Usage

```rust
{{#include ../../../examples/guide-supported-types-examples/src/string.rs}}
```

### Example JavaScript Usage

```js
{{#include ../../../examples/guide-supported-types-examples/string.js}}
```

## `char`

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value | JavaScript representation |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Yes | No | No | Yes | No | No | A JavaScript string value |

### Example Rust Usage

```rust
{{#include ../../../examples/guide-supported-types-examples/src/char.rs}}
```

### Example JavaScript Usage

```js
{{#include ../../../examples/guide-supported-types-examples/char.js}}
```

## `bool`

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value | JavaScript representation |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Yes | No | No | Yes | No | No | A JavaScript boolean value |

### Example Rust Usage

```rust
{{#include ../../../examples/guide-supported-types-examples/src/bool.rs}}
```

### Example JavaScript Usage

```js
{{#include ../../../examples/guide-supported-types-examples/bool.js}}
```

## `wasm_bindgen::JsValue`

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value | JavaScript representation |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Yes | Yes | No | Yes | No | No | Any JavaScript value |

### Example Rust Usage

```rust
{{#include ../../../examples/guide-supported-types-examples/src/js_value.rs}}
```

### Example JavaScript Usage

```js
{{#include ../../../examples/guide-supported-types-examples/js_value.js}}
```

## `Box<[JsValue]>`

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value | JavaScript representation |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Yes | No | No | Yes | Yes | Yes | A JavaScript `Array` object |

## `*const T` `*mut T`

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value | JavaScript representation |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Yes | No | No | Yes | No | No | A JavaScript number value |

## `u8` `i8` `u16` `i16` `u64` `i64` `isize` `size`

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value | JavaScript representation |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Yes | No | No | Yes | No | No | A JavaScript number value |

## `u32` `i32` `f32` `f64`

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value | JavaScript representation |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Yes | Yes | Yes | Yes | No | No | A JavaScript number value |

## `Box<[u8]>`  `Box<[i8]>` `Box<[u16]>` `Box<[i16]>` `Box<[u32]>` `Box<[i32]>` `Box<[u64]>` `Box<[i64]>` `Box<[f32]>` `Box<[f64]>`

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value | JavaScript representation |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Yes | No | No | Yes | Yes | Yes | A JavaScript `TypedArray` view of the Wasm memory for the boxed slice of the appropriate type (`Int32Array`, `Uint8Array`, etc) |

Note that this does ***not*** copy the whole slice of memory back and forth into
the JavaScript heap from the Wasm linear memory.

## `[u8]` `[i8]` `[u16]` `[i16]` `[u32]` `[i32]` `[u64]` `[i64]` `[f32]` `[f64]`

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value | JavaScript representation |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| No | Yes | Yes | No | Yes | No | A JavaScript `TypedArray` view of the Wasm memory for the boxed slice of the appropriate type (`Int32Array`, `Uint8Array`, etc) |

Note that this does ***not*** copy the whole slice of memory back and forth into
the JavaScript heap from the Wasm linear memory.
