use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = FontFace , typescript_type = "FontFace" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `FontFace` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace)
    ///
    ///*This API requires the following crate features to be activated: `FontFace`*
    pub type FontFace;

    # [ wasm_bindgen ( structural , method , getter , js_class = "FontFace" , js_name = family ) ]
    ///Getter for the `family` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/family)
    ///
    ///*This API requires the following crate features to be activated: `FontFace`*
    pub fn family(this: &FontFace) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "FontFace" , js_name = family ) ]
    ///Setter for the `family` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/family)
    ///
    ///*This API requires the following crate features to be activated: `FontFace`*
    pub fn set_family(this: &FontFace, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "FontFace" , js_name = style ) ]
    ///Getter for the `style` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/style)
    ///
    ///*This API requires the following crate features to be activated: `FontFace`*
    pub fn style(this: &FontFace) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "FontFace" , js_name = style ) ]
    ///Setter for the `style` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/style)
    ///
    ///*This API requires the following crate features to be activated: `FontFace`*
    pub fn set_style(this: &FontFace, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "FontFace" , js_name = weight ) ]
    ///Getter for the `weight` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/weight)
    ///
    ///*This API requires the following crate features to be activated: `FontFace`*
    pub fn weight(this: &FontFace) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "FontFace" , js_name = weight ) ]
    ///Setter for the `weight` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/weight)
    ///
    ///*This API requires the following crate features to be activated: `FontFace`*
    pub fn set_weight(this: &FontFace, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "FontFace" , js_name = stretch ) ]
    ///Getter for the `stretch` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/stretch)
    ///
    ///*This API requires the following crate features to be activated: `FontFace`*
    pub fn stretch(this: &FontFace) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "FontFace" , js_name = stretch ) ]
    ///Setter for the `stretch` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/stretch)
    ///
    ///*This API requires the following crate features to be activated: `FontFace`*
    pub fn set_stretch(this: &FontFace, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "FontFace" , js_name = unicodeRange ) ]
    ///Getter for the `unicodeRange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/unicodeRange)
    ///
    ///*This API requires the following crate features to be activated: `FontFace`*
    pub fn unicode_range(this: &FontFace) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "FontFace" , js_name = unicodeRange ) ]
    ///Setter for the `unicodeRange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/unicodeRange)
    ///
    ///*This API requires the following crate features to be activated: `FontFace`*
    pub fn set_unicode_range(this: &FontFace, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "FontFace" , js_name = variant ) ]
    ///Getter for the `variant` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/variant)
    ///
    ///*This API requires the following crate features to be activated: `FontFace`*
    pub fn variant(this: &FontFace) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "FontFace" , js_name = variant ) ]
    ///Setter for the `variant` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/variant)
    ///
    ///*This API requires the following crate features to be activated: `FontFace`*
    pub fn set_variant(this: &FontFace, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "FontFace" , js_name = featureSettings ) ]
    ///Getter for the `featureSettings` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/featureSettings)
    ///
    ///*This API requires the following crate features to be activated: `FontFace`*
    pub fn feature_settings(this: &FontFace) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "FontFace" , js_name = featureSettings ) ]
    ///Setter for the `featureSettings` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/featureSettings)
    ///
    ///*This API requires the following crate features to be activated: `FontFace`*
    pub fn set_feature_settings(this: &FontFace, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "FontFace" , js_name = variationSettings ) ]
    ///Getter for the `variationSettings` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/variationSettings)
    ///
    ///*This API requires the following crate features to be activated: `FontFace`*
    pub fn variation_settings(this: &FontFace) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "FontFace" , js_name = variationSettings ) ]
    ///Setter for the `variationSettings` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/variationSettings)
    ///
    ///*This API requires the following crate features to be activated: `FontFace`*
    pub fn set_variation_settings(this: &FontFace, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "FontFace" , js_name = display ) ]
    ///Getter for the `display` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/display)
    ///
    ///*This API requires the following crate features to be activated: `FontFace`*
    pub fn display(this: &FontFace) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "FontFace" , js_name = display ) ]
    ///Setter for the `display` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/display)
    ///
    ///*This API requires the following crate features to be activated: `FontFace`*
    pub fn set_display(this: &FontFace, value: &str);

    #[cfg(feature = "FontFaceLoadStatus")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "FontFace" , js_name = status ) ]
    ///Getter for the `status` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/status)
    ///
    ///*This API requires the following crate features to be activated: `FontFace`, `FontFaceLoadStatus`*
    pub fn status(this: &FontFace) -> FontFaceLoadStatus;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "FontFace" , js_name = loaded ) ]
    ///Getter for the `loaded` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/loaded)
    ///
    ///*This API requires the following crate features to be activated: `FontFace`*
    pub fn loaded(this: &FontFace) -> Result<::js_sys::Promise, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "FontFace")]
    ///The `new FontFace(..)` constructor, creating a new instance of `FontFace`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)
    ///
    ///*This API requires the following crate features to be activated: `FontFace`*
    pub fn new_with_str(family: &str, source: &str) -> Result<FontFace, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "FontFace")]
    ///The `new FontFace(..)` constructor, creating a new instance of `FontFace`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)
    ///
    ///*This API requires the following crate features to be activated: `FontFace`*
    pub fn new_with_array_buffer(
        family: &str,
        source: &::js_sys::ArrayBuffer,
    ) -> Result<FontFace, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "FontFace")]
    ///The `new FontFace(..)` constructor, creating a new instance of `FontFace`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)
    ///
    ///*This API requires the following crate features to be activated: `FontFace`*
    pub fn new_with_array_buffer_view(
        family: &str,
        source: &::js_sys::Object,
    ) -> Result<FontFace, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "FontFace")]
    ///The `new FontFace(..)` constructor, creating a new instance of `FontFace`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)
    ///
    ///*This API requires the following crate features to be activated: `FontFace`*
    pub fn new_with_u8_array(family: &str, source: &mut [u8]) -> Result<FontFace, JsValue>;

    #[cfg(feature = "FontFaceDescriptors")]
    #[wasm_bindgen(catch, constructor, js_class = "FontFace")]
    ///The `new FontFace(..)` constructor, creating a new instance of `FontFace`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)
    ///
    ///*This API requires the following crate features to be activated: `FontFace`, `FontFaceDescriptors`*
    pub fn new_with_str_and_descriptors(
        family: &str,
        source: &str,
        descriptors: &FontFaceDescriptors,
    ) -> Result<FontFace, JsValue>;

    #[cfg(feature = "FontFaceDescriptors")]
    #[wasm_bindgen(catch, constructor, js_class = "FontFace")]
    ///The `new FontFace(..)` constructor, creating a new instance of `FontFace`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)
    ///
    ///*This API requires the following crate features to be activated: `FontFace`, `FontFaceDescriptors`*
    pub fn new_with_array_buffer_and_descriptors(
        family: &str,
        source: &::js_sys::ArrayBuffer,
        descriptors: &FontFaceDescriptors,
    ) -> Result<FontFace, JsValue>;

    #[cfg(feature = "FontFaceDescriptors")]
    #[wasm_bindgen(catch, constructor, js_class = "FontFace")]
    ///The `new FontFace(..)` constructor, creating a new instance of `FontFace`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)
    ///
    ///*This API requires the following crate features to be activated: `FontFace`, `FontFaceDescriptors`*
    pub fn new_with_array_buffer_view_and_descriptors(
        family: &str,
        source: &::js_sys::Object,
        descriptors: &FontFaceDescriptors,
    ) -> Result<FontFace, JsValue>;

    #[cfg(feature = "FontFaceDescriptors")]
    #[wasm_bindgen(catch, constructor, js_class = "FontFace")]
    ///The `new FontFace(..)` constructor, creating a new instance of `FontFace`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)
    ///
    ///*This API requires the following crate features to be activated: `FontFace`, `FontFaceDescriptors`*
    pub fn new_with_u8_array_and_descriptors(
        family: &str,
        source: &mut [u8],
        descriptors: &FontFaceDescriptors,
    ) -> Result<FontFace, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "FontFace" , js_name = load ) ]
    ///The `load()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/load)
    ///
    ///*This API requires the following crate features to be activated: `FontFace`*
    pub fn load(this: &FontFace) -> Result<::js_sys::Promise, JsValue>;

}
