# Examples

This directory contains a number of examples of the `#[wasm_bindgen]` macro and
how to display them in the browser. Each directory should contain a `build.sh`
which assembles all the relevant files, and then if you open up `index.html` in
a web browser you should be able to see everything in action!

The examples here are:

* `hello_world` - the "hello world" of `#[wasm_bindgen]`, aka throwing up a
  dialog greeting you
* `console_log` - a showcase of `#[wasm_bindgen]` importing classes and how to
  bind `console.log`
* `math` - like `console_log` except showing how to import Math-related
  functions instead
* `dom` - an example of accessing the global `document` object and appending
  HTML to it
* `smorgasboard` - a bunch of features all thrown into one, showing off the
  various capabilities of the `#[wasm_bindgen]` macro and what you can do with
  it from JS
* `performance` - how to import APIs like `performance.now()` and time various
  operations in Rust
* `wasm-in-wasm` - how to interact with namespaced APIs like
  `WebAssembly.Module` and shows off creation of a WebAssembly module from Rust
* `closures` - an example of how to invoke functions like `setInterval` or use
  the `onclick` property in conjunction with closures.
