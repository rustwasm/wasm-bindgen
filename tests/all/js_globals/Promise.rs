#![allow(non_snake_case)]

use project;

#[test]
fn new() {
  project()
    .file("src/lib.rs", r#"
      #![feature(proc_macro, wasm_custom_section)]

      extern crate wasm_bindgen;
      use wasm_bindgen::prelude::*;
      use wasm_bindgen::js;

      #[wasm_bindgen]
      pub fn new_promise() -> js::Promise {
        let closure = Closure::new(|resolve, _reject| { resolve(42) });
        js::Promise::new(&closure)
      }
    "#)
    .file("test.ts", r#"
      import * as assert from "assert";
      import * as wasm from "./out";

      export function test() {
          assert.equal(wasm.new_promise() instanceof Promise, true);
      }
    "#)
    .test();
}
