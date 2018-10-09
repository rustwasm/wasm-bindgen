# Testing in Headless Browsers

## Configure Your Test Crate

Add this to the root of your test crate, e.g. `$MY_CRATE/tests/web.rs`:

```rust
use wasm_bindgen_test::wasm_bindgen_test_configure;

wasm_bindgen_test_configure!(run_in_browser);
```

Note that although a particular test crate must target either headless browsers
or Node.js, you can have test suites for both Node.js and browsers for your
project by using multiple test crates. For example:

```
$MY_CRATE/
`-- tests
    |-- node.rs    # The tests in this suite use the default Node.js.
    `-- web.rs     # The tests in this suite are configured for browsers.
```

## Configuring Which Browser is Used

To control which browser is used for headless testing, use the appropriate flag
with `wasm-pack test`:

* `wasm-pack test --chrome` &mdash; Run the tests in Chrome. This machine must
  have Chrome installed.

* `wasm-pack test --firefox` &mdash; Run the tests in Firefox. This machine must
  have Firefox installed.

* `wasm-pack test --safari` &mdash; Run the tests in Safari. This machine must
  have Safari installed.

If multiple browser flags are passed, the tests will be run under each browser.

## Running the Tests in the Headless Browser

Once the tests are configured to run in a headless browser, just run `wasm-pack
test` with the appropriate browser flags and `--headless`:

```bash
wasm-pack test --headless --chrome --firefox --safari
```

### Debugging Headless Browser Tests

Omitting the `--headless` flag will disable headless mode, and allow you to
debug failing tests in your browser's devtools.

--------------------------------------------------------------------------------

## Appendix: Testing in headless browsers without `wasm-pack`

**⚠️ The recommended way to use `wasm-bindgen-test` is with `wasm-pack`, since it
will handle installing the test runner, installing a WebDriver client for your
browser, and informing `cargo` how to use the custom test runner.** However, you
can also manage those tasks yourself, if you wish.

### Configuring Which Browser is Used

If one of the following environment variables is set, then the corresponding
WebDriver and browser will be used. If none of these environment variables are
set, then the `$PATH` is searched for a suitable WebDriver implementation.

#### `GECKODRIVER=path/to/geckodriver`

Use Firefox for headless browser testing, and `geckodriver` as its
WebDriver.

The `firefox` binary must be on your `$PATH`.

[Get `geckodriver` here](https://github.com/mozilla/geckodriver/releases)

#### `CHROMEDRIVER=path/to/chromedriver`

Use Chrome for headless browser testing, and `chromedriver` as its
WebDriver.

The `chrome` binary must be on your `$PATH`.

[Get `chromedriver` here](http://chromedriver.chromium.org/downloads)

#### `SAFARIDRIVER=path/to/safaridriver`

Use Safari for headless browser testing, and `safaridriver` as its
WebDriver.

This is installed by default on Mac OS. It should be able to find your Safari
installation by default.

### Running the Tests in the Headless Browser

Once the tests are configured to run in a headless browser and the appropriate
environment variables are set, executing the tests for headless browsers is the
same as executing them for Node.js:

```bash
cargo test --target wasm32-unknown-unknown
```

#### Debugging Headless Browser Tests

Set the `NO_HEADLESS=1` environment variable and the browser tests will not run
headless. Instead, the tests will start a local server that you can visit in
your Web browser of choices, and headless testing should not be used. You can
then use your browser's devtools to debug.
