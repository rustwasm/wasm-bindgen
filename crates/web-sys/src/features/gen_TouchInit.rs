#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TouchInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TouchInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub type TouchInit;
    #[doc = "Get the `clientX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    #[wasm_bindgen(method, getter = "clientX")]
    pub fn get_client_x(this: &TouchInit) -> Option<i32>;
    #[wasm_bindgen(method, setter = "clientX")]
    fn set_client_x(this: &TouchInit, val: i32);
    #[doc = "Get the `clientY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    #[wasm_bindgen(method, getter = "clientY")]
    pub fn get_client_y(this: &TouchInit) -> Option<i32>;
    #[wasm_bindgen(method, setter = "clientY")]
    fn set_client_y(this: &TouchInit, val: i32);
    #[doc = "Get the `force` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    #[wasm_bindgen(method, getter = "force")]
    pub fn get_force(this: &TouchInit) -> Option<f32>;
    #[wasm_bindgen(method, setter = "force")]
    fn set_force(this: &TouchInit, val: f32);
    #[doc = "Get the `identifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    #[wasm_bindgen(method, getter = "identifier")]
    pub fn get_identifier(this: &TouchInit) -> i32;
    #[wasm_bindgen(method, setter = "identifier")]
    fn set_identifier(this: &TouchInit, val: i32);
    #[doc = "Get the `pageX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    #[wasm_bindgen(method, getter = "pageX")]
    pub fn get_page_x(this: &TouchInit) -> Option<i32>;
    #[wasm_bindgen(method, setter = "pageX")]
    fn set_page_x(this: &TouchInit, val: i32);
    #[doc = "Get the `pageY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    #[wasm_bindgen(method, getter = "pageY")]
    pub fn get_page_y(this: &TouchInit) -> Option<i32>;
    #[wasm_bindgen(method, setter = "pageY")]
    fn set_page_y(this: &TouchInit, val: i32);
    #[doc = "Get the `radiusX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    #[wasm_bindgen(method, getter = "radiusX")]
    pub fn get_radius_x(this: &TouchInit) -> Option<f32>;
    #[wasm_bindgen(method, setter = "radiusX")]
    fn set_radius_x(this: &TouchInit, val: f32);
    #[doc = "Get the `radiusY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    #[wasm_bindgen(method, getter = "radiusY")]
    pub fn get_radius_y(this: &TouchInit) -> Option<f32>;
    #[wasm_bindgen(method, setter = "radiusY")]
    fn set_radius_y(this: &TouchInit, val: f32);
    #[doc = "Get the `rotationAngle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    #[wasm_bindgen(method, getter = "rotationAngle")]
    pub fn get_rotation_angle(this: &TouchInit) -> Option<f32>;
    #[wasm_bindgen(method, setter = "rotationAngle")]
    fn set_rotation_angle(this: &TouchInit, val: f32);
    #[doc = "Get the `screenX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    #[wasm_bindgen(method, getter = "screenX")]
    pub fn get_screen_x(this: &TouchInit) -> Option<i32>;
    #[wasm_bindgen(method, setter = "screenX")]
    fn set_screen_x(this: &TouchInit, val: i32);
    #[doc = "Get the `screenY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    #[wasm_bindgen(method, getter = "screenY")]
    pub fn get_screen_y(this: &TouchInit) -> Option<i32>;
    #[wasm_bindgen(method, setter = "screenY")]
    fn set_screen_y(this: &TouchInit, val: i32);
    #[cfg(feature = "EventTarget")]
    #[doc = "Get the `target` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`, `TouchInit`*"]
    #[wasm_bindgen(method, getter = "target")]
    pub fn get_target(this: &TouchInit) -> EventTarget;
    #[cfg(feature = "EventTarget")]
    #[wasm_bindgen(method, setter = "target")]
    fn set_target(this: &TouchInit, val: &EventTarget);
}
impl TouchInit {
    #[cfg(feature = "EventTarget")]
    #[doc = "Construct a new `TouchInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`, `TouchInit`*"]
    pub fn new(identifier: i32, target: &EventTarget) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.identifier(identifier);
        ret.target(target);
        ret
    }
    #[doc = "Change the `clientX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn client_x(&mut self, val: i32) -> &mut Self {
        self.set_client_x(val);
        self
    }
    #[doc = "Change the `clientY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn client_y(&mut self, val: i32) -> &mut Self {
        self.set_client_y(val);
        self
    }
    #[doc = "Change the `force` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn force(&mut self, val: f32) -> &mut Self {
        self.set_force(val);
        self
    }
    #[doc = "Change the `identifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn identifier(&mut self, val: i32) -> &mut Self {
        self.set_identifier(val);
        self
    }
    #[doc = "Change the `pageX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn page_x(&mut self, val: i32) -> &mut Self {
        self.set_page_x(val);
        self
    }
    #[doc = "Change the `pageY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn page_y(&mut self, val: i32) -> &mut Self {
        self.set_page_y(val);
        self
    }
    #[doc = "Change the `radiusX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn radius_x(&mut self, val: f32) -> &mut Self {
        self.set_radius_x(val);
        self
    }
    #[doc = "Change the `radiusY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn radius_y(&mut self, val: f32) -> &mut Self {
        self.set_radius_y(val);
        self
    }
    #[doc = "Change the `rotationAngle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn rotation_angle(&mut self, val: f32) -> &mut Self {
        self.set_rotation_angle(val);
        self
    }
    #[doc = "Change the `screenX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn screen_x(&mut self, val: i32) -> &mut Self {
        self.set_screen_x(val);
        self
    }
    #[doc = "Change the `screenY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn screen_y(&mut self, val: i32) -> &mut Self {
        self.set_screen_y(val);
        self
    }
    #[cfg(feature = "EventTarget")]
    #[doc = "Change the `target` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`, `TouchInit`*"]
    pub fn target(&mut self, val: &EventTarget) -> &mut Self {
        self.set_target(val);
        self
    }
}
