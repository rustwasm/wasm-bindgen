use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = ConsoleStackEntry ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ConsoleStackEntry` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `ConsoleStackEntry`*
    pub type ConsoleStackEntry;

}

impl ConsoleStackEntry {
    ///Construct a new `ConsoleStackEntry`.
    ///
    ///*This API requires the following crate features to be activated: `ConsoleStackEntry`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `asyncCause` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `ConsoleStackEntry`*

    pub fn async_cause(&mut self, val: Option<&str>) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("asyncCause"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `columnNumber` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `ConsoleStackEntry`*

    pub fn column_number(&mut self, val: u32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("columnNumber"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `filename` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `ConsoleStackEntry`*

    pub fn filename(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("filename"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `functionName` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `ConsoleStackEntry`*

    pub fn function_name(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("functionName"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `lineNumber` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `ConsoleStackEntry`*

    pub fn line_number(&mut self, val: u32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("lineNumber"),
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
