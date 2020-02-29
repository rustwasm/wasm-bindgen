use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SocketsDict ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SocketsDict` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `SocketsDict`*
    pub type SocketsDict;

}

impl SocketsDict {
    ///Construct a new `SocketsDict`.
    ///
    ///*This API requires the following crate features to be activated: `SocketsDict`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `received` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `SocketsDict`*

    pub fn received(&mut self, val: f64) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("received"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `sent` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `SocketsDict`*

    pub fn sent(&mut self, val: f64) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("sent"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `sockets` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `SocketsDict`*

    pub fn sockets(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("sockets"),
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
