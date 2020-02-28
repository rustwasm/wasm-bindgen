use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = DOMQuad , typescript_name = DOMQuad ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomQuad` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad)\n\n*This API requires the following crate features to be activated: `DomQuad`*"]
    pub type DomQuad;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMQuad" , js_name = p1 ) ]
    #[cfg(feature = "DomPoint")]
    #[doc = "Getter for the `p1` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/p1)\n\n*This API requires the following crate features to be activated: `DomPoint`, `DomQuad`*"]
    pub fn p1(this: &DomQuad) -> DomPoint;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMQuad" , js_name = p2 ) ]
    #[cfg(feature = "DomPoint")]
    #[doc = "Getter for the `p2` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/p2)\n\n*This API requires the following crate features to be activated: `DomPoint`, `DomQuad`*"]
    pub fn p2(this: &DomQuad) -> DomPoint;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMQuad" , js_name = p3 ) ]
    #[cfg(feature = "DomPoint")]
    #[doc = "Getter for the `p3` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/p3)\n\n*This API requires the following crate features to be activated: `DomPoint`, `DomQuad`*"]
    pub fn p3(this: &DomQuad) -> DomPoint;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMQuad" , js_name = p4 ) ]
    #[cfg(feature = "DomPoint")]
    #[doc = "Getter for the `p4` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/p4)\n\n*This API requires the following crate features to be activated: `DomPoint`, `DomQuad`*"]
    pub fn p4(this: &DomQuad) -> DomPoint;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMQuad" , js_name = bounds ) ]
    #[cfg(feature = "DomRectReadOnly")]
    #[doc = "Getter for the `bounds` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/bounds)\n\n*This API requires the following crate features to be activated: `DomQuad`, `DomRectReadOnly`*"]
    pub fn bounds(this: &DomQuad) -> DomRectReadOnly;
    #[wasm_bindgen(catch, js_class = "DOMQuad", constructor)]
    #[doc = "The `new DomQuad(..)` constructor, creating a new instance of `DomQuad`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/DOMQuad)\n\n*This API requires the following crate features to be activated: `DomQuad`*"]
    pub fn new(this: &DomQuad) -> Result<DomQuad, JsValue>;
    #[cfg(feature = "DomPointInit")]
    #[wasm_bindgen(catch, js_class = "DOMQuad", constructor)]
    #[doc = "The `new DomQuad(..)` constructor, creating a new instance of `DomQuad`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/DOMQuad)\n\n*This API requires the following crate features to be activated: `DomPointInit`, `DomQuad`*"]
    pub fn new_with_dom_point_init(this: &DomQuad, p1: &DomPointInit) -> Result<DomQuad, JsValue>;
    #[cfg(feature = "DomPointInit")]
    #[wasm_bindgen(catch, js_class = "DOMQuad", constructor)]
    #[doc = "The `new DomQuad(..)` constructor, creating a new instance of `DomQuad`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/DOMQuad)\n\n*This API requires the following crate features to be activated: `DomPointInit`, `DomQuad`*"]
    pub fn new_with_dom_point_init_and_p2(
        this: &DomQuad,
        p1: &DomPointInit,
        p2: &DomPointInit,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(feature = "DomPointInit")]
    #[wasm_bindgen(catch, js_class = "DOMQuad", constructor)]
    #[doc = "The `new DomQuad(..)` constructor, creating a new instance of `DomQuad`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/DOMQuad)\n\n*This API requires the following crate features to be activated: `DomPointInit`, `DomQuad`*"]
    pub fn new_with_dom_point_init_and_p2_and_p3(
        this: &DomQuad,
        p1: &DomPointInit,
        p2: &DomPointInit,
        p3: &DomPointInit,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(feature = "DomPointInit")]
    #[wasm_bindgen(catch, js_class = "DOMQuad", constructor)]
    #[doc = "The `new DomQuad(..)` constructor, creating a new instance of `DomQuad`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/DOMQuad)\n\n*This API requires the following crate features to be activated: `DomPointInit`, `DomQuad`*"]
    pub fn new_with_dom_point_init_and_p2_and_p3_and_p4(
        this: &DomQuad,
        p1: &DomPointInit,
        p2: &DomPointInit,
        p3: &DomPointInit,
        p4: &DomPointInit,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(feature = "DomRectReadOnly")]
    #[wasm_bindgen(catch, js_class = "DOMQuad", constructor)]
    #[doc = "The `new DomQuad(..)` constructor, creating a new instance of `DomQuad`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/DOMQuad)\n\n*This API requires the following crate features to be activated: `DomQuad`, `DomRectReadOnly`*"]
    pub fn new_with_rect(this: &DomQuad, rect: &DomRectReadOnly) -> Result<DomQuad, JsValue>;
    #[cfg(feature = "DomRectReadOnly")]
    # [ wasm_bindgen ( method , structural , js_class = "DOMQuad" , js_name = getBounds ) ]
    #[doc = "The `getBounds()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/getBounds)\n\n*This API requires the following crate features to be activated: `DomQuad`, `DomRectReadOnly`*"]
    pub fn get_bounds(this: &DomQuad) -> DomRectReadOnly;
    #[cfg(feature = "DomQuadJson")]
    # [ wasm_bindgen ( method , structural , js_class = "DOMQuad" , js_name = toJSON ) ]
    #[doc = "The `toJSON()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/toJSON)\n\n*This API requires the following crate features to be activated: `DomQuad`, `DomQuadJson`*"]
    pub fn to_json(this: &DomQuad) -> DomQuadJson;
}
