use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = FakePluginMimeEntry ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `FakePluginMimeEntry` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `FakePluginMimeEntry`*
    pub type FakePluginMimeEntry;

}

impl FakePluginMimeEntry {
    ///Construct a new `FakePluginMimeEntry`.
    ///
    ///*This API requires the following crate features to be activated: `FakePluginMimeEntry`*

    pub fn new(type_: &str) -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret.type_(type_);

        ret
    }

    ///Change the `description` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `FakePluginMimeEntry`*

    pub fn description(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("description"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `extension` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `FakePluginMimeEntry`*

    pub fn extension(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("extension"),
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
    ///*This API requires the following crate features to be activated: `FakePluginMimeEntry`*

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
