use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = ConnStatusDict ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ConnStatusDict` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `ConnStatusDict`*
    pub type ConnStatusDict;

}

impl ConnStatusDict {
    ///Construct a new `ConnStatusDict`.
    ///
    ///*This API requires the following crate features to be activated: `ConnStatusDict`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `status` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `ConnStatusDict`*

    pub fn status(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("status"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
