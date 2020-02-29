use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = CacheQueryOptions ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `CacheQueryOptions` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `CacheQueryOptions`*
    pub type CacheQueryOptions;

}

impl CacheQueryOptions {
    ///Construct a new `CacheQueryOptions`.
    ///
    ///*This API requires the following crate features to be activated: `CacheQueryOptions`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `cacheName` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `CacheQueryOptions`*

    pub fn cache_name(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("cacheName"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `ignoreMethod` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `CacheQueryOptions`*

    pub fn ignore_method(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("ignoreMethod"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `ignoreSearch` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `CacheQueryOptions`*

    pub fn ignore_search(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("ignoreSearch"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `ignoreVary` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `CacheQueryOptions`*

    pub fn ignore_vary(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("ignoreVary"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
