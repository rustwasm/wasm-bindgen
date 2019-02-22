# Without a Bundler

[View full source code][code]

[code]: https://github.com/rustwasm/wasm-bindgen/tree/master/examples/without-a-bundler

This example shows how the `--no-modules` flag can be used load code in a
browser directly. For this deployment strategy bundlers like Webpack are not
required. For more information on deployment see the [dedicated
documentation](../reference/deployment.html).

First let's take a look at the code and see how when we're using `--no-modules`
we're not actually losing any functionality!

```rust
{{#include ../../../examples/without-a-bundler/src/lib.rs}}
```

Otherwise the rest of the deployment magic happens in `index.html`:

```html
{{#include ../../../examples/without-a-bundler/index.html}}
```

And that's it! Be sure to read up on the [deployment options] to see what it
means to deploy without a bundler.

[hello]: hello-world.html
[mod-imp]: ../reference/attributes/on-js-imports/module.html
