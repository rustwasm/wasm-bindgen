use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = AudioDestinationNode , typescript_type = "AudioDestinationNode" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `AudioDestinationNode` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioDestinationNode)
    ///
    ///*This API requires the following crate features to be activated: `AudioDestinationNode`*
    pub type AudioDestinationNode;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioDestinationNode" , js_name = maxChannelCount ) ]
    ///Getter for the `maxChannelCount` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioDestinationNode/maxChannelCount)
    ///
    ///*This API requires the following crate features to be activated: `AudioDestinationNode`*
    pub fn max_channel_count(this: &AudioDestinationNode) -> u32;

}
