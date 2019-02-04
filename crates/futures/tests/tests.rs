#![cfg(target_arch = "wasm32")]

extern crate futures;
extern crate js_sys;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;
extern crate wasm_bindgen_test;

use futures::unsync::oneshot;
use futures::Future;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{future_to_promise, spawn_local, JsFuture};
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

#[wasm_bindgen_test(async)]
fn spawn_local_runs() -> impl Future<Item = (), Error = JsValue> {
    let (tx, rx) = oneshot::channel::<u32>();
    let fn_box = Box::new(move || {
        tx.send(42).unwrap();
    });
    spawn_local(futures::future::ok::<(), ()>(()).map(|_| {
        fn_box();
    }));
    rx.then(|val| {
        if val == Ok(42) {
            Ok(())
        } else {
            Err(JsValue::undefined())
        }
    })
}

/// check that `spawn_local` does not forward the `future::err` as an unchecked rejection
#[wasm_bindgen_test(async)]
fn spawn_local_err_no_exception() -> impl Future<Item = (), Error = JsValue> {
    let (tx, rx) = oneshot::channel::<u32>();
    let fn_box = Box::new(move || {
        tx.send(42).unwrap();
    });
    // Promises should run in a deterministic order, so the `err` should be handled during the execution of this test.
    spawn_local(futures::future::err::<(), ()>(()));
    spawn_local(futures::future::ok::<(), ()>(()).map(|_| {
        fn_box();
    }));
    rx.then(|val| {
        if val == Ok(42) {
            Ok(())
        } else {
            Err(JsValue::undefined())
        }
    })
}

#[wasm_bindgen_test(async)]
fn can_create_multiple_futures_from_same_promise() -> impl Future<Item = (), Error = JsValue> {
    let promise = js_sys::Promise::resolve(&JsValue::null());
    let a = JsFuture::from(promise.clone());
    let b = JsFuture::from(promise);
    futures::future::join_all(vec![a, b]).map(|_| ())
}
