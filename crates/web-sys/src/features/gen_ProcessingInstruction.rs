use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = CharacterData , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = ProcessingInstruction , typescript_name = ProcessingInstruction ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ProcessingInstruction` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProcessingInstruction)\n\n*This API requires the following crate features to be activated: `ProcessingInstruction`*"]
    pub type ProcessingInstruction;
    # [ wasm_bindgen ( structural , method , getter , js_name = target ) ]
    #[doc = "Getter for the `target` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProcessingInstruction/target)\n\n*This API requires the following crate features to be activated: `ProcessingInstruction`*"]
    pub fn target(this: &ProcessingInstruction) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = sheet ) ]
    #[cfg(feature = "StyleSheet")]
    #[doc = "Getter for the `sheet` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProcessingInstruction/sheet)\n\n*This API requires the following crate features to be activated: `ProcessingInstruction`, `StyleSheet`*"]
    pub fn sheet(this: &ProcessingInstruction) -> Option<StyleSheet>;
}
