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
    #[wasm_bindgen(method, getter = "clientX")]
    fn client_x_shim(this: &TouchInit) -> i32;
    #[wasm_bindgen(method, setter = "clientX")]
    fn set_client_x_shim(this: &TouchInit, val: i32);
    #[wasm_bindgen(method, getter = "clientY")]
    fn client_y_shim(this: &TouchInit) -> i32;
    #[wasm_bindgen(method, setter = "clientY")]
    fn set_client_y_shim(this: &TouchInit, val: i32);
    #[wasm_bindgen(method, getter = "force")]
    fn force_shim(this: &TouchInit) -> f32;
    #[wasm_bindgen(method, setter = "force")]
    fn set_force_shim(this: &TouchInit, val: f32);
    #[wasm_bindgen(method, getter = "identifier")]
    fn identifier_shim(this: &TouchInit) -> i32;
    #[wasm_bindgen(method, setter = "identifier")]
    fn set_identifier_shim(this: &TouchInit, val: i32);
    #[wasm_bindgen(method, getter = "pageX")]
    fn page_x_shim(this: &TouchInit) -> i32;
    #[wasm_bindgen(method, setter = "pageX")]
    fn set_page_x_shim(this: &TouchInit, val: i32);
    #[wasm_bindgen(method, getter = "pageY")]
    fn page_y_shim(this: &TouchInit) -> i32;
    #[wasm_bindgen(method, setter = "pageY")]
    fn set_page_y_shim(this: &TouchInit, val: i32);
    #[wasm_bindgen(method, getter = "radiusX")]
    fn radius_x_shim(this: &TouchInit) -> f32;
    #[wasm_bindgen(method, setter = "radiusX")]
    fn set_radius_x_shim(this: &TouchInit, val: f32);
    #[wasm_bindgen(method, getter = "radiusY")]
    fn radius_y_shim(this: &TouchInit) -> f32;
    #[wasm_bindgen(method, setter = "radiusY")]
    fn set_radius_y_shim(this: &TouchInit, val: f32);
    #[wasm_bindgen(method, getter = "rotationAngle")]
    fn rotation_angle_shim(this: &TouchInit) -> f32;
    #[wasm_bindgen(method, setter = "rotationAngle")]
    fn set_rotation_angle_shim(this: &TouchInit, val: f32);
    #[wasm_bindgen(method, getter = "screenX")]
    fn screen_x_shim(this: &TouchInit) -> i32;
    #[wasm_bindgen(method, setter = "screenX")]
    fn set_screen_x_shim(this: &TouchInit, val: i32);
    #[wasm_bindgen(method, getter = "screenY")]
    fn screen_y_shim(this: &TouchInit) -> i32;
    #[wasm_bindgen(method, setter = "screenY")]
    fn set_screen_y_shim(this: &TouchInit, val: i32);
    #[cfg(feature = "EventTarget")]
    #[wasm_bindgen(method, getter = "target")]
    fn target_shim(this: &TouchInit) -> EventTarget;
    #[cfg(feature = "EventTarget")]
    #[wasm_bindgen(method, setter = "target")]
    fn set_target_shim(this: &TouchInit, val: &EventTarget);
}
#[doc = "The trait to access properties on the `TouchInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
pub trait TouchInitGetters {
    #[doc = "Get the `clientX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    fn client_x(&self) -> i32;
    #[doc = "Get the `clientY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    fn client_y(&self) -> i32;
    #[doc = "Get the `force` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    fn force(&self) -> f32;
    #[doc = "Get the `identifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    fn identifier(&self) -> i32;
    #[doc = "Get the `pageX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    fn page_x(&self) -> i32;
    #[doc = "Get the `pageY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    fn page_y(&self) -> i32;
    #[doc = "Get the `radiusX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    fn radius_x(&self) -> f32;
    #[doc = "Get the `radiusY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    fn radius_y(&self) -> f32;
    #[doc = "Get the `rotationAngle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    fn rotation_angle(&self) -> f32;
    #[doc = "Get the `screenX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    fn screen_x(&self) -> i32;
    #[doc = "Get the `screenY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    fn screen_y(&self) -> i32;
    #[cfg(feature = "EventTarget")]
    #[doc = "Get the `target` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`, `TouchInit`*"]
    fn target(&self) -> EventTarget;
}
impl TouchInitGetters for TouchInit {
    fn client_x(&self) -> i32 {
        self.client_x_shim()
    }
    fn client_y(&self) -> i32 {
        self.client_y_shim()
    }
    fn force(&self) -> f32 {
        self.force_shim()
    }
    fn identifier(&self) -> i32 {
        self.identifier_shim()
    }
    fn page_x(&self) -> i32 {
        self.page_x_shim()
    }
    fn page_y(&self) -> i32 {
        self.page_y_shim()
    }
    fn radius_x(&self) -> f32 {
        self.radius_x_shim()
    }
    fn radius_y(&self) -> f32 {
        self.radius_y_shim()
    }
    fn rotation_angle(&self) -> f32 {
        self.rotation_angle_shim()
    }
    fn screen_x(&self) -> i32 {
        self.screen_x_shim()
    }
    fn screen_y(&self) -> i32 {
        self.screen_y_shim()
    }
    #[cfg(feature = "EventTarget")]
    fn target(&self) -> EventTarget {
        self.target_shim()
    }
}
impl TouchInit {
    #[cfg(feature = "EventTarget")]
    #[doc = "Construct a new `TouchInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`, `TouchInit`*"]
    pub fn new(identifier: i32, target: &EventTarget) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::identifier(&mut ret, identifier);
        Self::target(&mut ret, target);
        ret
    }
    #[doc = "Change the `clientX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn client_x(&mut self, val: i32) -> &mut Self {
        self.set_client_x_shim(val);
        self
    }
    #[doc = "Change the `clientY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn client_y(&mut self, val: i32) -> &mut Self {
        self.set_client_y_shim(val);
        self
    }
    #[doc = "Change the `force` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn force(&mut self, val: f32) -> &mut Self {
        self.set_force_shim(val);
        self
    }
    #[doc = "Change the `identifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn identifier(&mut self, val: i32) -> &mut Self {
        self.set_identifier_shim(val);
        self
    }
    #[doc = "Change the `pageX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn page_x(&mut self, val: i32) -> &mut Self {
        self.set_page_x_shim(val);
        self
    }
    #[doc = "Change the `pageY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn page_y(&mut self, val: i32) -> &mut Self {
        self.set_page_y_shim(val);
        self
    }
    #[doc = "Change the `radiusX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn radius_x(&mut self, val: f32) -> &mut Self {
        self.set_radius_x_shim(val);
        self
    }
    #[doc = "Change the `radiusY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn radius_y(&mut self, val: f32) -> &mut Self {
        self.set_radius_y_shim(val);
        self
    }
    #[doc = "Change the `rotationAngle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn rotation_angle(&mut self, val: f32) -> &mut Self {
        self.set_rotation_angle_shim(val);
        self
    }
    #[doc = "Change the `screenX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn screen_x(&mut self, val: i32) -> &mut Self {
        self.set_screen_x_shim(val);
        self
    }
    #[doc = "Change the `screenY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn screen_y(&mut self, val: i32) -> &mut Self {
        self.set_screen_y_shim(val);
        self
    }
    #[cfg(feature = "EventTarget")]
    #[doc = "Change the `target` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`, `TouchInit`*"]
    pub fn target(&mut self, val: &EventTarget) -> &mut Self {
        self.set_target_shim(val);
        self
    }
}
