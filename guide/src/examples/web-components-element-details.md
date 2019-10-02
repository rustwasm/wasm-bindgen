# `<element-details>`

Port of [MDN example](https://github.com/mdn/web-components-examples/tree/master/element-details).

Displays a box containing an HTML element name and description. Provides an example of an autonomous custom element that gets its structure from a `<template>` element (that also has its own styling defined), and also contains `<slot>` elements populated at runtime.

[View full source code][code] or [view the compiled example online][online]

[online]: https://rustwasm.github.io/wasm-bindgen/exbuild/web-components-element-details/
[code]: https://github.com/rustwasm/wasm-bindgen/tree/master/examples/web-components-element-details

## `src/lib.rs`

```rust
{{#include ../../../examples/web-components-element-details/src/lib.rs}}
```