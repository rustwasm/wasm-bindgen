# `<word-count>`

Port of [MDN example](https://github.com/mdn/web-components-examples/tree/master/word-count-web-component).

When added to an element, counts all the words inside that element and displays them inside an attached shadow DOM. It also contains an interval that periodically updates the word count as it changes. Provides an example of a customized built-in element (the class inherits from `HTMLParagraphElement` rather than `HTMLElement`).

[View full source code][code] or [view the compiled example online][online]

[online]: https://rustwasm.github.io/wasm-bindgen/exbuild/web-components-word-count-web-component/
[code]: https://github.com/rustwasm/wasm-bindgen/tree/master/examples/web-components-word-count-web-component

## `src/lib.rs`

```rust
{{#include ../../../examples/web-components-word-count-web-component/src/lib.rs}}
```