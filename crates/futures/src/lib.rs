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

#![deny(missing_docs)]

extern crate futures;
extern crate js_sys;
extern crate wasm_bindgen;

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::Arc;

use futures::executor::{self, Notify, Spawn};
use futures::prelude::*;
use futures::future;
use futures::sync::oneshot;
use js_sys::{Function, Promise};
use wasm_bindgen::prelude::*;

/// A Rust `Future` backed by a JavaScript `Promise`.
///
/// This type is constructed with a JavaScript `Promise` object and translates
/// it to a Rust `Future`. This type implements the `Future` trait from the
/// `futures` crate and will either succeed or fail depending on what happens
/// with the JavaScript `Promise`.
///
/// Currently this type is constructed with `JsFuture::from`.
pub struct JsFuture {
    resolved: oneshot::Receiver<JsValue>,
    rejected: oneshot::Receiver<JsValue>,
    callbacks: Option<(Closure<FnMut(JsValue)>, Closure<FnMut(JsValue)>)>,
}

impl From<Promise> for JsFuture {
    fn from(js: Promise) -> JsFuture {
        // Use the `then` method to schedule two callbacks, one for the
        // resolved value and one for the rejected value. These two callbacks
        // will be connected to oneshot channels which feed back into our
        // future.
        //
        // This may not be the speediest option today but it should work!
        let (tx1, rx1) = oneshot::channel();
        let (tx2, rx2) = oneshot::channel();
        let mut tx1 = Some(tx1);
        let resolve = Closure::wrap(Box::new(move |val| {
            drop(tx1.take().unwrap().send(val));
        }) as Box<FnMut(_)>);
        let mut tx2 = Some(tx2);
        let reject = Closure::wrap(Box::new(move |val| {
            drop(tx2.take().unwrap().send(val));
        }) as Box<FnMut(_)>);

        js.then2(&resolve, &reject);

        JsFuture {
            resolved: rx1,
            rejected: rx2,
            callbacks: Some((resolve, reject)),
        }
    }
}

impl Future for JsFuture {
    type Item = JsValue;
    type Error = JsValue;

    fn poll(&mut self) -> Poll<JsValue, JsValue> {
        // Test if either our resolved or rejected side is finished yet. Note
        // that they will return errors if they're disconnected which can't
        // happen until we drop the `callbacks` field, which doesn't happen
        // till we're done, so we dont need to handle that.
        if let Ok(Async::Ready(val)) = self.resolved.poll() {
            drop(self.callbacks.take());
            return Ok(val.into());
        }
        if let Ok(Async::Ready(val)) = self.rejected.poll() {
            drop(self.callbacks.take());
            return Err(val);
        }
        Ok(Async::NotReady)
    }
}

/// Converts a Rust `Future` into a JavaScript `Promise`.
///
/// This function will take any future in Rust and schedule it to be executed,
/// returning a JavaScript `Promise` which can then be passed back to JavaScript
/// to get plumbed into the rest of a system.
///
/// The `future` provided must adhere to `'static` because it'll be scheduled
/// to run in the background and cannot contain any stack references. The
/// returned `Promise` will be resolved or rejected when the future completes,
/// depending on whether it finishes with `Ok` or `Err`.
///
/// # Panics
///
/// Note that in wasm panics are currently translated to aborts, but "abort" in
/// this case means that a JavaScript exception is thrown. The wasm module is
/// still usable (likely erroneously) after Rust panics.
///
/// If the `future` provided panics then the returned `Promise` **will not
/// resolve**. Instead it will be a leaked promise. This is an unfortunate
/// limitation of wasm currently that's hoped to be fixed one day!
pub fn future_to_promise<F>(future: F) -> Promise
where
    F: Future<Item = JsValue, Error = JsValue> + 'static,
{
    _future_to_promise(Box::new(future))
}

