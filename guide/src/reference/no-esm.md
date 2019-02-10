# No ES Modules

Explained a bit more in the [internal design](../contributing/design/index.md) section one of the
key foundational principles of `wasm-bindgen` is ES modules. It supports working
without ES modules, however! Not all JS tooling and browsers are ready for ES
modules by default, so it can sometimes be helpful to quickly get up and running
without them to kick the tires and see how `wasm-bindgen` works.

Let's start out with our hello-world example from previous chapters, and you can
also [follow along in the repository][repo].

[repo]: https://github.com/rustwasm/wasm-bindgen/tree/master/examples/no_modules

```rust
#[wasm_bindgen]
extern "C" {
    fn alert(msg: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    alert(&format!("Hello, {}!", name);
}
```

Like usual, we first compile this to wasm:

```
$ cargo build --target wasm32-unknown-unknown
```

Next, to avoid using ES modules, pass the `--no-modules` option to the
`wasm-bindgen` command:

```
$ wasm-bindgen target/wasm32-unknown-unknown/debug/hello.wasm --no-modules --out-dir .
```

Next up we need to write some HTML to interact with the wasm:

```html
<html>
  <body>
    <script src='./hello.js'></script>
    <script>
      wasm_bindgen('./hello_bg.wasm')
        .then(() => wasm_bindgen.greet('World'));
    </script>
  </body>
</html>
```

and that's it! If you open up that web page in a browser (needs to be over HTTP)
then you should see an alert for "Hello, World!".

The `--no-modules` output will not instantiate or compile the wasm module when
included on a web page, instead it just parses and configures the JS bindings
for the wasm-module-to-be. The page is configured with one exported global, in
this case `wasm_bindgen`. The name of this global can be configured with the
`--no-modules-global` option.

The global `wasm_bindgen` is a function that takes one argument: either the path
to the wasm file to fetch or a `WebAssembly.Module`. When invoked `wasm_bindgen`
will return a promise for when the wasm module is ready-to-go. After that all
exported functionality on `wasm_bindgen` will be functional.

In the example above, after calling `wasm_bindgen('./hello_bg.wasm')` we wait
for the wasm module to be fetched and compiled, and afterwards we're invoking
our `greet` export.

Note that exports are available for binding before the wasm module has been
instantiated, for example this would have also worked:

```js
const { greet } = wasm_bindgen;

wasm_bindgen('./hello_bg.wasm')
  .then(() => greet('World'));
```
