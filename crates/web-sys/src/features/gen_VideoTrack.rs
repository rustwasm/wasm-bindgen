use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = VideoTrack , typescript_name = VideoTrack ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoTrack` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack)\n\n*This API requires the following crate features to be activated: `VideoTrack`*"]
    pub type VideoTrack;
    # [ wasm_bindgen ( structural , method , getter , js_name = id ) ]
    #[doc = "Getter for the `id` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/id)\n\n*This API requires the following crate features to be activated: `VideoTrack`*"]
    pub fn id(this: &VideoTrack) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = kind ) ]
    #[doc = "Getter for the `kind` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/kind)\n\n*This API requires the following crate features to be activated: `VideoTrack`*"]
    pub fn kind(this: &VideoTrack) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = label ) ]
    #[doc = "Getter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/label)\n\n*This API requires the following crate features to be activated: `VideoTrack`*"]
    pub fn label(this: &VideoTrack) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = language ) ]
    #[doc = "Getter for the `language` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/language)\n\n*This API requires the following crate features to be activated: `VideoTrack`*"]
    pub fn language(this: &VideoTrack) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = selected ) ]
    #[doc = "Getter for the `selected` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/selected)\n\n*This API requires the following crate features to be activated: `VideoTrack`*"]
    pub fn selected(this: &VideoTrack) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = selected ) ]
    #[doc = "Setter for the `selected` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/selected)\n\n*This API requires the following crate features to be activated: `VideoTrack`*"]
    pub fn set_selected(this: &VideoTrack, value: bool);
}
