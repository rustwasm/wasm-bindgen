use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = AnimationPropertyValueDetails ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `AnimationPropertyValueDetails` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`*
    pub type AnimationPropertyValueDetails;

}

impl AnimationPropertyValueDetails {
    #[cfg(feature = "CompositeOperation")]
    ///Construct a new `AnimationPropertyValueDetails`.
    ///
    ///*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`, `CompositeOperation`*

    pub fn new(composite: CompositeOperation, offset: f64) -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret.composite(composite);

        ret.offset(offset);

        ret
    }

    #[cfg(feature = "CompositeOperation")]
    ///Change the `composite` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`, `CompositeOperation`*

    pub fn composite(&mut self, val: CompositeOperation) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("composite"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `easing` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`*

    pub fn easing(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("easing"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `offset` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`*

    pub fn offset(&mut self, val: f64) -> &mut Self {

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

    ///Change the `value` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`*

    pub fn value(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("value"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
