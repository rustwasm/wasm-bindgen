#![feature(use_extern_macros)]

extern crate wasm_bindgen;
extern crate js_sys;
extern crate web_sys;
extern crate wasm_bindgen_futures;
extern crate futures;
#[macro_use]
extern crate serde_derive;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::Promise;
use web_sys::{Request, RequestInit, RequestMode, Response, Window};
use wasm_bindgen_futures::JsFuture;
use futures::{future, Future};
use wasm_bindgen_futures::future_to_promise;

// A struct to hold some data from the github Branch API.
// Note how we don't have to define every member -- serde will ignore extra data when deserializing
#[derive(Debug, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub commit: Commit,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub sha: String,
    pub commit: CommitDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitDetails {
    pub author: Signature,
    pub committer: Signature,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Signature {
    pub name: String,
    pub email: String,
}

#[wasm_bindgen]
extern "C" {
    static window: Window;
}

#[wasm_bindgen]
pub fn run() -> Promise {
    let mut request_options = RequestInit::new();
    request_options.method("GET");
    request_options.mode(RequestMode::Cors);

    let req = Request::new_using_usv_str_and_request_init("https://api.github.com/repos/rustwasm/wasm-bindgen/branches/master", &request_options).unwrap();

    // the RequestInit struct will eventually support setting headers, but that's missing right now
    req.headers().set("Accept", "application/vnd.github.v3+json").unwrap();

    let req_promise = window.fetch_using_request(&req);

    let to_return = JsFuture::from(req_promise).and_then(|resp_value| {
        // resp_value is a Response object
        assert!(resp_value.is_instance_of::<Response>());
        let resp: Response = resp_value.dyn_into().unwrap();

        resp.json()


    }).and_then(|json_value: Promise| {
        // convert this other promise into a rust Future
        JsFuture::from(json_value)
    }).and_then(|json| {
        // Use serde to parse this into a struct
        let branch_info: Branch = json.into_serde().unwrap();

        // Send the Branch struct back to javascript as an object
        future::ok(JsValue::from_serde(&branch_info).unwrap())
    });

    // Convert this rust future back into a javascript promise.
    // Return it to javascript so that it can be driven to completion.
    future_to_promise(to_return)
}
