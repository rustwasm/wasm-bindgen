use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = ElementDefinitionOptions ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ElementDefinitionOptions` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `ElementDefinitionOptions`*
    pub type ElementDefinitionOptions;

}

impl ElementDefinitionOptions {
    ///Construct a new `ElementDefinitionOptions`.
    ///
    ///*This API requires the following crate features to be activated: `ElementDefinitionOptions`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `extends` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `ElementDefinitionOptions`*

    pub fn extends(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("extends"),
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
