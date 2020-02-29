use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = DeviceAccelerationInit ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `DeviceAccelerationInit` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `DeviceAccelerationInit`*
    pub type DeviceAccelerationInit;

}

impl DeviceAccelerationInit {
    ///Construct a new `DeviceAccelerationInit`.
    ///
    ///*This API requires the following crate features to be activated: `DeviceAccelerationInit`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `x` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `DeviceAccelerationInit`*

    pub fn x(&mut self, val: Option<f64>) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("x"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `y` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `DeviceAccelerationInit`*

    pub fn y(&mut self, val: Option<f64>) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("y"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `z` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `DeviceAccelerationInit`*

    pub fn z(&mut self, val: Option<f64>) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("z"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
