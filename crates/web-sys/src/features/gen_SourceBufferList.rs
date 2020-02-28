use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = SourceBufferList , typescript_name = SourceBufferList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SourceBufferList` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList)\n\n*This API requires the following crate features to be activated: `SourceBufferList`*"]
    pub type SourceBufferList;
    # [ wasm_bindgen ( structural , method , getter , js_name = length ) ]
    #[doc = "Getter for the `length` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList/length)\n\n*This API requires the following crate features to be activated: `SourceBufferList`*"]
    pub fn length(this: &SourceBufferList) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_name = onaddsourcebuffer ) ]
    #[doc = "Getter for the `onaddsourcebuffer` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList/onaddsourcebuffer)\n\n*This API requires the following crate features to be activated: `SourceBufferList`*"]
    pub fn onaddsourcebuffer(this: &SourceBufferList) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onaddsourcebuffer ) ]
    #[doc = "Setter for the `onaddsourcebuffer` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList/onaddsourcebuffer)\n\n*This API requires the following crate features to be activated: `SourceBufferList`*"]
    pub fn set_onaddsourcebuffer(this: &SourceBufferList, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onremovesourcebuffer ) ]
    #[doc = "Getter for the `onremovesourcebuffer` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList/onremovesourcebuffer)\n\n*This API requires the following crate features to be activated: `SourceBufferList`*"]
    pub fn onremovesourcebuffer(this: &SourceBufferList) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onremovesourcebuffer ) ]
    #[doc = "Setter for the `onremovesourcebuffer` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList/onremovesourcebuffer)\n\n*This API requires the following crate features to be activated: `SourceBufferList`*"]
    pub fn set_onremovesourcebuffer(this: &SourceBufferList, value: Option<&::js_sys::Function>);
    #[cfg(feature = "SourceBuffer")]
    #[wasm_bindgen(method, structural, indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `SourceBuffer`, `SourceBufferList`*"]
    pub fn get(this: &SourceBufferList, index: u32) -> Option<SourceBuffer>;
}
