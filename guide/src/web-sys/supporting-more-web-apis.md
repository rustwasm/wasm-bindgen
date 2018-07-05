# Supporting More Web APIs in `web-sys`

* <input type="checkbox"/> Ensure that the `.webidl` file describing the
  interface exists in the `crates/web-sys/webidls/available` directory.

  ```sh
  grep -rn MyWebApi crates/web-sys/webidls/available
  ```

  If it isn't in there yet, find the WebIDL definition and add it. Make sure
  that it is a standard Web API! We don't want to add non-standard APIs to this
  crate.

* <input type="checkbox"/> Ensure that the `.webidl` file from
  `crates/web-sys/webidls/available` that contains your interface is linked into
  `crates/web-sys/webidls/enabled`. If it isn't, you can link it with:

  ```sh
  cd crates/web-sys/webidls/enabled
  ln -s ../available/MyWebApi.webidl MyWebApi.webidl
  ```

  * <input type="checkbox"/> Verify that the `web-sys` crate still builds and
    that its tests still pass with the new `.webidl` file enabled:

    ```sh
    cd crates/web-sys
    cargo build
    cargo test
    ```

* <input type="checkbox"/> Verify that bindings are being generated for your new
  API by generating the documentation and searching for the new API in it:

  ```sh
  cd crates/web-sys
  cargo doc --open
  # search for the new API in the opened docs
  ```

  * <input type="checkbox"/> If the new API is **not** showing up in the docs,
    rebuild the `web-sys` crate [with logging enabled](/web-sys/logging.html)
    and look for warning messages that mention your new API. Figure out why
    bindings weren't generated and then add support to `wasm_bindgen_webidl` for
    whatever is needed to generate your API's bindings.

* <input type="checkbox"/> Add a simple test for your new API to
  `crates/web-sys/tests/all/`. See the [`web-sys` testing
  documentation](/web-sys/testing.html) for details.

* <input type="checkbox"/> Send a pull request! 😊
