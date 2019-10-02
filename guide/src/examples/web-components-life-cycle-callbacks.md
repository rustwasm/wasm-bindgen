# `<custom-square l="" c="">`

Port of [MDN example](https://github.com/mdn/web-components-examples/tree/master/life-cycle-callbacks).

A trivial example web component that creates a square colored box on the page. The demo also includes buttons to create, destroy, and change attributes on the element, to demonstrate how the [web components life cycle callbacks](https://developer.mozilla.org/en-US/docs/Web/Web_Components/Using_custom_elements#Using_the_lifecycle_callbacks) work.

[View full source code][code] or [view the compiled example online][online]

[online]: https://rustwasm.github.io/wasm-bindgen/exbuild/web-components-life-cycle-callbacks/
[code]: https://github.com/rustwasm/wasm-bindgen/tree/master/examples/web-components-life-cycle-callbacks

## `src/lib.rs`

```rust
{{#include ../../../examples/web-components-life-cycle-callbacks/src/lib.rs}}
```