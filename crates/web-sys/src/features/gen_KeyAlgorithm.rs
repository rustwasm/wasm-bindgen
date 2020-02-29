use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = KeyAlgorithm ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `KeyAlgorithm` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `KeyAlgorithm`*
    pub type KeyAlgorithm;

}

impl KeyAlgorithm {
    ///Construct a new `KeyAlgorithm`.
    ///
    ///*This API requires the following crate features to be activated: `KeyAlgorithm`*

    pub fn new(name: &str) -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret.name(name);

        ret
    }

    ///Change the `name` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `KeyAlgorithm`*

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
}
