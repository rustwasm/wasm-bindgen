use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = FuzzingFunctions , typescript_name = FuzzingFunctions ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `FuzzingFunctions` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FuzzingFunctions)
    ///
    ///*This API requires the following crate features to be activated: `FuzzingFunctions`*
    pub type FuzzingFunctions;

    # [ wasm_bindgen ( static_method_of = FuzzingFunctions , js_class = "FuzzingFunctions" , js_name = cycleCollect ) ]
    ///The `cycleCollect()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FuzzingFunctions/cycleCollect)
    ///
    ///*This API requires the following crate features to be activated: `FuzzingFunctions`*
    pub fn cycle_collect();

    # [ wasm_bindgen ( catch , static_method_of = FuzzingFunctions , js_class = "FuzzingFunctions" , js_name = enableAccessibility ) ]
    ///The `enableAccessibility()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FuzzingFunctions/enableAccessibility)
    ///
    ///*This API requires the following crate features to be activated: `FuzzingFunctions`*
    pub fn enable_accessibility() -> Result<(), JsValue>;

    # [ wasm_bindgen ( static_method_of = FuzzingFunctions , js_class = "FuzzingFunctions" , js_name = garbageCollect ) ]
    ///The `garbageCollect()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FuzzingFunctions/garbageCollect)
    ///
    ///*This API requires the following crate features to be activated: `FuzzingFunctions`*
    pub fn garbage_collect();

}
