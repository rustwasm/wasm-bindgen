use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = TextTrackCue , extends = EventTarget , extends = :: js_sys :: Object , js_name = VTTCue , typescript_type = "VTTCue" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `VttCue` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue)
    ///
    ///*This API requires the following crate features to be activated: `VttCue`*
    pub type VttCue;

    #[cfg(feature = "VttRegion")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "VTTCue" , js_name = region ) ]
    ///Getter for the `region` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/region)
    ///
    ///*This API requires the following crate features to be activated: `VttCue`, `VttRegion`*
    pub fn region(this: &VttCue) -> Option<VttRegion>;

    #[cfg(feature = "VttRegion")]
    # [ wasm_bindgen ( structural , method , setter , js_class = "VTTCue" , js_name = region ) ]
    ///Setter for the `region` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/region)
    ///
    ///*This API requires the following crate features to be activated: `VttCue`, `VttRegion`*
    pub fn set_region(this: &VttCue, value: Option<&VttRegion>);

    #[cfg(feature = "DirectionSetting")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "VTTCue" , js_name = vertical ) ]
    ///Getter for the `vertical` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/vertical)
    ///
    ///*This API requires the following crate features to be activated: `DirectionSetting`, `VttCue`*
    pub fn vertical(this: &VttCue) -> DirectionSetting;

    #[cfg(feature = "DirectionSetting")]
    # [ wasm_bindgen ( structural , method , setter , js_class = "VTTCue" , js_name = vertical ) ]
    ///Setter for the `vertical` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/vertical)
    ///
    ///*This API requires the following crate features to be activated: `DirectionSetting`, `VttCue`*
    pub fn set_vertical(this: &VttCue, value: DirectionSetting);

    # [ wasm_bindgen ( structural , method , getter , js_class = "VTTCue" , js_name = snapToLines ) ]
    ///Getter for the `snapToLines` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/snapToLines)
    ///
    ///*This API requires the following crate features to be activated: `VttCue`*
    pub fn snap_to_lines(this: &VttCue) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "VTTCue" , js_name = snapToLines ) ]
    ///Setter for the `snapToLines` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/snapToLines)
    ///
    ///*This API requires the following crate features to be activated: `VttCue`*
    pub fn set_snap_to_lines(this: &VttCue, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "VTTCue" , js_name = line ) ]
    ///Getter for the `line` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/line)
    ///
    ///*This API requires the following crate features to be activated: `VttCue`*
    pub fn line(this: &VttCue) -> ::wasm_bindgen::JsValue;

    # [ wasm_bindgen ( structural , method , setter , js_class = "VTTCue" , js_name = line ) ]
    ///Setter for the `line` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/line)
    ///
    ///*This API requires the following crate features to be activated: `VttCue`*
    pub fn set_line(this: &VttCue, value: &::wasm_bindgen::JsValue);

    #[cfg(feature = "LineAlignSetting")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "VTTCue" , js_name = lineAlign ) ]
    ///Getter for the `lineAlign` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/lineAlign)
    ///
    ///*This API requires the following crate features to be activated: `LineAlignSetting`, `VttCue`*
    pub fn line_align(this: &VttCue) -> LineAlignSetting;

    #[cfg(feature = "LineAlignSetting")]
    # [ wasm_bindgen ( structural , method , setter , js_class = "VTTCue" , js_name = lineAlign ) ]
    ///Setter for the `lineAlign` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/lineAlign)
    ///
    ///*This API requires the following crate features to be activated: `LineAlignSetting`, `VttCue`*
    pub fn set_line_align(this: &VttCue, value: LineAlignSetting);

    # [ wasm_bindgen ( structural , method , getter , js_class = "VTTCue" , js_name = position ) ]
    ///Getter for the `position` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/position)
    ///
    ///*This API requires the following crate features to be activated: `VttCue`*
    pub fn position(this: &VttCue) -> ::wasm_bindgen::JsValue;

    # [ wasm_bindgen ( structural , method , setter , js_class = "VTTCue" , js_name = position ) ]
    ///Setter for the `position` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/position)
    ///
    ///*This API requires the following crate features to be activated: `VttCue`*
    pub fn set_position(this: &VttCue, value: &::wasm_bindgen::JsValue);

    #[cfg(feature = "PositionAlignSetting")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "VTTCue" , js_name = positionAlign ) ]
    ///Getter for the `positionAlign` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/positionAlign)
    ///
    ///*This API requires the following crate features to be activated: `PositionAlignSetting`, `VttCue`*
    pub fn position_align(this: &VttCue) -> PositionAlignSetting;

    #[cfg(feature = "PositionAlignSetting")]
    # [ wasm_bindgen ( structural , method , setter , js_class = "VTTCue" , js_name = positionAlign ) ]
    ///Setter for the `positionAlign` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/positionAlign)
    ///
    ///*This API requires the following crate features to be activated: `PositionAlignSetting`, `VttCue`*
    pub fn set_position_align(this: &VttCue, value: PositionAlignSetting);

    # [ wasm_bindgen ( structural , method , getter , js_class = "VTTCue" , js_name = size ) ]
    ///Getter for the `size` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/size)
    ///
    ///*This API requires the following crate features to be activated: `VttCue`*
    pub fn size(this: &VttCue) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "VTTCue" , js_name = size ) ]
    ///Setter for the `size` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/size)
    ///
    ///*This API requires the following crate features to be activated: `VttCue`*
    pub fn set_size(this: &VttCue, value: f64);

    #[cfg(feature = "AlignSetting")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "VTTCue" , js_name = align ) ]
    ///Getter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/align)
    ///
    ///*This API requires the following crate features to be activated: `AlignSetting`, `VttCue`*
    pub fn align(this: &VttCue) -> AlignSetting;

    #[cfg(feature = "AlignSetting")]
    # [ wasm_bindgen ( structural , method , setter , js_class = "VTTCue" , js_name = align ) ]
    ///Setter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/align)
    ///
    ///*This API requires the following crate features to be activated: `AlignSetting`, `VttCue`*
    pub fn set_align(this: &VttCue, value: AlignSetting);

    # [ wasm_bindgen ( structural , method , getter , js_class = "VTTCue" , js_name = text ) ]
    ///Getter for the `text` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/text)
    ///
    ///*This API requires the following crate features to be activated: `VttCue`*
    pub fn text(this: &VttCue) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "VTTCue" , js_name = text ) ]
    ///Setter for the `text` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/text)
    ///
    ///*This API requires the following crate features to be activated: `VttCue`*
    pub fn set_text(this: &VttCue, value: &str);

    #[wasm_bindgen(catch, constructor, js_class = "VTTCue")]
    ///The `new VttCue(..)` constructor, creating a new instance of `VttCue`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/VTTCue)
    ///
    ///*This API requires the following crate features to be activated: `VttCue`*
    pub fn new(start_time: f64, end_time: f64, text: &str) -> Result<VttCue, JsValue>;

    #[cfg(feature = "DocumentFragment")]
    # [ wasm_bindgen ( method , structural , js_class = "VTTCue" , js_name = getCueAsHTML ) ]
    ///The `getCueAsHTML()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/getCueAsHTML)
    ///
    ///*This API requires the following crate features to be activated: `DocumentFragment`, `VttCue`*
    pub fn get_cue_as_html(this: &VttCue) -> DocumentFragment;

}
