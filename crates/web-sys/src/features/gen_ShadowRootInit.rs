use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = ShadowRootInit ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ShadowRootInit` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `ShadowRootInit`*
    pub type ShadowRootInit;

}

impl ShadowRootInit {
    #[cfg(feature = "ShadowRootMode")]
    ///Construct a new `ShadowRootInit`.
    ///
    ///*This API requires the following crate features to be activated: `ShadowRootInit`, `ShadowRootMode`*

    pub fn new(mode: ShadowRootMode) -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret.mode(mode);

        ret
    }

    #[cfg(feature = "ShadowRootMode")]
    ///Change the `mode` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `ShadowRootInit`, `ShadowRootMode`*

    pub fn mode(&mut self, val: ShadowRootMode) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("mode"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
