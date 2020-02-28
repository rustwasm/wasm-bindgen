use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = WorkerNavigator , typescript_name = WorkerNavigator ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WorkerNavigator` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator)\n\n*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    pub type WorkerNavigator;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = connection ) ]
    #[cfg(feature = "NetworkInformation")]
    #[doc = "Getter for the `connection` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/connection)\n\n*This API requires the following crate features to be activated: `NetworkInformation`, `WorkerNavigator`*"]
    pub fn connection(this: &WorkerNavigator) -> Result<NetworkInformation, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = mediaCapabilities ) ]
    #[cfg(feature = "MediaCapabilities")]
    #[doc = "Getter for the `mediaCapabilities` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/mediaCapabilities)\n\n*This API requires the following crate features to be activated: `MediaCapabilities`, `WorkerNavigator`*"]
    pub fn media_capabilities(this: &WorkerNavigator) -> MediaCapabilities;
    # [ wasm_bindgen ( structural , method , getter , js_name = hardwareConcurrency ) ]
    #[doc = "Getter for the `hardwareConcurrency` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/hardwareConcurrency)\n\n*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    pub fn hardware_concurrency(this: &WorkerNavigator) -> f64;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = appCodeName ) ]
    #[doc = "Getter for the `appCodeName` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/appCodeName)\n\n*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    pub fn app_code_name(this: &WorkerNavigator) -> Result<String, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = appName ) ]
    #[doc = "Getter for the `appName` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/appName)\n\n*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    pub fn app_name(this: &WorkerNavigator) -> String;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = appVersion ) ]
    #[doc = "Getter for the `appVersion` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/appVersion)\n\n*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    pub fn app_version(this: &WorkerNavigator) -> Result<String, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = platform ) ]
    #[doc = "Getter for the `platform` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/platform)\n\n*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    pub fn platform(this: &WorkerNavigator) -> Result<String, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = userAgent ) ]
    #[doc = "Getter for the `userAgent` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/userAgent)\n\n*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    pub fn user_agent(this: &WorkerNavigator) -> Result<String, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = product ) ]
    #[doc = "Getter for the `product` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/product)\n\n*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    pub fn product(this: &WorkerNavigator) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = language ) ]
    #[doc = "Getter for the `language` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/language)\n\n*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    pub fn language(this: &WorkerNavigator) -> Option<String>;
    # [ wasm_bindgen ( structural , method , getter , js_name = languages ) ]
    #[doc = "Getter for the `languages` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/languages)\n\n*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    pub fn languages(this: &WorkerNavigator) -> ::js_sys::Array;
    # [ wasm_bindgen ( structural , method , getter , js_name = onLine ) ]
    #[doc = "Getter for the `onLine` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/onLine)\n\n*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    pub fn on_line(this: &WorkerNavigator) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = storage ) ]
    #[cfg(feature = "StorageManager")]
    #[doc = "Getter for the `storage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/storage)\n\n*This API requires the following crate features to be activated: `StorageManager`, `WorkerNavigator`*"]
    pub fn storage(this: &WorkerNavigator) -> StorageManager;
    # [ wasm_bindgen ( method , structural , js_name = taintEnabled ) ]
    #[doc = "The `taintEnabled()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/taintEnabled)\n\n*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    pub fn taint_enabled(this: &WorkerNavigator) -> bool;
}
