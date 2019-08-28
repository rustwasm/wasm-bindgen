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

#![cfg_attr(target_feature = "atomics", feature(stdsimd))]
#![deny(missing_docs)]

use cfg_if::cfg_if;

mod shared;
pub use shared::*;

cfg_if! {
    if #[cfg(target_feature = "atomics")] {
        mod wait_async_polyfill;
        mod multithread;
        pub use multithread::*;
    } else {
        mod singlethread;
        pub use singlethread::*;
     }
}
