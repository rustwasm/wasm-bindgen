use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGElement , typescript_name = SVGElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub type SvgElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = id ) ]
    #[doc = "Getter for the `id` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/id)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn id(this: &SvgElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = id ) ]
    #[doc = "Setter for the `id` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/id)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_id(this: &SvgElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = className ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `className` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/className)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgElement`*"]
    pub fn class_name(this: &SvgElement) -> SvgAnimatedString;
    # [ wasm_bindgen ( structural , method , getter , js_name = dataset ) ]
    #[cfg(feature = "DomStringMap")]
    #[doc = "Getter for the `dataset` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/dataset)\n\n*This API requires the following crate features to be activated: `DomStringMap`, `SvgElement`*"]
    pub fn dataset(this: &SvgElement) -> DomStringMap;
    # [ wasm_bindgen ( structural , method , getter , js_name = style ) ]
    #[cfg(feature = "CssStyleDeclaration")]
    #[doc = "Getter for the `style` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/style)\n\n*This API requires the following crate features to be activated: `CssStyleDeclaration`, `SvgElement`*"]
    pub fn style(this: &SvgElement) -> CssStyleDeclaration;
    # [ wasm_bindgen ( structural , method , getter , js_name = ownerSVGElement ) ]
    #[cfg(feature = "SvgsvgElement")]
    #[doc = "Getter for the `ownerSVGElement` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ownerSVGElement)\n\n*This API requires the following crate features to be activated: `SvgElement`, `SvgsvgElement`*"]
    pub fn owner_svg_element(this: &SvgElement) -> Option<SvgsvgElement>;
    # [ wasm_bindgen ( structural , method , getter , js_name = viewportElement ) ]
    #[doc = "Getter for the `viewportElement` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/viewportElement)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn viewport_element(this: &SvgElement) -> Option<SvgElement>;
    # [ wasm_bindgen ( structural , method , getter , js_name = tabIndex ) ]
    #[doc = "Getter for the `tabIndex` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/tabIndex)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn tab_index(this: &SvgElement) -> i32;
    # [ wasm_bindgen ( structural , method , setter , js_name = tabIndex ) ]
    #[doc = "Setter for the `tabIndex` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/tabIndex)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_tab_index(this: &SvgElement, value: i32);
    # [ wasm_bindgen ( structural , method , getter , js_name = oncopy ) ]
    #[doc = "Getter for the `oncopy` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncopy)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn oncopy(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = oncopy ) ]
    #[doc = "Setter for the `oncopy` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncopy)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_oncopy(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = oncut ) ]
    #[doc = "Getter for the `oncut` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncut)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn oncut(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = oncut ) ]
    #[doc = "Setter for the `oncut` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncut)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_oncut(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onpaste ) ]
    #[doc = "Getter for the `onpaste` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpaste)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onpaste(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onpaste ) ]
    #[doc = "Setter for the `onpaste` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpaste)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onpaste(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onabort ) ]
    #[doc = "Getter for the `onabort` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onabort)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onabort(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onabort ) ]
    #[doc = "Setter for the `onabort` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onabort)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onabort(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onblur ) ]
    #[doc = "Getter for the `onblur` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onblur)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onblur(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onblur ) ]
    #[doc = "Setter for the `onblur` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onblur)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onblur(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onfocus ) ]
    #[doc = "Getter for the `onfocus` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onfocus)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onfocus(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onfocus ) ]
    #[doc = "Setter for the `onfocus` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onfocus)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onfocus(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onauxclick ) ]
    #[doc = "Getter for the `onauxclick` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onauxclick)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onauxclick(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onauxclick ) ]
    #[doc = "Setter for the `onauxclick` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onauxclick)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onauxclick(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = oncanplay ) ]
    #[doc = "Getter for the `oncanplay` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncanplay)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn oncanplay(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = oncanplay ) ]
    #[doc = "Setter for the `oncanplay` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncanplay)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_oncanplay(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = oncanplaythrough ) ]
    #[doc = "Getter for the `oncanplaythrough` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncanplaythrough)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn oncanplaythrough(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = oncanplaythrough ) ]
    #[doc = "Setter for the `oncanplaythrough` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncanplaythrough)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_oncanplaythrough(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onchange ) ]
    #[doc = "Getter for the `onchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onchange)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onchange(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onchange ) ]
    #[doc = "Setter for the `onchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onchange)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onchange(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onclick ) ]
    #[doc = "Getter for the `onclick` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onclick)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onclick(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onclick ) ]
    #[doc = "Setter for the `onclick` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onclick)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onclick(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onclose ) ]
    #[doc = "Getter for the `onclose` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onclose)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onclose(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onclose ) ]
    #[doc = "Setter for the `onclose` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onclose)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onclose(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = oncontextmenu ) ]
    #[doc = "Getter for the `oncontextmenu` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncontextmenu)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn oncontextmenu(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = oncontextmenu ) ]
    #[doc = "Setter for the `oncontextmenu` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncontextmenu)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_oncontextmenu(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ondblclick ) ]
    #[doc = "Getter for the `ondblclick` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondblclick)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ondblclick(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ondblclick ) ]
    #[doc = "Setter for the `ondblclick` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondblclick)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ondblclick(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ondrag ) ]
    #[doc = "Getter for the `ondrag` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondrag)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ondrag(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ondrag ) ]
    #[doc = "Setter for the `ondrag` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondrag)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ondrag(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ondragend ) ]
    #[doc = "Getter for the `ondragend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragend)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ondragend(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ondragend ) ]
    #[doc = "Setter for the `ondragend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragend)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ondragend(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ondragenter ) ]
    #[doc = "Getter for the `ondragenter` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragenter)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ondragenter(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ondragenter ) ]
    #[doc = "Setter for the `ondragenter` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragenter)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ondragenter(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ondragexit ) ]
    #[doc = "Getter for the `ondragexit` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragexit)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ondragexit(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ondragexit ) ]
    #[doc = "Setter for the `ondragexit` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragexit)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ondragexit(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ondragleave ) ]
    #[doc = "Getter for the `ondragleave` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragleave)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ondragleave(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ondragleave ) ]
    #[doc = "Setter for the `ondragleave` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragleave)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ondragleave(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ondragover ) ]
    #[doc = "Getter for the `ondragover` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragover)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ondragover(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ondragover ) ]
    #[doc = "Setter for the `ondragover` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragover)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ondragover(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ondragstart ) ]
    #[doc = "Getter for the `ondragstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragstart)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ondragstart(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ondragstart ) ]
    #[doc = "Setter for the `ondragstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragstart)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ondragstart(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ondrop ) ]
    #[doc = "Getter for the `ondrop` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondrop)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ondrop(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ondrop ) ]
    #[doc = "Setter for the `ondrop` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondrop)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ondrop(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ondurationchange ) ]
    #[doc = "Getter for the `ondurationchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondurationchange)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ondurationchange(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ondurationchange ) ]
    #[doc = "Setter for the `ondurationchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondurationchange)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ondurationchange(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onemptied ) ]
    #[doc = "Getter for the `onemptied` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onemptied)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onemptied(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onemptied ) ]
    #[doc = "Setter for the `onemptied` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onemptied)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onemptied(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onended ) ]
    #[doc = "Getter for the `onended` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onended)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onended(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onended ) ]
    #[doc = "Setter for the `onended` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onended)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onended(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = oninput ) ]
    #[doc = "Getter for the `oninput` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oninput)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn oninput(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = oninput ) ]
    #[doc = "Setter for the `oninput` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oninput)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_oninput(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = oninvalid ) ]
    #[doc = "Getter for the `oninvalid` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oninvalid)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn oninvalid(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = oninvalid ) ]
    #[doc = "Setter for the `oninvalid` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oninvalid)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_oninvalid(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onkeydown ) ]
    #[doc = "Getter for the `onkeydown` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onkeydown)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onkeydown(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onkeydown ) ]
    #[doc = "Setter for the `onkeydown` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onkeydown)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onkeydown(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onkeypress ) ]
    #[doc = "Getter for the `onkeypress` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onkeypress)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onkeypress(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onkeypress ) ]
    #[doc = "Setter for the `onkeypress` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onkeypress)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onkeypress(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onkeyup ) ]
    #[doc = "Getter for the `onkeyup` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onkeyup)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onkeyup(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onkeyup ) ]
    #[doc = "Setter for the `onkeyup` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onkeyup)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onkeyup(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onload ) ]
    #[doc = "Getter for the `onload` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onload)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onload(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onload ) ]
    #[doc = "Setter for the `onload` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onload)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onload(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onloadeddata ) ]
    #[doc = "Getter for the `onloadeddata` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadeddata)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onloadeddata(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onloadeddata ) ]
    #[doc = "Setter for the `onloadeddata` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadeddata)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onloadeddata(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onloadedmetadata ) ]
    #[doc = "Getter for the `onloadedmetadata` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadedmetadata)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onloadedmetadata(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onloadedmetadata ) ]
    #[doc = "Setter for the `onloadedmetadata` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadedmetadata)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onloadedmetadata(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onloadend ) ]
    #[doc = "Getter for the `onloadend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadend)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onloadend(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onloadend ) ]
    #[doc = "Setter for the `onloadend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadend)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onloadend(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onloadstart ) ]
    #[doc = "Getter for the `onloadstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadstart)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onloadstart(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onloadstart ) ]
    #[doc = "Setter for the `onloadstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadstart)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onloadstart(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onmousedown ) ]
    #[doc = "Getter for the `onmousedown` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmousedown)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onmousedown(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onmousedown ) ]
    #[doc = "Setter for the `onmousedown` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmousedown)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onmousedown(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onmouseenter ) ]
    #[doc = "Getter for the `onmouseenter` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseenter)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onmouseenter(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onmouseenter ) ]
    #[doc = "Setter for the `onmouseenter` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseenter)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onmouseenter(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onmouseleave ) ]
    #[doc = "Getter for the `onmouseleave` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseleave)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onmouseleave(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onmouseleave ) ]
    #[doc = "Setter for the `onmouseleave` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseleave)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onmouseleave(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onmousemove ) ]
    #[doc = "Getter for the `onmousemove` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmousemove)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onmousemove(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onmousemove ) ]
    #[doc = "Setter for the `onmousemove` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmousemove)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onmousemove(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onmouseout ) ]
    #[doc = "Getter for the `onmouseout` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseout)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onmouseout(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onmouseout ) ]
    #[doc = "Setter for the `onmouseout` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseout)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onmouseout(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onmouseover ) ]
    #[doc = "Getter for the `onmouseover` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseover)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onmouseover(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onmouseover ) ]
    #[doc = "Setter for the `onmouseover` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseover)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onmouseover(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onmouseup ) ]
    #[doc = "Getter for the `onmouseup` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseup)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onmouseup(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onmouseup ) ]
    #[doc = "Setter for the `onmouseup` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseup)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onmouseup(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onwheel ) ]
    #[doc = "Getter for the `onwheel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwheel)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onwheel(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onwheel ) ]
    #[doc = "Setter for the `onwheel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwheel)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onwheel(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onpause ) ]
    #[doc = "Getter for the `onpause` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpause)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onpause(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onpause ) ]
    #[doc = "Setter for the `onpause` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpause)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onpause(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onplay ) ]
    #[doc = "Getter for the `onplay` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onplay)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onplay(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onplay ) ]
    #[doc = "Setter for the `onplay` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onplay)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onplay(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onplaying ) ]
    #[doc = "Getter for the `onplaying` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onplaying)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onplaying(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onplaying ) ]
    #[doc = "Setter for the `onplaying` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onplaying)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onplaying(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onprogress ) ]
    #[doc = "Getter for the `onprogress` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onprogress)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onprogress(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onprogress ) ]
    #[doc = "Setter for the `onprogress` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onprogress)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onprogress(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onratechange ) ]
    #[doc = "Getter for the `onratechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onratechange)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onratechange(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onratechange ) ]
    #[doc = "Setter for the `onratechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onratechange)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onratechange(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onreset ) ]
    #[doc = "Getter for the `onreset` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onreset)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onreset(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onreset ) ]
    #[doc = "Setter for the `onreset` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onreset)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onreset(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onresize ) ]
    #[doc = "Getter for the `onresize` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onresize)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onresize(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onresize ) ]
    #[doc = "Setter for the `onresize` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onresize)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onresize(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onscroll ) ]
    #[doc = "Getter for the `onscroll` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onscroll)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onscroll(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onscroll ) ]
    #[doc = "Setter for the `onscroll` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onscroll)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onscroll(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onseeked ) ]
    #[doc = "Getter for the `onseeked` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onseeked)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onseeked(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onseeked ) ]
    #[doc = "Setter for the `onseeked` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onseeked)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onseeked(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onseeking ) ]
    #[doc = "Getter for the `onseeking` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onseeking)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onseeking(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onseeking ) ]
    #[doc = "Setter for the `onseeking` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onseeking)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onseeking(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onselect ) ]
    #[doc = "Getter for the `onselect` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onselect)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onselect(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onselect ) ]
    #[doc = "Setter for the `onselect` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onselect)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onselect(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onshow ) ]
    #[doc = "Getter for the `onshow` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onshow)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onshow(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onshow ) ]
    #[doc = "Setter for the `onshow` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onshow)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onshow(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onstalled ) ]
    #[doc = "Getter for the `onstalled` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onstalled)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onstalled(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onstalled ) ]
    #[doc = "Setter for the `onstalled` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onstalled)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onstalled(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onsubmit ) ]
    #[doc = "Getter for the `onsubmit` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onsubmit)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onsubmit(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onsubmit ) ]
    #[doc = "Setter for the `onsubmit` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onsubmit)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onsubmit(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onsuspend ) ]
    #[doc = "Getter for the `onsuspend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onsuspend)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onsuspend(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onsuspend ) ]
    #[doc = "Setter for the `onsuspend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onsuspend)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onsuspend(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ontimeupdate ) ]
    #[doc = "Getter for the `ontimeupdate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontimeupdate)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ontimeupdate(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ontimeupdate ) ]
    #[doc = "Setter for the `ontimeupdate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontimeupdate)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ontimeupdate(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onvolumechange ) ]
    #[doc = "Getter for the `onvolumechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onvolumechange)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onvolumechange(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onvolumechange ) ]
    #[doc = "Setter for the `onvolumechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onvolumechange)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onvolumechange(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onwaiting ) ]
    #[doc = "Getter for the `onwaiting` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwaiting)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onwaiting(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onwaiting ) ]
    #[doc = "Setter for the `onwaiting` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwaiting)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onwaiting(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onselectstart ) ]
    #[doc = "Getter for the `onselectstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onselectstart)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onselectstart(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onselectstart ) ]
    #[doc = "Setter for the `onselectstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onselectstart)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onselectstart(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ontoggle ) ]
    #[doc = "Getter for the `ontoggle` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontoggle)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ontoggle(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ontoggle ) ]
    #[doc = "Setter for the `ontoggle` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontoggle)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ontoggle(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onpointercancel ) ]
    #[doc = "Getter for the `onpointercancel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointercancel)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onpointercancel(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onpointercancel ) ]
    #[doc = "Setter for the `onpointercancel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointercancel)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onpointercancel(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onpointerdown ) ]
    #[doc = "Getter for the `onpointerdown` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerdown)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onpointerdown(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onpointerdown ) ]
    #[doc = "Setter for the `onpointerdown` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerdown)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onpointerdown(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onpointerup ) ]
    #[doc = "Getter for the `onpointerup` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerup)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onpointerup(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onpointerup ) ]
    #[doc = "Setter for the `onpointerup` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerup)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onpointerup(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onpointermove ) ]
    #[doc = "Getter for the `onpointermove` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointermove)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onpointermove(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onpointermove ) ]
    #[doc = "Setter for the `onpointermove` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointermove)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onpointermove(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onpointerout ) ]
    #[doc = "Getter for the `onpointerout` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerout)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onpointerout(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onpointerout ) ]
    #[doc = "Setter for the `onpointerout` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerout)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onpointerout(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onpointerover ) ]
    #[doc = "Getter for the `onpointerover` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerover)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onpointerover(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onpointerover ) ]
    #[doc = "Setter for the `onpointerover` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerover)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onpointerover(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onpointerenter ) ]
    #[doc = "Getter for the `onpointerenter` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerenter)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onpointerenter(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onpointerenter ) ]
    #[doc = "Setter for the `onpointerenter` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerenter)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onpointerenter(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onpointerleave ) ]
    #[doc = "Getter for the `onpointerleave` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerleave)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onpointerleave(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onpointerleave ) ]
    #[doc = "Setter for the `onpointerleave` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerleave)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onpointerleave(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ongotpointercapture ) ]
    #[doc = "Getter for the `ongotpointercapture` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ongotpointercapture)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ongotpointercapture(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ongotpointercapture ) ]
    #[doc = "Setter for the `ongotpointercapture` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ongotpointercapture)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ongotpointercapture(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onlostpointercapture ) ]
    #[doc = "Getter for the `onlostpointercapture` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onlostpointercapture)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onlostpointercapture(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onlostpointercapture ) ]
    #[doc = "Setter for the `onlostpointercapture` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onlostpointercapture)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onlostpointercapture(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onanimationcancel ) ]
    #[doc = "Getter for the `onanimationcancel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationcancel)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onanimationcancel(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onanimationcancel ) ]
    #[doc = "Setter for the `onanimationcancel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationcancel)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onanimationcancel(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onanimationend ) ]
    #[doc = "Getter for the `onanimationend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationend)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onanimationend(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onanimationend ) ]
    #[doc = "Setter for the `onanimationend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationend)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onanimationend(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onanimationiteration ) ]
    #[doc = "Getter for the `onanimationiteration` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationiteration)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onanimationiteration(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onanimationiteration ) ]
    #[doc = "Setter for the `onanimationiteration` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationiteration)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onanimationiteration(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onanimationstart ) ]
    #[doc = "Getter for the `onanimationstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationstart)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onanimationstart(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onanimationstart ) ]
    #[doc = "Setter for the `onanimationstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationstart)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onanimationstart(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ontransitioncancel ) ]
    #[doc = "Getter for the `ontransitioncancel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitioncancel)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ontransitioncancel(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ontransitioncancel ) ]
    #[doc = "Setter for the `ontransitioncancel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitioncancel)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ontransitioncancel(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ontransitionend ) ]
    #[doc = "Getter for the `ontransitionend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitionend)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ontransitionend(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ontransitionend ) ]
    #[doc = "Setter for the `ontransitionend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitionend)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ontransitionend(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ontransitionrun ) ]
    #[doc = "Getter for the `ontransitionrun` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitionrun)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ontransitionrun(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ontransitionrun ) ]
    #[doc = "Setter for the `ontransitionrun` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitionrun)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ontransitionrun(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ontransitionstart ) ]
    #[doc = "Getter for the `ontransitionstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitionstart)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ontransitionstart(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ontransitionstart ) ]
    #[doc = "Setter for the `ontransitionstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitionstart)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ontransitionstart(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onwebkitanimationend ) ]
    #[doc = "Getter for the `onwebkitanimationend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkitanimationend)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onwebkitanimationend(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onwebkitanimationend ) ]
    #[doc = "Setter for the `onwebkitanimationend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkitanimationend)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onwebkitanimationend(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onwebkitanimationiteration ) ]
    #[doc = "Getter for the `onwebkitanimationiteration` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkitanimationiteration)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onwebkitanimationiteration(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onwebkitanimationiteration ) ]
    #[doc = "Setter for the `onwebkitanimationiteration` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkitanimationiteration)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onwebkitanimationiteration(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onwebkitanimationstart ) ]
    #[doc = "Getter for the `onwebkitanimationstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkitanimationstart)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onwebkitanimationstart(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onwebkitanimationstart ) ]
    #[doc = "Setter for the `onwebkitanimationstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkitanimationstart)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onwebkitanimationstart(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onwebkittransitionend ) ]
    #[doc = "Getter for the `onwebkittransitionend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkittransitionend)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onwebkittransitionend(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onwebkittransitionend ) ]
    #[doc = "Setter for the `onwebkittransitionend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkittransitionend)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onwebkittransitionend(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onerror ) ]
    #[doc = "Getter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onerror)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onerror(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onerror ) ]
    #[doc = "Setter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onerror)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onerror(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ontouchstart ) ]
    #[doc = "Getter for the `ontouchstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchstart)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ontouchstart(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ontouchstart ) ]
    #[doc = "Setter for the `ontouchstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchstart)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ontouchstart(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ontouchend ) ]
    #[doc = "Getter for the `ontouchend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchend)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ontouchend(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ontouchend ) ]
    #[doc = "Setter for the `ontouchend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchend)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ontouchend(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ontouchmove ) ]
    #[doc = "Getter for the `ontouchmove` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchmove)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ontouchmove(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ontouchmove ) ]
    #[doc = "Setter for the `ontouchmove` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchmove)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ontouchmove(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ontouchcancel ) ]
    #[doc = "Getter for the `ontouchcancel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchcancel)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ontouchcancel(this: &SvgElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ontouchcancel ) ]
    #[doc = "Setter for the `ontouchcancel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchcancel)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ontouchcancel(this: &SvgElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( catch , method , structural , js_name = blur ) ]
    #[doc = "The `blur()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/blur)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn blur(this: &SvgElement) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = focus ) ]
    #[doc = "The `focus()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/focus)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn focus(this: &SvgElement) -> Result<(), JsValue>;
}
