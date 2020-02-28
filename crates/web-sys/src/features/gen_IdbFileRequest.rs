use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = DomRequest , extends = EventTarget , extends = :: js_sys :: Object , js_name = IDBFileRequest , typescript_name = IDBFileRequest ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbFileRequest` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileRequest)\n\n*This API requires the following crate features to be activated: `IdbFileRequest`*"]
    pub type IdbFileRequest;
    # [ wasm_bindgen ( structural , method , getter , js_name = fileHandle ) ]
    #[cfg(feature = "IdbFileHandle")]
    #[doc = "Getter for the `fileHandle` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileRequest/fileHandle)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    pub fn file_handle(this: &IdbFileRequest) -> Option<IdbFileHandle>;
    # [ wasm_bindgen ( structural , method , getter , js_name = lockedFile ) ]
    #[cfg(feature = "IdbFileHandle")]
    #[doc = "Getter for the `lockedFile` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileRequest/lockedFile)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    pub fn locked_file(this: &IdbFileRequest) -> Option<IdbFileHandle>;
    # [ wasm_bindgen ( structural , method , getter , js_name = onprogress ) ]
    #[doc = "Getter for the `onprogress` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileRequest/onprogress)\n\n*This API requires the following crate features to be activated: `IdbFileRequest`*"]
    pub fn onprogress(this: &IdbFileRequest) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onprogress ) ]
    #[doc = "Setter for the `onprogress` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileRequest/onprogress)\n\n*This API requires the following crate features to be activated: `IdbFileRequest`*"]
    pub fn set_onprogress(this: &IdbFileRequest, value: Option<&::js_sys::Function>);
}
