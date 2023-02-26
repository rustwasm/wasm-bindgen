# Wasm in Web Worker

[View full source code][code] or [view compiled example online][online]

[code]: https://github.com/rustwasm/wasm-bindgen/tree/master/examples/wasm-in-web-worker
[online]: https://sgasse.github.io/wasm_worker_interaction/wasm_worker_interaction/www/index.html

A simple example of parallel execution by spawning a web worker with `web_sys`,
loading Wasm code in the web worker and interacting between the main thread and
the worker.

## Building & compatibility

At the time of this writing, only Chrome supports modules in web workers, e.g.
Firefox does not. To have compatibility across browsers, the whole example is
set up without relying on ES modules as target. Therefore we have to build
with `--target no-modules`. The full command can be found in `build.sh`.

However note that this may no longer be true when you read this. The
[related bug][ff_worker_module_bug] has been closed. ES modules in web workers
already work in Firefox 111.a nightly build. If you want to check if ES
modules work as target for web workers in your current browser, check out if
[this example][wasm_worker_with_modules] behaves as expected.

[ff_worker_module_bug]: https://bugzilla.mozilla.org/show_bug.cgi?id=1247687
[wasm_worker_with_modules]: https://sgasse.github.io/wasm_worker_interaction/wasm_module_js_worker/www/index.html

## `Cargo.toml`

The `Cargo.toml` enables features necessary to work with the DOM, log output to
the JS console, creating a worker and reacting to message events.

```toml
{{#include ../../../examples/wasm-in-web-worker/Cargo.toml}}
```

## `src/lib.rs`

Creates a struct `NumberEval` with methods to act as stateful object in the
worker and function `startup` to be launched in the main thread. Also includes
internal helper functions `setup_input_oninput_callback` to attach a
`wasm_bindgen::Closure` as callback to the `oninput` event of the input field
and `get_on_msg_callback` to create a `wasm_bindgen::Closure` which is triggered
when the worker returns a message.

```rust
{{#include ../../../examples/wasm-in-web-worker/src/lib.rs}}
```

## `index.html`

Includes the input element `#inputNumber` to type a number into and a HTML
element `#resultField` were the result of the evaluation even/odd is written to.
Since we require to build with `--target no-modules` to be able to load Wasm
code in in the worker across browsers, the `index.html` also includes loading
both `wasm_in_web_worker.js` and `index.js`.

```html
{{#include ../../../examples/wasm-in-web-worker/index.html}}
```

## `index.js`

Loads our Wasm file asynchronously and calls the entry point `startup` of the
main thread which will create a worker.

```js
{{#include ../../../examples/wasm-in-web-worker/index.js}}
```

## `worker.js`

Loads our Wasm file by first importing `wasm_bindgen` via
`importScripts('./pkg/wasm_in_web_worker.js')` and then awaiting the Promise
returned by `wasm_bindgen(...)`. Creates a new object to do the background
calculation and bind a method of the object to the `onmessage` callback of the
worker.

```js
{{#include ../../../examples/wasm-in-web-worker/worker.js}}
```
