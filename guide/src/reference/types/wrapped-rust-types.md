# `WasmType<Whatever>`

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value | JavaScript representation |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Yes | No | No | Yes | Yes | Yes | Wrapped instances of a `wasm-bindgen`-generated JavaScript `class Whatever { ... }` |

Exported Rust types can *only* be instantiated using an invocation of the `instantiate!` macro within a `#[wasm_bindgen(constructor)]` associated method.  Such macro invocations evaluate to values of `WasmType<T>`, which is a mere type alias for a vendored `Rc<RefCell<T>>`.  You can `borrow()` and `borrow_mut()` to obtain a (vendored) `Ref` and `RefMut` (which deref to `&T` and `&mut T`) respectively, or move/clone the `WasmType<T>` itself (cloning merely creates an additional reference to the same underlying object).

## Example Rust Usage

```rust
{{#include ../../../../examples/guide-supported-types-examples/src/exported_types.rs}}
```

## Example JavaScript Usage

```js
{{#include ../../../../examples/guide-supported-types-examples/exported_types.js}}
```
