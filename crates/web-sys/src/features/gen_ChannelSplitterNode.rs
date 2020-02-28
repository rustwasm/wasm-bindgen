use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = ChannelSplitterNode , typescript_name = ChannelSplitterNode ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ChannelSplitterNode` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ChannelSplitterNode)\n\n*This API requires the following crate features to be activated: `ChannelSplitterNode`*"]
    pub type ChannelSplitterNode;
    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, js_class = "ChannelSplitterNode", constructor)]
    #[doc = "The `new ChannelSplitterNode(..)` constructor, creating a new instance of `ChannelSplitterNode`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ChannelSplitterNode/ChannelSplitterNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `ChannelSplitterNode`*"]
    pub fn new(
        this: &ChannelSplitterNode,
        context: &BaseAudioContext,
    ) -> Result<ChannelSplitterNode, JsValue>;
    #[cfg(all(feature = "BaseAudioContext", feature = "ChannelSplitterOptions",))]
    #[wasm_bindgen(catch, js_class = "ChannelSplitterNode", constructor)]
    #[doc = "The `new ChannelSplitterNode(..)` constructor, creating a new instance of `ChannelSplitterNode`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ChannelSplitterNode/ChannelSplitterNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `ChannelSplitterNode`, `ChannelSplitterOptions`*"]
    pub fn new_with_options(
        this: &ChannelSplitterNode,
        context: &BaseAudioContext,
        options: &ChannelSplitterOptions,
    ) -> Result<ChannelSplitterNode, JsValue>;
}
