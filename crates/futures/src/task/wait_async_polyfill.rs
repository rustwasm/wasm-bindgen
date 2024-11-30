//!
//! The polyfill was kindly borrowed from https://github.com/tc39/proposal-atomics-wait-async
//! and ported to Rust
//!

#![allow(clippy::incompatible_msrv)]

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * Author: Lars T Hansen, lhansen@mozilla.com
 */

/* Polyfill for Atomics.waitAsync() for web browsers.
 *
 * Any kind of agent that is able to create a new Worker can use this polyfill.
 *
 * Load this file in all agents that will use Atomics.waitAsync.
 *
 * Agents that don't call Atomics.waitAsync need do nothing special.
 *
 * Any kind of agent can wake another agent that is sleeping in
 * Atomics.waitAsync by just calling Atomics.wake for the location being slept
 * on, as normal.
 *
 * The implementation is not completely faithful to the proposed semantics: in
 * the case where an agent first asyncWaits and then waits on the same location:
 * when it is woken, the two waits will be woken in order, while in the real
 * semantics, the sync wait will be woken first.
 *
 * In this polyfill Atomics.waitAsync is not very fast.
 */

/* Implementation:
 *
 * For every wait we fork off a Worker to perform the wait.  Workers are reused
 * when possible.  The worker communicates with its parent using postMessage.
 */

use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::sync::atomic::AtomicI32;
use js_sys::{Array, Promise};
use wasm_bindgen::prelude::*;
use web_sys::{MessageEvent, Worker};

struct Helpers;

impl Helpers {
    #[cfg(feature = "std")]
    pub(crate) fn with<R>(f: impl FnOnce(&RefCell<Vec<Worker>>) -> R) -> R {
        thread_local! {
            static HELPERS: RefCell<Vec<Worker>> = RefCell::new(vec![]);
        }

        HELPERS.with(f)
    }

    #[cfg(not(feature = "std"))]
    pub(crate) fn with<R>(f: impl FnOnce(&RefCell<Vec<Worker>>) -> R) -> R {
        #[thread_local]
        static HELPERS: RefCell<Vec<Worker>> = RefCell::new(vec![]);

        f(&HELPERS)
    }
}

fn alloc_helper() -> Worker {
    Helpers::with(|helpers| {
        if let Some(helper) = helpers.borrow_mut().pop() {
            return helper;
        }

        let worker_url = wasm_bindgen::link_to!(module = "/src/task/worker.js");
        Worker::new(&worker_url).unwrap_or_else(|js| wasm_bindgen::throw_val(js))
    })
}

fn free_helper(helper: Worker) {
    Helpers::with(move |helpers| {
        let mut helpers = helpers.borrow_mut();
        helpers.push(helper.clone());
        helpers.truncate(10); // random arbitrary limit chosen here
    });
}

pub fn wait_async(ptr: &AtomicI32, value: i32) -> Promise {
    Promise::new(&mut |resolve, _reject| {
        let helper = alloc_helper();
        let helper_ref = helper.clone();

        let onmessage_callback = Closure::once_into_js(move |e: MessageEvent| {
            // Our helper is done waiting so it's available to wait on a
            // different location, so return it to the free list.
            free_helper(helper_ref);
            drop(resolve.call1(&JsValue::NULL, &e.data()));
        });
        helper.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));

        let data = Array::of3(
            &wasm_bindgen::memory(),
            &JsValue::from(ptr.as_ptr() as u32 / 4),
            &JsValue::from(value),
        );

        helper
            .post_message(&data)
            .unwrap_or_else(|js| wasm_bindgen::throw_val(js));
    })
}
