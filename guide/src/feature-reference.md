# Feature Reference

Here this section will attempt to be a reference for the various features
implemented in this project. This is likely not exhaustive but the [tests]
should also be a great place to look for examples.

[tests]: https://github.com/rustwasm/wasm-bindgen/tree/master/tests

The `#[wasm_bindgen]` attribute can be attached to functions, structs,
impls, and foreign modules. Impls can only contain functions, and the attribute
cannot be attached to functions in an impl block or functions in a foreign
module. No lifetime parameters or type parameters are allowed on any of these
types. Foreign modules must have the `"C"` abi (or none listed). Free functions
with `#[wasm_bindgen]` might not have the `"C"` abi or none listed, and it's also not
necessary to annotate with the `#[no_mangle]` attribute.

All structs referenced through arguments to functions should be defined in the
macro itself. Arguments allowed implement the `WasmBoundary` trait, and examples
are:

* Integers (u64/i64 require `BigInt` support)
* Floats
* Borrowed strings (`&str`)
* Owned strings (`String`)
* Exported structs (`Foo`, annotated with `#[wasm_bindgen]`)
* Exported C-like enums (`Foo`, annotated with `#[wasm_bindgen]`)
* Imported types in a foreign module annotated with `#[wasm_bindgen]`
* Borrowed exported structs (`&Foo` or `&mut Bar`)
* The `JsValue` type and `&JsValue` (not mutable references)
* Vectors and slices of supported integer types and of the `JsValue` type.

All of the above can also be returned except borrowed references. Passing
`Vec<JsValue>` as an argument to a function is not currently supported. Strings are
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
