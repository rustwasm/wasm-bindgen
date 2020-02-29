use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = PushSubscriptionJSON ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PushSubscriptionJson` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `PushSubscriptionJson`*
    pub type PushSubscriptionJson;

}

impl PushSubscriptionJson {
    ///Construct a new `PushSubscriptionJson`.
    ///
    ///*This API requires the following crate features to be activated: `PushSubscriptionJson`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `endpoint` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `PushSubscriptionJson`*

    pub fn endpoint(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("endpoint"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(feature = "PushSubscriptionKeys")]
    ///Change the `keys` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `PushSubscriptionJson`, `PushSubscriptionKeys`*

    pub fn keys(&mut self, val: &PushSubscriptionKeys) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("keys"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
