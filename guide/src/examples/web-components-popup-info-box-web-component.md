# `<popup-info img="" text="">`

Port of [MDN example](https://github.com/mdn/web-components-examples/tree/master/popup-info-box-web-component).

Creates an info icon that when focused displays a popup info box. Provides an example of an autonomous custom element that takes information from its attributes, and defines structure and basic style in an attached shadow DOM.

[View full source code][code] or [view the compiled example online][online]

[online]: https://rustwasm.github.io/wasm-bindgen/exbuild/web-components-popup-info-box-web-component/
[code]: https://github.com/rustwasm/wasm-bindgen/tree/master/examples/web-components-popup-info-box-web-component

## `src/lib.rs`

```rust
{{#include ../../../examples/web-components-popup-info-box-web-component/src/lib.rs}}
```