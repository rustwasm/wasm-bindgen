use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLImageElement , typescript_name = HTMLImageElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlImageElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub type HtmlImageElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = alt ) ]
    #[doc = "Getter for the `alt` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/alt)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn alt(this: &HtmlImageElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = alt ) ]
    #[doc = "Setter for the `alt` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/alt)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn set_alt(this: &HtmlImageElement, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = src ) ]
    #[doc = "Getter for the `src` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/src)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn src(this: &HtmlImageElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = src ) ]
    #[doc = "Setter for the `src` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/src)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn set_src(this: &HtmlImageElement, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = srcset ) ]
    #[doc = "Getter for the `srcset` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/srcset)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn srcset(this: &HtmlImageElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = srcset ) ]
    #[doc = "Setter for the `srcset` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/srcset)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn set_srcset(this: &HtmlImageElement, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = crossOrigin ) ]
    #[doc = "Getter for the `crossOrigin` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/crossOrigin)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn cross_origin(this: &HtmlImageElement) -> Option<String>;
    # [ wasm_bindgen ( structural , method , setter , js_name = crossOrigin ) ]
    #[doc = "Setter for the `crossOrigin` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/crossOrigin)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn set_cross_origin(this: &HtmlImageElement, value: Option<String>);
    # [ wasm_bindgen ( structural , method , getter , js_name = useMap ) ]
    #[doc = "Getter for the `useMap` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/useMap)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn use_map(this: &HtmlImageElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = useMap ) ]
    #[doc = "Setter for the `useMap` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/useMap)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn set_use_map(this: &HtmlImageElement, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = referrerPolicy ) ]
    #[doc = "Getter for the `referrerPolicy` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/referrerPolicy)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn referrer_policy(this: &HtmlImageElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = referrerPolicy ) ]
    #[doc = "Setter for the `referrerPolicy` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/referrerPolicy)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn set_referrer_policy(this: &HtmlImageElement, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = isMap ) ]
    #[doc = "Getter for the `isMap` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/isMap)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn is_map(this: &HtmlImageElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = isMap ) ]
    #[doc = "Setter for the `isMap` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/isMap)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn set_is_map(this: &HtmlImageElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = width ) ]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/width)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn width(this: &HtmlImageElement) -> u32;
    # [ wasm_bindgen ( structural , method , setter , js_name = width ) ]
    #[doc = "Setter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/width)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn set_width(this: &HtmlImageElement, value: u32);
    # [ wasm_bindgen ( structural , method , getter , js_name = height ) ]
    #[doc = "Getter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/height)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn height(this: &HtmlImageElement) -> u32;
    # [ wasm_bindgen ( structural , method , setter , js_name = height ) ]
    #[doc = "Setter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/height)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn set_height(this: &HtmlImageElement, value: u32);
    # [ wasm_bindgen ( structural , method , getter , js_name = decoding ) ]
    #[doc = "Getter for the `decoding` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/decoding)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn decoding(this: &HtmlImageElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = decoding ) ]
    #[doc = "Setter for the `decoding` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/decoding)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn set_decoding(this: &HtmlImageElement, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = naturalWidth ) ]
    #[doc = "Getter for the `naturalWidth` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/naturalWidth)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn natural_width(this: &HtmlImageElement) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_name = naturalHeight ) ]
    #[doc = "Getter for the `naturalHeight` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/naturalHeight)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn natural_height(this: &HtmlImageElement) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_name = complete ) ]
    #[doc = "Getter for the `complete` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/complete)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn complete(this: &HtmlImageElement) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/name)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn name(this: &HtmlImageElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = name ) ]
    #[doc = "Setter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/name)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn set_name(this: &HtmlImageElement, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = align ) ]
    #[doc = "Getter for the `align` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/align)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn align(this: &HtmlImageElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = align ) ]
    #[doc = "Setter for the `align` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/align)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn set_align(this: &HtmlImageElement, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = hspace ) ]
    #[doc = "Getter for the `hspace` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/hspace)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn hspace(this: &HtmlImageElement) -> u32;
    # [ wasm_bindgen ( structural , method , setter , js_name = hspace ) ]
    #[doc = "Setter for the `hspace` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/hspace)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn set_hspace(this: &HtmlImageElement, value: u32);
    # [ wasm_bindgen ( structural , method , getter , js_name = vspace ) ]
    #[doc = "Getter for the `vspace` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/vspace)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn vspace(this: &HtmlImageElement) -> u32;
    # [ wasm_bindgen ( structural , method , setter , js_name = vspace ) ]
    #[doc = "Setter for the `vspace` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/vspace)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn set_vspace(this: &HtmlImageElement, value: u32);
    # [ wasm_bindgen ( structural , method , getter , js_name = longDesc ) ]
    #[doc = "Getter for the `longDesc` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/longDesc)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn long_desc(this: &HtmlImageElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = longDesc ) ]
    #[doc = "Setter for the `longDesc` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/longDesc)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn set_long_desc(this: &HtmlImageElement, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = border ) ]
    #[doc = "Getter for the `border` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/border)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn border(this: &HtmlImageElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = border ) ]
    #[doc = "Setter for the `border` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/border)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn set_border(this: &HtmlImageElement, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = sizes ) ]
    #[doc = "Getter for the `sizes` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/sizes)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn sizes(this: &HtmlImageElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = sizes ) ]
    #[doc = "Setter for the `sizes` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/sizes)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn set_sizes(this: &HtmlImageElement, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = currentSrc ) ]
    #[doc = "Getter for the `currentSrc` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/currentSrc)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn current_src(this: &HtmlImageElement) -> String;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new HtmlImageElement(..)` constructor, creating a new instance of `HtmlImageElement`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/HTMLImageElement)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn new(this: &HtmlImageElement) -> Result<HtmlImageElement, JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new HtmlImageElement(..)` constructor, creating a new instance of `HtmlImageElement`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/HTMLImageElement)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn new_with_width(this: &HtmlImageElement, width: u32)
        -> Result<HtmlImageElement, JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new HtmlImageElement(..)` constructor, creating a new instance of `HtmlImageElement`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/HTMLImageElement)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn new_with_width_and_height(
        this: &HtmlImageElement,
        width: u32,
        height: u32,
    ) -> Result<HtmlImageElement, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = decode ) ]
    #[doc = "The `decode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/decode)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    pub fn decode(this: &HtmlImageElement) -> ::js_sys::Promise;
}
