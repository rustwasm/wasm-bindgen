# Without a Bundler

[View full source code][code]

[code]: https://github.com/rustwasm/wasm-bindgen/tree/master/examples/without-a-bundler

This example shows how the `--web` flag can be used load code in a
browser directly. For this deployment strategy bundlers like Webpack are not
required. For more information on deployment see the [dedicated
documentation][deployment].

First let's take a look at the code and see how when we're using `--web`
we're not actually losing any functionality!

```rust
{{#include ../../../examples/without-a-bundler/src/lib.rs}}
```

Otherwise the rest of the deployment magic happens in `index.html`:

```html
{{#include ../../../examples/without-a-bundler/index.html}}
```

And that's it! Be sure to read up on the [deployment options][deployment] to see
what it means to deploy without a bundler.

[deployment]: ../reference/deployment.html

## Using the older `--no-modules`

[View full source code][code]

[code]: https://github.com/rustwasm/wasm-bindgen/tree/master/examples/without-a-bundler-no-modules

The older version of using `wasm-bindgen` without a bundler is to use the
`--no-modules` flag to the `wasm-bindgen` CLI. This corresponds to `--target
no-modules` in `wasm-pack`.

While similar to the newer `--web`, the `--no-modules` flag has a few
caveats:

* It does not support [local JS snippets][snippets]
* It does not generate an ES module

With that in mind the main difference is how the wasm/JS code is loaded, and
here's an example of loading the output of `wasm-pack` for the same module as
above.

```html
{{#include ../../../examples/without-a-bundler-no-modules/index.html}}
```

[snippets]: ../reference/js-snippets.html
