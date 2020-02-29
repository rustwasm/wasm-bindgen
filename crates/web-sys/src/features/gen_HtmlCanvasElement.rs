use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLCanvasElement , typescript_type = "HTMLCanvasElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlCanvasElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCanvasElement`*
    pub type HtmlCanvasElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLCanvasElement" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/width)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCanvasElement`*
    pub fn width(this: &HtmlCanvasElement) -> u32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLCanvasElement" , js_name = width ) ]
    ///Setter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/width)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCanvasElement`*
    pub fn set_width(this: &HtmlCanvasElement, value: u32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLCanvasElement" , js_name = height ) ]
    ///Getter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/height)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCanvasElement`*
    pub fn height(this: &HtmlCanvasElement) -> u32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLCanvasElement" , js_name = height ) ]
    ///Setter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/height)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCanvasElement`*
    pub fn set_height(this: &HtmlCanvasElement, value: u32);

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLCanvasElement" , js_name = getContext ) ]
    ///The `getContext()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/getContext)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCanvasElement`*
    pub fn get_context(
        this: &HtmlCanvasElement,
        context_id: &str,
    ) -> Result<Option<::js_sys::Object>, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLCanvasElement" , js_name = getContext ) ]
    ///The `getContext()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/getContext)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCanvasElement`*
    pub fn get_context_with_context_options(
        this: &HtmlCanvasElement,
        context_id: &str,
        context_options: &::wasm_bindgen::JsValue,
    ) -> Result<Option<::js_sys::Object>, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLCanvasElement" , js_name = toBlob ) ]
    ///The `toBlob()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/toBlob)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCanvasElement`*
    pub fn to_blob(this: &HtmlCanvasElement, callback: &::js_sys::Function) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLCanvasElement" , js_name = toBlob ) ]
    ///The `toBlob()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/toBlob)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCanvasElement`*
    pub fn to_blob_with_type(
        this: &HtmlCanvasElement,
        callback: &::js_sys::Function,
        type_: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLCanvasElement" , js_name = toBlob ) ]
    ///The `toBlob()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/toBlob)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCanvasElement`*
    pub fn to_blob_with_type_and_encoder_options(
        this: &HtmlCanvasElement,
        callback: &::js_sys::Function,
        type_: &str,
        encoder_options: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLCanvasElement" , js_name = toDataURL ) ]
    ///The `toDataURL()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/toDataURL)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCanvasElement`*
    pub fn to_data_url(this: &HtmlCanvasElement) -> Result<String, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLCanvasElement" , js_name = toDataURL ) ]
    ///The `toDataURL()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/toDataURL)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCanvasElement`*
    pub fn to_data_url_with_type(this: &HtmlCanvasElement, type_: &str) -> Result<String, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLCanvasElement" , js_name = toDataURL ) ]
    ///The `toDataURL()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/toDataURL)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCanvasElement`*
    pub fn to_data_url_with_type_and_encoder_options(
        this: &HtmlCanvasElement,
        type_: &str,
        encoder_options: &::wasm_bindgen::JsValue,
    ) -> Result<String, JsValue>;

    #[cfg(feature = "OffscreenCanvas")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLCanvasElement" , js_name = transferControlToOffscreen ) ]
    ///The `transferControlToOffscreen()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/transferControlToOffscreen)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCanvasElement`, `OffscreenCanvas`*
    pub fn transfer_control_to_offscreen(
        this: &HtmlCanvasElement,
    ) -> Result<OffscreenCanvas, JsValue>;

}
