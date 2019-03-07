# The `wasm-bindgen` Command Line Interface

The `wasm-bindgen` command line tool has a number of options available to it to
tweak the JavaScript that is generated. The most up-to-date set of flags can
always be listed via `wasm-bindgen --help`.

> Note: usually, one should use a [`wasm-pack`-based workflow][wasm-pack] rather
> than running the `wasm-bindgen` command line tool by hand.

[wasm-pack]: https://github.com/rustwasm/wasm-pack

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

For more information about this see the section on [deployment]

[deployment]: deployment.html

### `--web`

This flag will generate output suitable for loading natively in browsers today.
The generated JS shims are an ES module which export a `default` instantiation
function, like `--no-modules` below.

For more information about this see the section on [deployment]

### `--no-modules` and  `--no-modules-global VAR`

The default output of `wasm-bindgen` uses ECMAScript modules. These options
indicate that ECMAScript modules should *not* be used, and that output should be
tailored for a properties on the JavaScript global object (e.g. `window`).

The `--no-modules-global VAR` option makes `VAR` the global property that the
JavaScript bindings are attached to.

For more information about this see the section on [deployment]

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

### `--browser`

When generating bundler-compatible code (see the section on [deployment]) this
indicates that the bundled code is always intended to go into a browser so a few
checks for Node.js can be elided.
