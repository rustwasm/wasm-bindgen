#![cfg(target_arch = "wasm32")]

extern crate futures;
extern crate js_sys;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;
extern crate wasm_bindgen_test;

use futures::unsync::oneshot;
use futures::Future;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use wasm_bindgen_test::*;

#[wasm_bindgen_test(async)]
fn promise_resolve_is_ok_future() -> impl Future<Item = (), Error = JsValue> {
    let p = js_sys::Promise::resolve(&JsValue::from(42));
    JsFuture::from(p)
        .map(|x| {
            assert_eq!(x, 42);
        })
        .map_err(|_| unreachable!())
}

#[wasm_bindgen_test(async)]
fn promise_reject_is_error_future() -> impl Future<Item = (), Error = JsValue> {
    let p = js_sys::Promise::reject(&JsValue::from(42));
    JsFuture::from(p).map(|_| unreachable!()).or_else(|e| {
        assert_eq!(e, 42);
        Ok(())
    })
}

#[wasm_bindgen_test(async)]
fn ok_future_is_resolved_promise() -> impl Future<Item = (), Error = JsValue> {
    let f = futures::future::ok(JsValue::from(42));
    let p = future_to_promise(f);
    JsFuture::from(p)
        .map(|x| {
            assert_eq!(x, 42);
        })
        .map_err(|_| unreachable!())
}

#[wasm_bindgen_test(async)]
fn error_future_is_rejected_promise() -> impl Future<Item = (), Error = JsValue> {
    let f = futures::future::err(JsValue::from(42));
    let p = future_to_promise(f);
    JsFuture::from(p).map(|_| unreachable!()).or_else(|e| {
        assert_eq!(e, 42);
        Ok(())
    })
}

#[wasm_bindgen]
extern "C" {
    fn setTimeout(c: &Closure<FnMut()>);
}

#[wasm_bindgen_test(async)]
fn oneshot_works() -> impl Future<Item = (), Error = JsValue> {
    let (tx, rx) = oneshot::channel::<u32>();
    let mut tx = Some(tx);
    let closure = Closure::wrap(Box::new(move || {
        drop(tx.take().unwrap());
    }) as Box<FnMut()>);
    setTimeout(&closure);
    closure.forget();
    rx.then(|_| Ok(()))
}
