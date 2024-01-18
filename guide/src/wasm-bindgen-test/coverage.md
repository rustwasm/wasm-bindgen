# Generating Coverage Data

You can ask the runner to generate coverage data from functions marked as `#[wasm_bindgen_test]` in the `.profraw` format.

<div class="warning">
  Coverage is still in an experimental state and may be unreliable and could experience
  breaking changes at any time.
</div>

## Enabling the feature

To enable this feature, you need to enable the `"unstable-coverage"` feature in your `wasm-bindgen-test` dependency.

## Generating the data

### `RUSTFLAGS` that need to be present

Make sure you are using `RUSTFLAGS=-Cinstrument-coverage -Zno-profiler-runtime`.

Due to the current limitation of `llvm-cov`, you can't get debug information out of a `.wasm`.
Instead, [we can grab them from the LLVM IR][wasmcov], so consider adding `--emit=llvm-ir` as well.

[wasmcov]: https://github.com/hknio/code-coverage-for-webassembly

### Arguments to the test runner

If you are using `wasm-pack` to run the tests, refer to the `wasm-pack` documentation.

Otherwise, you can use the following environment variables when [executing the test runner][1] to control the coverage output:

[1]: usage.html#appendix-using-wasm-bindgen-test-without-wasm-pack

- `WASM_BINDGEN_UNSTABLE_TEST_COVERAGE` to generate a single `.profraw` in your current working directory.
- `WASM_BINDGEN_UNSTABLE_TEST_PROFRAW_OUT` to control the file name of the profraw or the directory in which it is placed
- `WASM_BINDGEN_UNSTABLE_TEST_PROFRAW_PREFIX` to add a custom prefix to the profraw files. This can be useful if you're running the tests automatically in succession.

### Example

```sh
RUSTFLAGS="-Cinstrument-coverage -Zno-profiler-runtime --emit=llvm-ir" wasm-pack test --coverage --profraw-out cov_data/
# Generate the debug info on the host
clang target/wasm32-unknown-unknown/debug/deps/{your-dot-wasm-without-extension}.ll -Wno-override-module -c -o wasm.o
llvm-profdata merge --sparse cov_data/*.profraw -o cov_data/coverage.profdata
llvm-cov --instr-profile=cov_data/coverage.profdata wasm.o --format=html --output-dir=coverage/ --sources .
```
