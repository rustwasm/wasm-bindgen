use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = ScriptProcessorNode , typescript_name = ScriptProcessorNode ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ScriptProcessorNode` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScriptProcessorNode)\n\n*This API requires the following crate features to be activated: `ScriptProcessorNode`*"]
    pub type ScriptProcessorNode;
    # [ wasm_bindgen ( structural , method , getter , js_name = onaudioprocess ) ]
    #[doc = "Getter for the `onaudioprocess` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScriptProcessorNode/onaudioprocess)\n\n*This API requires the following crate features to be activated: `ScriptProcessorNode`*"]
    pub fn onaudioprocess(this: &ScriptProcessorNode) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onaudioprocess ) ]
    #[doc = "Setter for the `onaudioprocess` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScriptProcessorNode/onaudioprocess)\n\n*This API requires the following crate features to be activated: `ScriptProcessorNode`*"]
    pub fn set_onaudioprocess(this: &ScriptProcessorNode, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = bufferSize ) ]
    #[doc = "Getter for the `bufferSize` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScriptProcessorNode/bufferSize)\n\n*This API requires the following crate features to be activated: `ScriptProcessorNode`*"]
    pub fn buffer_size(this: &ScriptProcessorNode) -> i32;
}
