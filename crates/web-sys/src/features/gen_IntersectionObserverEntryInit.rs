#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = IntersectionObserverEntryInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IntersectionObserverEntryInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverEntryInit`*"]
    pub type IntersectionObserverEntryInit;
    #[cfg(feature = "DomRectInit")]
    #[wasm_bindgen(method, getter = "boundingClientRect")]
    fn bounding_client_rect_shim(this: &IntersectionObserverEntryInit) -> DomRectInit;
    #[cfg(feature = "DomRectInit")]
    #[wasm_bindgen(method, setter = "boundingClientRect")]
    fn set_bounding_client_rect_shim(this: &IntersectionObserverEntryInit, val: &DomRectInit);
    #[cfg(feature = "DomRectInit")]
    #[wasm_bindgen(method, getter = "intersectionRect")]
    fn intersection_rect_shim(this: &IntersectionObserverEntryInit) -> DomRectInit;
    #[cfg(feature = "DomRectInit")]
    #[wasm_bindgen(method, setter = "intersectionRect")]
    fn set_intersection_rect_shim(this: &IntersectionObserverEntryInit, val: &DomRectInit);
    #[cfg(feature = "DomRectInit")]
    #[wasm_bindgen(method, getter = "rootBounds")]
    fn root_bounds_shim(this: &IntersectionObserverEntryInit) -> DomRectInit;
    #[cfg(feature = "DomRectInit")]
    #[wasm_bindgen(method, setter = "rootBounds")]
    fn set_root_bounds_shim(this: &IntersectionObserverEntryInit, val: &DomRectInit);
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, getter = "target")]
    fn target_shim(this: &IntersectionObserverEntryInit) -> Element;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, setter = "target")]
    fn set_target_shim(this: &IntersectionObserverEntryInit, val: &Element);
    #[wasm_bindgen(method, getter = "time")]
    fn time_shim(this: &IntersectionObserverEntryInit) -> f64;
    #[wasm_bindgen(method, setter = "time")]
    fn set_time_shim(this: &IntersectionObserverEntryInit, val: f64);
}
#[doc = "The trait to access properties on the `IntersectionObserverEntryInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `IntersectionObserverEntryInit`*"]
pub trait IntersectionObserverEntryInitGetters {
    #[cfg(feature = "DomRectInit")]
    #[doc = "Get the `boundingClientRect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`, `IntersectionObserverEntryInit`*"]
    fn bounding_client_rect(&self) -> DomRectInit;
    #[cfg(feature = "DomRectInit")]
    #[doc = "Get the `intersectionRect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`, `IntersectionObserverEntryInit`*"]
    fn intersection_rect(&self) -> DomRectInit;
    #[cfg(feature = "DomRectInit")]
    #[doc = "Get the `rootBounds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`, `IntersectionObserverEntryInit`*"]
    fn root_bounds(&self) -> DomRectInit;
    #[cfg(feature = "Element")]
    #[doc = "Get the `target` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `IntersectionObserverEntryInit`*"]
    fn target(&self) -> Element;
    #[doc = "Get the `time` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverEntryInit`*"]
    fn time(&self) -> f64;
}
impl IntersectionObserverEntryInitGetters for IntersectionObserverEntryInit {
    #[cfg(feature = "DomRectInit")]
    fn bounding_client_rect(&self) -> DomRectInit {
        self.bounding_client_rect_shim()
    }
    #[cfg(feature = "DomRectInit")]
    fn intersection_rect(&self) -> DomRectInit {
        self.intersection_rect_shim()
    }
    #[cfg(feature = "DomRectInit")]
    fn root_bounds(&self) -> DomRectInit {
        self.root_bounds_shim()
    }
    #[cfg(feature = "Element")]
    fn target(&self) -> Element {
        self.target_shim()
    }
    fn time(&self) -> f64 {
        self.time_shim()
    }
}
impl IntersectionObserverEntryInit {
    #[cfg(all(feature = "DomRectInit", feature = "Element",))]
    #[doc = "Construct a new `IntersectionObserverEntryInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`, `Element`, `IntersectionObserverEntryInit`*"]
    pub fn new(
        bounding_client_rect: &DomRectInit,
        intersection_rect: &DomRectInit,
        root_bounds: &DomRectInit,
        target: &Element,
        time: f64,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.bounding_client_rect(bounding_client_rect);
        ret.intersection_rect(intersection_rect);
        ret.root_bounds(root_bounds);
        ret.target(target);
        ret.time(time);
        ret
    }
    #[cfg(feature = "DomRectInit")]
    #[doc = "Change the `boundingClientRect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`, `IntersectionObserverEntryInit`*"]
    pub fn bounding_client_rect(&mut self, val: &DomRectInit) -> &mut Self {
        self.set_bounding_client_rect_shim(val);
        self
    }
    #[cfg(feature = "DomRectInit")]
    #[doc = "Change the `intersectionRect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`, `IntersectionObserverEntryInit`*"]
    pub fn intersection_rect(&mut self, val: &DomRectInit) -> &mut Self {
        self.set_intersection_rect_shim(val);
        self
    }
    #[cfg(feature = "DomRectInit")]
    #[doc = "Change the `rootBounds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`, `IntersectionObserverEntryInit`*"]
    pub fn root_bounds(&mut self, val: &DomRectInit) -> &mut Self {
        self.set_root_bounds_shim(val);
        self
    }
    #[cfg(feature = "Element")]
    #[doc = "Change the `target` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `IntersectionObserverEntryInit`*"]
    pub fn target(&mut self, val: &Element) -> &mut Self {
        self.set_target_shim(val);
        self
    }
    #[doc = "Change the `time` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverEntryInit`*"]
    pub fn time(&mut self, val: f64) -> &mut Self {
        self.set_time_shim(val);
        self
    }
}
