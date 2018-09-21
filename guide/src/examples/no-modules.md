# Using `--no-modules`

[View full source code][code]

[code]: https://github.com/rustwasm/wasm-bindgen/tree/master/examples/no_modules

This example shows how the `--no-modules` flag can be used load code in a
browser directly (using the same code as the [hello world example][hello]).
Most of the magic happens in `index.html`:

```html
{{#include ../../../examples/no_modules/index.html}}
```

And that's it! It's worth pointing out that if [`#[wasm_bindgen(module =
"...")]` imports are used][mod-imp] then `wasm-bindgen --no-modules` will fail
(as it doesn't know how to import modules).

[hello]: hello-world.html
[mod-imp]: ../reference/attributes/on-js-imports/module.html
