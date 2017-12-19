#![feature(use_extern_macros)]

extern crate wasm_bindgen_macro;

use std::mem;

pub mod prelude {
    pub use wasm_bindgen_macro::wasm_bindgen;
    pub use JsObject;
}

pub struct JsObject {
    idx: u32,
}

impl JsObject {
    #[doc(hidden)]
    pub fn __from_idx(idx: u32) -> JsObject {
        JsObject { idx }
    }

    #[doc(hidden)]
    pub fn __get_idx(&self) -> u32 {
        self.idx
    }

    #[doc(hidden)]
    pub fn __into_idx(self) -> u32 {
        let ret = self.idx;
        mem::forget(self);
        return ret
    }
}

extern {
    fn __wasm_bindgen_object_clone_ref(idx: u32) -> u32;
    fn __wasm_bindgen_object_drop_ref(idx: u32);
}

impl Clone for JsObject {
    fn clone(&self) -> JsObject {
        unsafe {
            let idx = __wasm_bindgen_object_clone_ref(self.idx);
            JsObject { idx }
        }
    }
}

impl Drop for JsObject {
    fn drop(&mut self) {
        unsafe {
            __wasm_bindgen_object_drop_ref(self.idx);
        }
    }
}
