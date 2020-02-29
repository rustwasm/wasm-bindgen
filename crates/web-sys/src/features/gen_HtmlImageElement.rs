use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLImageElement , typescript_type = "HTMLImageElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlImageElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub type HtmlImageElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLImageElement" , js_name = alt ) ]
    ///Getter for the `alt` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/alt)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn alt(this: &HtmlImageElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLImageElement" , js_name = alt ) ]
    ///Setter for the `alt` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/alt)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn set_alt(this: &HtmlImageElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLImageElement" , js_name = src ) ]
    ///Getter for the `src` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/src)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn src(this: &HtmlImageElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLImageElement" , js_name = src ) ]
    ///Setter for the `src` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/src)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn set_src(this: &HtmlImageElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLImageElement" , js_name = srcset ) ]
    ///Getter for the `srcset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/srcset)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn srcset(this: &HtmlImageElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLImageElement" , js_name = srcset ) ]
    ///Setter for the `srcset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/srcset)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn set_srcset(this: &HtmlImageElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLImageElement" , js_name = crossOrigin ) ]
    ///Getter for the `crossOrigin` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/crossOrigin)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn cross_origin(this: &HtmlImageElement) -> Option<String>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLImageElement" , js_name = crossOrigin ) ]
    ///Setter for the `crossOrigin` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/crossOrigin)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn set_cross_origin(this: &HtmlImageElement, value: Option<&str>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLImageElement" , js_name = useMap ) ]
    ///Getter for the `useMap` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/useMap)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn use_map(this: &HtmlImageElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLImageElement" , js_name = useMap ) ]
    ///Setter for the `useMap` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/useMap)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn set_use_map(this: &HtmlImageElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLImageElement" , js_name = referrerPolicy ) ]
    ///Getter for the `referrerPolicy` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/referrerPolicy)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn referrer_policy(this: &HtmlImageElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLImageElement" , js_name = referrerPolicy ) ]
    ///Setter for the `referrerPolicy` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/referrerPolicy)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn set_referrer_policy(this: &HtmlImageElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLImageElement" , js_name = isMap ) ]
    ///Getter for the `isMap` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/isMap)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn is_map(this: &HtmlImageElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLImageElement" , js_name = isMap ) ]
    ///Setter for the `isMap` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/isMap)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn set_is_map(this: &HtmlImageElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLImageElement" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/width)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn width(this: &HtmlImageElement) -> u32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLImageElement" , js_name = width ) ]
    ///Setter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/width)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn set_width(this: &HtmlImageElement, value: u32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLImageElement" , js_name = height ) ]
    ///Getter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/height)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn height(this: &HtmlImageElement) -> u32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLImageElement" , js_name = height ) ]
    ///Setter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/height)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn set_height(this: &HtmlImageElement, value: u32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLImageElement" , js_name = decoding ) ]
    ///Getter for the `decoding` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/decoding)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn decoding(this: &HtmlImageElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLImageElement" , js_name = decoding ) ]
    ///Setter for the `decoding` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/decoding)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn set_decoding(this: &HtmlImageElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLImageElement" , js_name = naturalWidth ) ]
    ///Getter for the `naturalWidth` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/naturalWidth)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn natural_width(this: &HtmlImageElement) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLImageElement" , js_name = naturalHeight ) ]
    ///Getter for the `naturalHeight` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/naturalHeight)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn natural_height(this: &HtmlImageElement) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLImageElement" , js_name = complete ) ]
    ///Getter for the `complete` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/complete)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn complete(this: &HtmlImageElement) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLImageElement" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn name(this: &HtmlImageElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLImageElement" , js_name = name ) ]
    ///Setter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn set_name(this: &HtmlImageElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLImageElement" , js_name = align ) ]
    ///Getter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/align)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn align(this: &HtmlImageElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLImageElement" , js_name = align ) ]
    ///Setter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/align)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn set_align(this: &HtmlImageElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLImageElement" , js_name = hspace ) ]
    ///Getter for the `hspace` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/hspace)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn hspace(this: &HtmlImageElement) -> u32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLImageElement" , js_name = hspace ) ]
    ///Setter for the `hspace` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/hspace)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn set_hspace(this: &HtmlImageElement, value: u32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLImageElement" , js_name = vspace ) ]
    ///Getter for the `vspace` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/vspace)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn vspace(this: &HtmlImageElement) -> u32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLImageElement" , js_name = vspace ) ]
    ///Setter for the `vspace` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/vspace)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn set_vspace(this: &HtmlImageElement, value: u32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLImageElement" , js_name = longDesc ) ]
    ///Getter for the `longDesc` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/longDesc)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn long_desc(this: &HtmlImageElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLImageElement" , js_name = longDesc ) ]
    ///Setter for the `longDesc` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/longDesc)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn set_long_desc(this: &HtmlImageElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLImageElement" , js_name = border ) ]
    ///Getter for the `border` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/border)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn border(this: &HtmlImageElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLImageElement" , js_name = border ) ]
    ///Setter for the `border` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/border)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn set_border(this: &HtmlImageElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLImageElement" , js_name = sizes ) ]
    ///Getter for the `sizes` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/sizes)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn sizes(this: &HtmlImageElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLImageElement" , js_name = sizes ) ]
    ///Setter for the `sizes` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/sizes)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn set_sizes(this: &HtmlImageElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLImageElement" , js_name = currentSrc ) ]
    ///Getter for the `currentSrc` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/currentSrc)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn current_src(this: &HtmlImageElement) -> String;

    #[wasm_bindgen(catch, constructor, js_class = "HTMLImageElement")]
    ///The `new HtmlImageElement(..)` constructor, creating a new instance of `HtmlImageElement`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/HTMLImageElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn new() -> Result<HtmlImageElement, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "HTMLImageElement")]
    ///The `new HtmlImageElement(..)` constructor, creating a new instance of `HtmlImageElement`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/HTMLImageElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn new_with_width(width: u32) -> Result<HtmlImageElement, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "HTMLImageElement")]
    ///The `new HtmlImageElement(..)` constructor, creating a new instance of `HtmlImageElement`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/HTMLImageElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn new_with_width_and_height(width: u32, height: u32) -> Result<HtmlImageElement, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLImageElement" , js_name = decode ) ]
    ///The `decode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/decode)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`*
    pub fn decode(this: &HtmlImageElement) -> ::js_sys::Promise;

}
