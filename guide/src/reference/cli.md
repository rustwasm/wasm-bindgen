# The `wasm-bindgen` Command Line Interface

The `wasm-bindgen` tool has a number of options available to it to tweak the JS
that is generated. By default the generated JS uses ES modules and is compatible
with both Node and browsers (but will likely require a bundler for both use
cases).

Supported flags of the CLI tool can be learned via `wasm-bindgen --help`, but
some notable options are:

* `--nodejs`: this flag will tailor output for Node instead of browsers,
  allowing for native usage of `require` of the generated JS and internally
  using `require` instead of ES modules. When using this flag no further
  postprocessing (aka a bundler) should be necessary to work with the wasm.

* `--browser`: this flag will tailor the output specifically for browsers,
  making it incompatible with Node. This will basically make the generated JS a
  tiny bit smaller as runtime checks for Node won't be necessary.

* `--no-modules`: the default output of `wasm-bindgen` uses ES modules but this
  option indicates that ES modules should not be used and output should be
  tailored for a web browser. More information on this flag, and
  `--no-modules-global`, can be found in the [no ES modules
  documentation](./no-esm.html).

* `--no-typescript`: by default a `*.d.ts` file is generated for the generated
  JS file, but this flag will disable generating this TypeScript file.

* `--debug`: generates a bit more JS and wasm in "debug mode" to help catch
  programmer errors, but this output isn't intended to be shipped to production.
