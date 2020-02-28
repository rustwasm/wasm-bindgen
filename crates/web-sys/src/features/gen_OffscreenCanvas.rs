use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = OffscreenCanvas , typescript_name = OffscreenCanvas ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OffscreenCanvas` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas)\n\n*This API requires the following crate features to be activated: `OffscreenCanvas`*"]
    pub type OffscreenCanvas;
    # [ wasm_bindgen ( structural , method , getter , js_name = width ) ]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/width)\n\n*This API requires the following crate features to be activated: `OffscreenCanvas`*"]
    pub fn width(this: &OffscreenCanvas) -> u32;
    # [ wasm_bindgen ( structural , method , setter , js_name = width ) ]
    #[doc = "Setter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/width)\n\n*This API requires the following crate features to be activated: `OffscreenCanvas`*"]
    pub fn set_width(this: &OffscreenCanvas, value: u32);
    # [ wasm_bindgen ( structural , method , getter , js_name = height ) ]
    #[doc = "Getter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/height)\n\n*This API requires the following crate features to be activated: `OffscreenCanvas`*"]
    pub fn height(this: &OffscreenCanvas) -> u32;
    # [ wasm_bindgen ( structural , method , setter , js_name = height ) ]
    #[doc = "Setter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/height)\n\n*This API requires the following crate features to be activated: `OffscreenCanvas`*"]
    pub fn set_height(this: &OffscreenCanvas, value: u32);
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new OffscreenCanvas(..)` constructor, creating a new instance of `OffscreenCanvas`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/OffscreenCanvas)\n\n*This API requires the following crate features to be activated: `OffscreenCanvas`*"]
    pub fn new(this: &OffscreenCanvas, width: u32, height: u32)
        -> Result<OffscreenCanvas, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = getContext ) ]
    #[doc = "The `getContext()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/getContext)\n\n*This API requires the following crate features to be activated: `OffscreenCanvas`*"]
    pub fn get_context(
        this: &OffscreenCanvas,
        context_id: &str,
    ) -> Result<Option<::js_sys::Object>, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = getContext ) ]
    #[doc = "The `getContext()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/getContext)\n\n*This API requires the following crate features to be activated: `OffscreenCanvas`*"]
    pub fn get_context_with_context_options(
        this: &OffscreenCanvas,
        context_id: &str,
        context_options: &::wasm_bindgen::JsValue,
    ) -> Result<Option<::js_sys::Object>, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = toBlob ) ]
    #[doc = "The `toBlob()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/toBlob)\n\n*This API requires the following crate features to be activated: `OffscreenCanvas`*"]
    pub fn to_blob(this: &OffscreenCanvas) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = toBlob ) ]
    #[doc = "The `toBlob()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/toBlob)\n\n*This API requires the following crate features to be activated: `OffscreenCanvas`*"]
    pub fn to_blob_with_type(
        this: &OffscreenCanvas,
        type_: &str,
    ) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = toBlob ) ]
    #[doc = "The `toBlob()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/toBlob)\n\n*This API requires the following crate features to be activated: `OffscreenCanvas`*"]
    pub fn to_blob_with_type_and_encoder_options(
        this: &OffscreenCanvas,
        type_: &str,
        encoder_options: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "ImageBitmap")]
    # [ wasm_bindgen ( catch , method , structural , js_name = transferToImageBitmap ) ]
    #[doc = "The `transferToImageBitmap()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/transferToImageBitmap)\n\n*This API requires the following crate features to be activated: `ImageBitmap`, `OffscreenCanvas`*"]
    pub fn transfer_to_image_bitmap(this: &OffscreenCanvas) -> Result<ImageBitmap, JsValue>;
}
