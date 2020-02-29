use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLTrackElement , typescript_name = HTMLTrackElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlTrackElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTrackElement`*
    pub type HtmlTrackElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTrackElement" , js_name = kind ) ]
    ///Getter for the `kind` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/kind)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTrackElement`*
    pub fn kind(this: &HtmlTrackElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTrackElement" , js_name = kind ) ]
    ///Setter for the `kind` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/kind)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTrackElement`*
    pub fn set_kind(this: &HtmlTrackElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTrackElement" , js_name = src ) ]
    ///Getter for the `src` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/src)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTrackElement`*
    pub fn src(this: &HtmlTrackElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTrackElement" , js_name = src ) ]
    ///Setter for the `src` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/src)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTrackElement`*
    pub fn set_src(this: &HtmlTrackElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTrackElement" , js_name = srclang ) ]
    ///Getter for the `srclang` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/srclang)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTrackElement`*
    pub fn srclang(this: &HtmlTrackElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTrackElement" , js_name = srclang ) ]
    ///Setter for the `srclang` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/srclang)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTrackElement`*
    pub fn set_srclang(this: &HtmlTrackElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTrackElement" , js_name = label ) ]
    ///Getter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/label)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTrackElement`*
    pub fn label(this: &HtmlTrackElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTrackElement" , js_name = label ) ]
    ///Setter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/label)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTrackElement`*
    pub fn set_label(this: &HtmlTrackElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTrackElement" , js_name = default ) ]
    ///Getter for the `default` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/default)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTrackElement`*
    pub fn default(this: &HtmlTrackElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTrackElement" , js_name = default ) ]
    ///Setter for the `default` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/default)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTrackElement`*
    pub fn set_default(this: &HtmlTrackElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTrackElement" , js_name = readyState ) ]
    ///Getter for the `readyState` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/readyState)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTrackElement`*
    pub fn ready_state(this: &HtmlTrackElement) -> u16;

    #[cfg(feature = "TextTrack")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTrackElement" , js_name = track ) ]
    ///Getter for the `track` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/track)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTrackElement`, `TextTrack`*
    pub fn track(this: &HtmlTrackElement) -> Option<TextTrack>;

}

impl HtmlTrackElement {
    ///The `HTMLTrackElement.NONE` const.
    ///
    ///*This API requires the following crate features to be activated: `HtmlTrackElement`*

    pub const NONE: u16 = 0i64 as u16;

    ///The `HTMLTrackElement.LOADING` const.
    ///
    ///*This API requires the following crate features to be activated: `HtmlTrackElement`*

    pub const LOADING: u16 = 1u64 as u16;

    ///The `HTMLTrackElement.LOADED` const.
    ///
    ///*This API requires the following crate features to be activated: `HtmlTrackElement`*

    pub const LOADED: u16 = 2u64 as u16;

    ///The `HTMLTrackElement.ERROR` const.
    ///
    ///*This API requires the following crate features to be activated: `HtmlTrackElement`*

    pub const ERROR: u16 = 3u64 as u16;
}
