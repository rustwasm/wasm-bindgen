# `<editable-list>`

Port of [MDN example](https://github.com/mdn/web-components-examples/tree/master/editable-list).

A simple example showing how elements can be consolidated to create a list with addable/removable items. Items are added by using a `list-item` attribute or by entering text and clicking the plus sign.

[View full source code][code] or [view the compiled example online][online]

[online]: https://rustwasm.github.io/wasm-bindgen/exbuild/web-components-editable-list/
[code]: https://github.com/rustwasm/wasm-bindgen/tree/master/examples/web-components-editable-list

## `src/lib.rs`

```rust
{{#include ../../../examples/web-components-editable-list/src/lib.rs}}
```