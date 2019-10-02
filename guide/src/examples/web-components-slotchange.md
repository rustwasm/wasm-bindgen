# `<summary-display>`

Port of [MDN example](https://github.com/mdn/web-components-examples/tree/master/slotchange).

An example that takes as its two slot values a list of possible choices, and a description for the selected choice. Multiple paragraphs are included inside the element containing all the possible descriptions; when a choice is clicked, its corresponding description paragraph is given an appropriate slot attribute so that it appears in the second slot. This example is written to demonstrate usage of the slotchange attribute, and features of the `HTMLSlotElement` interface.

[View full source code][code] or [view the compiled example online][online]

[online]: https://rustwasm.github.io/wasm-bindgen/exbuild/web-components-slotchange/
[code]: https://github.com/rustwasm/wasm-bindgen/tree/master/examples/web-components-slotchange

## `src/lib.rs`

```rust
{{#include ../../../examples/web-components-slotchange/src/lib.rs}}
```