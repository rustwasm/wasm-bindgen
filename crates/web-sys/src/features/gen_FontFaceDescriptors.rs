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
    #[wasm_bindgen(method, getter = "display")]
    fn display_shim(this: &FontFaceDescriptors) -> String;
    #[wasm_bindgen(method, setter = "display")]
    fn set_display_shim(this: &FontFaceDescriptors, val: &str);
    #[wasm_bindgen(method, getter = "featureSettings")]
    fn feature_settings_shim(this: &FontFaceDescriptors) -> String;
    #[wasm_bindgen(method, setter = "featureSettings")]
    fn set_feature_settings_shim(this: &FontFaceDescriptors, val: &str);
    #[wasm_bindgen(method, getter = "stretch")]
    fn stretch_shim(this: &FontFaceDescriptors) -> String;
    #[wasm_bindgen(method, setter = "stretch")]
    fn set_stretch_shim(this: &FontFaceDescriptors, val: &str);
    #[wasm_bindgen(method, getter = "style")]
    fn style_shim(this: &FontFaceDescriptors) -> String;
    #[wasm_bindgen(method, setter = "style")]
    fn set_style_shim(this: &FontFaceDescriptors, val: &str);
    #[wasm_bindgen(method, getter = "unicodeRange")]
    fn unicode_range_shim(this: &FontFaceDescriptors) -> String;
    #[wasm_bindgen(method, setter = "unicodeRange")]
    fn set_unicode_range_shim(this: &FontFaceDescriptors, val: &str);
    #[wasm_bindgen(method, getter = "variant")]
    fn variant_shim(this: &FontFaceDescriptors) -> String;
    #[wasm_bindgen(method, setter = "variant")]
    fn set_variant_shim(this: &FontFaceDescriptors, val: &str);
    #[wasm_bindgen(method, getter = "variationSettings")]
    fn variation_settings_shim(this: &FontFaceDescriptors) -> String;
    #[wasm_bindgen(method, setter = "variationSettings")]
    fn set_variation_settings_shim(this: &FontFaceDescriptors, val: &str);
    #[wasm_bindgen(method, getter = "weight")]
    fn weight_shim(this: &FontFaceDescriptors) -> String;
    #[wasm_bindgen(method, setter = "weight")]
    fn set_weight_shim(this: &FontFaceDescriptors, val: &str);
}
#[doc = "The trait to access properties on the `FontFaceDescriptors` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
pub trait FontFaceDescriptorsGetters {
    #[doc = "Get the `display` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    fn display(&self) -> String;
    #[doc = "Get the `featureSettings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    fn feature_settings(&self) -> String;
    #[doc = "Get the `stretch` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    fn stretch(&self) -> String;
    #[doc = "Get the `style` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    fn style(&self) -> String;
    #[doc = "Get the `unicodeRange` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    fn unicode_range(&self) -> String;
    #[doc = "Get the `variant` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    fn variant(&self) -> String;
    #[doc = "Get the `variationSettings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    fn variation_settings(&self) -> String;
    #[doc = "Get the `weight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    fn weight(&self) -> String;
}
impl FontFaceDescriptorsGetters for FontFaceDescriptors {
    fn display(&self) -> String {
        self.display_shim()
    }
    fn feature_settings(&self) -> String {
        self.feature_settings_shim()
    }
    fn stretch(&self) -> String {
        self.stretch_shim()
    }
    fn style(&self) -> String {
        self.style_shim()
    }
    fn unicode_range(&self) -> String {
        self.unicode_range_shim()
    }
    fn variant(&self) -> String {
        self.variant_shim()
    }
    fn variation_settings(&self) -> String {
        self.variation_settings_shim()
    }
    fn weight(&self) -> String {
        self.weight_shim()
    }
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
        self.set_display_shim(val);
        self
    }
    #[doc = "Change the `featureSettings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    pub fn feature_settings(&mut self, val: &str) -> &mut Self {
        self.set_feature_settings_shim(val);
        self
    }
    #[doc = "Change the `stretch` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    pub fn stretch(&mut self, val: &str) -> &mut Self {
        self.set_stretch_shim(val);
        self
    }
    #[doc = "Change the `style` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    pub fn style(&mut self, val: &str) -> &mut Self {
        self.set_style_shim(val);
        self
    }
    #[doc = "Change the `unicodeRange` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    pub fn unicode_range(&mut self, val: &str) -> &mut Self {
        self.set_unicode_range_shim(val);
        self
    }
    #[doc = "Change the `variant` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    pub fn variant(&mut self, val: &str) -> &mut Self {
        self.set_variant_shim(val);
        self
    }
    #[doc = "Change the `variationSettings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    pub fn variation_settings(&mut self, val: &str) -> &mut Self {
        self.set_variation_settings_shim(val);
        self
    }
    #[doc = "Change the `weight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    pub fn weight(&mut self, val: &str) -> &mut Self {
        self.set_weight_shim(val);
        self
    }
}
impl Default for FontFaceDescriptors {
    fn default() -> Self {
        Self::new()
    }
}
