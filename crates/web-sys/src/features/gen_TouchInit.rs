use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = TouchInit ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `TouchInit` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `TouchInit`*
    pub type TouchInit;

}

impl TouchInit {
    #[cfg(feature = "EventTarget")]
    ///Construct a new `TouchInit`.
    ///
    ///*This API requires the following crate features to be activated: `EventTarget`, `TouchInit`*

    pub fn new(identifier: i32, target: &EventTarget) -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret.identifier(identifier);

        ret.target(target);

        ret
    }

    ///Change the `clientX` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `TouchInit`*

    pub fn client_x(&mut self, val: i32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("clientX"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `clientY` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `TouchInit`*

    pub fn client_y(&mut self, val: i32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("clientY"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `force` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `TouchInit`*

    pub fn force(&mut self, val: f32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("force"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `identifier` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `TouchInit`*

    pub fn identifier(&mut self, val: i32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("identifier"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `pageX` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `TouchInit`*

    pub fn page_x(&mut self, val: i32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("pageX"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `pageY` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `TouchInit`*

    pub fn page_y(&mut self, val: i32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("pageY"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `radiusX` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `TouchInit`*

    pub fn radius_x(&mut self, val: f32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("radiusX"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `radiusY` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `TouchInit`*

    pub fn radius_y(&mut self, val: f32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("radiusY"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `rotationAngle` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `TouchInit`*

    pub fn rotation_angle(&mut self, val: f32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("rotationAngle"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `screenX` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `TouchInit`*

    pub fn screen_x(&mut self, val: i32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("screenX"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `screenY` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `TouchInit`*

    pub fn screen_y(&mut self, val: i32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("screenY"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(feature = "EventTarget")]
    ///Change the `target` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `EventTarget`, `TouchInit`*

    pub fn target(&mut self, val: &EventTarget) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("target"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
