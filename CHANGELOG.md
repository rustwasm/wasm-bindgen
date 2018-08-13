# `wasm-bindgen` Change Log

--------------------------------------------------------------------------------

## Unreleased

Released YYYY-MM-DD.

### Added

* TODO (or remove section if none)

### Changed

* TODO (or remove section if none)

### Deprecated

* TODO (or remove section if none)

### Removed

* TODO (or remove section if none)

### Fixed

* TODO (or remove section if none)

### Security

* TODO (or remove section if none)

--------------------------------------------------------------------------------

## 0.2.16

Released 2018-08-13.

### Added

* Added the `wasm_bindgen::JsCast` trait, as described in [RFC #2][rfc-2].
* Added [the `#[wasm_bindgen(extends = ...)]` attribute][extends-attr] to
  describe inheritance relationships, as described in [RFC #2][rfc-2].
* Added support for receiving `Option<&T>` parameters from JavaScript in
  exported Rust functions and methods.
* Added support for receiving `Option<u32>` and other option-wrapped scalars.
* Added reference documentation to the guide for every `#[wasm_bindgen]`
  attribute and how it affects the generated bindings.
* Published the `wasm-bindgen-futures` crate for converting between JS
  `Promise`s and Rust `Future`s.

### Changed

* Overhauled the guide's documentation on passing JS closures to Rust, and Rust
  closures to JS.
* Overhauled the guide's documentation on using serde to serialize complex data
  to `JsValue` and deserialize `JsValue`s back into complex data.
* Static methods are now always bound to their JS class, as is required for
  `Promise`'s static methods.

### Removed

* Removed internal usage of `syn`'s `visit-mut` cargo feature, which should
  result in faster build times.

### Fixed

* Various usage errors for the `#[wasm_bindgen]` proc-macro are now properly
  reported with source span information, rather than `panic!()`s inside the
  proc-macro.
* Fixed a bug where taking a struct by reference and returning a slice resulted
  in lexical variable redeclaration errors in the generated JS glue. [#662][]
* The `#[wasm_bindgen(js_class = "....")]` attribute for binding methods to
  renamed imported JS classes now properly works with constructors.

[rfc-2]: https://rustwasm.github.io/rfcs/002-wasm-bindgen-inheritance-casting.html
[extends-attr]: https://rustwasm.github.io/wasm-bindgen/reference/attributes/on-js-imports/extends.html
[#662]: https://github.com/rustwasm/wasm-bindgen/pull/662

--------------------------------------------------------------------------------

## 0.2.15

Released 2018-07-26.

### Fixed

* Fixed `wasm-bindgen` CLI version mismatch checks that got broken in the last
  point release.

--------------------------------------------------------------------------------

## 0.2.14

Released 2018-07-25.

### Fixed

* Fixed compilation errors on targets that use
  Mach-O. [#545](https://github.com/rustwasm/wasm-bindgen/issues/545)

--------------------------------------------------------------------------------

## 0.2.13

Released 2018-07-22.

### Added

* Support the `#[wasm_bindgen(js_name = foo)]` attribute on exported functions
  and methods to allow renaming an export to JS. This allows JS to call it by
  one name and Rust to call it by another, for example using `camelCase` in JS
  and `snake_case` in Rust

### Fixed

* Compilation with the latest nightly compiler has been fixed (nightlies on and
  after 2018-07-21)

--------------------------------------------------------------------------------

## 0.2.12

Released 2018-07-19.

This release is mostly internal refactorings and minor improvements to the
existing crates and functionality, but the bigs news is an upcoming `js-sys` and
`web-sys` set of crates. The `js-sys` crate will expose [all global JS
bindings][js-all] and the `web-sys` crate will be generated from WebIDL to
expose all APIs browsers have. More info on this soon!

[js-all]: https://github.com/rustwasm/wasm-bindgen/issues/275

### Added

* Support for `Option<T>` was added where `T` can be a number of slices or
  imported types.
* Comments in Rust are now preserved in generated JS bindings, as well as
  comments being generated to indicate the types of arguments/return values.
* The online documentation has been reorganized [into a book][book].
* The generated JS is now formatted better by default for readability.
* A `--keep-debug` flag has been added to the CLI to retain debug sections by
  default. This happens by default when `--debug` is passed.

[book]: https://rustwasm.github.io/wasm-bindgen/

### Fixed

* Compilation with the latest nightly compiler has been fixed (nightlies on and
  after 2018-07-19)
* Declarations of an imported function in multiple crates have been fixed to not
  conflict.
* Compilation with `#![deny(missing_docs)]` has been fixed.

--------------------------------------------------------------------------------

## 0.2.11

Released 2018-05-24.

--------------------------------------------------------------------------------

## 0.2.10

Released 2018-05-17.

--------------------------------------------------------------------------------

## 0.2.9

Released 2018-05-11.
