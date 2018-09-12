# Testing in Headless Browsers

## Configure Your Test Crate

Add this to the root of your test crate, e.g. `$MY_CRATE/tests/wasm.rs`:

```rust
use wasm_bindgen_test::wasm_bindgen_test_configure;

wasm_bindgen_test_configure!(run_in_browser);
```

## Configuring Which Browser is Used

> ⚡ If you are using `wasm-pack`, skip this step! Instead, use `wasm-pack test
> --chrome`, `wasm-pack test --firefox`, or `wasm-pack test --safari`.
> `wasm-pack` will automatically download and configure the appropriate
> WebDriver client for you.

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

## Running the Tests in the Headless Browser

Once the tests are configured to run in a headless browser, executing the tests
is the same:

```bash
cargo test --target wasm32-unknown-unknown

# or, if you're using wasm-pack
wasm-pack test --headless --chrome --firefox --safari
```

### Debugging Headless Browser Tests

> If you're using `wasm-pack`, omitting the `--headless` flag will disable
> headless mode, and allow you to debug failing tests in your browser's
> devtools.

Set the `NO_HEADLESS=1` environment variable and the browser tests will not run
headless. Instead, the tests will start a local server that you can visit in
your Web browser of choices, and headless testing should not be used. You can
then use your browser's devtools to debug.
