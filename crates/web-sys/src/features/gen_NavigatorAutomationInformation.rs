use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = NavigatorAutomationInformation , typescript_type = "NavigatorAutomationInformation" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `NavigatorAutomationInformation` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorAutomationInformation)
    ///
    ///*This API requires the following crate features to be activated: `NavigatorAutomationInformation`*
    pub type NavigatorAutomationInformation;

    # [ wasm_bindgen ( structural , method , getter , js_class = "NavigatorAutomationInformation" , js_name = webdriver ) ]
    ///Getter for the `webdriver` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorAutomationInformation/webdriver)
    ///
    ///*This API requires the following crate features to be activated: `NavigatorAutomationInformation`*
    pub fn webdriver(this: &NavigatorAutomationInformation) -> bool;

}
