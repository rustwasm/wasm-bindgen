use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = TextTrackCue , extends = EventTarget , extends = :: js_sys :: Object , js_name = VTTCue , typescript_name = VTTCue ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VttCue` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue)\n\n*This API requires the following crate features to be activated: `VttCue`*"]
    pub type VttCue;
    # [ wasm_bindgen ( structural , method , getter , js_name = region ) ]
    #[cfg(feature = "VttRegion")]
    #[doc = "Getter for the `region` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/region)\n\n*This API requires the following crate features to be activated: `VttCue`, `VttRegion`*"]
    pub fn region(this: &VttCue) -> Option<VttRegion>;
    # [ wasm_bindgen ( structural , method , setter , js_name = region ) ]
    #[cfg(feature = "VttRegion")]
    #[doc = "Setter for the `region` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/region)\n\n*This API requires the following crate features to be activated: `VttCue`, `VttRegion`*"]
    pub fn set_region(this: &VttCue, value: Option<VttRegion>);
    # [ wasm_bindgen ( structural , method , getter , js_name = vertical ) ]
    #[cfg(feature = "DirectionSetting")]
    #[doc = "Getter for the `vertical` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/vertical)\n\n*This API requires the following crate features to be activated: `DirectionSetting`, `VttCue`*"]
    pub fn vertical(this: &VttCue) -> DirectionSetting;
    # [ wasm_bindgen ( structural , method , setter , js_name = vertical ) ]
    #[cfg(feature = "DirectionSetting")]
    #[doc = "Setter for the `vertical` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/vertical)\n\n*This API requires the following crate features to be activated: `DirectionSetting`, `VttCue`*"]
    pub fn set_vertical(this: &VttCue, value: DirectionSetting);
    # [ wasm_bindgen ( structural , method , getter , js_name = snapToLines ) ]
    #[doc = "Getter for the `snapToLines` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/snapToLines)\n\n*This API requires the following crate features to be activated: `VttCue`*"]
    pub fn snap_to_lines(this: &VttCue) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = snapToLines ) ]
    #[doc = "Setter for the `snapToLines` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/snapToLines)\n\n*This API requires the following crate features to be activated: `VttCue`*"]
    pub fn set_snap_to_lines(this: &VttCue, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = line ) ]
    #[doc = "Getter for the `line` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/line)\n\n*This API requires the following crate features to be activated: `VttCue`*"]
    pub fn line(this: &VttCue) -> ::wasm_bindgen::JsValue;
    # [ wasm_bindgen ( structural , method , setter , js_name = line ) ]
    #[doc = "Setter for the `line` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/line)\n\n*This API requires the following crate features to be activated: `VttCue`*"]
    pub fn set_line(this: &VttCue, value: ::wasm_bindgen::JsValue);
    # [ wasm_bindgen ( structural , method , getter , js_name = lineAlign ) ]
    #[cfg(feature = "LineAlignSetting")]
    #[doc = "Getter for the `lineAlign` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/lineAlign)\n\n*This API requires the following crate features to be activated: `LineAlignSetting`, `VttCue`*"]
    pub fn line_align(this: &VttCue) -> LineAlignSetting;
    # [ wasm_bindgen ( structural , method , setter , js_name = lineAlign ) ]
    #[cfg(feature = "LineAlignSetting")]
    #[doc = "Setter for the `lineAlign` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/lineAlign)\n\n*This API requires the following crate features to be activated: `LineAlignSetting`, `VttCue`*"]
    pub fn set_line_align(this: &VttCue, value: LineAlignSetting);
    # [ wasm_bindgen ( structural , method , getter , js_name = position ) ]
    #[doc = "Getter for the `position` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/position)\n\n*This API requires the following crate features to be activated: `VttCue`*"]
    pub fn position(this: &VttCue) -> ::wasm_bindgen::JsValue;
    # [ wasm_bindgen ( structural , method , setter , js_name = position ) ]
    #[doc = "Setter for the `position` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/position)\n\n*This API requires the following crate features to be activated: `VttCue`*"]
    pub fn set_position(this: &VttCue, value: ::wasm_bindgen::JsValue);
    # [ wasm_bindgen ( structural , method , getter , js_name = positionAlign ) ]
    #[cfg(feature = "PositionAlignSetting")]
    #[doc = "Getter for the `positionAlign` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/positionAlign)\n\n*This API requires the following crate features to be activated: `PositionAlignSetting`, `VttCue`*"]
    pub fn position_align(this: &VttCue) -> PositionAlignSetting;
    # [ wasm_bindgen ( structural , method , setter , js_name = positionAlign ) ]
    #[cfg(feature = "PositionAlignSetting")]
    #[doc = "Setter for the `positionAlign` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/positionAlign)\n\n*This API requires the following crate features to be activated: `PositionAlignSetting`, `VttCue`*"]
    pub fn set_position_align(this: &VttCue, value: PositionAlignSetting);
    # [ wasm_bindgen ( structural , method , getter , js_name = size ) ]
    #[doc = "Getter for the `size` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/size)\n\n*This API requires the following crate features to be activated: `VttCue`*"]
    pub fn size(this: &VttCue) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = size ) ]
    #[doc = "Setter for the `size` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/size)\n\n*This API requires the following crate features to be activated: `VttCue`*"]
    pub fn set_size(this: &VttCue, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = align ) ]
    #[cfg(feature = "AlignSetting")]
    #[doc = "Getter for the `align` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/align)\n\n*This API requires the following crate features to be activated: `AlignSetting`, `VttCue`*"]
    pub fn align(this: &VttCue) -> AlignSetting;
    # [ wasm_bindgen ( structural , method , setter , js_name = align ) ]
    #[cfg(feature = "AlignSetting")]
    #[doc = "Setter for the `align` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/align)\n\n*This API requires the following crate features to be activated: `AlignSetting`, `VttCue`*"]
    pub fn set_align(this: &VttCue, value: AlignSetting);
    # [ wasm_bindgen ( structural , method , getter , js_name = text ) ]
    #[doc = "Getter for the `text` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/text)\n\n*This API requires the following crate features to be activated: `VttCue`*"]
    pub fn text(this: &VttCue) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = text ) ]
    #[doc = "Setter for the `text` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/text)\n\n*This API requires the following crate features to be activated: `VttCue`*"]
    pub fn set_text(this: &VttCue, value: String);
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new VttCue(..)` constructor, creating a new instance of `VttCue`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/VTTCue)\n\n*This API requires the following crate features to be activated: `VttCue`*"]
    pub fn new(
        this: &VttCue,
        start_time: f64,
        end_time: f64,
        text: &str,
    ) -> Result<VttCue, JsValue>;
    #[cfg(feature = "DocumentFragment")]
    # [ wasm_bindgen ( method , structural , js_name = getCueAsHTML ) ]
    #[doc = "The `getCueAsHTML()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/getCueAsHTML)\n\n*This API requires the following crate features to be activated: `DocumentFragment`, `VttCue`*"]
    pub fn get_cue_as_html(this: &VttCue) -> DocumentFragment;
}
