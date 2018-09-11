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

   * If your interface is defined in a `.webidl` file within any of the
     `crates/web-sys/webidls/unavailable_*` directories, you need to move it into
     `crates/web-sys/webidls/enabled`, e.g.:

     ```sh
     cd crates/web-sys
     git mv webidls/unavailable_enum_ident/MyWebApi.webidl webidls/enabled/MyWebApi.webidl
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
     rebuild the `web-sys` crate [with logging enabled](logging.html)
     and look for warning messages that mention your new API. Figure out why
     bindings weren't generated and then add support to `wasm_bindgen_webidl` for
     whatever is needed to generate your API's bindings.

     > You might find it helpful to view the generated rust bindings, to see if
     > they are what you would expect. The file will be located at
     > `target/wasm32-unknown-unknown/debug/build/web-sys-xxx/out/bindings.rs`,
     > where `xxx` is a combinations of numbers and letters that represents your
     > build. This file is pretty unintelligable until you run `rustfmt` on it, like
     > `rustfmt target/wasm32-unknown-unknown/debug/build/web-sys-xxx/out/bindings.rs`.

     > There are commented out lines in `web-sys/build.rs` that run rustfmt as part of
     > the build process, and this can be very helpful for debugging as any error
     > messages with inline code will display it in a readable format.

4. <input type="checkbox"/> Add tests for as many of the features in the WebIDL file
    as possible to `crates/web-sys/tests/all/`. See the
    [`web-sys` testing documentation](testing.html) for details.

    > __Note__: Start here at __4__ if the WebIDL has already been added but doesn't have 
    > full test coverage, then go back to __3__ if you find any problems.

5. <input type="checkbox"/> If all entities in the WebIDL file have full test coverage,
    mark the WebIDL script in the `README.md` file as complete by changing `[ ]` to `[x]`.

6. <input type="checkbox"/> Send a pull request! 😊
