use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = UIEventInit ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `UiEventInit` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `UiEventInit`*
    pub type UiEventInit;

}

impl UiEventInit {
    ///Construct a new `UiEventInit`.
    ///
    ///*This API requires the following crate features to be activated: `UiEventInit`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `bubbles` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `UiEventInit`*

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
    ///*This API requires the following crate features to be activated: `UiEventInit`*

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
    ///*This API requires the following crate features to be activated: `UiEventInit`*

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

    ///Change the `detail` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `UiEventInit`*

    pub fn detail(&mut self, val: i32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("detail"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(feature = "Window")]
    ///Change the `view` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `UiEventInit`, `Window`*

    pub fn view(&mut self, val: Option<&Window>) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("view"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
