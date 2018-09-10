# Publishing New `wasm-bindgen` Releases

* [ ] Make sure that your git working copy is clean.

* [ ] Make sure that you are on the latest `master`:

  ```
  git pull origin master
  ```

* [ ] Run `rustc ./publish.rs`

* [ ] Run `./publish bump` - this will update all version numbers

* [ ] Write a "0.X.Z" entry in the CHANGELOG.md

* [ ] Commit the version bump:

  ```
  git commit -m "Bump to version 0.X.Z"
  ```

* [ ] Send a PR to the `wasm-bindgen` repository, get it merged

* [ ] Check out the merge commit of `wasm-bindgen`

* [ ] Comment out the `[patch]` section in the root `Cargo.toml` that only
      exists to make sure that `console_error_panic_hook` in tests is using
      *this* `wasm-bindgen` rather than one from crates.io.

* [ ] Run `rustc ./publish.rs`

* [ ] Run `./publish publish`

* [ ] Tag the release and push it:

  ```
  git tag 0.X.Z
  git push origin --tags
  ```
