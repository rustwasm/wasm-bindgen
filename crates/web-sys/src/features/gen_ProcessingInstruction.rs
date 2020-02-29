use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = CharacterData , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = ProcessingInstruction , typescript_type = "ProcessingInstruction" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ProcessingInstruction` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProcessingInstruction)
    ///
    ///*This API requires the following crate features to be activated: `ProcessingInstruction`*
    pub type ProcessingInstruction;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ProcessingInstruction" , js_name = target ) ]
    ///Getter for the `target` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProcessingInstruction/target)
    ///
    ///*This API requires the following crate features to be activated: `ProcessingInstruction`*
    pub fn target(this: &ProcessingInstruction) -> String;

    #[cfg(feature = "StyleSheet")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "ProcessingInstruction" , js_name = sheet ) ]
    ///Getter for the `sheet` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProcessingInstruction/sheet)
    ///
    ///*This API requires the following crate features to be activated: `ProcessingInstruction`, `StyleSheet`*
    pub fn sheet(this: &ProcessingInstruction) -> Option<StyleSheet>;

}
