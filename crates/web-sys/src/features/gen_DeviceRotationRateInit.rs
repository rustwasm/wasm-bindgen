use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = DeviceRotationRateInit ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `DeviceRotationRateInit` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `DeviceRotationRateInit`*
    pub type DeviceRotationRateInit;

}

impl DeviceRotationRateInit {
    ///Construct a new `DeviceRotationRateInit`.
    ///
    ///*This API requires the following crate features to be activated: `DeviceRotationRateInit`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `alpha` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `DeviceRotationRateInit`*

    pub fn alpha(&mut self, val: Option<f64>) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("alpha"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `beta` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `DeviceRotationRateInit`*

    pub fn beta(&mut self, val: Option<f64>) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("beta"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `gamma` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `DeviceRotationRateInit`*

    pub fn gamma(&mut self, val: Option<f64>) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("gamma"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
