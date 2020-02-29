use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = MimeType , typescript_name = MimeType ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MimeType` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeType)
    ///
    ///*This API requires the following crate features to be activated: `MimeType`*
    pub type MimeType;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MimeType" , js_name = description ) ]
    ///Getter for the `description` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeType/description)
    ///
    ///*This API requires the following crate features to be activated: `MimeType`*
    pub fn description(this: &MimeType) -> String;

    #[cfg(feature = "Plugin")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "MimeType" , js_name = enabledPlugin ) ]
    ///Getter for the `enabledPlugin` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeType/enabledPlugin)
    ///
    ///*This API requires the following crate features to be activated: `MimeType`, `Plugin`*
    pub fn enabled_plugin(this: &MimeType) -> Option<Plugin>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MimeType" , js_name = suffixes ) ]
    ///Getter for the `suffixes` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeType/suffixes)
    ///
    ///*This API requires the following crate features to be activated: `MimeType`*
    pub fn suffixes(this: &MimeType) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MimeType" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeType/type)
    ///
    ///*This API requires the following crate features to be activated: `MimeType`*
    pub fn type_(this: &MimeType) -> String;

}
