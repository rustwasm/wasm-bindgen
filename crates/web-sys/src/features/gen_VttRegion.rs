use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = VTTRegion , typescript_type = "VTTRegion" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `VttRegion` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion)
    ///
    ///*This API requires the following crate features to be activated: `VttRegion`*
    pub type VttRegion;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VTTRegion" , js_name = id ) ]
    ///Getter for the `id` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/id)
    ///
    ///*This API requires the following crate features to be activated: `VttRegion`*
    pub fn id(this: &VttRegion) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "VTTRegion" , js_name = id ) ]
    ///Setter for the `id` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/id)
    ///
    ///*This API requires the following crate features to be activated: `VttRegion`*
    pub fn set_id(this: &VttRegion, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "VTTRegion" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/width)
    ///
    ///*This API requires the following crate features to be activated: `VttRegion`*
    pub fn width(this: &VttRegion) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "VTTRegion" , js_name = width ) ]
    ///Setter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/width)
    ///
    ///*This API requires the following crate features to be activated: `VttRegion`*
    pub fn set_width(this: &VttRegion, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "VTTRegion" , js_name = lines ) ]
    ///Getter for the `lines` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/lines)
    ///
    ///*This API requires the following crate features to be activated: `VttRegion`*
    pub fn lines(this: &VttRegion) -> i32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "VTTRegion" , js_name = lines ) ]
    ///Setter for the `lines` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/lines)
    ///
    ///*This API requires the following crate features to be activated: `VttRegion`*
    pub fn set_lines(this: &VttRegion, value: i32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "VTTRegion" , js_name = regionAnchorX ) ]
    ///Getter for the `regionAnchorX` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/regionAnchorX)
    ///
    ///*This API requires the following crate features to be activated: `VttRegion`*
    pub fn region_anchor_x(this: &VttRegion) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "VTTRegion" , js_name = regionAnchorX ) ]
    ///Setter for the `regionAnchorX` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/regionAnchorX)
    ///
    ///*This API requires the following crate features to be activated: `VttRegion`*
    pub fn set_region_anchor_x(this: &VttRegion, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "VTTRegion" , js_name = regionAnchorY ) ]
    ///Getter for the `regionAnchorY` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/regionAnchorY)
    ///
    ///*This API requires the following crate features to be activated: `VttRegion`*
    pub fn region_anchor_y(this: &VttRegion) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "VTTRegion" , js_name = regionAnchorY ) ]
    ///Setter for the `regionAnchorY` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/regionAnchorY)
    ///
    ///*This API requires the following crate features to be activated: `VttRegion`*
    pub fn set_region_anchor_y(this: &VttRegion, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "VTTRegion" , js_name = viewportAnchorX ) ]
    ///Getter for the `viewportAnchorX` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/viewportAnchorX)
    ///
    ///*This API requires the following crate features to be activated: `VttRegion`*
    pub fn viewport_anchor_x(this: &VttRegion) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "VTTRegion" , js_name = viewportAnchorX ) ]
    ///Setter for the `viewportAnchorX` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/viewportAnchorX)
    ///
    ///*This API requires the following crate features to be activated: `VttRegion`*
    pub fn set_viewport_anchor_x(this: &VttRegion, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "VTTRegion" , js_name = viewportAnchorY ) ]
    ///Getter for the `viewportAnchorY` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/viewportAnchorY)
    ///
    ///*This API requires the following crate features to be activated: `VttRegion`*
    pub fn viewport_anchor_y(this: &VttRegion) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "VTTRegion" , js_name = viewportAnchorY ) ]
    ///Setter for the `viewportAnchorY` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/viewportAnchorY)
    ///
    ///*This API requires the following crate features to be activated: `VttRegion`*
    pub fn set_viewport_anchor_y(this: &VttRegion, value: f64);

    #[cfg(feature = "ScrollSetting")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "VTTRegion" , js_name = scroll ) ]
    ///Getter for the `scroll` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/scroll)
    ///
    ///*This API requires the following crate features to be activated: `ScrollSetting`, `VttRegion`*
    pub fn scroll(this: &VttRegion) -> ScrollSetting;

    #[cfg(feature = "ScrollSetting")]
    # [ wasm_bindgen ( structural , method , setter , js_class = "VTTRegion" , js_name = scroll ) ]
    ///Setter for the `scroll` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/scroll)
    ///
    ///*This API requires the following crate features to be activated: `ScrollSetting`, `VttRegion`*
    pub fn set_scroll(this: &VttRegion, value: ScrollSetting);

    #[wasm_bindgen(catch, constructor, js_class = "VTTRegion")]
    ///The `new VttRegion(..)` constructor, creating a new instance of `VttRegion`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/VTTRegion)
    ///
    ///*This API requires the following crate features to be activated: `VttRegion`*
    pub fn new() -> Result<VttRegion, JsValue>;

}
