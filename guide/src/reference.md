# Reference

The table below provides an overview of all the types that wasm-bindgen can send/receive across the wasm ABI boundary.

| Type | `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value |
|:---:|:---:|:---:|:---:|:---:|
| `str` | No | Yes | No | Yes |
| `char` | Yes | No | No | Yes |
| `bool` | Yes | No | No | Yes |
| `JsValue` | Yes | Yes | Yes | Yes |
| `Box<[JsValue]>` | Yes | No | No | Yes |
| `*const T` | Yes | No | No | Yes |
| `*mut T` | Yes | No | No | Yes |
| `u8` `i8` `u16` `i16` `u64` `i64` `isize` `size` | Yes | No | No | Yes |
| `u32` `i32` `f32` `f64` | Yes | Yes | Yes | Yes |
| `Box<[u8]>`  `Box<[i8]>` `Box<[u16]>` `Box<[i16]>` `Box<[u32]>` `Box<[i32]>` `Box<[u64]>` `Box<[i64]>` | Yes | No | No | Yes |
| `Box<[f32]>` `Box<[f64]>` | Yes | No | No | Yes |
| `[u8]` `[i8]` `[u16]` `[i16]` `[u32]` `[i32]` `[u64]` `[i64]` | No | Yes | Yes | No |
| `&[u8]` `&mut[u8]` `&[i8]`  `&mut[i8]` `&[u16]` `&mut[u16]` `&[i16]` `&mut[i16]` `&[u32]` `&mut[u32]` `&[i32]` `&mut[i32]` `&[u64]` `&mut[u64]` `&[i64]` `&mut[i64]` | No | No | No | Yes |
| `[f32]` `[f64]` | No | Yes | Yes | No |
|  `&[f32]` `&mut[f32]` `&[f64]` `&mut[f64]` | No | No | No | Yes |