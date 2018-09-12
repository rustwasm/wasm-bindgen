# Writing Asynchronous Tests

Not all tests can execute immediately and some may need to do "blocking" work
like fetching resources and/or other bits and pieces. To accommodate this
asynchronous tests are also supported through the `futures` and
`wasm-bindgen-futures` crates.

To write an asynchronous test:

1. Change `#[wasm_bindgen_test]` into `#[wasm_bindgen_test(async)]`

2. Change the return type of the test function to `impl Future<Item = (), Error
   = JsValue>`

The test will pass if the future resolves without panicking or returning an
error, and otherwise the test will fail.

## Example

```rust
extern crate futures;
extern crate js_sys;
extern crate wasm_bindgen_futures;

use futures::Future;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen_test(async)]
fn my_async_test() -> impl Future<Item = (), Error = JsValue> {
    // Create a promise that is ready on the next tick of the micro task queue.
    let promise = js_sys::Promise::resolve(&JsValue::from(42));

    // Convert that promise into a future and make the test wait on it.
    JsFuture::from(promise)
        .map(|x| {
            assert_eq!(x, 42);
        })
        .map_err(|_| unreachable!())
}
```
