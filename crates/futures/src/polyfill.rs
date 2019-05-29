/*
 * The polyfill was kindly borrowed from https://github.com/tc39/proposal-atomics-wait-async
 * and ported to Rust
 */

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

use std::cell::RefCell;
use std::rc::Rc;

use js_sys::{
    encode_uri_component, Array, Atomics, Error, Function, Int32Array, JsString, Promise, Reflect,
    SharedArrayBuffer,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{MessageEvent, Worker};

const HELPER_CODE: &'static str = "
onmessage = function (ev) {
    try {
        switch (ev.data[0]) {
            case 'wait': {
                let [_, ia, index, value, timeout] = ev.data;
                let result = Atomics.wait(ia, index, value, timeout);
                postMessage(['ok', result]);
                break;
            }
            default: {
                throw new Error('Wrong message sent to wait helper: ' + ev.data.join(','));
            }
        }
    } catch (e) {
        console.log('Exception in wait helper');
        postMessage(['error', 'Exception']);
    }
}
";

thread_local! {
    static HELPERS: RefCell<Vec<Rc<RefCell<Worker>>>> = RefCell::new(vec![]);
}

fn alloc_helper() -> Rc<RefCell<Worker>> {
    HELPERS.with(|helpers| {
        if let Some(helper) = helpers.borrow_mut().pop() {
            return helper;
        }

        let mut initialization_string = "data:application/javascript,".to_owned();
        let encoded: String = encode_uri_component(HELPER_CODE).into();
        initialization_string.push_str(&encoded);

        return Rc::new(RefCell::new(
            Worker::new(&initialization_string).expect("Should create a Worker"),
        ));
    })
}

fn free_helper(helper: &Rc<RefCell<Worker>>) {
    HELPERS.with(move |helpers| {
        helpers.borrow_mut().push(helper.clone());
    });
}

pub fn wait_async(indexed_array: Int32Array, index: u32, value: i32) -> Result<Promise, JsValue> {
    let timeout = 0.0;
    wait_async_with_timeout(indexed_array, index, value, timeout)
}

fn get_array_item(array: &JsValue, index: u32) -> JsValue {
    Reflect::get(array, &JsValue::from(index))
        .expect(&format!("Array should contain the index {}", index))
}

// Atomics.waitAsync always returns a promise. Throws standard errors
// for parameter validation.  The promise is resolved with a string as from
// Atomics.wait, or, in the case something went completely wrong, it is
// rejected with an error string.
pub fn wait_async_with_timeout(
    indexed_array: Int32Array,
    index: u32,
    value: i32,
    timeout: f64,
) -> Result<Promise, JsValue> {
    if !indexed_array.buffer().has_type::<SharedArrayBuffer>() {
        return Err(Error::new("Indexed array must be created from SharedArrayBuffer").into());
    }

    // Optimization, avoid the helper thread in this common case.
    if Atomics::load(&indexed_array, index)? != value {
        return Ok(Promise::resolve(&JsString::from("not-equal")));
    }

    // General case, we must wait.

    Ok(Promise::new(
        &mut Box::new(move |resolve: Function, reject: Function| {
            let helper = alloc_helper();
            let helper_ref = helper.clone();

            let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
                // Free the helper early so that it can be reused if the resolution
                // needs a helper.
                free_helper(&helper_ref);
                match String::from(
                    get_array_item(&e.data(), 0)
                        .as_string()
                        .expect("data[0] should return a String"),
                )
                .as_str()
                {
                    "ok" => {
                        resolve
                            .call1(&JsValue::NULL, &get_array_item(&e.data(), 1))
                            .expect("Should successfully call a resolve callback");
                    }
                    "error" => {
                        // Note, rejection is not in the spec, it is an artifact of the polyfill.
                        // The helper already printed an error to the console.
                        reject
                            .call1(&JsValue::NULL, &get_array_item(&e.data(), 1))
                            .expect("Should successfully call a reject callback");
                    }
                    // it's not specified in the proposal yet
                    _ => (),
                }
            }) as Box<dyn FnMut(MessageEvent)>);
            helper
                .borrow()
                .set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));

            // It's possible to do better here if the ia is already known to the
            // helper.  In that case we can communicate the other data through
            // shared memory and wake the agent.  And it is possible to make ia
            // known to the helper by waking it with a special value so that it
            // checks its messages, and then posting the ia to the helper.  Some
            // caching / decay scheme is useful no doubt, to improve performance
            // and avoid leaks.
            //
            // In the event we wake the helper directly, we can micro-wait here
            // for a quick result.  We'll need to restructure some code to make
            // that work out properly, and some synchronization is necessary for
            // the helper to know that we've picked up the result and no
            // postMessage is necessary.

            let data = Array::of5(
                &JsString::from("wait"),
                &indexed_array,
                &JsValue::from(index),
                &JsValue::from(value),
                &JsValue::from(timeout),
            );

            helper
                .borrow()
                .post_message(&data)
                .expect("Should successfully post data to a Worker");
        }) as &mut dyn FnMut(Function, Function),
    ))
}
