use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = AudioDestinationNode , typescript_name = AudioDestinationNode ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioDestinationNode` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioDestinationNode)\n\n*This API requires the following crate features to be activated: `AudioDestinationNode`*"]
    pub type AudioDestinationNode;
    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioDestinationNode" , js_name = maxChannelCount ) ]
    #[doc = "Getter for the `maxChannelCount` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioDestinationNode/maxChannelCount)\n\n*This API requires the following crate features to be activated: `AudioDestinationNode`*"]
    pub fn max_channel_count(this: &AudioDestinationNode) -> u32;
}
