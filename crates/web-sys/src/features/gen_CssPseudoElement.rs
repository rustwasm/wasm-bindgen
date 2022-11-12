#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = CSSPseudoElement , typescript_type = "CSSPseudoElement")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssPseudoElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssPseudoElement`*"]
    pub type CssPseudoElement;
    # [wasm_bindgen (structural , method , getter , js_class = "CSSPseudoElement" , js_name = type)]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssPseudoElement`*"]
    pub fn type_(this: &CssPseudoElement) -> String;
    #[cfg(feature = "Element")]
    # [wasm_bindgen (structural , method , getter , js_class = "CSSPseudoElement" , js_name = parentElement)]
    #[doc = "Getter for the `parentElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/parentElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssPseudoElement`, `Element`*"]
    pub fn parent_element(this: &CssPseudoElement) -> Element;
    #[cfg(feature = "Animation")]
    # [wasm_bindgen (catch , method , structural , js_class = "CSSPseudoElement" , js_name = animate)]
    #[doc = "The `animate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/animate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`, `CssPseudoElement`*"]
    pub fn animate(
        this: &CssPseudoElement,
        keyframes: Option<&::js_sys::Object>,
    ) -> Result<Animation, JsValue>;
    #[cfg(feature = "Animation")]
    # [wasm_bindgen (catch , method , structural , js_class = "CSSPseudoElement" , js_name = animate)]
    #[doc = "The `animate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/animate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`, `CssPseudoElement`*"]
    pub fn animate_with_f64(
        this: &CssPseudoElement,
        keyframes: Option<&::js_sys::Object>,
        options: f64,
    ) -> Result<Animation, JsValue>;
    #[cfg(all(feature = "Animation", feature = "KeyframeAnimationOptions",))]
    # [wasm_bindgen (catch , method , structural , js_class = "CSSPseudoElement" , js_name = animate)]
    #[doc = "The `animate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/animate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`, `CssPseudoElement`, `KeyframeAnimationOptions`*"]
    pub fn animate_with_keyframe_animation_options(
        this: &CssPseudoElement,
        keyframes: Option<&::js_sys::Object>,
        options: &KeyframeAnimationOptions,
    ) -> Result<Animation, JsValue>;
    # [wasm_bindgen (method , structural , js_class = "CSSPseudoElement" , js_name = getAnimations)]
    #[doc = "The `getAnimations()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/getAnimations)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssPseudoElement`*"]
    pub fn get_animations(this: &CssPseudoElement) -> ::js_sys::Array;
    #[cfg(feature = "GetAnimationsOptions")]
    # [wasm_bindgen (method , structural , js_class = "CSSPseudoElement" , js_name = getAnimations)]
    #[doc = "The `getAnimations()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/getAnimations)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssPseudoElement`, `GetAnimationsOptions`*"]
    pub fn get_animations_with_options(
        this: &CssPseudoElement,
        options: &GetAnimationsOptions,
    ) -> ::js_sys::Array;
}
