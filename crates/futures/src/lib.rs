//! A JS `Promise` to Rust `Future` bridge
//!
//! This crate provides a bridge for working with JS `Promise` types as a Rust
//! `Future`, and similarly contains utilities to turn a rust `Future` into a JS
//! `Promise`. This can be useful when working with asynchronous or otherwise
//! blocking work in Rust (wasm), and provides the ability to interoperate with
//! JS events and JS I/O primitives.
//!
//! There are two main interfaces in this crate currently:
//!
//! * `JsFuture` - a type that is constructed with a `Promise` and can then be
//!   used as a `Future<Item = JsValue, Error = JsValue>`. This Rust future will
//!   resolve or reject with the value coming out of the `Promise`.
//! * `rust2js` - converts a Rust `Future<Item = JsValue, Error = JsValue>` into
//!   a JS `Promise`. The future's result will translate to either a rejected or
//!   resolved `Promise` in JS.
//!
//! These two types should provide enough of a bridge to interoperate the two
//! systems and make sure that Rust/JS can work together with asynchronous and
//! I/O work.

#![deny(missing_docs)]
#![feature(use_extern_macros)]

extern crate futures;
extern crate wasm_bindgen;
extern crate js_sys;

use std::sync::Arc;
use std::rc::{Rc, Weak};
use std::cell::{RefCell, RefMut, Cell};

use futures::executor::{self, Spawn, Notify};
use futures::prelude::*;
use futures::task::{self, Task};
use js_sys::{Array, Function, Promise};
use wasm_bindgen::prelude::*;

/// A Rust `Future` backed by a JS `Promise`.
///
/// This type is constructed with a JS `Promise` object and translates it to a
/// Rust `Future`. This type implements the `Future` trait from the `futures`
/// crate and will either succeed or fail depending on what happens with the JS
/// `Promise`.
///
/// Currently this type is constructed with `JsFuture::from`.
pub struct JsFuture {
    promise: Promise,
    state: Rc<RefCell<State>>,
    registered: bool,
}

#[derive(Default)]
struct State {
    // TODO: investigate oneshot channels in the futures crate, may be slightly
    // more lightweight here.
    result: Option<Result<JsValue, JsValue>>,
    task: Option<Task>,
    callbacks: Option<(Closure<FnMut(JsValue)>, Closure<FnMut(JsValue)>)>,
}

impl JsFuture {
    fn register<'a>(&'a mut self) -> RefMut<'a, State> {
        if self.registered {
            return self.state.borrow_mut();
        }
        self.registered = true;
        let s = Rc::downgrade(&self.state);
        let cb1 = Closure::wrap(Box::new(move |val| finish(&s, Ok(val))) as Box<_>);
        let s = Rc::downgrade(&self.state);
        let cb2 = Closure::wrap(Box::new(move |val| finish(&s, Err(val))) as Box<_>);
        self.promise.then2(&cb1, &cb2);
        self.registered = true;
        // The callbacks are "neutered" or become inert as soon as we drop them,
        // so we store them locally until they're invoked, then we drop them to
        // deallocate their resources.
        let mut state = self.state.borrow_mut();
        state.callbacks = Some((cb1, cb2));
        return state
    }
}

impl Future for JsFuture {
    type Item = JsValue;
    type Error = JsValue;

    fn poll(&mut self) -> Poll<JsValue, JsValue> {
        // The implementation here of `Future` is pretty simple. The basic idea
        // is that we'll the `Promise.then` method to schedule some callbacks to
        // get notified when the value is ready to go. Once fired we'll fill in
        // a shared memory slot with the resulting value, and notify a future's
        // task, if any, that it's ready to go.
        //
        // As a result, the first step here is to make sure we've called
        // `Promise.then` (which only happens once).
        //
        // After that we take a look at our result to see if it's filled in yet.
        // If it is, great! If it's not, we stash away our task to get notified
        // and we're done!
        let mut state = self.register();
        match state.result.take() {
            Some(Ok(val)) => Ok(val.into()),
            Some(Err(val)) => Err(val),
            None => {
                state.task = Some(task::current());
                Ok(Async::NotReady)
            }
        }
    }
}

