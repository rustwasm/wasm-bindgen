use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = NavigatorAutomationInformation , typescript_name = NavigatorAutomationInformation ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NavigatorAutomationInformation` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorAutomationInformation)\n\n*This API requires the following crate features to be activated: `NavigatorAutomationInformation`*"]
    pub type NavigatorAutomationInformation;
    # [ wasm_bindgen ( structural , method , getter , js_name = webdriver ) ]
    #[doc = "Getter for the `webdriver` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorAutomationInformation/webdriver)\n\n*This API requires the following crate features to be activated: `NavigatorAutomationInformation`*"]
    pub fn webdriver(this: &NavigatorAutomationInformation) -> bool;
}
