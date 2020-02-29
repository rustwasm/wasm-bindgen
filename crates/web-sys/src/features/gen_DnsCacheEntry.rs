use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = DnsCacheEntry ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `DnsCacheEntry` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `DnsCacheEntry`*
    pub type DnsCacheEntry;

}

impl DnsCacheEntry {
    ///Construct a new `DnsCacheEntry`.
    ///
    ///*This API requires the following crate features to be activated: `DnsCacheEntry`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `expiration` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `DnsCacheEntry`*

    pub fn expiration(&mut self, val: f64) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("expiration"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `family` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `DnsCacheEntry`*

    pub fn family(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("family"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `hostaddr` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `DnsCacheEntry`*

    pub fn hostaddr(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("hostaddr"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `hostname` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `DnsCacheEntry`*

    pub fn hostname(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("hostname"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `trr` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `DnsCacheEntry`*

    pub fn trr(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("trr"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
