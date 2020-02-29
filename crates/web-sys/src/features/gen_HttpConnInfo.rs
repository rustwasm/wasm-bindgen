use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = HttpConnInfo ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HttpConnInfo` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `HttpConnInfo`*
    pub type HttpConnInfo;

}

impl HttpConnInfo {
    ///Construct a new `HttpConnInfo`.
    ///
    ///*This API requires the following crate features to be activated: `HttpConnInfo`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `protocolVersion` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `HttpConnInfo`*

    pub fn protocol_version(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("protocolVersion"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `rtt` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `HttpConnInfo`*

    pub fn rtt(&mut self, val: u32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("rtt"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `ttl` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `HttpConnInfo`*

    pub fn ttl(&mut self, val: u32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("ttl"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
