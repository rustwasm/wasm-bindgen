# Basic Usage

Let's implement the equivalent of "Hello, world!" for this crate.

> **Note:** Currently this projects uses *nightly Rust* which you can acquire
> through [rustup] and configure with `rustup default nightly`

[rustup]: https://rustup.rs

If you'd like you dive [straight into an online example][hello-online], but
if you'd prefer to follow along in your own console let's install the tools we
need:

```shell
$ rustup target add wasm32-unknown-unknown
$ cargo +nightly install wasm-bindgen-cli
```

The first command here installs the wasm target so you can compile to it, and
the latter will install the `wasm-bindgen` CLI tool we'll be using later.

Next up let's make our project

```shell
$ cargo +nightly new js-hello-world --lib
```

Now let's add a dependency on this project inside `Cargo.toml` as well as
configuring our build output:

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
```

Next up our actual code! We'll write this in `src/lib.rs`:

```rust,ignore
#![feature(use_extern_macros, wasm_import_module)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
```

And that's it! If we were to write the `greet` function naively without the
`#[wasm_bindgen]` attribute then JS wouldn't be able to communicate with the
types like `str`, so slapping a `#[wasm_bindgen]` on the function and the import
of `alert` ensures that the right shims are generated.

Next up let's build our project:

```shell
$ cargo +nightly build --target wasm32-unknown-unknown
```

After this you'll have a wasm file at
`target/wasm32-unknown-unknown/debug/js_hello_world.wasm`. Don't be alarmed at
the size, this is an unoptimized program!

Now that we've generated the wasm module it's time to run the bindgen tool
itself! This tool will postprocess the wasm file rustc generated, generating a
new wasm file and a set of JS bindings as well. Let's invoke it!

```shell
$ wasm-bindgen target/wasm32-unknown-unknown/debug/js_hello_world.wasm \
  --out-dir .
```

This is the main point where the magic happens. The `js_hello_world.wasm` file
emitted by rustc contains *descriptors* of how to communicate via richer types
than wasm currently supports. The `wasm-bindgen` tool will interpret this
information, emitting a **replacement module** for the wasm file.

The previous `js_hello_world.wasm` file is interpreted as if it were an ES6
module. The `js_hello_world.js` file emitted by `wasm-bindgen` should have the
intended interface of the wasm file, notably with rich types like strings,
classes, etc.

The `wasm-bindgen` tool also emits a few other files needed to implement this
module. For example `js_hello_world_bg.wasm` is the original wasm file but
postprocessed a bit. It's intended that the `js_hello_world_bg.wasm` file,
like before, acts like an ES6 module.

At this point you'll probably plug these files into a larger build system.
Files emitted by `wasm-bindgen` act like normal ES6 modules (one just happens to
be wasm). As of the time of this writing there's unfortunately not a lot of
tools that natively do this, but Webpack's 4.0 beta release has native wasm
support!. Let's take a look at that and see how it works.

First create an `index.js` file:

```js
const js = import("./js_hello_world");

js.then(js => {
  js.greet("World!");
});
```

Note that we're using `import(..)` here because Webpack [doesn't
support][webpack-issue] synchronously importing modules from the main chunk just
yet.

[webpack-issue]: https://github.com/webpack/webpack/issues/6615

Next our JS dependencies by creating a `package.json`:

```json
{
  "scripts": {
    "serve": "webpack-dev-server"
  },
  "devDependencies": {
    "webpack": "^4.0.1",
    "webpack-cli": "^2.0.10",
    "webpack-dev-server": "^3.1.0"
  }
}
```

and our webpack configuration

```js
// webpack.config.js
const path = require('path');

module.exports = {
  entry: "./index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
  },
  mode: "development"
};
```

Our corresponding `index.html`:

```html
<html>
  <head>
    <meta content="text/html;charset=utf-8" http-equiv="Content-Type"/>
  </head>
  <body>
    <script src='./index.js'></script>
  </body>
</html>
```

And finally:

```shell
$ npm install
$ npm run serve
```

If you open [http://localhost:8080](http://localhost:8080) in a browser you should see a `Hello, world!`
dialog pop up!

If that was all a bit much, no worries! You can [execute this code
online][hello-online] thanks to [WebAssembly Studio](https://webassembly.studio)
or you can [follow along on GitHub][hello-tree] to see all the files necessary
as well as a script to set it all up.

[hello-tree]: https://github.com/rustwasm/wasm-bindgen/tree/master/examples/hello_world
[hello-readme]: https://github.com/rustwasm/wasm-bindgen/tree/master/examples/hello_world/README.md
[hello-online]: https://webassembly.studio/?f=gzubao6tg3
