# Introduction

`wasm-bindgen` facilitates high-level interactions between wasm modules and
JavaScript.

This project is sort of half polyfill for features like the [host bindings
proposal][host] and half features for empowering high-level interactions between
JS and wasm-compiled code (currently mostly from Rust). More specifically this
project allows JS/wasm to communicate with strings, JS objects, classes, etc, as
opposed to purely integers and floats. Using `wasm-bindgen` for example you can
define a JS class in Rust or take a string from JS or return one. The
functionality is growing as well!

Currently this tool is Rust-focused but the underlying foundation is
language-independent, and it's hoping that over time as this tool stabilizes
that it can be used for languages like C/C++!

Notable features of this project includes:

* Importing JS functionality in to Rust such as [DOM manipulation][dom-ex],
  [console logging][console-log], or [performance monitoring][perf-ex].
* [Exporting Rust functionality][smorg-ex] to JS such as classes, functions, etc.
* Working with rich types like strings, numbers, classes, closures, and objects
  rather than simply `u32` and floats.

This project is still relatively new but feedback is of course always
welcome! If you're curious about the design plus even more information about
what this crate can do, check out the [design doc].

[host]: https://github.com/WebAssembly/host-bindings
[design doc]: https://rustwasm.github.io/wasm-bindgen/design.html
[dom-ex]: https://github.com/rustwasm/wasm-bindgen/tree/master/examples/dom
[console-log]: https://github.com/rustwasm/wasm-bindgen/tree/master/examples/console_log
[perf-ex]: https://github.com/rustwasm/wasm-bindgen/tree/master/examples/performance
[smorg-ex]: https://github.com/rustwasm/wasm-bindgen/tree/master/examples/smorgasboard
[hello-online]: https://webassembly.studio/?f=gzubao6tg3
