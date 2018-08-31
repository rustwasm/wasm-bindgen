# Examples

This directory contains a number of examples of the `#[wasm_bindgen]` macro and
how to display them in the browser. Each directory contains a README with a link
to https://webassembly.studio so you can also explore the example online
(apologies if they're out of sync!), and each directory also contains a
`build.sh` which assembles all the relevant files locally. If you open up
`index.html` in a web browser you should be able to see everything in action
when using `build.sh`!

The examples here are:

* `add` - an example of generating a tiny wasm binary, one that only adds two
  numbers.
* `asm.js` - an example of using the `wasm2asm` tool from [binaryen] to convert
  the generated WebAssembly to normal JS
* `char` - an example of passing the rust `char` type to and from the js `string` type
* `closures` - an example of how to invoke functions like `setInterval` or use
  the `onclick` property in conjunction with closures.
* `comments` - an example of how Rust comments are copied into js bindings
* `console_log` - a showcase of `#[wasm_bindgen]` importing classes and how to
  bind `console.log`
* `dom` - an example of accessing the global `document` object and appending
  HTML to it
* `fetch` -- how to use the Fetch API to make async http requests 
* `hello_world` - the "hello world" of `#[wasm_bindgen]`, aka throwing up a
  dialog greeting you
* `import_js` - an example of importing local JS functionality into a crate
* `math` - like `console_log` except showing how to import Math-related
  functions instead
* `no_modules` - an example of how to use the `--no-modules` flag to
  the `wasm-bindgen` CLI tool
* `performance` - how to import APIs like `performance.now()` and time various
  operations in Rust
* `smorgasboard` - a bunch of features all thrown into one, showing off the
  various capabilities of the `#[wasm_bindgen]` macro and what you can do with
  it from JS
* `wasm-in-wasm` - how to interact with namespaced APIs like
  `WebAssembly.Module` and shows off creation of a WebAssembly module from Rust
* `webaudio` - how to use the Web Audio APIs to generate sounds

[binaryen]: https://github.com/WebAssembly/binaryen
