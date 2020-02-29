use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = WorkerNavigator , typescript_name = WorkerNavigator ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `WorkerNavigator` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator)
    ///
    ///*This API requires the following crate features to be activated: `WorkerNavigator`*
    pub type WorkerNavigator;

    #[cfg(feature = "NetworkInformation")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "WorkerNavigator" , js_name = connection ) ]
    ///Getter for the `connection` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/connection)
    ///
    ///*This API requires the following crate features to be activated: `NetworkInformation`, `WorkerNavigator`*
    pub fn connection(this: &WorkerNavigator) -> Result<NetworkInformation, JsValue>;

    #[cfg(feature = "MediaCapabilities")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "WorkerNavigator" , js_name = mediaCapabilities ) ]
    ///Getter for the `mediaCapabilities` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/mediaCapabilities)
    ///
    ///*This API requires the following crate features to be activated: `MediaCapabilities`, `WorkerNavigator`*
    pub fn media_capabilities(this: &WorkerNavigator) -> MediaCapabilities;

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Gpu")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "WorkerNavigator" , js_name = gpu ) ]
    ///Getter for the `gpu` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/gpu)
    ///
    ///*This API requires the following crate features to be activated: `Gpu`, `WorkerNavigator`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn gpu(this: &WorkerNavigator) -> Gpu;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WorkerNavigator" , js_name = hardwareConcurrency ) ]
    ///Getter for the `hardwareConcurrency` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/hardwareConcurrency)
    ///
    ///*This API requires the following crate features to be activated: `WorkerNavigator`*
    pub fn hardware_concurrency(this: &WorkerNavigator) -> f64;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "WorkerNavigator" , js_name = appCodeName ) ]
    ///Getter for the `appCodeName` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/appCodeName)
    ///
    ///*This API requires the following crate features to be activated: `WorkerNavigator`*
    pub fn app_code_name(this: &WorkerNavigator) -> Result<String, JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WorkerNavigator" , js_name = appName ) ]
    ///Getter for the `appName` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/appName)
    ///
    ///*This API requires the following crate features to be activated: `WorkerNavigator`*
    pub fn app_name(this: &WorkerNavigator) -> String;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "WorkerNavigator" , js_name = appVersion ) ]
    ///Getter for the `appVersion` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/appVersion)
    ///
    ///*This API requires the following crate features to be activated: `WorkerNavigator`*
    pub fn app_version(this: &WorkerNavigator) -> Result<String, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "WorkerNavigator" , js_name = platform ) ]
    ///Getter for the `platform` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/platform)
    ///
    ///*This API requires the following crate features to be activated: `WorkerNavigator`*
    pub fn platform(this: &WorkerNavigator) -> Result<String, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "WorkerNavigator" , js_name = userAgent ) ]
    ///Getter for the `userAgent` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/userAgent)
    ///
    ///*This API requires the following crate features to be activated: `WorkerNavigator`*
    pub fn user_agent(this: &WorkerNavigator) -> Result<String, JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WorkerNavigator" , js_name = product ) ]
    ///Getter for the `product` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/product)
    ///
    ///*This API requires the following crate features to be activated: `WorkerNavigator`*
    pub fn product(this: &WorkerNavigator) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WorkerNavigator" , js_name = language ) ]
    ///Getter for the `language` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/language)
    ///
    ///*This API requires the following crate features to be activated: `WorkerNavigator`*
    pub fn language(this: &WorkerNavigator) -> Option<String>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WorkerNavigator" , js_name = languages ) ]
    ///Getter for the `languages` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/languages)
    ///
    ///*This API requires the following crate features to be activated: `WorkerNavigator`*
    pub fn languages(this: &WorkerNavigator) -> ::js_sys::Array;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WorkerNavigator" , js_name = onLine ) ]
    ///Getter for the `onLine` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/onLine)
    ///
    ///*This API requires the following crate features to be activated: `WorkerNavigator`*
    pub fn on_line(this: &WorkerNavigator) -> bool;

    #[cfg(feature = "StorageManager")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "WorkerNavigator" , js_name = storage ) ]
    ///Getter for the `storage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/storage)
    ///
    ///*This API requires the following crate features to be activated: `StorageManager`, `WorkerNavigator`*
    pub fn storage(this: &WorkerNavigator) -> StorageManager;

    # [ wasm_bindgen ( method , structural , js_class = "WorkerNavigator" , js_name = taintEnabled ) ]
    ///The `taintEnabled()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/taintEnabled)
    ///
    ///*This API requires the following crate features to be activated: `WorkerNavigator`*
    pub fn taint_enabled(this: &WorkerNavigator) -> bool;

}
