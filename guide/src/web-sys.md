# `web-sys`

The `web-sys` crate provides raw bindings to all of the Web's APIs, and its
source lives at `wasm-bindgen/crates/web-sys`.

The `web-sys` crate is **entirely** mechanically generated inside `build.rs`
using `wasm-bindgen`'s WebIDL frontend and the WebIDL interface definitions for
Web APIs. This means that `web-sys` isn't always the most ergonomic crate to
use, but it's intended to provide verified and correct bindings to the web
platform, and then better interfaces can be iterated on crates.io!

### Using `web-sys`

Let's say you want to use an API defined on the web. Chances are this API is
defined in `web-sys`, so let's go through some steps necessary to use it!

First up, search the [api documentation][api] for your API. For example if
we're looking for JS's [`fetch`][jsfetch] API we'd start out by [searching for
`fetch`][search-fetch]. The first thing you'll probably notice is that there's
no function called `fetch`! Fear not, though, as the API exists in multiple
forms:

* [`Window::fetch_with_str`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Window.html#method.fetch_with_str)
* [`Window::fetch_with_request`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Window.html#method.fetch_with_request)
* [`Window::fetch_with_str_and_init`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/str_and_inituct.Window.html#method.fetch_with_str_and_init)
* [`Window::fetch_with_request_and_init`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Window.html#method.fetch_with_request_and_init)

What's happening here is that the [`fetch` function][fetchfn] actually supports
multiple signatures of arguments, and we've taken the WebIDL definition for this
function and expanded it to unique signatures in Rust (as Rust doesn't have
function name overloading).

When an API is selected it should have documentation pointing at MDN indicating
what web API its binding. This is often a great way to double check arguments
and such as well, MDN is a great resource! You'll also notice in the
documentation that the API may require some `web-sys` Cargo features to be
activated. For example [`fetch_with_str`] requires the `Window` feature to be
activated. In general an API needs features corresponding to all the types
you'll find in the signature to be activated.

To load up this API let's depend on `web-sys`:

```toml
[dependencies]
wasm-bindgen = "0.2"
web-sys = { version = "0.1", features = ['Window'] }

# Or optionally,
# [target.wasm32-unknown-unknown.dependencies]
# ...
```

> **Note**: Currently `web-sys` is not available on crates.io so you'll also
> need to do this in your manifest:
>
> ```toml
> [patch.crates-io]
> web-sys = { git = 'https://github.com/rustwasm/wasm-bindgen' }
> wasm-bindgen = { git = 'https://github.com/rustwasm/wasm-bindgen' }
> ```

And next up we can use the API like so:

```rust
extern crate web_sys;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use web_sys::Window;

#[wasm_bindgen]
pub fn run() {
    let promise = Window::fetch_with_str("http://example.com/");
    // ...
}
```

and you should be good to go!

### Type translations in `web-sys`

Most of the types specified in WebIDL have relatively straightforward
translations into `web-sys`, but it's worth calling out a few in particular:

* `BufferSource` and `ArrayBufferView` - these two types show up in a number of
  APIs that generally deal with a buffer of bytes. We bind them in `web-sys`
  with two different types, `Object` and `&mut [u8]`. Using `Object` allows
  passing in arbitrary JS values which represent a view of bytes (like any typed
  array object), and `&mut [u8]` allows using a raw slice in Rust. Unfortunately
  we must pessimistically assume that JS will modify all slices as we don't
  currently have information of whether they're modified or not.

* Callbacks are all represented as `js_sys::Function`. This means that all
  callbacks going through `web-sys` are a raw JS value. You can work with this
  by either juggling actual `js_sys::Function` instances or you can create a
  `Closure<FnMut(...)>`, extract the underlying `JsValue` with `as_ref`, and
  then use `JsCast::unchecked_ref` to convert it to a `js_sys::Function`.

[api]: https://rustwasm.github.io/wasm-bindgen/api/web_sys/
[jsfetch]: https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API
[search-fetch]: https://rustwasm.github.io/wasm-bindgen/api/web_sys/?search=fetch
[fetchfn]: https://developer.mozilla.org/en-US/docs/Web/API/WindowOrWorkerGlobalScope/fetch
[`fetch_with_str`]: https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Window.html#method.fetch_with_str
