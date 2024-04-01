#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = FontFaceDescriptors)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FontFaceDescriptors` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    pub type FontFaceDescriptors;
    #[wasm_bindgen(method, setter = "display")]
    fn display_shim(this: &FontFaceDescriptors, val: &str);
    #[wasm_bindgen(method, setter = "featureSettings")]
    fn feature_settings_shim(this: &FontFaceDescriptors, val: &str);
    #[wasm_bindgen(method, setter = "stretch")]
    fn stretch_shim(this: &FontFaceDescriptors, val: &str);
    #[wasm_bindgen(method, setter = "style")]
    fn style_shim(this: &FontFaceDescriptors, val: &str);
    #[wasm_bindgen(method, setter = "unicodeRange")]
    fn unicode_range_shim(this: &FontFaceDescriptors, val: &str);
    #[wasm_bindgen(method, setter = "variant")]
    fn variant_shim(this: &FontFaceDescriptors, val: &str);
    #[wasm_bindgen(method, setter = "variationSettings")]
    fn variation_settings_shim(this: &FontFaceDescriptors, val: &str);
    #[wasm_bindgen(method, setter = "weight")]
    fn weight_shim(this: &FontFaceDescriptors, val: &str);
}
impl FontFaceDescriptors {
    #[doc = "Construct a new `FontFaceDescriptors`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `display` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    pub fn display(&mut self, val: &str) -> &mut Self {
        self.display_shim(val);
        self
    }
    #[doc = "Change the `featureSettings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    pub fn feature_settings(&mut self, val: &str) -> &mut Self {
        self.feature_settings_shim(val);
        self
    }
    #[doc = "Change the `stretch` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    pub fn stretch(&mut self, val: &str) -> &mut Self {
        self.stretch_shim(val);
        self
    }
    #[doc = "Change the `style` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    pub fn style(&mut self, val: &str) -> &mut Self {
        self.style_shim(val);
        self
    }
    #[doc = "Change the `unicodeRange` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    pub fn unicode_range(&mut self, val: &str) -> &mut Self {
        self.unicode_range_shim(val);
        self
    }
    #[doc = "Change the `variant` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    pub fn variant(&mut self, val: &str) -> &mut Self {
        self.variant_shim(val);
        self
    }
    #[doc = "Change the `variationSettings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    pub fn variation_settings(&mut self, val: &str) -> &mut Self {
        self.variation_settings_shim(val);
        self
    }
    #[doc = "Change the `weight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    pub fn weight(&mut self, val: &str) -> &mut Self {
        self.weight_shim(val);
        self
    }
}
impl Default for FontFaceDescriptors {
    fn default() -> Self {
        Self::new()
    }
}
