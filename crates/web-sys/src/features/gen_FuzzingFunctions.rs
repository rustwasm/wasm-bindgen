use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = FuzzingFunctions , typescript_name = FuzzingFunctions ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FuzzingFunctions` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FuzzingFunctions)\n\n*This API requires the following crate features to be activated: `FuzzingFunctions`*"]
    pub type FuzzingFunctions;
    # [ wasm_bindgen ( method , structural , static_method_of = FuzzingFunctions , js_name = cycleCollect ) ]
    #[doc = "The `cycleCollect()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FuzzingFunctions/cycleCollect)\n\n*This API requires the following crate features to be activated: `FuzzingFunctions`*"]
    pub fn cycle_collect();
    # [ wasm_bindgen ( catch , method , structural , static_method_of = FuzzingFunctions , js_name = enableAccessibility ) ]
    #[doc = "The `enableAccessibility()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FuzzingFunctions/enableAccessibility)\n\n*This API requires the following crate features to be activated: `FuzzingFunctions`*"]
    pub fn enable_accessibility() -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , static_method_of = FuzzingFunctions , js_name = garbageCollect ) ]
    #[doc = "The `garbageCollect()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FuzzingFunctions/garbageCollect)\n\n*This API requires the following crate features to be activated: `FuzzingFunctions`*"]
    pub fn garbage_collect();
}
