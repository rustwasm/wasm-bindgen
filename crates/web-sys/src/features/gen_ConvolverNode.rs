use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = ConvolverNode , typescript_type = "ConvolverNode" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ConvolverNode` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode)
    ///
    ///*This API requires the following crate features to be activated: `ConvolverNode`*
    pub type ConvolverNode;

    #[cfg(feature = "AudioBuffer")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "ConvolverNode" , js_name = buffer ) ]
    ///Getter for the `buffer` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/buffer)
    ///
    ///*This API requires the following crate features to be activated: `AudioBuffer`, `ConvolverNode`*
    pub fn buffer(this: &ConvolverNode) -> Option<AudioBuffer>;

    #[cfg(feature = "AudioBuffer")]
    # [ wasm_bindgen ( structural , method , setter , js_class = "ConvolverNode" , js_name = buffer ) ]
    ///Setter for the `buffer` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/buffer)
    ///
    ///*This API requires the following crate features to be activated: `AudioBuffer`, `ConvolverNode`*
    pub fn set_buffer(this: &ConvolverNode, value: Option<&AudioBuffer>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "ConvolverNode" , js_name = normalize ) ]
    ///Getter for the `normalize` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/normalize)
    ///
    ///*This API requires the following crate features to be activated: `ConvolverNode`*
    pub fn normalize(this: &ConvolverNode) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "ConvolverNode" , js_name = normalize ) ]
    ///Setter for the `normalize` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/normalize)
    ///
    ///*This API requires the following crate features to be activated: `ConvolverNode`*
    pub fn set_normalize(this: &ConvolverNode, value: bool);

    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, constructor, js_class = "ConvolverNode")]
    ///The `new ConvolverNode(..)` constructor, creating a new instance of `ConvolverNode`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/ConvolverNode)
    ///
    ///*This API requires the following crate features to be activated: `BaseAudioContext`, `ConvolverNode`*
    pub fn new(context: &BaseAudioContext) -> Result<ConvolverNode, JsValue>;

    #[cfg(all(feature = "BaseAudioContext", feature = "ConvolverOptions",))]
    #[wasm_bindgen(catch, constructor, js_class = "ConvolverNode")]
    ///The `new ConvolverNode(..)` constructor, creating a new instance of `ConvolverNode`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/ConvolverNode)
    ///
    ///*This API requires the following crate features to be activated: `BaseAudioContext`, `ConvolverNode`, `ConvolverOptions`*
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &ConvolverOptions,
    ) -> Result<ConvolverNode, JsValue>;

}
