use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = MediaQueryListEvent , typescript_name = MediaQueryListEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaQueryListEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryListEvent)\n\n*This API requires the following crate features to be activated: `MediaQueryListEvent`*"]
    pub type MediaQueryListEvent;
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaQueryListEvent" , js_name = media ) ]
    #[doc = "Getter for the `media` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryListEvent/media)\n\n*This API requires the following crate features to be activated: `MediaQueryListEvent`*"]
    pub fn media(this: &MediaQueryListEvent) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaQueryListEvent" , js_name = matches ) ]
    #[doc = "Getter for the `matches` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryListEvent/matches)\n\n*This API requires the following crate features to be activated: `MediaQueryListEvent`*"]
    pub fn matches(this: &MediaQueryListEvent) -> bool;
    #[wasm_bindgen(catch, js_class = "MediaQueryListEvent", constructor)]
    #[doc = "The `new MediaQueryListEvent(..)` constructor, creating a new instance of `MediaQueryListEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryListEvent/MediaQueryListEvent)\n\n*This API requires the following crate features to be activated: `MediaQueryListEvent`*"]
    pub fn new(this: &MediaQueryListEvent, type_: &str) -> Result<MediaQueryListEvent, JsValue>;
    #[cfg(feature = "MediaQueryListEventInit")]
    #[wasm_bindgen(catch, js_class = "MediaQueryListEvent", constructor)]
    #[doc = "The `new MediaQueryListEvent(..)` constructor, creating a new instance of `MediaQueryListEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryListEvent/MediaQueryListEvent)\n\n*This API requires the following crate features to be activated: `MediaQueryListEvent`, `MediaQueryListEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &MediaQueryListEvent,
        type_: &str,
        event_init_dict: &MediaQueryListEventInit,
    ) -> Result<MediaQueryListEvent, JsValue>;
}
