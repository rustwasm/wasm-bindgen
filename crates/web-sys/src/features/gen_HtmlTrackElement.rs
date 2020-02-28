use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLTrackElement , typescript_name = HTMLTrackElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlTrackElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement)\n\n*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    pub type HtmlTrackElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = kind ) ]
    #[doc = "Getter for the `kind` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/kind)\n\n*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    pub fn kind(this: &HtmlTrackElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = kind ) ]
    #[doc = "Setter for the `kind` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/kind)\n\n*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    pub fn set_kind(this: &HtmlTrackElement, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = src ) ]
    #[doc = "Getter for the `src` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/src)\n\n*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    pub fn src(this: &HtmlTrackElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = src ) ]
    #[doc = "Setter for the `src` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/src)\n\n*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    pub fn set_src(this: &HtmlTrackElement, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = srclang ) ]
    #[doc = "Getter for the `srclang` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/srclang)\n\n*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    pub fn srclang(this: &HtmlTrackElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = srclang ) ]
    #[doc = "Setter for the `srclang` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/srclang)\n\n*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    pub fn set_srclang(this: &HtmlTrackElement, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = label ) ]
    #[doc = "Getter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/label)\n\n*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    pub fn label(this: &HtmlTrackElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = label ) ]
    #[doc = "Setter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/label)\n\n*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    pub fn set_label(this: &HtmlTrackElement, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = default ) ]
    #[doc = "Getter for the `default` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/default)\n\n*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    pub fn default(this: &HtmlTrackElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = default ) ]
    #[doc = "Setter for the `default` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/default)\n\n*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    pub fn set_default(this: &HtmlTrackElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = readyState ) ]
    #[doc = "Getter for the `readyState` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/readyState)\n\n*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    pub fn ready_state(this: &HtmlTrackElement) -> u16;
    # [ wasm_bindgen ( structural , method , getter , js_name = track ) ]
    #[cfg(feature = "TextTrack")]
    #[doc = "Getter for the `track` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/track)\n\n*This API requires the following crate features to be activated: `HtmlTrackElement`, `TextTrack`*"]
    pub fn track(this: &HtmlTrackElement) -> Option<TextTrack>;
}
impl HtmlTrackElement {
    pub const NONE: u16 = 0i64 as u16;
    pub const LOADING: u16 = 1u64 as u16;
    pub const LOADED: u16 = 2u64 as u16;
    pub const ERROR: u16 = 3u64 as u16;
}
