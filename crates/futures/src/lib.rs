//! Converting between JavaScript `Promise`s to Rust `Future`s.
//!
//! This crate provides a bridge for working with JavaScript `Promise` types as
//! a Rust `Future`, and similarly contains utilities to turn a rust `Future`
//! into a JavaScript `Promise`. This can be useful when working with
//! asynchronous or otherwise blocking work in Rust (wasm), and provides the
//! ability to interoperate with JavaScript events and JavaScript I/O
//! primitives.
//!
//! There are two main interfaces in this crate currently:
//!
//! 1. [**`JsFuture`**](./struct.JsFuture.html)
//!
//!    A type that is constructed with a `Promise` and can then be used as a
//!    `Future<Item = JsValue, Error = JsValue>`. This Rust future will resolve
//!    or reject with the value coming out of the `Promise`.
//!
//! 2. [**`future_to_promise`**](./fn.future_to_promise.html)
//!
//!    Converts a Rust `Future<Item = JsValue, Error = JsValue>` into a
//!    JavaScript `Promise`. The future's result will translate to either a
//!    rejected or resolved `Promise` in JavaScript.
//!
//! These two items should provide enough of a bridge to interoperate the two
//! systems and make sure that Rust/JavaScript can work together with
//! asynchronous and I/O work.
//!
//! # Example Usage
//!
//! This example wraps JavaScript's `Promise.resolve()` into a Rust `Future` for
//! running tasks on the next tick of the micro task queue. The futures built on
//! top of it can be scheduled for execution by conversion into a JavaScript
//! `Promise`.
//!
//! ```rust,no_run
//! extern crate futures;
//! extern crate js_sys;
//! extern crate wasm_bindgen;
//! extern crate wasm_bindgen_futures;
//!
//! use futures::{Async, Future, Poll};
//! use wasm_bindgen::prelude::*;
//! use wasm_bindgen_futures::{JsFuture, future_to_promise};
//!
//! /// A future that becomes ready after a tick of the micro task queue.
//! pub struct NextTick {
//!     inner: JsFuture,
//! }
//!
//! impl NextTick {
//!     /// Construct a new `NextTick` future.
//!     pub fn new() -> NextTick {
//!         // Create a resolved promise that will run its callbacks on the next
//!         // tick of the micro task queue.
//!         let promise = js_sys::Promise::resolve(&JsValue::NULL);
//!         // Convert the promise into a `JsFuture`.
//!         let inner = JsFuture::from(promise);
//!         NextTick { inner }
//!     }
//! }
//!
//! impl Future for NextTick {
//!     type Item = ();
//!     type Error = ();
//!
//!     fn poll(&mut self) -> Poll<(), ()> {
//!         // Polling a `NextTick` just forwards to polling if the inner promise is
//!         // ready.
//!         match self.inner.poll() {
//!             Ok(Async::Ready(_)) => Ok(Async::Ready(())),
//!             Ok(Async::NotReady) => Ok(Async::NotReady),
//!             Err(_) => unreachable!(
//!                 "We only create NextTick with a resolved inner promise, never \
//!                  a rejected one, so we can't get an error here"
//!             ),
//!         }
//!     }
//! }
//!
//! /// Export a function to JavaScript that does some work in the next tick of the
//! /// micro task queue!
//! #[wasm_bindgen]
//! pub fn schedule_some_work_for_next_tick() -> js_sys::Promise {
//!     let future = NextTick::new()
//!         // Do some work...
//!         .and_then(|_| {
//!             Ok(42)
//!         })
//!         // And then convert the `Item` and `Error` into `JsValue`.
//!         .map(|result| {
//!             JsValue::from(result)
//!         })
//!         .map_err(|error| {
//!             let js_error = js_sys::Error::new(&format!("uh oh! {:?}", error));
//!             JsValue::from(js_error)
//!         });
//!
//!     // Convert the `Future<Item = JsValue, Error = JsValue>` into a JavaScript
//!     // `Promise`!
//!     future_to_promise(future)
//! }
//! ```

#![cfg_attr(target_feature = "atomics", feature(stdsimd))]
#![deny(missing_docs)]

use cfg_if::cfg_if;

mod legacy_js2rust;
pub use legacy_js2rust::*;

cfg_if! {
    if #[cfg(target_feature = "atomics")] {
        /// Contains a thread-safe version of this crate, with Futures 0.1
        mod atomics;

        /// Polyfill for `Atomics.waitAsync` function
        mod polyfill;

        pub use atomics::*;
    } else {
        mod stable;
        pub use stable::*;
     }
}

#[cfg(feature = "futures_0_3")]
cfg_if! {
    if #[cfg(target_feature = "atomics")] {
        compile_error!("futures 0.3 support is not implemented with atomics yet");
    } else {
        /// Contains a Futures 0.3 implementation of this crate.
        pub mod futures_0_3;
     }
}
