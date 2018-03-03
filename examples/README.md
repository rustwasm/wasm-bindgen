# Examples

This directory contains a number of examples of the `#[wasm_bindgen]` macro and
how to display them in the browser. Each directory should contain a `build.sh`
which assembles all the relevant files, and then if you open up `index.html` in
a web browser you should be able to see everything in action!

The examples here are:

* `hello_world` - the "hello world" of `#[wasm_bindgen]`, aka throwing up a
  dialog greeting you
* `smorgasboard` - a bunch of features all thrown into one, showing off the
  various capabilities of the `#[wasm_bindgen]` macro and what you can do with
  it from JS
