# wasm-bindgen-test

This crate is an experimental test harness for `wasm32-unknown-unknown`, with
the goal of allowing you to write tests as you normally do in Rust and then
simply:

```
cargo test --target wasm32-unknown-unknown
```

This project is still in the early stages of its development so there's not a
ton of documentation just yet, but a taste of how it works is:

* First, install the test runner.

  ```
  cargo install --path crates/cli
  ```

  (this comes with the normal `wasm-bindgen` CLI tool

* Next, add this to your `.cargo/config`:

  ```toml
  [target.wasm32-unknown-unknown]
  runner = 'wasm-bindgen-test-runner'
  ```

* Next, configure your project's dev-dependencies:

  ```toml
  [dev-dependencies]
  # or [target.'cfg(target_arch = "wasm32")'.dev-dependencies]
  wasm-bindgen-test = { git = 'https://github.com/rustwasm/wasm-bindgen' }
  ```

* Next, write some tests!

  ```rust
  // in tests/wasm.rs
  #![feature(use_extern_macros)]

  extern crate wasm_bindgen_test;

  use wasm_bindgen_test::*;

  #[wasm_bindgen_test]
  fn pass() {
      assert_eq!(1, 1);
  }

  #[wasm_bindgen_test]
  fn fail() {
      assert_eq!(1, 2);
  }
  ```

* And finally, execute your tests:

  ```
  $ cargo test --target wasm32-unknown-unknown
      Finished dev [unoptimized + debuginfo] target(s) in 0.11s
       Running /home/.../target/wasm32-unknown-unknown/debug/deps/wasm-4a309ffe6ad80503.wasm
  running 2 tests

  test wasm::pass ... ok
  test wasm::fail ... FAILED

  failures:

  ---- wasm::fail output ----
      error output:
          panicked at 'assertion failed: `(left == right)`
            left: `1`,
           right: `2`', crates/test/tests/wasm.rs:14:5

      JS exception that was thrown:
          RuntimeError: unreachable
              at __rust_start_panic (wasm-function[1362]:33)
              at rust_panic (wasm-function[1357]:30)
              at std::panicking::rust_panic_with_hook::h56e5e464b0e7fc22 (wasm-function[1352]:444)
              at std::panicking::continue_panic_fmt::had70ba48785b9a8f (wasm-function[1350]:122)
              at std::panicking::begin_panic_fmt::h991e7d1ca9bf9c0c (wasm-function[1351]:95)
              at wasm::fail::ha4c23c69dfa0eea9 (wasm-function[88]:477)
              at core::ops::function::FnOnce::call_once::h633718dad359559a (wasm-function[21]:22)
              at wasm_bindgen_test::__rt::Context::execute::h2f669104986475eb (wasm-function[13]:291)
              at __wbg_test_fail_1 (wasm-function[87]:57)
              at module.exports.__wbg_apply_2ba774592c5223a7 (/home/alex/code/wasm-bindgen/target/wasm32-unknown-unknown/wbg-tmp/wasm-4a309ffe6ad80503.js:61:66)


  failures:

      wasm::fail

  test result: FAILED. 1 passed; 1 failed; 0 ignored

  error: test failed, to rerun pass '--test wasm'
  ```

And that's it! You've now got a test harness executing native wasm code inside
of Node.js and you can use `cargo test` as you normally would for workflows.

## Asynchronous Tests

Not all tests can execute immediately and some may need to do "blocking" work
like fetching resources and/or other bits and pieces. To accommodate this
asynchronous tests are also supported through the `futures` crate:

```rust
#[wasm_bindgen_test(async)]
fn my_test() -> impl Future<Item = (), Error = JsValue> {
    // ...
}
```

The test will pass if the future resolves without panicking or returning an
error, and otherwise the test will fail.

This support is currently powered by the `wasm-bindgen-futures` crate.

## Components

The test harness is made of three separate components, but you typically don't
have to worry about most of them. They're documented here for documentation
purposes!

### `wasm-bindgen-test-macro`

This crate, living at `crates/test-macro`, is a procedural macro that defines
the `#[wasm_bindgen_test]` macro. **The normal `#[test]` cannot be used and will
not work.** Eventually it's intended that the `#[wasm_bindgen_test]` attribute
could gain arguments like "run in a browser" or something like a minimum Node
version.

For now though the macro is pretty simple and reexported from the next crate,
`wasm-bindgen-test`.

### `wasm-bindgen-test`

This is the runtime support needed to execute tests. This is basically the same
thing as the `test` crate in the Rust repository, and one day it will likely use
the `test` crate itself! For now though it's a minimal reimplementation that
provides the support for:

* Printing what test cases are running
* Collecting `console.log` and `console.error` output of each test case for
  printing later
* Rendering the failure output of each test case
* Catching JS exceptions so tests can continue to run after a test fails
* Driving execution of all tests

This is the crate which you actually link to in your wasm test and through which
you import the `#[wasm_bindgen_test]` macro. Otherwise this crate provides a
`console_log!` macro that's a utility like `println!` only using `console.log`.

This crate may grow more functionality in the future, but for now it's somewhat
bare bones!

### `wasm-bindgen-test-runner`

This is where the secret sauce comes into play. We configured Cargo to execute
this binary *instead* of directly executing the `*.wasm` file (which Cargo would
otherwise try to do). This means that whenever a test is executed it executes
this binary with the wasm file as an argument, allowing it to take full control
over the test process!

The test runner is currently pretty simple, executing a few steps:

* First, it runs the equivalent of `wasm-bindgen`. This'll generate wasm-bindgen
  output in a temoprary directory.
* Next, it generates a small shim JS file which imports these
  wasm-bindgen-generated files and executes the test harness.
* Finally, it executes `node` over the generated JS file, executing all of your
  tests.

In essence what happens is that this test runner automatically executes
`wasm-bindgen` and then uses Node to actually execute the wasm file, meaning
that your wasm code currently runs in a Node environment.

## Future Work

Things that'd be awesome to support in the future:

* Arguments to `wasm-bindgen-test-runner` which are the same as `wasm-bindgen`,
  for example `--debug` to affect the generated output.
* Running each test in its own wasm instance to avoid poisoning the environment
  on panic
