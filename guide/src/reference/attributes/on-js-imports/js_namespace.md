# `js_namespace = blah`

This attribute indicates that the JavaScript type is accessed through the given
namespace. For example, the `WebAssembly.Module` APIs are all accessed through
the `WebAssembly` namespace. `js_namespace` can be applied to any import
(function or type) and whenever the generated JavaScript attempts to reference a
name (like a class or function name) it'll be accessed through this namespace.

```rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

log("hello, console!");
```

This is an example of how to bind `console.log` in Rust. The `log` function will
be available in the Rust module and will be invoked as `console.log` in
JavaScript.
