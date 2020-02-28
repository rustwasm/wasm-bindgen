use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = FontFace , typescript_name = FontFace ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FontFace` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    pub type FontFace;
    # [ wasm_bindgen ( structural , method , getter , js_name = family ) ]
    #[doc = "Getter for the `family` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/family)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn family(this: &FontFace) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = family ) ]
    #[doc = "Setter for the `family` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/family)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn set_family(this: &FontFace, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = style ) ]
    #[doc = "Getter for the `style` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/style)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn style(this: &FontFace) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = style ) ]
    #[doc = "Setter for the `style` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/style)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn set_style(this: &FontFace, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = weight ) ]
    #[doc = "Getter for the `weight` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/weight)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn weight(this: &FontFace) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = weight ) ]
    #[doc = "Setter for the `weight` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/weight)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn set_weight(this: &FontFace, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = stretch ) ]
    #[doc = "Getter for the `stretch` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/stretch)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn stretch(this: &FontFace) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = stretch ) ]
    #[doc = "Setter for the `stretch` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/stretch)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn set_stretch(this: &FontFace, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = unicodeRange ) ]
    #[doc = "Getter for the `unicodeRange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/unicodeRange)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn unicode_range(this: &FontFace) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = unicodeRange ) ]
    #[doc = "Setter for the `unicodeRange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/unicodeRange)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn set_unicode_range(this: &FontFace, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = variant ) ]
    #[doc = "Getter for the `variant` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/variant)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn variant(this: &FontFace) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = variant ) ]
    #[doc = "Setter for the `variant` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/variant)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn set_variant(this: &FontFace, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = featureSettings ) ]
    #[doc = "Getter for the `featureSettings` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/featureSettings)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn feature_settings(this: &FontFace) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = featureSettings ) ]
    #[doc = "Setter for the `featureSettings` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/featureSettings)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn set_feature_settings(this: &FontFace, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = variationSettings ) ]
    #[doc = "Getter for the `variationSettings` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/variationSettings)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn variation_settings(this: &FontFace) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = variationSettings ) ]
    #[doc = "Setter for the `variationSettings` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/variationSettings)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn set_variation_settings(this: &FontFace, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = display ) ]
    #[doc = "Getter for the `display` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/display)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn display(this: &FontFace) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = display ) ]
    #[doc = "Setter for the `display` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/display)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn set_display(this: &FontFace, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = status ) ]
    #[cfg(feature = "FontFaceLoadStatus")]
    #[doc = "Getter for the `status` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/status)\n\n*This API requires the following crate features to be activated: `FontFace`, `FontFaceLoadStatus`*"]
    pub fn status(this: &FontFace) -> FontFaceLoadStatus;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = loaded ) ]
    #[doc = "Getter for the `loaded` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/loaded)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn loaded(this: &FontFace) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new FontFace(..)` constructor, creating a new instance of `FontFace`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn new_with_str(this: &FontFace, family: &str, source: &str) -> Result<FontFace, JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new FontFace(..)` constructor, creating a new instance of `FontFace`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn new_with_array_buffer(
        this: &FontFace,
        family: &str,
        source: &::js_sys::ArrayBuffer,
    ) -> Result<FontFace, JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new FontFace(..)` constructor, creating a new instance of `FontFace`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn new_with_array_buffer_view(
        this: &FontFace,
        family: &str,
        source: &::js_sys::Object,
    ) -> Result<FontFace, JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new FontFace(..)` constructor, creating a new instance of `FontFace`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn new_with_u8_array(
        this: &FontFace,
        family: &str,
        source: &mut [u8],
    ) -> Result<FontFace, JsValue>;
    #[cfg(feature = "FontFaceDescriptors")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new FontFace(..)` constructor, creating a new instance of `FontFace`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)\n\n*This API requires the following crate features to be activated: `FontFace`, `FontFaceDescriptors`*"]
    pub fn new_with_str_and_descriptors(
        this: &FontFace,
        family: &str,
        source: &str,
        descriptors: &FontFaceDescriptors,
    ) -> Result<FontFace, JsValue>;
    #[cfg(feature = "FontFaceDescriptors")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new FontFace(..)` constructor, creating a new instance of `FontFace`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)\n\n*This API requires the following crate features to be activated: `FontFace`, `FontFaceDescriptors`*"]
    pub fn new_with_array_buffer_and_descriptors(
        this: &FontFace,
        family: &str,
        source: &::js_sys::ArrayBuffer,
        descriptors: &FontFaceDescriptors,
    ) -> Result<FontFace, JsValue>;
    #[cfg(feature = "FontFaceDescriptors")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new FontFace(..)` constructor, creating a new instance of `FontFace`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)\n\n*This API requires the following crate features to be activated: `FontFace`, `FontFaceDescriptors`*"]
    pub fn new_with_array_buffer_view_and_descriptors(
        this: &FontFace,
        family: &str,
        source: &::js_sys::Object,
        descriptors: &FontFaceDescriptors,
    ) -> Result<FontFace, JsValue>;
    #[cfg(feature = "FontFaceDescriptors")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new FontFace(..)` constructor, creating a new instance of `FontFace`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)\n\n*This API requires the following crate features to be activated: `FontFace`, `FontFaceDescriptors`*"]
    pub fn new_with_u8_array_and_descriptors(
        this: &FontFace,
        family: &str,
        source: &mut [u8],
        descriptors: &FontFaceDescriptors,
    ) -> Result<FontFace, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = load ) ]
    #[doc = "The `load()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/load)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn load(this: &FontFace) -> Result<::js_sys::Promise, JsValue>;
}
