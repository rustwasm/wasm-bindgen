#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DOMMatrix2DInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomMatrix2dInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub type DomMatrix2dInit;
}
impl DomMatrix2dInit {
    #[doc = "Construct a new `DomMatrix2dInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `a` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn a(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("a"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `b` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn b(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("b"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `c` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn c(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("c"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `d` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn d(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("d"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `e` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn e(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("e"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `f` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn f(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("f"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `m11` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn m11(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("m11"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `m12` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn m12(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("m12"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `m21` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn m21(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("m21"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `m22` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn m22(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("m22"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `m41` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn m41(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("m41"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `m42` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn m42(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("m42"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for DomMatrix2dInit {
    fn default() -> Self {
        Self::new()
    }
}
