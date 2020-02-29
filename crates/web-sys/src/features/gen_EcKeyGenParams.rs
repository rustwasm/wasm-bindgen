use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = EcKeyGenParams ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `EcKeyGenParams` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `EcKeyGenParams`*
    pub type EcKeyGenParams;

}

impl EcKeyGenParams {
    ///Construct a new `EcKeyGenParams`.
    ///
    ///*This API requires the following crate features to be activated: `EcKeyGenParams`*

    pub fn new(name: &str, named_curve: &str) -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret.name(name);

        ret.named_curve(named_curve);

        ret
    }

    ///Change the `name` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `EcKeyGenParams`*

    pub fn name(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("name"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `namedCurve` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `EcKeyGenParams`*

    pub fn named_curve(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("namedCurve"),
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
