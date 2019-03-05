# Without a Bundler

[View documentation for this example online][dox]

[dox]: https://rustwasm.github.io/wasm-bindgen/examples/without-a-bundler.html

You can build the example locally with:

```
$ cargo build --target wasm32-unknown-unknown --release
$ cargo run -p wasm-bindgen-cli --bin wasm-bindgen -- \
  ../../target/wasm32-unknown-unknown/release/without_a_bundler.wasm \
  --out-dir pkg \
  --browser
```

and then opening `index.html` in a browser should run the example!
