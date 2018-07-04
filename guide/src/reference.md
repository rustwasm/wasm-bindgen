# Reference

The table below provides an overview of all the types that wasm-bindgen can send/receive across the wasm ABI boundary.

| Type | `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value |
|:---:|:---:|:---:|:---:|:---:|
| `u8` | Yes | No | No | Yes |
| `i8` | Yes | No | No | Yes |
| `u16` | Yes | No | No | Yes |
| `i16` | Yes | No | No | Yes |
| `u32` | Yes | Yes | Yes | Yes |
| `i32` | Yes | Yes | Yes | Yes |
| `u64` | Yes | No | No | Yes |
| `i64` | Yes | No | No | Yes |
| `f32` | Yes | Yes | Yes | Yes |
| `f64` | Yes | Yes | Yes | Yes |
| `isize` | Yes | No | No | Yes |
| `usize` | Yes |No  | No | Yes |
| `str` | No | Yes | No | Yes |
| `char` | Yes | No | No | Yes |
| `bool` | Yes | No | No | Yes |
| `JsValue` | Yes | Yes | Yes | Yes |
| `Box<[JsValue]>` | Yes | No | No | Yes |
| `*const T` | Yes | No | No | Yes |
| `*mut T` | Yes | No | No | Yes |
| `Box<[u8]>` | Yes | No | No | Yes |
| `&[u8]` | No | No | No | Yes |
| `&mut[u8]` | No | No | No | Yes |
| `[u8]` | No | Yes | Yes | No |
| `Box<[i8]>` | Yes | No | No | Yes |
| `&[i8]` | No | No | No | Yes |
| `&mut[i8]` | No | No | No | Yes |
| `[i8]` | No | Yes | Yes | No |
| `Box<[u16]>` | Yes | No | No | Yes |
| `&[u16]` | No | No | No | Yes |
| `&mut[u16]` | No | No | No | Yes |
| `[u16]` | No | Yes | Yes | No |
| `Box<[i16]>` | Yes | No | No | Yes |
| `&[i16]` | No | No | No | Yes |
| `&mut[i16]` | No | No | No | Yes |
| `[i16]` | No | Yes | Yes | No |
| `Box<[u32]>` | Yes | No | No | Yes |
| `&[u32]` | No | No | No | Yes |
| `&mut[u32]` | No | No | No | Yes |
| `[u32]` | No | Yes | Yes | No |
| `Box<[i32]>` | Yes | No | No | Yes |
| `&[i32]` | No | No | No | Yes |
| `&mut[i32]` | No | No | No | Yes |
| `[i32]` | No | Yes | Yes | No |
| `Box<[u64]>` | Yes | No | No | Yes |
| `&[u64]` | No | No | No | Yes |
| `&mut[u64]` | No | No | No | Yes |
| `[u64]` | No | Yes | Yes | No |
| `Box<[i64]>` | Yes | No | No | Yes |
| `&[i64]` | No | No | No | Yes |
| `&mut[i64]` | No | No | No | Yes |
| `[i64]` | No | Yes | Yes | No |
| `Box<[f32]>` | Yes | No | No | Yes |
| `&[f32]` | No | No | No | Yes |
| `&mut[f32]` | No | No | No | Yes |
| `[f32]` | No | Yes | Yes | No |
| `Box<[f64]>` | Yes | No | No | Yes |
| `&[f64]` | No | No | No | Yes |
| `&mut[f64]` | No | No | No | Yes |
| `[f64]` | No | Yes | Yes | No |