# Publishing New `wasm-bindgen` Releases

* [ ] Make sure that your git working copy is clean.

* [ ] Make sure that you are on the latest `master`:

  ```
  git pull origin master
  ```

* [ ] Update `Cargo.toml` versions and dependency versions:

  ```
  git ls-files | grep Cargo.toml | xargs sed -i '' -e 's/0\.X\.Y/0.X.Z/g'
  ```

  where "0.X.Y" is the old version and "0.X.Z" is the new version.

* [ ] Write a "0.X.Z" entry in the CHANGELOG.md

* [ ] Run all the tests for sanity

  ```
  cargo test
  cargo test -p js-sys
  cargo test -p js-sys --target wasm32-unknown-unknown
  cargo test -p wasm-bindgen-webidl
  cargo test -p web-sys
  ```

* [ ] Commit the version bump:

  ```
  git commit -m "Bump to version 0.X.Z"
  ```

* [ ] Comment out the `[patch]` section in the root `Cargo.toml` that only
      exists to make sure that `console_error_panic_hook` in tests is using
      *this* `wasm-bindgen` rather than one from crates.io.

* [ ] Publish the crates in reverse dependency order:

  ```
  cd crates/shared && cargo publish && cd -
  cd crates/backend && cargo publish && cd -
  cd crates/macro && cargo publish && cd -
  cd crates/cli-support && cargo publish && cd -
  cd crates/test-macro && cargo publish && cd -
  cd crates/test && cargo publish --no-verify && cd -
  cd crates/cli && cargo publish && cd -
  cargo publish --allow-dirty # because of uncommitted, commented out [patch] section
  ```

* [ ] Push the commit:

  ```
  git push origin master
  ```

* [ ] Tag the release and push it:

  ```
  git tag 0.X.Z
  git push origin --tags
  ```