fn finish(state: &Weak<RefCell<State>>, result: Result<JsValue, JsValue>) {
    let task = {
        let state = match state.upgrade() {
            Some(s) => s,
            None => return,
        };
        let mut s = state.borrow_mut();
        // Fill in the result, this'll get picked up in the `Future`
        // implementation above.
        assert!(s.result.is_none());
        s.result = Some(result);

        // Drop our callbacks to ensure that their memory is deallocate. We're
        // currently invoking one and the other won't get invoked, so no need to
        // keep them around.
        assert!(s.callbacks.take().is_some());
        s.task.take()
    };

    // If a task was waiting on the data coming in, let it know!
    if let Some(task) = task {
        task.notify();
    }
}

impl From<Promise> for JsFuture {
    fn from(js: Promise) -> JsFuture {
        // Defer execution of the promise's `then` callbacks to a when the
        // future is itself polled.
        JsFuture {
            promise: js,
            registered: false,
            state: Default::default(),
        }
    }
}

/// Converts a Rust `Future` into a JS `Promise`.
///
/// This function will take any future in Rust and schedule it to be executed,
/// returning a JS `Promise` which can then be passed back to JS to get plumbed
/// into the rest of a system.
///
/// The `future` provided must adhere to `'static` because it'll be scheduled
/// to run in the background and cannot contain any stack references. The
/// returned `Promise` will be resolved or rejected when the future completes,
/// depending on whether it finishes with `Ok` or `Err`.
///
/// # Panics
///
/// Note that in wasm panics are currently translated to aborts, but "abort" in
/// this case means that a JS exception is thrown. The wasm module is still
/// usable (likely erroneously) after Rust panics.
///
/// If the `future` provided panics then the returned `Promise` **will not
/// resolve**. Instead it will be a leaked promise. This is an unfortunate
/// limitation of wasm currently that's hoped to be fixed one day!
pub fn rust2js<F>(future: F) -> Promise
    where F: Future<Item = JsValue, Error = JsValue> + 'static,
{
    _rust2js(Box::new(future))
}

// Implementation of actually transforming a future into a JS `Promise`.
//
// The only primitive we have to work with here is `Promise::new`, which gives
// us two callbacks that we can use to either reject or resolve the promise.
// It's our job to ensure that one of those callbacks is called at the
// appropriate time.
//
// Now we know that JS (in general) can't block and is largely
// notification/callback driven. That means that our future must either have
// synchronous computational work to do, or it's "scheduled a notification" to
// happen. These notifications are likely callbacks to get executed when things
// finish (like a different promise or something like `setTimeout`). The general
// idea here is thus to do as much synchronous work as we can and then otherwise
// translate notifications of a future's task into "let's poll the future!"
//
// This isn't necessarily the greatest future executor in the world, but it
// should get the job done for now hopefully.
fn _rust2js(future: Box<Future<Item = JsValue, Error = JsValue>>) -> Promise {
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

        // Our two callbacks connected to the `Promise` that we returned to JS.
        // We'll be invoking one of these at the end.
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
                        break
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

                let array = Array::new();
                array.push(&val);
                drop(f.apply(&JsValue::undefined(), &array));
                break
            }
        }
    }

    impl Notify for Package {
        fn notify(&self, _id: usize) {
            match self.notified.replace(State::Notified) {
                // we need to schedule polling to resume, so we do so
                // immediately for now
                State::Waiting(me) => Package::poll(&me),

                // we were already notified, and were just notified again;
                // having now coalesced the notifications we return as it's
                // still someone else's job to process this
                State::Notified => {}

                // the future was previously being polled, and we've just
                // switched it to the "you're notified" state. We don't have
                // access to the future as it's being polled, so the future
                // polling process later sees this notification and will
                // continue polling. For us, though, there's nothing else to do,
                // so we bail out.
                // later see
                State::Polling => {}
            }
        }
    }
}
