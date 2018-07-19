<meta charset="utf-8"/>

# `wasm-bindgen`

**Facilitating high-level interactions between wasm modules and JavaScript.**

[Introduction blog post: "JavaScript to Rust and Back Again: A `wasm-bindgen` Tale"][post]

[host]: https://github.com/WebAssembly/host-bindings
[post]: https://hacks.mozilla.org/2018/04/javascript-to-rust-and-back-again-a-wasm-bindgen-tale/

[![Build Status](https://travis-ci.org/rustwasm/wasm-bindgen.svg?branch=master)](https://travis-ci.org/rustwasm/wasm-bindgen)
[![Build status](https://ci.appveyor.com/api/projects/status/559c0lj5oh271u4c?svg=true)](https://ci.appveyor.com/project/alexcrichton/wasm-bindgen)
[![](http://meritbadge.herokuapp.com/wasm-bindgen)](https://crates.io/crates/wasm-bindgen)
[![](https://img.shields.io/crates/d/wasm-bindgen.svg)](https://crates.io/crates/wasm-bindgen)
[![API Documentation on docs.rs](https://docs.rs/wasm-bindgen/badge.svg)](https://docs.rs/wasm-bindgen)

Import JavaScript things into Rust and export Rust things to JavaScript.

`src/lib.rs`:

```rust
#![feature(use_extern_macros, wasm_import_module)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

// Import the `window.alert` function from the Web.
#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

// Export a `greet` function from Rust to JavaScript, that alerts a
// hello message.
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
```

Use exported Rust things from JavaScript!

`index.js`:

```js
// Asynchronously load, compile, and import the Rust's WebAssembly
// and JavaScript interface.
import("./hello_world").then(module => {
  // Alert "Hello, World!"
  module.greet("World!");
});
```

## Guide

[📚 Read the `wasm-bindgen` guide here! 📚](https://rustwasm.github.io/wasm-bindgen)

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

## Contribution

**[See the "Contributing" section of the guide for information on
hacking on `wasm-bindgen`!][contributing]**

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.

[contributing]: https://rustwasm.github.io/wasm-bindgen/contributing.html
