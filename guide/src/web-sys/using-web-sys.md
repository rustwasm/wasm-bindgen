# Using `web-sys`

## Add `web-sys` as a dependency to your `Cargo.toml`

***Note:** `web-sys` is not available on crates.io yet, so you'll need to depend
on the git version of it, and of `wasm-bindgen`:*

```toml
[dependencies]
wasm-bindgen = { git = "https://github.com/rustwasm/wasm-bindgen" }
web-sys = {
  git = "https://github.com/rustwasm/wasm-bindgen",
  features = [
  ]
}
```

## Enable the cargo features for the APIs you're using

To keep build times super speedy, [`web-sys` gates each Web interface behind a
cargo feature](./cargo-features.html). Find the type or method you want to use
in the [API documentation][api]; it will list the features that must be enabled
to access that API.

For example, if we're looking for [the `window.resizeTo`
function][js-resize-to], we would [search for `resizeTo` in the API
documentation][search-resize-to]. We would find [the
`web_sys::Window::resize_to` function][rust-resize-to], which requires the
`Window` feature. To get access to that function, we enable the `Window` feature
in `Cargo.toml`:

```toml
web-sys = {
  git = "https://github.com/rustwasm/wasm-bindgen",
  features = [
    "Window",
  ]
}
```

## Call the method!

```rust
extern crate web_sys;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use web_sys::Window;

#[wasm_bindgen]
pub fn make_the_window_small() {
    // Resize the window to 500px by 500px.
    Window::resize_to(500, 500)
        .expect("could not resize the window");
}
```

[api]: https://rustwasm.github.io/wasm-bindgen/api/web_sys/
[js-resize-to]: https://developer.mozilla.org/en-US/docs/Web/API/window/resizeTo
[search-resize-to]: https://rustwasm.github.io/wasm-bindgen/api/web_sys/?search=resizeTo
[rust-resize-to]: https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Window.html#method.resize_to
