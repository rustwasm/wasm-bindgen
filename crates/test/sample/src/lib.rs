#![feature(use_extern_macros)]

#[macro_use]
extern crate futures;
extern crate js_sys;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;

use std::time::Duration;

use futures::prelude::*;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

pub struct Timeout {
    id: u32,
    inner: JsFuture,
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_name = setTimeout)]
    fn set_timeout(closure: JsValue, millis: f64) -> u32;

    #[wasm_bindgen(js_name = clearTimeout)]
    fn clear_timeout(id: u32);
}

impl Timeout {
    pub fn new(dur: Duration) -> Timeout {
        let millis = dur.as_secs()
            .checked_mul(1000)
            .unwrap()
            .checked_add(dur.subsec_millis() as u64)
            .unwrap() as f64; // TODO: checked cast

        let mut id = None;
        let promise = Promise::new(&mut |resolve, _reject| {
            id = Some(set_timeout(resolve.into(), millis));
        });

        Timeout {
            id: id.unwrap(),
            inner: JsFuture::from(promise),
        }
    }
}

impl Future for Timeout {
    type Item = ();
    type Error = JsValue;

    fn poll(&mut self) -> Poll<(), JsValue> {
        let _obj = try_ready!(self.inner.poll());
        Ok(().into())
    }
}

impl Drop for Timeout {
    fn drop(&mut self) {
        clear_timeout(self.id);
    }
}
