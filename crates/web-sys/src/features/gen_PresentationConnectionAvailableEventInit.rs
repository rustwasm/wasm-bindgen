use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = PresentationConnectionAvailableEventInit ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PresentationConnectionAvailableEventInit` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnectionAvailableEventInit`*
    pub type PresentationConnectionAvailableEventInit;

}

impl PresentationConnectionAvailableEventInit {
    #[cfg(feature = "PresentationConnection")]
    ///Construct a new `PresentationConnectionAvailableEventInit`.
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnection`, `PresentationConnectionAvailableEventInit`*

    pub fn new(connection: &PresentationConnection) -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret.connection(connection);

        ret
    }

    ///Change the `bubbles` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnectionAvailableEventInit`*

    pub fn bubbles(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("bubbles"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `cancelable` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnectionAvailableEventInit`*

    pub fn cancelable(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("cancelable"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `composed` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnectionAvailableEventInit`*

    pub fn composed(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("composed"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(feature = "PresentationConnection")]
    ///Change the `connection` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnection`, `PresentationConnectionAvailableEventInit`*

    pub fn connection(&mut self, val: &PresentationConnection) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("connection"),
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
