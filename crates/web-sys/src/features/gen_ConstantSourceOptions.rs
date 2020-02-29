use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = ConstantSourceOptions ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ConstantSourceOptions` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `ConstantSourceOptions`*
    pub type ConstantSourceOptions;

}

impl ConstantSourceOptions {
    ///Construct a new `ConstantSourceOptions`.
    ///
    ///*This API requires the following crate features to be activated: `ConstantSourceOptions`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `offset` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `ConstantSourceOptions`*

    pub fn offset(&mut self, val: f32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("offset"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
