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
    #[wasm_bindgen(method, setter = "clientX")]
    fn client_x_shim(this: &TouchInit, val: i32);
    #[wasm_bindgen(method, setter = "clientY")]
    fn client_y_shim(this: &TouchInit, val: i32);
    #[wasm_bindgen(method, setter = "force")]
    fn force_shim(this: &TouchInit, val: f32);
    #[wasm_bindgen(method, setter = "identifier")]
    fn identifier_shim(this: &TouchInit, val: i32);
    #[wasm_bindgen(method, setter = "pageX")]
    fn page_x_shim(this: &TouchInit, val: i32);
    #[wasm_bindgen(method, setter = "pageY")]
    fn page_y_shim(this: &TouchInit, val: i32);
    #[wasm_bindgen(method, setter = "radiusX")]
    fn radius_x_shim(this: &TouchInit, val: f32);
    #[wasm_bindgen(method, setter = "radiusY")]
    fn radius_y_shim(this: &TouchInit, val: f32);
    #[wasm_bindgen(method, setter = "rotationAngle")]
    fn rotation_angle_shim(this: &TouchInit, val: f32);
    #[wasm_bindgen(method, setter = "screenX")]
    fn screen_x_shim(this: &TouchInit, val: i32);
    #[wasm_bindgen(method, setter = "screenY")]
    fn screen_y_shim(this: &TouchInit, val: i32);
    #[cfg(feature = "EventTarget")]
    #[wasm_bindgen(method, setter = "target")]
    fn target_shim(this: &TouchInit, val: &EventTarget);
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
        self.client_x_shim(val);
        self
    }
    #[doc = "Change the `clientY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn client_y(&mut self, val: i32) -> &mut Self {
        self.client_y_shim(val);
        self
    }
    #[doc = "Change the `force` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn force(&mut self, val: f32) -> &mut Self {
        self.force_shim(val);
        self
    }
    #[doc = "Change the `identifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn identifier(&mut self, val: i32) -> &mut Self {
        self.identifier_shim(val);
        self
    }
    #[doc = "Change the `pageX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn page_x(&mut self, val: i32) -> &mut Self {
        self.page_x_shim(val);
        self
    }
    #[doc = "Change the `pageY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn page_y(&mut self, val: i32) -> &mut Self {
        self.page_y_shim(val);
        self
    }
    #[doc = "Change the `radiusX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn radius_x(&mut self, val: f32) -> &mut Self {
        self.radius_x_shim(val);
        self
    }
    #[doc = "Change the `radiusY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn radius_y(&mut self, val: f32) -> &mut Self {
        self.radius_y_shim(val);
        self
    }
    #[doc = "Change the `rotationAngle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn rotation_angle(&mut self, val: f32) -> &mut Self {
        self.rotation_angle_shim(val);
        self
    }
    #[doc = "Change the `screenX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn screen_x(&mut self, val: i32) -> &mut Self {
        self.screen_x_shim(val);
        self
    }
    #[doc = "Change the `screenY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn screen_y(&mut self, val: i32) -> &mut Self {
        self.screen_y_shim(val);
        self
    }
    #[cfg(feature = "EventTarget")]
    #[doc = "Change the `target` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`, `TouchInit`*"]
    pub fn target(&mut self, val: &EventTarget) -> &mut Self {
        self.target_shim(val);
        self
    }
}
