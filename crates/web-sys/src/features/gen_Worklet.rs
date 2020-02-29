use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = Worklet , typescript_name = Worklet ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Worklet` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worklet)
    ///
    ///*This API requires the following crate features to be activated: `Worklet`*
    pub type Worklet;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Worklet" , js_name = addModule ) ]
    ///The `addModule()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worklet/addModule)
    ///
    ///*This API requires the following crate features to be activated: `Worklet`*
    pub fn add_module(this: &Worklet, module_url: &str) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "WorkletOptions")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Worklet" , js_name = addModule ) ]
    ///The `addModule()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worklet/addModule)
    ///
    ///*This API requires the following crate features to be activated: `Worklet`, `WorkletOptions`*
    pub fn add_module_with_options(
        this: &Worklet,
        module_url: &str,
        options: &WorkletOptions,
    ) -> Result<::js_sys::Promise, JsValue>;

}
