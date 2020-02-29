use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = PushSubscriptionKeys ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PushSubscriptionKeys` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `PushSubscriptionKeys`*
    pub type PushSubscriptionKeys;

}

impl PushSubscriptionKeys {
    ///Construct a new `PushSubscriptionKeys`.
    ///
    ///*This API requires the following crate features to be activated: `PushSubscriptionKeys`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `auth` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `PushSubscriptionKeys`*

    pub fn auth(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("auth"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `p256dh` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `PushSubscriptionKeys`*

    pub fn p256dh(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("p256dh"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
