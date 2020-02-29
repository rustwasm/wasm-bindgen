use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = SourceBufferList , typescript_name = SourceBufferList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SourceBufferList` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList)
    ///
    ///*This API requires the following crate features to be activated: `SourceBufferList`*
    pub type SourceBufferList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SourceBufferList" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList/length)
    ///
    ///*This API requires the following crate features to be activated: `SourceBufferList`*
    pub fn length(this: &SourceBufferList) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SourceBufferList" , js_name = onaddsourcebuffer ) ]
    ///Getter for the `onaddsourcebuffer` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList/onaddsourcebuffer)
    ///
    ///*This API requires the following crate features to be activated: `SourceBufferList`*
    pub fn onaddsourcebuffer(this: &SourceBufferList) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SourceBufferList" , js_name = onaddsourcebuffer ) ]
    ///Setter for the `onaddsourcebuffer` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList/onaddsourcebuffer)
    ///
    ///*This API requires the following crate features to be activated: `SourceBufferList`*
    pub fn set_onaddsourcebuffer(this: &SourceBufferList, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SourceBufferList" , js_name = onremovesourcebuffer ) ]
    ///Getter for the `onremovesourcebuffer` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList/onremovesourcebuffer)
    ///
    ///*This API requires the following crate features to be activated: `SourceBufferList`*
    pub fn onremovesourcebuffer(this: &SourceBufferList) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SourceBufferList" , js_name = onremovesourcebuffer ) ]
    ///Setter for the `onremovesourcebuffer` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList/onremovesourcebuffer)
    ///
    ///*This API requires the following crate features to be activated: `SourceBufferList`*
    pub fn set_onremovesourcebuffer(this: &SourceBufferList, value: Option<&::js_sys::Function>);

    #[cfg(feature = "SourceBuffer")]
    #[wasm_bindgen(method, structural, js_class = "SourceBufferList", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `SourceBuffer`, `SourceBufferList`*
    pub fn get(this: &SourceBufferList, index: u32) -> Option<SourceBuffer>;

}
