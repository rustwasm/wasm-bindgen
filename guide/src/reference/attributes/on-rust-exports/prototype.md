# `prototype`

The `prototype` attribute can be used on types exported to JS, to indicate the type from which it should inherit.  The value can be a path to any type known to wasm-bindgen (either exported to or imported from JavaScript).

```rust
// superclass exported to JavaScript
#[wasm_bindgen]
struct Parent {}
#[wasm_bindgen(prototype=Parent)]
struct Child {}

// superclass imported from JS
#[wasm_bindgen]
extern "C" {
    pub type ImportedParent;
}
#[wasm_bindgen(prototype=ImportedParent)]
pub struct ChildOfImportedParent { ... }

// or a built-in published by the `js_sys` crate
#[wasm_bindgen(prototype=js_sys::Date)]
pub struct CustomDate { ... }

// or a Web IDL implementation published by the `web_sys` crate
#[wasm_bindgen(prototype=web_sys::XmlHttpRequest)]
pub struct MyAjaxRequest { ... }
```
