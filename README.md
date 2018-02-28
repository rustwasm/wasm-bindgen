# wasm-bindgen

A project for facilitating high-level interactions between wasm modules and JS.

[host]: https://github.com/WebAssembly/host-bindings

[![Build Status](https://travis-ci.org/alexcrichton/wasm-bindgen.svg?branch=master)](https://travis-ci.org/alexcrichton/wasm-bindgen)
[![Build status](https://ci.appveyor.com/api/projects/status/559c0lj5oh271u4c?svg=true)](https://ci.appveyor.com/project/alexcrichton/wasm-bindgen)

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

* Exposing Rust structs to JS as classes
* Exposing Rust functions to JS
* Managing arguments between JS/Rust (strings, numbers, classes, objects, etc)
* Importing JS functions with richer types (strings, objects)
* Importing JS classes and calling methods
* Receiving arbitrary JS objects in Rust, passing them through to JS
* Catching JS exceptions in imports

Planned features include:

* Field setters/getters in JS through Rust functions
* ... and more coming soon!

This project is still very "early days" but feedback is of course always
welcome! If you're curious about the design plus even more information about
what this crate can do, check out the [design doc].

[design doc]: https://github.com/alexcrichton/wasm-bindgen/blob/master/DESIGN.md

## Basic usage

Let's implement the equivalent of "Hello, world!" for this crate.

> **Note:** Currently this projects uses *nightly Rust* which you can acquire
> through [rustup] and configure with `rustup default nightly`

[rustup]: https://rustup.rs

First up, let's add the wasm target and generate a Rust project:

```
$ rustup target add wasm32-unknown-unknown
$ cargo new js-hello-world
```

Now let's add a dependency on this project inside `Cargo.toml` as well as
configuring our build output:

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = { git = 'https://github.com/alexcrichton/wasm-bindgen' }
```

Next up our actual code! We'll write this in `src/lib.rs`:

```rust
#![feature(proc_macro)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
#[no_mangle]
pub extern fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
```

And that's it! If we were to write the `greet` function naively without the
`#[wasm_bindgen]` attribute then JS wouldn't be able to communicate with the
types like `str`, so slapping a `#[wasm_bindgen]` on the function and the import
of `alert` ensures that the right shims are generated.

Next up let's build our project:

```
$ cargo build --release --target wasm32-unknown-unknown
```

Note that we're using `--release` here because unfortunately the current LLVM
backend for wasm has a few bugs in non-optimized mode. Those bugs will hopefully
get smoothed out over time!

After this you'll have a wasm file at
`target/wasm32-unknown-unknown/release/js_hello_world.wasm`. If you'd like you
can use [wasm-gc] to make this file a little smaller

[wasm-gc]: https://github.com/alexcrichton/wasm-gc

Now that we've generated the wasm module it's time to run the bindgen tool
itself! Let's install it:

```
$ cargo install --git https://github.com/alexcrichton/wasm-bindgen
```

This'll install a `wasm-bindgen` binary next to your `cargo` binary. This tool
will postprocess the wasm file rustc generated, generating a new wasm file and a
set of JS bindings as well. Let's invoke it!

```
$ wasm-bindgen target/wasm32-unknown-unknown/release/js_hello_world.wasm \
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

The `wasm-bindgen` tool also emits a secondary file, `js_hello_world_wasm.wasm`.
This is the original wasm file but postprocessed a bit. It's intended that the
`js_hello_world_wasm.wasm` file, like before, acts like an ES6 module. The
`js_hello_world.wasm` file, for example, uses `import` to import functionality
from the wasm.

Note that you can also pass a `--nodejs` argument to `wasm-bindgen` for emitting
Node-compatible JS as well as a `--typescript` argument to emit a `*.d.ts` file
describing the exported contents.

At this point you'll typically plug these files into a larger build system. Both
files emitted by `wasm-bindgen` act like normal ES6 modules (one just happens to
be wasm). As of the time of this writing there's unfortunately not a lot of
tools that natively do this (but they're coming!). In the meantime we can use
the `wasm2es6js` utility (aka "hack") from the `wasm-bindgen` tool we previously
installed along with the `parcel-bundler` packager. Note that these steps will
differ depending on your build system.

Alright first create an `index.js` file:

```js
import { greet } from "./js_hello_world";
import { booted } from "./js_hello_world_wasm";

booted.then(() => {
  greet("World!");
});
```

Then a corresponding `index.html`:

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

And run a local server with these files:

```
# Convert `*.wasm` to `*.js` where the JS internally instantiates the wasm
$ wasm2es6js js_hello_world_wasm.wasm -o js_hello_world_wasm.js --base64

# Install parcel and run it against the index files we use below.
$ npm install -g parcel-bundler
$ parcel index.html
```

If you open that in a browser you should see a `Hello, world!` dialog pop up!

## What just happened?

Phew! That was a lot of words and a lot ended up happening along the way. There
were two main pieces of magic happening: the `#[wasm_bindgen]` attribute and the
`wasm-bindgen` CLI tool.

**The `#[wasm_bindgen]` attribute**

This attribute, exported from the `wasm-bindgen` crate, is the entrypoint to
exposing Rust functions to JS. This is a procedural macro (hence requiring the
nightly Rust toolchain) which will generate the appropriate shims in Rust to
translate from your type signature to one that JS can interface with. Finally
the attribute also serializes some information to the output artifact which
`wasm-bindgen`-the-tool will discard after it parses.

There's a more thorough explanation below of the various bits and pieces of the
attribute, but it suffices for now to say that you can attach it to free
functions, structs, impl blocks for those structs and `extern { ... }` blocks.
Some Rust features like generics, lifetime parameters, etc, aren't supported on
functions tagged with `#[wasm_bindgen]` right now.

**The `wasm-bindgen` CLI tool**

The next half of what happened here was all in the `wasm-bindgen` tool. This
tool opened up the wasm module that rustc generated and found an encoded
description of what was passed to the `#[wasm_bindgen]` attribute. You can
think of this as the `#[wasm_bindgen]` attribute created a special section of
the output module which `wasm-bindgen` strips and processes.

This information gave `wasm-bindgen` all it needed to know to generate the JS
file that we then imported. The JS file wraps instantiating the underlying wasm
module (aka calling `WebAssembly.instantiate`) and then provides wrappers for
classes/functions within.

## What else can we do?

Much more! Here's a taste of various features you can use in this project:

```rust
// src/lib.rs
#![feature(proc_macro)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

// Strings can both be passed in and received
#[wasm_bindgen]
#[no_mangle]
pub extern fn concat(a: &str, b: &str) -> String {
    let mut a = a.to_string();
    a.push_str(b);
    return a
}

// A struct will show up as a class on the JS side of things
#[wasm_bindgen]
pub struct Foo {
    contents: u32,
}

#[wasm_bindgen]
impl Foo {
    pub fn new() -> Foo {
        Foo { contents: 0 }
    }

    // Methods can be defined with `&mut self` or `&self`, and arguments you
    // can pass to a normal free function also all work in methods.
    pub fn add(&mut self, amt: u32) -> u32 {
        self.contents += amt;
        return self.contents
    }

    // You can also take a limited set of references to other types as well.
    pub fn add_other(&mut self, bar: &Bar) {
        self.contents += bar.contents;
    }

    // Ownership can work too!
    pub fn consume_other(&mut self, bar: Bar) {
        self.contents += bar.contents;
    }
}

#[wasm_bindgen]
pub struct Bar {
    contents: u32,
    opaque: JsValue, // defined in `wasm_bindgen`, imported via prelude
}

#[wasm_bindgen(module = "./index")] // what ES6 module to import from
extern {
    fn bar_on_reset(to: &str, opaque: &JsValue);

    // We can import classes and annotate functionality on those classes as well
    type Awesome;
    #[wasm_bindgen(constructor)]
    fn new() -> Awesome;
    #[wasm_bindgen(method)]
    fn get_internal(this: &Awesome) -> u32;
}

#[wasm_bindgen]
impl Bar {
    pub fn from_str(s: &str, opaque: JsValue) -> Bar {
        let contents = s.parse().unwrap_or_else(|| {
            Awesome::new().get_internal()
        });
        Bar { contents, opaque }
    }

    pub fn reset(&mut self, s: &str) {
        if let Ok(n) = s.parse() {
            bar_on_reset(s, &self.opaque);
            self.contents = n;
        }
    }
}
```

The generated JS bindings for this invocation of the macro [look like
this][bindings]. You can view them in action like so:

[bindings]: https://gist.github.com/alexcrichton/3d85c505e785fb8ff32e2c1cf9618367

and our corresponding `index.js`:

```js
import { Foo, Bar, concat } from "./js_hello_world";
import { booted } from "./js_hello_world_wasm";

export function bar_on_reset(s, token) {
  console.log(token);
  console.log(`this instance of bar was reset to ${s}`);
}

function assertEq(a, b) {
  if (a !== b)
    throw new Error(`${a} != ${b}`);
  console.log(`found ${a} === ${b}`);
}

function main() {
  assertEq(concat('a', 'b'), 'ab');

  // Note the `new Foo()` syntax cannot be used, static function
  // constructors must be used instead. Additionally objects allocated
  // corresponding to Rust structs will need to be deallocated on the
  // Rust side of things with an explicit call to `free`.
  let foo = Foo.new();
  assertEq(foo.add(10), 10);
  foo.free();

  // Pass objects to one another
  let foo1 = Foo.new();
  let bar = Bar.from_str("22", { opaque: 'object' });
  foo1.add_other(bar);

  // We also don't have to `free` the `bar` variable as this function is
  // transferring ownership to `foo1`
  bar.reset('34');
  foo1.consume_other(bar);

  assertEq(foo1.add(2), 22 + 34 + 2);
  foo1.free();

  alert('all passed!')
}

export class Awesome {
  constructor() {
    this.internal = 32;
  }

  get_internal() {
    return this.internal;
  }
}

booted.then(main);
```

## Feature reference

Here this section will attempt to be a reference for the various features
implemented in this project. This is likely not exhaustive but the [tests]
should also be a great place to look for examples.

[tests]: https://github.com/alexcrichton/wasm-bindgen/tree/master/tests

The `#[wasm_bindgen]` attribute can be attached to functions, structs,
impls, and foreign modules. Impls can only contain functions, and the attribute
cannot be attached to functions in an impl block or functions in a foreign
module. No lifetime parameters or type parameters are allowed on any of these
types. Foreign modules must have the `"C"` abi (or none listed). Free functions
with `#[wasm_bindgen]` must also have the `"C"` abi or none listed and also be
annotated with the `#[no_mangle]` attribute.

All structs referenced through arguments to functions should be defined in the
macro itself. Arguments allowed implement the `WasmBoundary` trait, and examples
are:

* Integers (not u64/i64)
* Floats
* Borrowed strings (`&str`)
* Owned strings (`String`)
* Exported structs (`Foo`, annotated with `#[wasm_bindgen]`)
* Exported C-like enums (`Foo`, annotated with `#[wasm_bindgen]`)
* Imported types in a foreign module annotated with `#[wasm_bindgen]`
* Borrowed exported structs (`&Foo` or `&mut Bar`)
* The `JsValue` type and `&JsValue` (not mutable references)
* Vectors and slices of supported integer types and of the `JsValue` type.

All of the above can also be returned except borrowed references. Strings are
implemented with shim functions to copy data in/out of the Rust heap. That is, a
string passed to Rust from JS is copied to the Rust heap (using a generated shim
to malloc some space) and then will be freed appropriately.

Owned values are implemented through boxes. When you return a `Foo` it's
actually turned into `Box<RefCell<Foo>>` under the hood and returned to JS as a
pointer. The pointer is to have a defined ABI, and the `RefCell` is to ensure
safety with reentrancy and aliasing in JS. In general you shouldn't see
`RefCell` panics with normal usage.

JS-values-in-Rust are implemented through indexes that index a table generated
as part of the JS bindings. This table is managed via the ownership specified in
Rust and through the bindings that we're returning. More information about this
can be found in the [design doc].

All of these constructs currently create relatively straightforward code on the
JS side of things, mostly having a 1:1 match in Rust with JS.

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.

### Tests

In order to run the tests you will need (node.js)[https://nodejs.org/] version
8.9.4 or above. Running the tests is done by running `cargo test`.
