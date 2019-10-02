# `<ul is="expanding-list">`

Port of [MDN example](https://github.com/mdn/web-components-examples/tree/master/expanding-list-web-component).

Creates an unordered list with expandable/collapsible children. Provides an example of a customized built-in element (the class inherits from `HTMLUListElement` rather than `HTMLElement`).

[View full source code][code] or [view the compiled example online][online]

[online]: https://rustwasm.github.io/wasm-bindgen/exbuild/web-components-expanding-list-web-component/
[code]: https://github.com/rustwasm/wasm-bindgen/tree/master/examples/web-components-expanding-list-web-component

## `src/lib.rs`

```rust
{{#include ../../../examples/web-components-expanding-list-web-component/src/lib.rs}}
```