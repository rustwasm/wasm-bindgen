# The `wasm-bindgen` Command Line Interface

The `wasm-bindgen` command line tool has a number of options available to it to
tweak the JavaScript that is generated. The most up-to-date set of flags can
always be listed via `wasm-bindgen --help`.

## Usage

```
wasm-bindgen [options] ./target/wasm32-unknown-unknown/release/crate.wasm
```

## Options

### `--out-dir DIR`

The target directory to emit the JavaScript bindings, TypeScript definitions,
processed `.wasm` binary, etc...

### `--nodejs`

This flag will tailor output for Node instead of browsers, allowing for native
usage of `require` of the generated JS and internally using `require` instead of
ECMAScript modules. When using this flag no further postprocessing (aka a
bundler) should be necessary to work with the wasm.

### `--browser`

This flag will tailor the output specifically for browsers, making it
incompatible with Node. This will basically make the generated JS a tiny bit
smaller as runtime checks for Node won't be necessary.

### `--no-modules` and  `--no-modules-global VAR`

The default output of `wasm-bindgen` uses ECMAScript modules. These options
indicate that ECMAScript modules should *not* be used, and that output should be
tailored for a properties on the JavaScript global object (e.g. `window`).

The `--no-modules-global VAR` option makes `VAR` the global property that the
JavaScript bindings are attached to.

More information can be found in the [documentation for building without
ECMAScript modules](./no-esm.html).

### `--typescript`

Output a TypeScript declaration file for the generated JavaScript bindings. This
is on by default.

### `--no-typescript`

By default, a `*.d.ts` TypeScript declaration file is generated for the
generated JavaScript bindings, but this flag will disable that.

### `--debug`

Generates a bit more JS and wasm in "debug mode" to help catch programmer
errors, but this output isn't intended to be shipped to production.

### `--no-demangle`

When post-processing the `.wasm` binary, do not demangle Rust symbols in the
"names" custom section.

### `--keep-debug`

When post-processing the `.wasm` binary, do not strip DWARF debug info custom
sections.
