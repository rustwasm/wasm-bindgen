# Supported Types

The table below provides an overview of all the types that wasm-bindgen can send/receive across the wasm ABI boundary.

| Type | `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value |
|:---:|:---:|:---:|:---:|:---:|:---:|
| Imported `extern Whatever;` JavaScript Types | Yes | Yes | Yes | Yes | Yes | Yes |
| Exported `struct Whatever` Rust Types | Yes | Yes | Yes | Yes | Yes | Yes |
| `str` | No | Yes | No | Yes | Yes | No |
| `char` | Yes | No | No | Yes | No | No |
| `bool` | Yes | No | No | Yes | No | No |
| `JsValue` | Yes | Yes | Yes | Yes | No | No |
| `Box<[JsValue]>` | Yes | No | No | Yes | Yes | yes |
| `*const T` | Yes | No | No | Yes | No | No |
| `*mut T` | Yes | No | No | Yes | No | No |
| `u8` `i8` `u16` `i16` `u64` `i64` `isize` `size` | Yes | No | No | Yes | No | No |
| `u32` `i32` `f32` `f64` | Yes | Yes | Yes | Yes | No | No |
| `Box<[u8]>`  `Box<[i8]>` `Box<[u16]>` `Box<[i16]>` `Box<[u32]>` `Box<[i32]>` `Box<[u64]>` `Box<[i64]>` `Box<[f32]>` `Box<[f64]`> | Yes | No | No | Yes | Yes | Yes |
| `[u8]` `[i8]` `[u16]` `[i16]` `[u32]` `[i32]` `[u64]` `[i64]` `[f32]` `[f64]` | No | Yes | Yes | No | Yes | No |
