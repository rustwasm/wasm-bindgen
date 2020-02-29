use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = WebSocketElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `WebSocketElement` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `WebSocketElement`*
    pub type WebSocketElement;

}

impl WebSocketElement {
    ///Construct a new `WebSocketElement`.
    ///
    ///*This API requires the following crate features to be activated: `WebSocketElement`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `encrypted` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `WebSocketElement`*

    pub fn encrypted(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("encrypted"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `hostport` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `WebSocketElement`*

    pub fn hostport(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("hostport"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `msgreceived` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `WebSocketElement`*

    pub fn msgreceived(&mut self, val: u32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("msgreceived"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `msgsent` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `WebSocketElement`*

    pub fn msgsent(&mut self, val: u32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("msgsent"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `receivedsize` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `WebSocketElement`*

    pub fn receivedsize(&mut self, val: f64) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("receivedsize"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `sentsize` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `WebSocketElement`*

    pub fn sentsize(&mut self, val: f64) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("sentsize"),
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