// Implementation of actually transforming a future into a JavaScript `Promise`.
//
// The only primitive we have to work with here is `Promise::new`, which gives
// us two callbacks that we can use to either reject or resolve the promise.
// It's our job to ensure that one of those callbacks is called at the
// appropriate time.
//
// Now we know that JavaScript (in general) can't block and is largely
// notification/callback driven. That means that our future must either have
// synchronous computational work to do, or it's "scheduled a notification" to
// happen. These notifications are likely callbacks to get executed when things
// finish (like a different promise or something like `setTimeout`). The general
// idea here is thus to do as much synchronous work as we can and then otherwise
// translate notifications of a future's task into "let's poll the future!"
//
// This isn't necessarily the greatest future executor in the world, but it
// should get the job done for now hopefully.
fn _future_to_promise(future: Box<Future<Item = JsValue, Error = JsValue>>) -> Promise {
    let mut future = Some(executor::spawn(future));
    return Promise::new(&mut |resolve, reject| {
        Package::poll(&Arc::new(Package {
            spawn: RefCell::new(future.take().unwrap()),
            resolve,
            reject,
            notified: Cell::new(State::Notified),
        }));
    });

    struct Package {
        // Our "spawned future". This'll have everything we need to poll the
        // future and continue to move it forward.
        spawn: RefCell<Spawn<Box<Future<Item = JsValue, Error = JsValue>>>>,

        // The current state of this future, expressed in an enum below. This
        // indicates whether we're currently polling the future, received a
        // notification and need to keep polling, or if we're waiting for a
        // notification to come in (and no one is polling).
        notified: Cell<State>,

        // Our two callbacks connected to the `Promise` that we returned to
        // JavaScript.  We'll be invoking one of these at the end.
        resolve: Function,
        reject: Function,
    }

    // The possible states our `Package` (future) can be in, tracked internally
    // and used to guide what happens when polling a future.
    enum State {
        // This future is currently and actively being polled. Attempting to
        // access the future will result in a runtime panic and is considered a
        // bug.
        Polling,

        // This future has been notified, while it was being polled. This marker
        // is used in the `Notify` implementation below, and indicates that a
        // notification was received that the future is ready to make progress.
        // If seen, however, it probably means that the future is also currently
        // being polled.
        Notified,

        // The future is blocked, waiting for something to happen. Stored here
        // is a self-reference to the future itself so we can pull it out in
        // `Notify` and continue polling.
        //
        // Note that the self-reference here is an Arc-cycle that will leak
        // memory unless the future completes, but currently that should be ok
        // as we'll have to stick around anyway while the future is executing!
        //
        // This state is removed as soon as a notification comes in, so the leak
        // should only be "temporary"
        Waiting(Arc<Package>),
    }

    // No shared memory right now, wasm is single threaded, no need to worry
    // about this!
    unsafe impl Send for Package {}
    unsafe impl Sync for Package {}

    impl Package {
        // Move the future contained in `me` as far forward as we can. This will
        // do as much synchronous work as possible to complete the future,
        // ensuring that when it blocks we're scheduled to get notified via some
        // callback somewhere at some point (vague, right?)
        //
        // TODO: this probably shouldn't do as much synchronous work as possible
        //       as it can starve other computations. Rather it should instead
        //       yield every so often with something like `setTimeout` with the
        //       timeout set to zero.
        fn poll(me: &Arc<Package>) {
            loop {
                match me.notified.replace(State::Polling) {
                    // We received a notification while previously polling, or
                    // this is the initial poll. We've got work to do below!
                    State::Notified => {}

                    // We've gone through this loop once and no notification was
                    // received while we were executing work. That means we got
                    // `NotReady` below and we're scheduled to receive a
                    // notification. Block ourselves and wait for later.
                    //
                    // When the notification comes in it'll notify our task, see
                    // our `Waiting` state, and resume the polling process
                    State::Polling => {
                        me.notified.set(State::Waiting(me.clone()));
                        break;
                    }

                    State::Waiting(_) => panic!("shouldn't see waiting state!"),
                }

                let (val, f) = match me.spawn.borrow_mut().poll_future_notify(me, 0) {
                    // If the future is ready, immediately call the
                    // resolve/reject callback and then return as we're done.
                    Ok(Async::Ready(value)) => (value, &me.resolve),
                    Err(value) => (value, &me.reject),

                    // Otherwise keep going in our loop, if we weren't notified
                    // we'll break out and start waiting.
                    Ok(Async::NotReady) => continue,
                };

                drop(f.call1(&JsValue::undefined(), &val));
                break;
            }
        }
    }

    impl Notify for Package {
        fn notify(&self, _id: usize) {
            let me = match self.notified.replace(State::Notified) {
                // we need to schedule polling to resume, so keep going
                State::Waiting(me) => me,

                // we were already notified, and were just notified again;
                // having now coalesced the notifications we return as it's
                // still someone else's job to process this
                State::Notified => return,

                // the future was previously being polled, and we've just
                // switched it to the "you're notified" state. We don't have
                // access to the future as it's being polled, so the future
                // polling process later sees this notification and will
                // continue polling. For us, though, there's nothing else to do,
                // so we bail out.
                // later see
                State::Polling => return,
            };

            // Use `Promise.then` on a resolved promise to place our execution
            // onto the next turn of the microtask queue, enqueueing our poll
            // operation. We don't currently poll immediately as it turns out
            // `futures` crate adapters aren't compatible with it and it also
            // helps avoid blowing the stack by accident.
            //
            // Note that the `Rc`/`RefCell` trick here is basically to just
            // ensure that our `Closure` gets cleaned up appropriately.
            let promise = Promise::resolve(&JsValue::undefined());
            let slot = Rc::new(RefCell::new(None));
            let slot2 = slot.clone();
            let closure = Closure::wrap(Box::new(move |_| {
                let myself = slot2.borrow_mut().take();
                debug_assert!(myself.is_some());
                Package::poll(&me);
            }) as Box<FnMut(JsValue)>);
            promise.then(&closure);
            *slot.borrow_mut() = Some(closure);
        }
    }
}

/// Converts a Rust `Future` on a local task queue.
///
/// The `future` provided must adhere to `'static` because it'll be scheduled
/// to run in the background and cannot contain any stack references.
///
/// # Panics
///
/// This function has the same panic behavior as `future_to_promise`.
pub fn spawn_local<F>(future: F)
where
    F: Future<Item=(), Error=()> + 'static,
{
    future_to_promise(
        future
            .map(|()| JsValue::undefined())
            .or_else(|()| future::ok::<JsValue, JsValue>(JsValue::undefined())),
    );
}
