use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = DOMQuad , typescript_type = "DOMQuad" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `DomQuad` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad)
    ///
    ///*This API requires the following crate features to be activated: `DomQuad`*
    pub type DomQuad;

    #[cfg(feature = "DomPoint")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMQuad" , js_name = p1 ) ]
    ///Getter for the `p1` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/p1)
    ///
    ///*This API requires the following crate features to be activated: `DomPoint`, `DomQuad`*
    pub fn p1(this: &DomQuad) -> DomPoint;

    #[cfg(feature = "DomPoint")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMQuad" , js_name = p2 ) ]
    ///Getter for the `p2` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/p2)
    ///
    ///*This API requires the following crate features to be activated: `DomPoint`, `DomQuad`*
    pub fn p2(this: &DomQuad) -> DomPoint;

    #[cfg(feature = "DomPoint")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMQuad" , js_name = p3 ) ]
    ///Getter for the `p3` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/p3)
    ///
    ///*This API requires the following crate features to be activated: `DomPoint`, `DomQuad`*
    pub fn p3(this: &DomQuad) -> DomPoint;

    #[cfg(feature = "DomPoint")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMQuad" , js_name = p4 ) ]
    ///Getter for the `p4` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/p4)
    ///
    ///*This API requires the following crate features to be activated: `DomPoint`, `DomQuad`*
    pub fn p4(this: &DomQuad) -> DomPoint;

    #[cfg(feature = "DomRectReadOnly")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMQuad" , js_name = bounds ) ]
    ///Getter for the `bounds` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/bounds)
    ///
    ///*This API requires the following crate features to be activated: `DomQuad`, `DomRectReadOnly`*
    pub fn bounds(this: &DomQuad) -> DomRectReadOnly;

    #[wasm_bindgen(catch, constructor, js_class = "DOMQuad")]
    ///The `new DomQuad(..)` constructor, creating a new instance of `DomQuad`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/DOMQuad)
    ///
    ///*This API requires the following crate features to be activated: `DomQuad`*
    pub fn new() -> Result<DomQuad, JsValue>;

    #[cfg(feature = "DomPointInit")]
    #[wasm_bindgen(catch, constructor, js_class = "DOMQuad")]
    ///The `new DomQuad(..)` constructor, creating a new instance of `DomQuad`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/DOMQuad)
    ///
    ///*This API requires the following crate features to be activated: `DomPointInit`, `DomQuad`*
    pub fn new_with_dom_point_init(p1: &DomPointInit) -> Result<DomQuad, JsValue>;

    #[cfg(feature = "DomPointInit")]
    #[wasm_bindgen(catch, constructor, js_class = "DOMQuad")]
    ///The `new DomQuad(..)` constructor, creating a new instance of `DomQuad`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/DOMQuad)
    ///
    ///*This API requires the following crate features to be activated: `DomPointInit`, `DomQuad`*
    pub fn new_with_dom_point_init_and_p2(
        p1: &DomPointInit,
        p2: &DomPointInit,
    ) -> Result<DomQuad, JsValue>;

    #[cfg(feature = "DomPointInit")]
    #[wasm_bindgen(catch, constructor, js_class = "DOMQuad")]
    ///The `new DomQuad(..)` constructor, creating a new instance of `DomQuad`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/DOMQuad)
    ///
    ///*This API requires the following crate features to be activated: `DomPointInit`, `DomQuad`*
    pub fn new_with_dom_point_init_and_p2_and_p3(
        p1: &DomPointInit,
        p2: &DomPointInit,
        p3: &DomPointInit,
    ) -> Result<DomQuad, JsValue>;

    #[cfg(feature = "DomPointInit")]
    #[wasm_bindgen(catch, constructor, js_class = "DOMQuad")]
    ///The `new DomQuad(..)` constructor, creating a new instance of `DomQuad`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/DOMQuad)
    ///
    ///*This API requires the following crate features to be activated: `DomPointInit`, `DomQuad`*
    pub fn new_with_dom_point_init_and_p2_and_p3_and_p4(
        p1: &DomPointInit,
        p2: &DomPointInit,
        p3: &DomPointInit,
        p4: &DomPointInit,
    ) -> Result<DomQuad, JsValue>;

    #[cfg(feature = "DomRectReadOnly")]
    #[wasm_bindgen(catch, constructor, js_class = "DOMQuad")]
    ///The `new DomQuad(..)` constructor, creating a new instance of `DomQuad`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/DOMQuad)
    ///
    ///*This API requires the following crate features to be activated: `DomQuad`, `DomRectReadOnly`*
    pub fn new_with_rect(rect: &DomRectReadOnly) -> Result<DomQuad, JsValue>;

    #[cfg(feature = "DomRectReadOnly")]
    # [ wasm_bindgen ( method , structural , js_class = "DOMQuad" , js_name = getBounds ) ]
    ///The `getBounds()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/getBounds)
    ///
    ///*This API requires the following crate features to be activated: `DomQuad`, `DomRectReadOnly`*
    pub fn get_bounds(this: &DomQuad) -> DomRectReadOnly;

    #[cfg(feature = "DomQuadJson")]
    # [ wasm_bindgen ( method , structural , js_class = "DOMQuad" , js_name = toJSON ) ]
    ///The `toJSON()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/toJSON)
    ///
    ///*This API requires the following crate features to be activated: `DomQuad`, `DomQuadJson`*
    pub fn to_json(this: &DomQuad) -> DomQuadJson;

}
