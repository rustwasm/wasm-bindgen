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

Due to the current limitation of `llvm-cov`, we can't collect profiling symbols from the generated `.wasm` files. Instead, we can grab them from the LLVM IR with `--emit=llvm-ir` by using Clang. Additionally, the emitted LLVM IR files by Rust contain invalid code that can't be parsed by Clang, so they need to be adjusted.

At the time of writing Rust Nightly uses LLVM v19, however [minicov] only supports LLVM v18. Usage of Clang or any LLVM tools must match the version used by [minicov].

[minicov]: https://crates.io/crates/minicov

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

This adapts code taken from the [Rustc book], see that for more examples and general information on test coverage as well.

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
# - Extract a list of compiled artifacts from Cargo and filter them with `jq`.
# - Figure out the path to the LLVM IR file corresponding to an artifact.
# - Replace every function body with `unreachable`.
# - Remove Rust-specific `range` annotations from function signatures.
# - Compile to object file with Clang and store for later usage with `llvm-cov`.
crate_name=name_of_the_tested_crate_in_snake_case
objects=()
IFS=$'\n'
for file in $(
    CARGO_HOST_RUSTFLAGS=--cfg=wasm_bindgen_unstable_test_coverage \
    RUSTFLAGS="-Cinstrument-coverage -Zno-profiler-runtime --emit=llvm-ir --cfg=wasm_bindgen_unstable_test_coverage" \
    cargo +nightly test -Ztarget-applies-to-host -Zhost-config --tests --no-run --message-format=json | \
    jq -r "select(.reason == \"compiler-artifact\") | (select(.target.kind == [\"test\"]) // select(.target.name == \"$crate_name\")) | .filenames[0]"
)
do
    if [[ ${file##*.} == "rlib" ]]; then
        base=$(basename $file .rlib)
        file=$(dirname $file)/${base#"lib"}.ll
    else
        file=$(dirname $file)/$(basename $file .wasm).ll
    fi

    perl -i -p0e 's/(^define.*?$).*?^}/$1\nstart:\n  unreachable\n}/gms' $file
    counter=1
    while (( counter != 0 )); do
        counter=$(perl -i -p0e '$c+= s/(^(define|declare)(,? [^\n ]+)*),? range\(.*?\)/$1/gm; END{print "$c"}' $file)
    done

  output = $(basename $file .ll).o
  clang-18 $input -Wno-override-module -c -o $output
  objects+=(-object $output)
done
# Merge all generated raw profiling data.
llvm-profdata-18 merge -sparse *.profraw -o coverage.profdata
# Produce test coverage data in the HTML format and pass the object files we generated earlier.
llvm-cov-18 show -show-instantiations=false -Xdemangler=rustfilt -output-dir coverage -format=html -instr-profile=coverage.profdata ${objects[@]} -sources src
```

[rustc book]: https://doc.rust-lang.org/nightly/rustc/instrument-coverage.html

## Attribution

These methods have originally been pioneered by [Hacken OÜ], see [their guide][4] as well.

[4]: https://hknio.github.io/wasmcov
[Hacken OÜ]: https://hacken.io
