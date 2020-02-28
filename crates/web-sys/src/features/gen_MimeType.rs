use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = MimeType , typescript_name = MimeType ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MimeType` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeType)\n\n*This API requires the following crate features to be activated: `MimeType`*"]
    pub type MimeType;
    # [ wasm_bindgen ( structural , method , getter , js_class = "MimeType" , js_name = description ) ]
    #[doc = "Getter for the `description` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeType/description)\n\n*This API requires the following crate features to be activated: `MimeType`*"]
    pub fn description(this: &MimeType) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "MimeType" , js_name = enabledPlugin ) ]
    #[cfg(feature = "Plugin")]
    #[doc = "Getter for the `enabledPlugin` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeType/enabledPlugin)\n\n*This API requires the following crate features to be activated: `MimeType`, `Plugin`*"]
    pub fn enabled_plugin(this: &MimeType) -> Option<Plugin>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "MimeType" , js_name = suffixes ) ]
    #[doc = "Getter for the `suffixes` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeType/suffixes)\n\n*This API requires the following crate features to be activated: `MimeType`*"]
    pub fn suffixes(this: &MimeType) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "MimeType" , js_name = type ) ]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeType/type)\n\n*This API requires the following crate features to be activated: `MimeType`*"]
    pub fn type_(this: &MimeType) -> String;
}
