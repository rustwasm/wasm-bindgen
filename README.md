<meta charset="utf-8"/>

# `wasm-bindgen`

**Facilitating high-level interactions between wasm modules and JavaScript.**

[![Build Status](https://travis-ci.org/rustwasm/wasm-bindgen.svg?branch=master)](https://travis-ci.org/rustwasm/wasm-bindgen)
[![Build status](https://ci.appveyor.com/api/projects/status/559c0lj5oh271u4c?svg=true)](https://ci.appveyor.com/project/alexcrichton/wasm-bindgen)
[![](http://meritbadge.herokuapp.com/wasm-bindgen)](https://crates.io/crates/wasm-bindgen)
[![](https://img.shields.io/crates/d/wasm-bindgen.svg)](https://crates.io/crates/wasm-bindgen)
[![API Documentation on docs.rs](https://docs.rs/wasm-bindgen/badge.svg)](https://docs.rs/wasm-bindgen)

Import JavaScript things into Rust and export Rust things to JavaScript.

```rust
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

Use exported Rust things from JavaScript with ECMAScript modules!

```js
import { greet } from "./hello_world";

greet("World!");
```

## Features

* **Lightweight.** Only pay for what you use. `wasm-bindgen` only generates
  bindings and glue for the JavaScript imports you actually use and Rust
  functionality that you export. For example, importing and using the
  `document.querySelector` method doesn't cause `Node.prototype.appendChild` or
  `window.alert` to be included in the bindings as well.

* **ECMAScript modules.** Just import WebAssembly modules the same way you would
  import JavaScript modules. Future compatible with [WebAssembly modules and
  ECMAScript modules integration][wasm-es-modules].

* **Designed with the ["host bindings" proposal][host-bindings] in mind.**
  Eventually, there won't be any JavaScript shims between Rust-generated wasm
  functions and native DOM methods. Because the wasm functions are statically
  type checked, some of those native methods' dynamic type checks should become
  unnecessary, promising to unlock even-faster-than-JavaScript DOM access.

[wasm-es-modules]: https://github.com/WebAssembly/esm-integration
[host-bindings]: https://github.com/WebAssembly/host-bindings/blob/master/proposals/host-bindings/Overview.md

## Guide

[**📚 Read the `wasm-bindgen` guide here! 📚**](https://rustwasm.github.io/wasm-bindgen)

## API Docs

- [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/)
- [js-sys](https://rustwasm.github.io/wasm-bindgen/api/js_sys/)
- [web-sys](https://rustwasm.github.io/wasm-bindgen/api/web_sys/)

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
