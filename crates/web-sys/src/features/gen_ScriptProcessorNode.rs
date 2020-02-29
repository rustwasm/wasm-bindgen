use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = ScriptProcessorNode , typescript_name = ScriptProcessorNode ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ScriptProcessorNode` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScriptProcessorNode)
    ///
    ///*This API requires the following crate features to be activated: `ScriptProcessorNode`*
    pub type ScriptProcessorNode;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ScriptProcessorNode" , js_name = onaudioprocess ) ]
    ///Getter for the `onaudioprocess` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScriptProcessorNode/onaudioprocess)
    ///
    ///*This API requires the following crate features to be activated: `ScriptProcessorNode`*
    pub fn onaudioprocess(this: &ScriptProcessorNode) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "ScriptProcessorNode" , js_name = onaudioprocess ) ]
    ///Setter for the `onaudioprocess` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScriptProcessorNode/onaudioprocess)
    ///
    ///*This API requires the following crate features to be activated: `ScriptProcessorNode`*
    pub fn set_onaudioprocess(this: &ScriptProcessorNode, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "ScriptProcessorNode" , js_name = bufferSize ) ]
    ///Getter for the `bufferSize` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScriptProcessorNode/bufferSize)
    ///
    ///*This API requires the following crate features to be activated: `ScriptProcessorNode`*
    pub fn buffer_size(this: &ScriptProcessorNode) -> i32;

}
