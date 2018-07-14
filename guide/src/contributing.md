# Contributing to `wasm-bindgen`

This section contains instructions on how to get this project up and running for
development.

## Prerequisites

1. Rust Nightly. [Install Rust]. Once Rust is installed, run

    ```shell
    rustup default nightly
    rustup target add wasm32-unknown-unknown
    ```

[install Rust]: https://www.rust-lang.org/en-US/install.html

2. The tests for this project use Node. Make sure you have node >= 10 installed,
   as that is when WebAssembly support was introduced. [Install Node].

[Install Node]: https://nodejs.org/en/

3. The tests for this project also use `yarn`, a package manager for Node. To
   install `yarn`, run:

    ```shell
    npm install -g yarn
    ```

   or follow other platform-specific instructions
   [here](https://yarnpkg.com/en/docs/install).

   Once `yarn` is installed, run it in the top level directory:

   ```shell
   yarn
   ```

## Running Tests

Finally, you can run the tests with `cargo`:

```shell
cargo test
```

### Headless Browser Tests

Some tests are configured to run in a headless Firefox instance. To run these
tests, you must have Firefox installed. If you have Firefox installed in a
non-default, custom location you can set the `WASM_BINDGEN_FIREFOX_BIN_PATH`
environment variable to the path to your `firefox-bin`.

For example:

```shell
WASM_BINDGEN_FIREFOX_BIN_PATH=/home/fitzgen/firefox/firefox-bin cargo test
```
