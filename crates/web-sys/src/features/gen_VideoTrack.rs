use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = VideoTrack , typescript_name = VideoTrack ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `VideoTrack` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack)
    ///
    ///*This API requires the following crate features to be activated: `VideoTrack`*
    pub type VideoTrack;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VideoTrack" , js_name = id ) ]
    ///Getter for the `id` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/id)
    ///
    ///*This API requires the following crate features to be activated: `VideoTrack`*
    pub fn id(this: &VideoTrack) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VideoTrack" , js_name = kind ) ]
    ///Getter for the `kind` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/kind)
    ///
    ///*This API requires the following crate features to be activated: `VideoTrack`*
    pub fn kind(this: &VideoTrack) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VideoTrack" , js_name = label ) ]
    ///Getter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/label)
    ///
    ///*This API requires the following crate features to be activated: `VideoTrack`*
    pub fn label(this: &VideoTrack) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VideoTrack" , js_name = language ) ]
    ///Getter for the `language` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/language)
    ///
    ///*This API requires the following crate features to be activated: `VideoTrack`*
    pub fn language(this: &VideoTrack) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VideoTrack" , js_name = selected ) ]
    ///Getter for the `selected` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/selected)
    ///
    ///*This API requires the following crate features to be activated: `VideoTrack`*
    pub fn selected(this: &VideoTrack) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "VideoTrack" , js_name = selected ) ]
    ///Setter for the `selected` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/selected)
    ///
    ///*This API requires the following crate features to be activated: `VideoTrack`*
    pub fn set_selected(this: &VideoTrack, value: bool);

}
