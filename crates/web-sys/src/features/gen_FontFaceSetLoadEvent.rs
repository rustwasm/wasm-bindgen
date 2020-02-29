use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = FontFaceSetLoadEvent , typescript_type = "FontFaceSetLoadEvent" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `FontFaceSetLoadEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSetLoadEvent)
    ///
    ///*This API requires the following crate features to be activated: `FontFaceSetLoadEvent`*
    pub type FontFaceSetLoadEvent;

    # [ wasm_bindgen ( structural , method , getter , js_class = "FontFaceSetLoadEvent" , js_name = fontfaces ) ]
    ///Getter for the `fontfaces` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSetLoadEvent/fontfaces)
    ///
    ///*This API requires the following crate features to be activated: `FontFaceSetLoadEvent`*
    pub fn fontfaces(this: &FontFaceSetLoadEvent) -> ::js_sys::Array;

    #[wasm_bindgen(catch, constructor, js_class = "FontFaceSetLoadEvent")]
    ///The `new FontFaceSetLoadEvent(..)` constructor, creating a new instance of `FontFaceSetLoadEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSetLoadEvent/FontFaceSetLoadEvent)
    ///
    ///*This API requires the following crate features to be activated: `FontFaceSetLoadEvent`*
    pub fn new(type_: &str) -> Result<FontFaceSetLoadEvent, JsValue>;

    #[cfg(feature = "FontFaceSetLoadEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "FontFaceSetLoadEvent")]
    ///The `new FontFaceSetLoadEvent(..)` constructor, creating a new instance of `FontFaceSetLoadEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSetLoadEvent/FontFaceSetLoadEvent)
    ///
    ///*This API requires the following crate features to be activated: `FontFaceSetLoadEvent`, `FontFaceSetLoadEventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &FontFaceSetLoadEventInit,
    ) -> Result<FontFaceSetLoadEvent, JsValue>;

}
