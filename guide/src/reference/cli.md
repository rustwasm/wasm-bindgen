# The `wasm-bindgen` Command Line Interface

The `wasm-bindgen` command line tool has a number of options available to it to
tweak the JavaScript that is generated. The most up-to-date set of flags can
always be listed via `wasm-bindgen --help`.

> Note: usually, one should use a [`wasm-pack`-based workflow][wasm-pack] rather
> than running the `wasm-bindgen` command line tool by hand.

[wasm-pack]: https://github.com/rustwasm/wasm-pack

## Installation

The recommend way to install the `wasm-bindgen` command line tool is with the
`wasm-pack` installer described
[here](https://rustwasm.github.io/wasm-pack/installer/). After installing
`wasm-pack`, run `wasm-pack build` to install the command line tool. (Although
the command line tool is available through its own crate, `wasm-bindgen-cli`,
`wasm-pack build` will ensure the version installed matches the version of
`wasm-bindgen` in `Cargo.lock`.)

## Usage

```
wasm-bindgen [options] ./target/wasm32-unknown-unknown/release/crate.wasm
```

## Options

### `--out-dir DIR`

The target directory to emit the JavaScript bindings, TypeScript definitions,
processed `.wasm` binary, etc...

### `--target`

This flag indicates what flavor of output what `wasm-bindgen` should generate.
For example it could generate code to be loaded in a bundler like Webpack, a
native web page, or Node.js. For a full list of options to pass this flag, see
the section on [deployment]

[deployment]: deployment.html

### `--no-modules-global VAR`

When `--target no-modules` is used this flag can indicate what the name of the
global to assign generated bindings to.

For more information about this see the section on [deployment]

### `--typescript`

Output a TypeScript declaration file for the generated JavaScript bindings. This
is on by default.

### `--no-typescript`

By default, a `*.d.ts` TypeScript declaration file is generated for the
generated JavaScript bindings, but this flag will disable that.

### `--omit-imports`

When the `module` attribute is used with the `wasm-bindgen` macro, the code
generator will emit corresponding `import` or `require` statements in the header
section of the generated javascript. This flag causes those import statements to
be omitted. This is necessary for some use cases, such as generating javascript
which is intended to be used with Electron (with node integration disabled),
where the imports are instead handled through a separate preload script.

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
