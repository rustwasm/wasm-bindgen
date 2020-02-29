use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = MediaCapabilitiesInfo , typescript_type = "MediaCapabilitiesInfo" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MediaCapabilitiesInfo` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilitiesInfo)
    ///
    ///*This API requires the following crate features to be activated: `MediaCapabilitiesInfo`*
    pub type MediaCapabilitiesInfo;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaCapabilitiesInfo" , js_name = supported ) ]
    ///Getter for the `supported` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilitiesInfo/supported)
    ///
    ///*This API requires the following crate features to be activated: `MediaCapabilitiesInfo`*
    pub fn supported(this: &MediaCapabilitiesInfo) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaCapabilitiesInfo" , js_name = smooth ) ]
    ///Getter for the `smooth` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilitiesInfo/smooth)
    ///
    ///*This API requires the following crate features to be activated: `MediaCapabilitiesInfo`*
    pub fn smooth(this: &MediaCapabilitiesInfo) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaCapabilitiesInfo" , js_name = powerEfficient ) ]
    ///Getter for the `powerEfficient` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilitiesInfo/powerEfficient)
    ///
    ///*This API requires the following crate features to be activated: `MediaCapabilitiesInfo`*
    pub fn power_efficient(this: &MediaCapabilitiesInfo) -> bool;

}
