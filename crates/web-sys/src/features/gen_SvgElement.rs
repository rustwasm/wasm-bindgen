use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGElement , typescript_name = SVGElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub type SvgElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = id ) ]
    ///Getter for the `id` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/id)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn id(this: &SvgElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = id ) ]
    ///Setter for the `id` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/id)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_id(this: &SvgElement, value: &str);

    #[cfg(feature = "SvgAnimatedString")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = className ) ]
    ///Getter for the `className` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/className)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgElement`*
    pub fn class_name(this: &SvgElement) -> SvgAnimatedString;

    #[cfg(feature = "DomStringMap")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = dataset ) ]
    ///Getter for the `dataset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/dataset)
    ///
    ///*This API requires the following crate features to be activated: `DomStringMap`, `SvgElement`*
    pub fn dataset(this: &SvgElement) -> DomStringMap;

    #[cfg(feature = "CssStyleDeclaration")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = style ) ]
    ///Getter for the `style` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/style)
    ///
    ///*This API requires the following crate features to be activated: `CssStyleDeclaration`, `SvgElement`*
    pub fn style(this: &SvgElement) -> CssStyleDeclaration;

    #[cfg(feature = "SvgsvgElement")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = ownerSVGElement ) ]
    ///Getter for the `ownerSVGElement` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ownerSVGElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`, `SvgsvgElement`*
    pub fn owner_svg_element(this: &SvgElement) -> Option<SvgsvgElement>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = viewportElement ) ]
    ///Getter for the `viewportElement` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/viewportElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn viewport_element(this: &SvgElement) -> Option<SvgElement>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = tabIndex ) ]
    ///Getter for the `tabIndex` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/tabIndex)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn tab_index(this: &SvgElement) -> i32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = tabIndex ) ]
    ///Setter for the `tabIndex` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/tabIndex)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_tab_index(this: &SvgElement, value: i32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = oncopy ) ]
    ///Getter for the `oncopy` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncopy)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn oncopy(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = oncopy ) ]
    ///Setter for the `oncopy` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncopy)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_oncopy(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = oncut ) ]
    ///Getter for the `oncut` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncut)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn oncut(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = oncut ) ]
    ///Setter for the `oncut` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncut)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_oncut(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onpaste ) ]
    ///Getter for the `onpaste` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpaste)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onpaste(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onpaste ) ]
    ///Setter for the `onpaste` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpaste)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onpaste(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onabort ) ]
    ///Getter for the `onabort` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onabort)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onabort(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onabort ) ]
    ///Setter for the `onabort` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onabort)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onabort(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onblur ) ]
    ///Getter for the `onblur` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onblur)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onblur(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onblur ) ]
    ///Setter for the `onblur` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onblur)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onblur(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onfocus ) ]
    ///Getter for the `onfocus` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onfocus)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onfocus(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onfocus ) ]
    ///Setter for the `onfocus` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onfocus)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onfocus(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onauxclick ) ]
    ///Getter for the `onauxclick` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onauxclick)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onauxclick(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onauxclick ) ]
    ///Setter for the `onauxclick` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onauxclick)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onauxclick(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = oncanplay ) ]
    ///Getter for the `oncanplay` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncanplay)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn oncanplay(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = oncanplay ) ]
    ///Setter for the `oncanplay` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncanplay)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_oncanplay(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = oncanplaythrough ) ]
    ///Getter for the `oncanplaythrough` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncanplaythrough)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn oncanplaythrough(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = oncanplaythrough ) ]
    ///Setter for the `oncanplaythrough` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncanplaythrough)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_oncanplaythrough(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onchange ) ]
    ///Getter for the `onchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onchange)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onchange(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onchange ) ]
    ///Setter for the `onchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onchange)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onchange(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onclick ) ]
    ///Getter for the `onclick` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onclick)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onclick(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onclick ) ]
    ///Setter for the `onclick` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onclick)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onclick(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onclose ) ]
    ///Getter for the `onclose` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onclose)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onclose(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onclose ) ]
    ///Setter for the `onclose` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onclose)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onclose(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = oncontextmenu ) ]
    ///Getter for the `oncontextmenu` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncontextmenu)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn oncontextmenu(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = oncontextmenu ) ]
    ///Setter for the `oncontextmenu` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncontextmenu)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_oncontextmenu(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = ondblclick ) ]
    ///Getter for the `ondblclick` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondblclick)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn ondblclick(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = ondblclick ) ]
    ///Setter for the `ondblclick` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondblclick)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_ondblclick(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = ondrag ) ]
    ///Getter for the `ondrag` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondrag)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn ondrag(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = ondrag ) ]
    ///Setter for the `ondrag` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondrag)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_ondrag(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = ondragend ) ]
    ///Getter for the `ondragend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragend)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn ondragend(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = ondragend ) ]
    ///Setter for the `ondragend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragend)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_ondragend(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = ondragenter ) ]
    ///Getter for the `ondragenter` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragenter)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn ondragenter(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = ondragenter ) ]
    ///Setter for the `ondragenter` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragenter)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_ondragenter(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = ondragexit ) ]
    ///Getter for the `ondragexit` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragexit)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn ondragexit(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = ondragexit ) ]
    ///Setter for the `ondragexit` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragexit)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_ondragexit(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = ondragleave ) ]
    ///Getter for the `ondragleave` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragleave)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn ondragleave(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = ondragleave ) ]
    ///Setter for the `ondragleave` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragleave)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_ondragleave(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = ondragover ) ]
    ///Getter for the `ondragover` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragover)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn ondragover(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = ondragover ) ]
    ///Setter for the `ondragover` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragover)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_ondragover(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = ondragstart ) ]
    ///Getter for the `ondragstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragstart)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn ondragstart(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = ondragstart ) ]
    ///Setter for the `ondragstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragstart)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_ondragstart(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = ondrop ) ]
    ///Getter for the `ondrop` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondrop)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn ondrop(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = ondrop ) ]
    ///Setter for the `ondrop` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondrop)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_ondrop(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = ondurationchange ) ]
    ///Getter for the `ondurationchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondurationchange)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn ondurationchange(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = ondurationchange ) ]
    ///Setter for the `ondurationchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondurationchange)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_ondurationchange(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onemptied ) ]
    ///Getter for the `onemptied` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onemptied)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onemptied(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onemptied ) ]
    ///Setter for the `onemptied` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onemptied)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onemptied(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onended ) ]
    ///Getter for the `onended` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onended)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onended(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onended ) ]
    ///Setter for the `onended` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onended)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onended(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = oninput ) ]
    ///Getter for the `oninput` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oninput)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn oninput(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = oninput ) ]
    ///Setter for the `oninput` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oninput)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_oninput(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = oninvalid ) ]
    ///Getter for the `oninvalid` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oninvalid)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn oninvalid(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = oninvalid ) ]
    ///Setter for the `oninvalid` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oninvalid)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_oninvalid(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onkeydown ) ]
    ///Getter for the `onkeydown` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onkeydown)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onkeydown(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onkeydown ) ]
    ///Setter for the `onkeydown` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onkeydown)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onkeydown(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onkeypress ) ]
    ///Getter for the `onkeypress` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onkeypress)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onkeypress(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onkeypress ) ]
    ///Setter for the `onkeypress` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onkeypress)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onkeypress(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onkeyup ) ]
    ///Getter for the `onkeyup` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onkeyup)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onkeyup(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onkeyup ) ]
    ///Setter for the `onkeyup` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onkeyup)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onkeyup(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onload ) ]
    ///Getter for the `onload` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onload)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onload(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onload ) ]
    ///Setter for the `onload` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onload)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onload(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onloadeddata ) ]
    ///Getter for the `onloadeddata` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadeddata)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onloadeddata(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onloadeddata ) ]
    ///Setter for the `onloadeddata` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadeddata)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onloadeddata(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onloadedmetadata ) ]
    ///Getter for the `onloadedmetadata` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadedmetadata)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onloadedmetadata(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onloadedmetadata ) ]
    ///Setter for the `onloadedmetadata` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadedmetadata)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onloadedmetadata(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onloadend ) ]
    ///Getter for the `onloadend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadend)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onloadend(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onloadend ) ]
    ///Setter for the `onloadend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadend)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onloadend(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onloadstart ) ]
    ///Getter for the `onloadstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadstart)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onloadstart(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onloadstart ) ]
    ///Setter for the `onloadstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadstart)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onloadstart(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onmousedown ) ]
    ///Getter for the `onmousedown` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmousedown)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onmousedown(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onmousedown ) ]
    ///Setter for the `onmousedown` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmousedown)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onmousedown(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onmouseenter ) ]
    ///Getter for the `onmouseenter` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseenter)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onmouseenter(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onmouseenter ) ]
    ///Setter for the `onmouseenter` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseenter)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onmouseenter(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onmouseleave ) ]
    ///Getter for the `onmouseleave` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseleave)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onmouseleave(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onmouseleave ) ]
    ///Setter for the `onmouseleave` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseleave)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onmouseleave(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onmousemove ) ]
    ///Getter for the `onmousemove` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmousemove)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onmousemove(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onmousemove ) ]
    ///Setter for the `onmousemove` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmousemove)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onmousemove(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onmouseout ) ]
    ///Getter for the `onmouseout` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseout)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onmouseout(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onmouseout ) ]
    ///Setter for the `onmouseout` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseout)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onmouseout(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onmouseover ) ]
    ///Getter for the `onmouseover` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseover)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onmouseover(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onmouseover ) ]
    ///Setter for the `onmouseover` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseover)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onmouseover(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onmouseup ) ]
    ///Getter for the `onmouseup` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseup)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onmouseup(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onmouseup ) ]
    ///Setter for the `onmouseup` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseup)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onmouseup(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onwheel ) ]
    ///Getter for the `onwheel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwheel)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onwheel(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onwheel ) ]
    ///Setter for the `onwheel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwheel)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onwheel(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onpause ) ]
    ///Getter for the `onpause` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpause)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onpause(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onpause ) ]
    ///Setter for the `onpause` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpause)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onpause(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onplay ) ]
    ///Getter for the `onplay` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onplay)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onplay(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onplay ) ]
    ///Setter for the `onplay` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onplay)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onplay(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onplaying ) ]
    ///Getter for the `onplaying` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onplaying)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onplaying(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onplaying ) ]
    ///Setter for the `onplaying` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onplaying)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onplaying(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onprogress ) ]
    ///Getter for the `onprogress` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onprogress)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onprogress(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onprogress ) ]
    ///Setter for the `onprogress` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onprogress)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onprogress(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onratechange ) ]
    ///Getter for the `onratechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onratechange)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onratechange(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onratechange ) ]
    ///Setter for the `onratechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onratechange)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onratechange(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onreset ) ]
    ///Getter for the `onreset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onreset)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onreset(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onreset ) ]
    ///Setter for the `onreset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onreset)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onreset(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onresize ) ]
    ///Getter for the `onresize` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onresize)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onresize(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onresize ) ]
    ///Setter for the `onresize` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onresize)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onresize(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onscroll ) ]
    ///Getter for the `onscroll` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onscroll)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onscroll(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onscroll ) ]
    ///Setter for the `onscroll` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onscroll)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onscroll(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onseeked ) ]
    ///Getter for the `onseeked` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onseeked)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onseeked(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onseeked ) ]
    ///Setter for the `onseeked` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onseeked)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onseeked(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onseeking ) ]
    ///Getter for the `onseeking` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onseeking)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onseeking(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onseeking ) ]
    ///Setter for the `onseeking` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onseeking)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onseeking(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onselect ) ]
    ///Getter for the `onselect` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onselect)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onselect(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onselect ) ]
    ///Setter for the `onselect` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onselect)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onselect(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onshow ) ]
    ///Getter for the `onshow` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onshow)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onshow(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onshow ) ]
    ///Setter for the `onshow` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onshow)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onshow(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onstalled ) ]
    ///Getter for the `onstalled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onstalled)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onstalled(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onstalled ) ]
    ///Setter for the `onstalled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onstalled)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onstalled(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onsubmit ) ]
    ///Getter for the `onsubmit` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onsubmit)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onsubmit(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onsubmit ) ]
    ///Setter for the `onsubmit` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onsubmit)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onsubmit(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onsuspend ) ]
    ///Getter for the `onsuspend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onsuspend)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onsuspend(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onsuspend ) ]
    ///Setter for the `onsuspend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onsuspend)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onsuspend(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = ontimeupdate ) ]
    ///Getter for the `ontimeupdate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontimeupdate)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn ontimeupdate(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = ontimeupdate ) ]
    ///Setter for the `ontimeupdate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontimeupdate)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_ontimeupdate(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onvolumechange ) ]
    ///Getter for the `onvolumechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onvolumechange)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onvolumechange(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onvolumechange ) ]
    ///Setter for the `onvolumechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onvolumechange)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onvolumechange(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onwaiting ) ]
    ///Getter for the `onwaiting` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwaiting)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onwaiting(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onwaiting ) ]
    ///Setter for the `onwaiting` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwaiting)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onwaiting(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onselectstart ) ]
    ///Getter for the `onselectstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onselectstart)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onselectstart(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onselectstart ) ]
    ///Setter for the `onselectstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onselectstart)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onselectstart(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = ontoggle ) ]
    ///Getter for the `ontoggle` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontoggle)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn ontoggle(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = ontoggle ) ]
    ///Setter for the `ontoggle` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontoggle)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_ontoggle(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onpointercancel ) ]
    ///Getter for the `onpointercancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointercancel)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onpointercancel(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onpointercancel ) ]
    ///Setter for the `onpointercancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointercancel)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onpointercancel(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onpointerdown ) ]
    ///Getter for the `onpointerdown` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerdown)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onpointerdown(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onpointerdown ) ]
    ///Setter for the `onpointerdown` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerdown)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onpointerdown(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onpointerup ) ]
    ///Getter for the `onpointerup` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerup)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onpointerup(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onpointerup ) ]
    ///Setter for the `onpointerup` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerup)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onpointerup(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onpointermove ) ]
    ///Getter for the `onpointermove` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointermove)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onpointermove(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onpointermove ) ]
    ///Setter for the `onpointermove` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointermove)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onpointermove(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onpointerout ) ]
    ///Getter for the `onpointerout` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerout)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onpointerout(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onpointerout ) ]
    ///Setter for the `onpointerout` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerout)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onpointerout(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onpointerover ) ]
    ///Getter for the `onpointerover` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerover)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onpointerover(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onpointerover ) ]
    ///Setter for the `onpointerover` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerover)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onpointerover(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onpointerenter ) ]
    ///Getter for the `onpointerenter` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerenter)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onpointerenter(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onpointerenter ) ]
    ///Setter for the `onpointerenter` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerenter)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onpointerenter(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onpointerleave ) ]
    ///Getter for the `onpointerleave` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerleave)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onpointerleave(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onpointerleave ) ]
    ///Setter for the `onpointerleave` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerleave)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onpointerleave(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = ongotpointercapture ) ]
    ///Getter for the `ongotpointercapture` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ongotpointercapture)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn ongotpointercapture(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = ongotpointercapture ) ]
    ///Setter for the `ongotpointercapture` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ongotpointercapture)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_ongotpointercapture(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onlostpointercapture ) ]
    ///Getter for the `onlostpointercapture` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onlostpointercapture)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onlostpointercapture(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onlostpointercapture ) ]
    ///Setter for the `onlostpointercapture` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onlostpointercapture)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onlostpointercapture(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onanimationcancel ) ]
    ///Getter for the `onanimationcancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationcancel)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onanimationcancel(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onanimationcancel ) ]
    ///Setter for the `onanimationcancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationcancel)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onanimationcancel(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onanimationend ) ]
    ///Getter for the `onanimationend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationend)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onanimationend(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onanimationend ) ]
    ///Setter for the `onanimationend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationend)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onanimationend(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onanimationiteration ) ]
    ///Getter for the `onanimationiteration` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationiteration)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onanimationiteration(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onanimationiteration ) ]
    ///Setter for the `onanimationiteration` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationiteration)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onanimationiteration(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onanimationstart ) ]
    ///Getter for the `onanimationstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationstart)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onanimationstart(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onanimationstart ) ]
    ///Setter for the `onanimationstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationstart)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onanimationstart(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = ontransitioncancel ) ]
    ///Getter for the `ontransitioncancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitioncancel)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn ontransitioncancel(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = ontransitioncancel ) ]
    ///Setter for the `ontransitioncancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitioncancel)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_ontransitioncancel(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = ontransitionend ) ]
    ///Getter for the `ontransitionend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitionend)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn ontransitionend(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = ontransitionend ) ]
    ///Setter for the `ontransitionend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitionend)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_ontransitionend(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = ontransitionrun ) ]
    ///Getter for the `ontransitionrun` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitionrun)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn ontransitionrun(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = ontransitionrun ) ]
    ///Setter for the `ontransitionrun` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitionrun)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_ontransitionrun(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = ontransitionstart ) ]
    ///Getter for the `ontransitionstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitionstart)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn ontransitionstart(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = ontransitionstart ) ]
    ///Setter for the `ontransitionstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitionstart)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_ontransitionstart(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onwebkitanimationend ) ]
    ///Getter for the `onwebkitanimationend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkitanimationend)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onwebkitanimationend(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onwebkitanimationend ) ]
    ///Setter for the `onwebkitanimationend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkitanimationend)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onwebkitanimationend(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onwebkitanimationiteration ) ]
    ///Getter for the `onwebkitanimationiteration` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkitanimationiteration)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onwebkitanimationiteration(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onwebkitanimationiteration ) ]
    ///Setter for the `onwebkitanimationiteration` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkitanimationiteration)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onwebkitanimationiteration(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onwebkitanimationstart ) ]
    ///Getter for the `onwebkitanimationstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkitanimationstart)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onwebkitanimationstart(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onwebkitanimationstart ) ]
    ///Setter for the `onwebkitanimationstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkitanimationstart)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onwebkitanimationstart(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onwebkittransitionend ) ]
    ///Getter for the `onwebkittransitionend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkittransitionend)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onwebkittransitionend(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onwebkittransitionend ) ]
    ///Setter for the `onwebkittransitionend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkittransitionend)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onwebkittransitionend(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = onerror ) ]
    ///Getter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onerror)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn onerror(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = onerror ) ]
    ///Setter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onerror)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_onerror(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = ontouchstart ) ]
    ///Getter for the `ontouchstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchstart)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn ontouchstart(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = ontouchstart ) ]
    ///Setter for the `ontouchstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchstart)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_ontouchstart(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = ontouchend ) ]
    ///Getter for the `ontouchend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchend)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn ontouchend(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = ontouchend ) ]
    ///Setter for the `ontouchend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchend)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_ontouchend(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = ontouchmove ) ]
    ///Getter for the `ontouchmove` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchmove)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn ontouchmove(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = ontouchmove ) ]
    ///Setter for the `ontouchmove` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchmove)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_ontouchmove(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGElement" , js_name = ontouchcancel ) ]
    ///Getter for the `ontouchcancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchcancel)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn ontouchcancel(this: &SvgElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGElement" , js_name = ontouchcancel ) ]
    ///Setter for the `ontouchcancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchcancel)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn set_ontouchcancel(this: &SvgElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGElement" , js_name = blur ) ]
    ///The `blur()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/blur)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn blur(this: &SvgElement) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGElement" , js_name = focus ) ]
    ///The `focus()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/focus)
    ///
    ///*This API requires the following crate features to be activated: `SvgElement`*
    pub fn focus(this: &SvgElement) -> Result<(), JsValue>;

}
