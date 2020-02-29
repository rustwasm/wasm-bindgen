use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = MediaQueryListEvent , typescript_name = MediaQueryListEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MediaQueryListEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryListEvent)
    ///
    ///*This API requires the following crate features to be activated: `MediaQueryListEvent`*
    pub type MediaQueryListEvent;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaQueryListEvent" , js_name = media ) ]
    ///Getter for the `media` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryListEvent/media)
    ///
    ///*This API requires the following crate features to be activated: `MediaQueryListEvent`*
    pub fn media(this: &MediaQueryListEvent) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaQueryListEvent" , js_name = matches ) ]
    ///Getter for the `matches` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryListEvent/matches)
    ///
    ///*This API requires the following crate features to be activated: `MediaQueryListEvent`*
    pub fn matches(this: &MediaQueryListEvent) -> bool;

    #[wasm_bindgen(catch, constructor, js_class = "MediaQueryListEvent")]
    ///The `new MediaQueryListEvent(..)` constructor, creating a new instance of `MediaQueryListEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryListEvent/MediaQueryListEvent)
    ///
    ///*This API requires the following crate features to be activated: `MediaQueryListEvent`*
    pub fn new(type_: &str) -> Result<MediaQueryListEvent, JsValue>;

    #[cfg(feature = "MediaQueryListEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "MediaQueryListEvent")]
    ///The `new MediaQueryListEvent(..)` constructor, creating a new instance of `MediaQueryListEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryListEvent/MediaQueryListEvent)
    ///
    ///*This API requires the following crate features to be activated: `MediaQueryListEvent`, `MediaQueryListEventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &MediaQueryListEventInit,
    ) -> Result<MediaQueryListEvent, JsValue>;

}
