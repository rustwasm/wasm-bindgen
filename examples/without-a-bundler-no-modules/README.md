# Without a Bundler Using `--no-modules`

[View documentation for this example online][dox]

[dox]: https://rustwasm.github.io/wasm-bindgen/examples/without-a-bundler.html

You can build the example locally with:

```
$ wasm-pack build --target no-modules
```

and then opening `index.html` in a browser should run the example!

Note that this example is in contrast to the [without a bundler][wab] example
which performs a similar purpose except it uses `--no-modules` instead of
`--web`. The main difference here is how the shim JS and module are loaded,
where this example uses old-school `script` tags while `--web` uses ES
modules.

[wab]: https://github.com/rustwasm/wasm-bindgen/tree/master/examples/without-a-bundler
