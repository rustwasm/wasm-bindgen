# What Else Can We Do?

Much more! Here's a taste of various features you can use in this project. You
can also [explore this code online](https://webassembly.studio/?f=t61j18noqz):

```rust,ignore
// src/lib.rs
#![feature(use_extern_macros, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

// Strings can both be passed in and received
#[wasm_bindgen]
pub fn concat(a: &str, b: &str) -> String {
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
    #[wasm_bindgen(constructor)]
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
        let contents = s.parse().unwrap_or_else(|_| {
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

  // Note that to use `new Foo()` the constructor function must be annotated
  // with `#[wasm_bindgen(constructor)]`, otherwise only `Foo.new()` can be used.
  // Additionally objects allocated corresponding to Rust structs will need to
  // be deallocated on the Rust side of things with an explicit call to `free`.
  let foo = new Foo();
  assertEq(foo.add(10), 10);
  foo.free();

  // Pass objects to one another
  let foo1 = new Foo();
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
