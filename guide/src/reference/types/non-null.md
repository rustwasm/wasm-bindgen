# `NonNull<T>`

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value | JavaScript representation |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Yes | No | No | Yes | Yes | Yes | A JavaScript number value |

## Example Rust Usage

```rust
{{#include ../../../../examples/guide-supported-types-examples/src/non_null.rs}}
```

## Example JavaScript Usage

```js
{{#include ../../../../examples/guide-supported-types-examples/non_null.js}}
```
