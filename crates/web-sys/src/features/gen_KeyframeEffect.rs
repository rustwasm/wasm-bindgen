use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = AnimationEffect , extends = :: js_sys :: Object , js_name = KeyframeEffect , typescript_name = KeyframeEffect ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `KeyframeEffect` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect)\n\n*This API requires the following crate features to be activated: `KeyframeEffect`*"]
    pub type KeyframeEffect;
    # [ wasm_bindgen ( structural , method , getter , js_class = "KeyframeEffect" , js_name = target ) ]
    #[doc = "Getter for the `target` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/target)\n\n*This API requires the following crate features to be activated: `KeyframeEffect`*"]
    pub fn target(this: &KeyframeEffect) -> Option<::js_sys::Object>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "KeyframeEffect" , js_name = target ) ]
    #[doc = "Setter for the `target` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/target)\n\n*This API requires the following crate features to be activated: `KeyframeEffect`*"]
    pub fn set_target(this: &KeyframeEffect, value: Option<&::js_sys::Object>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "KeyframeEffect" , js_name = iterationComposite ) ]
    #[cfg(feature = "IterationCompositeOperation")]
    #[doc = "Getter for the `iterationComposite` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/iterationComposite)\n\n*This API requires the following crate features to be activated: `IterationCompositeOperation`, `KeyframeEffect`*"]
    pub fn iteration_composite(this: &KeyframeEffect) -> IterationCompositeOperation;
    # [ wasm_bindgen ( structural , method , setter , js_class = "KeyframeEffect" , js_name = iterationComposite ) ]
    #[cfg(feature = "IterationCompositeOperation")]
    #[doc = "Setter for the `iterationComposite` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/iterationComposite)\n\n*This API requires the following crate features to be activated: `IterationCompositeOperation`, `KeyframeEffect`*"]
    pub fn set_iteration_composite(this: &KeyframeEffect, value: IterationCompositeOperation);
    # [ wasm_bindgen ( structural , method , getter , js_class = "KeyframeEffect" , js_name = composite ) ]
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Getter for the `composite` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/composite)\n\n*This API requires the following crate features to be activated: `CompositeOperation`, `KeyframeEffect`*"]
    pub fn composite(this: &KeyframeEffect) -> CompositeOperation;
    # [ wasm_bindgen ( structural , method , setter , js_class = "KeyframeEffect" , js_name = composite ) ]
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Setter for the `composite` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/composite)\n\n*This API requires the following crate features to be activated: `CompositeOperation`, `KeyframeEffect`*"]
    pub fn set_composite(this: &KeyframeEffect, value: CompositeOperation);
    #[cfg(feature = "Element")]
    #[wasm_bindgen(catch, js_class = "KeyframeEffect", constructor)]
    #[doc = "The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)\n\n*This API requires the following crate features to be activated: `Element`, `KeyframeEffect`*"]
    pub fn new_with_opt_element_and_keyframes(
        this: &KeyframeEffect,
        target: Option<&Element>,
        keyframes: Option<&::js_sys::Object>,
    ) -> Result<KeyframeEffect, JsValue>;
    #[cfg(feature = "CssPseudoElement")]
    #[wasm_bindgen(catch, js_class = "KeyframeEffect", constructor)]
    #[doc = "The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)\n\n*This API requires the following crate features to be activated: `CssPseudoElement`, `KeyframeEffect`*"]
    pub fn new_with_opt_css_pseudo_element_and_keyframes(
        this: &KeyframeEffect,
        target: Option<&CssPseudoElement>,
        keyframes: Option<&::js_sys::Object>,
    ) -> Result<KeyframeEffect, JsValue>;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(catch, js_class = "KeyframeEffect", constructor)]
    #[doc = "The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)\n\n*This API requires the following crate features to be activated: `Element`, `KeyframeEffect`*"]
    pub fn new_with_opt_element_and_keyframes_and_f64(
        this: &KeyframeEffect,
        target: Option<&Element>,
        keyframes: Option<&::js_sys::Object>,
        options: f64,
    ) -> Result<KeyframeEffect, JsValue>;
    #[cfg(feature = "CssPseudoElement")]
    #[wasm_bindgen(catch, js_class = "KeyframeEffect", constructor)]
    #[doc = "The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)\n\n*This API requires the following crate features to be activated: `CssPseudoElement`, `KeyframeEffect`*"]
    pub fn new_with_opt_css_pseudo_element_and_keyframes_and_f64(
        this: &KeyframeEffect,
        target: Option<&CssPseudoElement>,
        keyframes: Option<&::js_sys::Object>,
        options: f64,
    ) -> Result<KeyframeEffect, JsValue>;
    #[cfg(all(feature = "Element", feature = "KeyframeEffectOptions",))]
    #[wasm_bindgen(catch, js_class = "KeyframeEffect", constructor)]
    #[doc = "The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)\n\n*This API requires the following crate features to be activated: `Element`, `KeyframeEffect`, `KeyframeEffectOptions`*"]
    pub fn new_with_opt_element_and_keyframes_and_keyframe_effect_options(
        this: &KeyframeEffect,
        target: Option<&Element>,
        keyframes: Option<&::js_sys::Object>,
        options: &KeyframeEffectOptions,
    ) -> Result<KeyframeEffect, JsValue>;
    #[cfg(all(feature = "CssPseudoElement", feature = "KeyframeEffectOptions",))]
    #[wasm_bindgen(catch, js_class = "KeyframeEffect", constructor)]
    #[doc = "The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)\n\n*This API requires the following crate features to be activated: `CssPseudoElement`, `KeyframeEffect`, `KeyframeEffectOptions`*"]
    pub fn new_with_opt_css_pseudo_element_and_keyframes_and_keyframe_effect_options(
        this: &KeyframeEffect,
        target: Option<&CssPseudoElement>,
        keyframes: Option<&::js_sys::Object>,
        options: &KeyframeEffectOptions,
    ) -> Result<KeyframeEffect, JsValue>;
    #[wasm_bindgen(catch, js_class = "KeyframeEffect", constructor)]
    #[doc = "The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)\n\n*This API requires the following crate features to be activated: `KeyframeEffect`*"]
    pub fn new_with_source(
        this: &KeyframeEffect,
        source: &KeyframeEffect,
    ) -> Result<KeyframeEffect, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "KeyframeEffect" , js_name = getKeyframes ) ]
    #[doc = "The `getKeyframes()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/getKeyframes)\n\n*This API requires the following crate features to be activated: `KeyframeEffect`*"]
    pub fn get_keyframes(this: &KeyframeEffect) -> Result<::js_sys::Array, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "KeyframeEffect" , js_name = setKeyframes ) ]
    #[doc = "The `setKeyframes()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/setKeyframes)\n\n*This API requires the following crate features to be activated: `KeyframeEffect`*"]
    pub fn set_keyframes(
        this: &KeyframeEffect,
        keyframes: Option<&::js_sys::Object>,
    ) -> Result<(), JsValue>;
}
