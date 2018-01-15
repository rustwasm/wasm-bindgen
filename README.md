# wasm-bindgen

A CLI and Rust dependency for generating JS bindings of an interface defined in
Rust (and maybe eventually other languages!)

[![Build Status](https://travis-ci.org/alexcrichton/wasm-bindgen.svg?branch=master)](https://travis-ci.org/alexcrichton/wasm-bindgen)
[![Build status](https://ci.appveyor.com/api/projects/status/559c0lj5oh271u4c?svg=true)](https://ci.appveyor.com/project/alexcrichton/wasm-bindgen)

This project is intended to be a framework for interoperating between JS and
Rust. Currently it's very Rust-focused but it's hoped that one day the
`wasm-bindgen-cli` tool will not be so Rust-specific and would be amenable to
bindgen for C/C++ modules.

Notable features of this project includes:

* Exposing Rust structs to JS as classes
* Exposing Rust functions to JS
* Managing arguments between JS/Rust (strings, numbers, classes, objects, etc)
* Importing JS functions with richer types (strings, objects)
* Receiving arbitrary JS objects in Rust, passing them through to JS
* Generates Typescript for now instead of JS (although that may come later)

Planned features include:

* Field setters/getters in JS through Rust functions
* ... and more coming soon!

This project is still very "early days" but feedback is of course always
welcome!

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

wasm_bindgen! {
    pub fn greet(name: &str) -> String {
        format!("Hello, {}!", name)
    }
}
```

Here we're wrapping the code we'd like to export to JS in the `wasm_bindgen!`
macro. We'll see more features later, but it suffices to say that most Rust
syntax fits inside here, it's not too special beyond what it generates!

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
  --output-ts hello.ts \
  --output-wasm hello.wasm
```

This'll create a `hello.ts` (a TypeScript file) which binds the functions
described in `js_hello_world.wasm`, and the `hello.wasm` will be a little
smaller than the input `js_hello_world.wasm`, but it's otherwise equivalent.
Note that `hello.ts` isn't very pretty so to read it you'll probably want to run
it through a formatter.

Typically you'll be feeding this typescript into a larger build system, and
often you'll be using this with your own typescript project as well. For now
though we'll just want the JS output, so let's convert it real quick:

```
$ npm install typescript @types/webassembly-js-api @types/text-encoding
$ ./node_modules/typescript/bin/tsc hello.ts --lib es6 -m es2015
```

Below we'll be using ES6 modules, but your browser may not support them natively
just yet. To see more information about this, you can browse
[online](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/import).

Ok let's see what this look like on the web!

```html
<html>
  <head>
    <meta content="text/html;charset=utf-8" http-equiv="Content-Type"/>
  </head>
  <body>
    <script type='module'>
      import { instantiate } from "./hello.js";

      // Send an async request to fetch the wasm file and get all its contents
      fetch("hello.wasm")
        .then(resp => resp.arrayBuffer())

        // Invoke the wasm-bindgen-generated function `instantiate` which will
        // give us a compiled wasm module when it's resolved
        .then(wasm => instantiate(wasm))

        // Using the module, call our Rust-exported function `greet` and then
        // use `alert` to display it
        .then(mod => {
          alert(mod.greet("world"));
        });
    </script>
  </body>
</html>
```

If you open that in a browser you should see a `Hello, world!` dialog pop up!

## What just happened?

Phew! That was a lot of words and a lot ended up happening along the way. There
were two main pieces of magic happening: the `wasm_bindgen!` macro and the
`wasm-bindgen` CLI tool.

**The `wasm_bindgen!` macro**

This macro, exported from the `wasm-bindgen` crate, is the entrypoint to
exposing Rust functions to JS. This is a procedural macro (hence requiring the
nightly Rust toolchain) which will transform the definitions inside and prepare
appropriate wrappers to receive JS-compatible types and convert them to
Rust-compatible types.

There's a more thorough explanation below of the various bits and pieces of the
macro, but it suffices for now to say that you can have free functions, structs,
and impl blocks for those structs in the macro right now. Many Rust features
aren't supported in these blocks like generics, lifetime parameters, etc.
Additionally not all types can be taken or returned from the functions. In
general though simple-ish types should work just fine!

**The `wasm-bindgen` CLI tool**

The next half of what happened here was all in the `wasm-bindgen` tool. This
tool opened up the wasm module that rustc generated and found an encoded
description of what was passed to the `wasm_bindgen!` macro. You can think of
this as the `wasm_bindgen!` macro created a special section of the output module
which `wasm-bindgen` strips and processes.

This information gave `wasm-bindgen` all it needed to know to generate the JS
file that we then imported. The JS file wraps instantiating the underlying wasm
module (aka calling `WebAssembly.instantiate`) and then provides wrappers for
classes/functions within.

Eventually `wasm-bindgen` will also take a list of imports where you can call
from Rust to JS without worrying about argument conversions and such. An example
to come here soon!

## What else can we do?

Turns out much more! Here's a taste of various features you can use in this
project:

```rust
// src/lib.rs
#![feature(proc_macro)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

wasm_bindgen! {
    // Strings can both be passed in and received
    pub fn concat(a: &str, b: &str) -> String {
        let mut a = a.to_string();
        a.push_str(b);
        return a
    }

    // A struct will show up as a class on the JS side of things
    pub struct Foo {
        contents: u32,
    }

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

    pub struct Bar {
        contents: u32,
        opaque: JsObject, // defined in `wasm_bindgen`, imported via prelude
    }

    extern "JS" {
        fn bar_on_reset(to: &str, opaque: &JsObject);
    }

    impl Bar {
        pub fn from_str(s: &str, opaque: JsObject) -> Bar {
            Bar { contents: s.parse().unwrap_or(0), opaque }
        }

        pub fn reset(&mut self, s: &str) {
            if let Ok(n) = s.parse() {
                bar_on_reset(s, &self.opaque);
                self.contents = n;
            }
        }
    }
}
```

The generated JS bindings for this invocation of the macro [look like
this][bindings]. You can view them in action like so:
[bindings]: https://gist.github.com/b7dfa241208ee858d5473c406225080f


```html
<html>
  <head>
    <meta content="text/html;charset=utf-8" http-equiv="Content-Type"/>
  </head>
  <body>
    <script type='module'>
      import { instantiate } from "./hello.js";
      function assertEq(a, b) {
        if (a !== b)
          throw new Error(`${a} != ${b}`);
        console.log(`found ${a} === ${b}`);
      }

      fetch("hello.wasm")
        .then(resp => resp.arrayBuffer())
        .then(bytes => {
          return instantiate(bytes, {
            bar_on_reset(s, token) {
              console.log(token);
              console.log(`this instance of bar was reset to ${s}`);
            },
          });
        })
        .then(mod => {
          assertEq(mod.concat('a', 'b'), 'ab');

          // Note the `new Foo()` syntax cannot be used, static function
          // constructors must be used instead. Additionally objects allocated
          // corresponding to Rust structs will need to be deallocated on the
          // Rust side of things with an explicit call to `free`.
          let foo = mod.Foo.new();
          assertEq(foo.add(10), 10);
          foo.free();

          // Pass objects to one another
          let foo1 = mod.Foo.new();
          let bar = mod.Bar.from_str("22", { opaque: 'object' });
          foo1.add_other(bar);

          // We also don't have to `free` the `bar` variable as this function is
          // transferring ownership to `foo1`
          bar.reset('34');
          foo1.consume_other(bar);

          assertEq(foo1.add(2), 22 + 34 + 2);
          foo1.free();

          alert('all passed!')
        });
    </script>
  </body>
</html>
```

## Feature reference

Here this section will attempt to be a reference for the various features
implemented in this project.

In the `wasm_bindgen!` macro you can have four items: functions, structs,
impls, and foreign modules. Impls can only contain functions. No lifetime
parameters or type parameters are allowed on any of these types. Foreign
modules must have the `"JS"` abi and currently only allow integer/string
arguments and integer return values.

All structs referenced through arguments to functions should be defined in the
macro itself. Arguments allowed are:

* Integers (not u64/i64)
* Floats
* Borrowed strings (`&str`)
* Owned strings (`String`)
* Owned structs (`Foo`) defined in the same bindgen macro
* Borrowed structs (`&Foo` or `&mut Bar`) defined in the same bindgen macro
* The `JsObject` type and `&JsObject` (not mutable references)

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
Rust and through the bindings that we're returning.

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
