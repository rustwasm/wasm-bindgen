#![cfg(target_arch = "wasm32")]
#![feature(use_extern_macros)]
#![allow(non_snake_case)]

extern crate js_sys;
extern crate wasm_bindgen;
extern crate wasm_bindgen_test;

pub mod global_fns;
pub mod Array;
pub mod ArrayBuffer;
pub mod ArrayIterator;
pub mod Boolean;
pub mod DataView;
pub mod Date;
pub mod Error;
pub mod Function;
pub mod Generator;
pub mod Intl;
pub mod JsString;
pub mod Map;
pub mod MapIterator;
pub mod Math;
pub mod Number;
pub mod Object;
pub mod Proxy;
pub mod Reflect;
pub mod RegExp;
pub mod Set;
pub mod SetIterator;
pub mod Symbol;
pub mod TypedArray;
pub mod WeakMap;
pub mod WeakSet;
pub mod WebAssembly;
