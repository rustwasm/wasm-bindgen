# Generating Coverage Data

You can ask the runner to generate coverage data from functions marked as `#[wasm_bindgen_test]` in the `.profraw` format.

<div class="warning">
  Coverage is still in an experimental state, requires Rust Nightly, may be
  unreliable and could experience breaking changes at any time.
</div>

## Enabling the feature

To enable this feature, you need to set `cfg(wasm_bindgen_unstable_test_coverage)` for `wasm-bindgen-test` and its dependencies.

Currently it is particularly difficult to [deliver compile-line arguments to proc-macros when cross-compiling with Cargo][1]. To circumvent this [host-config] can be used.

[1]: https://github.com/rust-lang/cargo/issues/4423
[host-config]: https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#host-config

## Generating the data

### `RUSTFLAGS` that need to be present

Make sure you are using `RUSTFLAGS=-Cinstrument-coverage -Zno-profiler-runtime`.

Due to the current limitation of `llvm-cov`, we can't collect profiling symbols from the generated `.wasm` files. Instead, we can grab them from the LLVM IR with `--emit=llvm-ir` by using Clang. Additionally, the emitted LLVM IR files by Rust contain invalid code that can't be parsed by Clang, so they need to be adjusted. Clang must use the same LLVM version that Rustc is using, which can be checkd by calling `rustc +nightly -vV`.

### Arguments to the test runner

The following environment variables can be used to control the coverage output when [executing the test runner][2]:

- `WASM_BINDGEN_UNSTABLE_TEST_PROFRAW_OUT` to control the file name of the profraw or the directory in which it is placed
- `WASM_BINDGEN_UNSTABLE_TEST_PROFRAW_PREFIX` to add a custom prefix to the profraw files. This can be useful if you're running the tests automatically in succession.

[2]: usage.html#appendix-using-wasm-bindgen-test-without-wasm-pack

### Target features

This feature relies on the [minicov] crate, which provides a profiling runtime for WebAssembly. It in turn uses [cc] to compile the runtime to Wasm, which [currently doesn't support accounting for target feature][3]. Use e.g. `CFLAGS_wasm32_unknown_unknown="-matomics -mbulk-memory"` to account for that.

[3]: https://github.com/rust-lang/cc-rs/issues/268
[cc]: https://crates.io/crates/cc
[minicov]: https://crates.io/crates/minicov

### Example

```sh
# Run the tests:
# - `CARGO_HOST_RUSTFLAGS` to pass the configuration to `wasm-bindgen-macro`.
# - `-Ztarget-applies-to-host -Zhost-config` to enable `CARGO_HOST_RUSTFLAGS`.
# - `--tests` to not run documentation tests, which is currently not supported.
CARGO_HOST_RUSTFLAGS=--cfg=wasm_bindgen_unstable_test_coverage \
RUSTFLAGS="-Cinstrument-coverage -Zno-profiler-runtime --emit=llvm-ir --cfg=wasm_bindgen_unstable_test_coverage" \
CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner \
cargo +nightly test -Ztarget-applies-to-host -Zhost-config --tests
# Adjust the LLVM IR and compile to object files:
# - Replaces every function body with `unreachable`.
# - Removes Rust-specific `range` annotations from function signatures.
name=name_of_the_tested_crate_in_snake_case; \
for file in `ls target/wasm32-unknown-unknown/debug/deps/$name-*.ll`; \
do \
    perl -i -p0e 's/(^define.*?$).*?^}/$1\nstart:\n  unreachable\n}/gms' $file && \
    perl -i -p0e 's/(?<=noundef) range\(.*?\)//g' $file && \
    clang $file -Wno-override-module -c; \
done
# Merge all generated raw profiling data.
# This uses `cargo-binutils` which uses LLVM tools shipped by Rust to make sure there is no LLVM version discrepancy.
# But `llvm-profdata` can be used directly as well.
# See <https://crates.io/crates/cargo-binutils>.
rust-profdata merge -sparse ./*.profraw -o coverage.profdata
# Produce test coverage data in the HTML format.
rust-cov show --instr-profile=coverage.profdata --object ./*.o --format=html --Xdemangler=rust-demangler --sources src --output-dir coverage
```

The [rustc book] has a lot more exapmles and information on test coverage as well.

[rustc book]: https://doc.rust-lang.org/nightly/rustc/instrument-coverage.html

## Attribution

These methods have originally been pioneered by [Hacken OÜ], see [their guide][4] as well.

[4]: https://hknio.github.io/wasmcov
[Hacken OÜ]: https://hacken.io
