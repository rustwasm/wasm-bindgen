use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = MediaCapabilitiesInfo , typescript_name = MediaCapabilitiesInfo ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaCapabilitiesInfo` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilitiesInfo)\n\n*This API requires the following crate features to be activated: `MediaCapabilitiesInfo`*"]
    pub type MediaCapabilitiesInfo;
    # [ wasm_bindgen ( structural , method , getter , js_name = supported ) ]
    #[doc = "Getter for the `supported` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilitiesInfo/supported)\n\n*This API requires the following crate features to be activated: `MediaCapabilitiesInfo`*"]
    pub fn supported(this: &MediaCapabilitiesInfo) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = smooth ) ]
    #[doc = "Getter for the `smooth` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilitiesInfo/smooth)\n\n*This API requires the following crate features to be activated: `MediaCapabilitiesInfo`*"]
    pub fn smooth(this: &MediaCapabilitiesInfo) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = powerEfficient ) ]
    #[doc = "Getter for the `powerEfficient` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilitiesInfo/powerEfficient)\n\n*This API requires the following crate features to be activated: `MediaCapabilitiesInfo`*"]
    pub fn power_efficient(this: &MediaCapabilitiesInfo) -> bool;
}
