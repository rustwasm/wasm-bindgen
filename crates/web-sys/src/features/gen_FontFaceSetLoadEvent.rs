use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = FontFaceSetLoadEvent , typescript_name = FontFaceSetLoadEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FontFaceSetLoadEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSetLoadEvent)\n\n*This API requires the following crate features to be activated: `FontFaceSetLoadEvent`*"]
    pub type FontFaceSetLoadEvent;
    # [ wasm_bindgen ( structural , method , getter , js_name = fontfaces ) ]
    #[doc = "Getter for the `fontfaces` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSetLoadEvent/fontfaces)\n\n*This API requires the following crate features to be activated: `FontFaceSetLoadEvent`*"]
    pub fn fontfaces(this: &FontFaceSetLoadEvent) -> ::js_sys::Array;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new FontFaceSetLoadEvent(..)` constructor, creating a new instance of `FontFaceSetLoadEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSetLoadEvent/FontFaceSetLoadEvent)\n\n*This API requires the following crate features to be activated: `FontFaceSetLoadEvent`*"]
    pub fn new(this: &FontFaceSetLoadEvent, type_: &str) -> Result<FontFaceSetLoadEvent, JsValue>;
    #[cfg(feature = "FontFaceSetLoadEventInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new FontFaceSetLoadEvent(..)` constructor, creating a new instance of `FontFaceSetLoadEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSetLoadEvent/FontFaceSetLoadEvent)\n\n*This API requires the following crate features to be activated: `FontFaceSetLoadEvent`, `FontFaceSetLoadEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &FontFaceSetLoadEvent,
        type_: &str,
        event_init_dict: &FontFaceSetLoadEventInit,
    ) -> Result<FontFaceSetLoadEvent, JsValue>;
}
