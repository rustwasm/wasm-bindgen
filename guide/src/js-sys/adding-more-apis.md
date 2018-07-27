# Adding Support for More JavaScript Global APIs

We've got a [GitHub issue][issue] tracking the remaining APIs that still need to
be added to the `js-sys` crate, and we'd love your help adding them! The steps
to follow are:

* [ ] Comment on the issue saying which thing you are going to make bindings for
  (so that we don't accidentally duplicate effort).

* [ ] Open the [MDN
  page](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects)
  for the relevant JS API.

* [ ] Open `crates/js-sys/src/lib.rs` in your editor; this is the file where we
  are implementing the bindings.

* [ ] Follow the instructions in the top of `crates/js-sys/src/lib.rs` about how
  to add new bindings.

* [ ] Add a test for the new binding to `crates/js-sys/tests/wasm/MyType.rs`

* [ ] Run the JS global API bindings tests with `cargo test -p js-sys --target
  wasm32-unknown-unknown`

* [ ] Send a pull request!

[issue]: https://github.com/rustwasm/wasm-bindgen/issues/275
