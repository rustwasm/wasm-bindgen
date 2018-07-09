# Supporting More Web APIs in `web-sys`

1. <input type="checkbox"/> Ensure that the `.webidl` file describing the
   interface exists somewhere within the `crates/web-sys/webidls/enabled`
   directory.

   First, check to see whether we have the WebIDL definition file for
   your API:

   ```sh
   grep -rn MyWebApi crates/web-sys/webidls
   ```

   * If your interface is defined in a `.webidl` file that is inside the
     `crates/web-sys/webidls/enabled` directory, skip to step (3).

   * If your interface isn't defined in any file yet, find the WebIDL definition
     in the relevant standard and add it as a new `.webidl` file in
     `crates/web-sys/webidls/enabled`. Make sure that it is a standard Web API!
     We don't want to add non-standard APIs to this crate.

   * If your interface is defined in a `.webidl` file within the
     `crates/web-sys/webidls/available` directory, you need to move it into
     `crates/web-sys/webidls/enabled`:

     ```sh
     cd crates/web-sys
     git mv webidls/available/MyWebApi.webidl webidls/enabled/MyWebApi.webidl
     ```

2. <input type="checkbox"/> Verify that the `web-sys` crate still builds and
   that its tests still pass with the new `.webidl` file enabled:

   ```sh
   cd crates/web-sys
   cargo build
   cargo test
   ```

3. <input type="checkbox"/> Verify that bindings are being generated for your new
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

4. <input type="checkbox"/> Add a simple test for your new API to
   `crates/web-sys/tests/all/`. See the [`web-sys` testing
   documentation](/web-sys/testing.html) for details.

5. <input type="checkbox"/> Send a pull request! 😊
