use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = CacheBatchOperation ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `CacheBatchOperation` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `CacheBatchOperation`*
    pub type CacheBatchOperation;

}

impl CacheBatchOperation {
    ///Construct a new `CacheBatchOperation`.
    ///
    ///*This API requires the following crate features to be activated: `CacheBatchOperation`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    #[cfg(feature = "CacheQueryOptions")]
    ///Change the `options` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `CacheBatchOperation`, `CacheQueryOptions`*

    pub fn options(&mut self, val: &CacheQueryOptions) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("options"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(feature = "Request")]
    ///Change the `request` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `CacheBatchOperation`, `Request`*

    pub fn request(&mut self, val: &Request) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("request"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(feature = "Response")]
    ///Change the `response` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `CacheBatchOperation`, `Response`*

    pub fn response(&mut self, val: &Response) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("response"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `type` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `CacheBatchOperation`*

    pub fn type_(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("type"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
