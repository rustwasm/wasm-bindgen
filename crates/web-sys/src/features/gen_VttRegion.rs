use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = VTTRegion , typescript_name = VTTRegion ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VttRegion` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion)\n\n*This API requires the following crate features to be activated: `VttRegion`*"]
    pub type VttRegion;
    # [ wasm_bindgen ( structural , method , getter , js_name = id ) ]
    #[doc = "Getter for the `id` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/id)\n\n*This API requires the following crate features to be activated: `VttRegion`*"]
    pub fn id(this: &VttRegion) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = id ) ]
    #[doc = "Setter for the `id` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/id)\n\n*This API requires the following crate features to be activated: `VttRegion`*"]
    pub fn set_id(this: &VttRegion, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = width ) ]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/width)\n\n*This API requires the following crate features to be activated: `VttRegion`*"]
    pub fn width(this: &VttRegion) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = width ) ]
    #[doc = "Setter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/width)\n\n*This API requires the following crate features to be activated: `VttRegion`*"]
    pub fn set_width(this: &VttRegion, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = lines ) ]
    #[doc = "Getter for the `lines` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/lines)\n\n*This API requires the following crate features to be activated: `VttRegion`*"]
    pub fn lines(this: &VttRegion) -> i32;
    # [ wasm_bindgen ( structural , method , setter , js_name = lines ) ]
    #[doc = "Setter for the `lines` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/lines)\n\n*This API requires the following crate features to be activated: `VttRegion`*"]
    pub fn set_lines(this: &VttRegion, value: i32);
    # [ wasm_bindgen ( structural , method , getter , js_name = regionAnchorX ) ]
    #[doc = "Getter for the `regionAnchorX` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/regionAnchorX)\n\n*This API requires the following crate features to be activated: `VttRegion`*"]
    pub fn region_anchor_x(this: &VttRegion) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = regionAnchorX ) ]
    #[doc = "Setter for the `regionAnchorX` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/regionAnchorX)\n\n*This API requires the following crate features to be activated: `VttRegion`*"]
    pub fn set_region_anchor_x(this: &VttRegion, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = regionAnchorY ) ]
    #[doc = "Getter for the `regionAnchorY` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/regionAnchorY)\n\n*This API requires the following crate features to be activated: `VttRegion`*"]
    pub fn region_anchor_y(this: &VttRegion) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = regionAnchorY ) ]
    #[doc = "Setter for the `regionAnchorY` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/regionAnchorY)\n\n*This API requires the following crate features to be activated: `VttRegion`*"]
    pub fn set_region_anchor_y(this: &VttRegion, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = viewportAnchorX ) ]
    #[doc = "Getter for the `viewportAnchorX` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/viewportAnchorX)\n\n*This API requires the following crate features to be activated: `VttRegion`*"]
    pub fn viewport_anchor_x(this: &VttRegion) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = viewportAnchorX ) ]
    #[doc = "Setter for the `viewportAnchorX` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/viewportAnchorX)\n\n*This API requires the following crate features to be activated: `VttRegion`*"]
    pub fn set_viewport_anchor_x(this: &VttRegion, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = viewportAnchorY ) ]
    #[doc = "Getter for the `viewportAnchorY` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/viewportAnchorY)\n\n*This API requires the following crate features to be activated: `VttRegion`*"]
    pub fn viewport_anchor_y(this: &VttRegion) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = viewportAnchorY ) ]
    #[doc = "Setter for the `viewportAnchorY` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/viewportAnchorY)\n\n*This API requires the following crate features to be activated: `VttRegion`*"]
    pub fn set_viewport_anchor_y(this: &VttRegion, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = scroll ) ]
    #[cfg(feature = "ScrollSetting")]
    #[doc = "Getter for the `scroll` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/scroll)\n\n*This API requires the following crate features to be activated: `ScrollSetting`, `VttRegion`*"]
    pub fn scroll(this: &VttRegion) -> ScrollSetting;
    # [ wasm_bindgen ( structural , method , setter , js_name = scroll ) ]
    #[cfg(feature = "ScrollSetting")]
    #[doc = "Setter for the `scroll` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/scroll)\n\n*This API requires the following crate features to be activated: `ScrollSetting`, `VttRegion`*"]
    pub fn set_scroll(this: &VttRegion, value: ScrollSetting);
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new VttRegion(..)` constructor, creating a new instance of `VttRegion`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/VTTRegion)\n\n*This API requires the following crate features to be activated: `VttRegion`*"]
    pub fn new(this: &VttRegion) -> Result<VttRegion, JsValue>;
}
