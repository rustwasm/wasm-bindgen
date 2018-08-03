# What Just Happened?

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
