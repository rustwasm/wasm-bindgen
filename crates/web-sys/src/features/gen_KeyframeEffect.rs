use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = AnimationEffect , extends = :: js_sys :: Object , js_name = KeyframeEffect , typescript_name = KeyframeEffect ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `KeyframeEffect` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect)
    ///
    ///*This API requires the following crate features to be activated: `KeyframeEffect`*
    pub type KeyframeEffect;

    # [ wasm_bindgen ( structural , method , getter , js_class = "KeyframeEffect" , js_name = target ) ]
    ///Getter for the `target` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/target)
    ///
    ///*This API requires the following crate features to be activated: `KeyframeEffect`*
    pub fn target(this: &KeyframeEffect) -> Option<::js_sys::Object>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "KeyframeEffect" , js_name = target ) ]
    ///Setter for the `target` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/target)
    ///
    ///*This API requires the following crate features to be activated: `KeyframeEffect`*
    pub fn set_target(this: &KeyframeEffect, value: Option<&::js_sys::Object>);

    #[cfg(feature = "IterationCompositeOperation")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "KeyframeEffect" , js_name = iterationComposite ) ]
    ///Getter for the `iterationComposite` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/iterationComposite)
    ///
    ///*This API requires the following crate features to be activated: `IterationCompositeOperation`, `KeyframeEffect`*
    pub fn iteration_composite(this: &KeyframeEffect) -> IterationCompositeOperation;

    #[cfg(feature = "IterationCompositeOperation")]
    # [ wasm_bindgen ( structural , method , setter , js_class = "KeyframeEffect" , js_name = iterationComposite ) ]
    ///Setter for the `iterationComposite` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/iterationComposite)
    ///
    ///*This API requires the following crate features to be activated: `IterationCompositeOperation`, `KeyframeEffect`*
    pub fn set_iteration_composite(this: &KeyframeEffect, value: IterationCompositeOperation);

    #[cfg(feature = "CompositeOperation")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "KeyframeEffect" , js_name = composite ) ]
    ///Getter for the `composite` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/composite)
    ///
    ///*This API requires the following crate features to be activated: `CompositeOperation`, `KeyframeEffect`*
    pub fn composite(this: &KeyframeEffect) -> CompositeOperation;

    #[cfg(feature = "CompositeOperation")]
    # [ wasm_bindgen ( structural , method , setter , js_class = "KeyframeEffect" , js_name = composite ) ]
    ///Setter for the `composite` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/composite)
    ///
    ///*This API requires the following crate features to be activated: `CompositeOperation`, `KeyframeEffect`*
    pub fn set_composite(this: &KeyframeEffect, value: CompositeOperation);

    #[cfg(feature = "Element")]
    #[wasm_bindgen(catch, constructor, js_class = "KeyframeEffect")]
    ///The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)
    ///
    ///*This API requires the following crate features to be activated: `Element`, `KeyframeEffect`*
    pub fn new_with_opt_element_and_keyframes(
        target: Option<&Element>,
        keyframes: Option<&::js_sys::Object>,
    ) -> Result<KeyframeEffect, JsValue>;

    #[cfg(feature = "CssPseudoElement")]
    #[wasm_bindgen(catch, constructor, js_class = "KeyframeEffect")]
    ///The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)
    ///
    ///*This API requires the following crate features to be activated: `CssPseudoElement`, `KeyframeEffect`*
    pub fn new_with_opt_css_pseudo_element_and_keyframes(
        target: Option<&CssPseudoElement>,
        keyframes: Option<&::js_sys::Object>,
    ) -> Result<KeyframeEffect, JsValue>;

    #[cfg(feature = "Element")]
    #[wasm_bindgen(catch, constructor, js_class = "KeyframeEffect")]
    ///The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)
    ///
    ///*This API requires the following crate features to be activated: `Element`, `KeyframeEffect`*
    pub fn new_with_opt_element_and_keyframes_and_f64(
        target: Option<&Element>,
        keyframes: Option<&::js_sys::Object>,
        options: f64,
    ) -> Result<KeyframeEffect, JsValue>;

    #[cfg(feature = "CssPseudoElement")]
    #[wasm_bindgen(catch, constructor, js_class = "KeyframeEffect")]
    ///The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)
    ///
    ///*This API requires the following crate features to be activated: `CssPseudoElement`, `KeyframeEffect`*
    pub fn new_with_opt_css_pseudo_element_and_keyframes_and_f64(
        target: Option<&CssPseudoElement>,
        keyframes: Option<&::js_sys::Object>,
        options: f64,
    ) -> Result<KeyframeEffect, JsValue>;

    #[cfg(all(feature = "Element", feature = "KeyframeEffectOptions",))]
    #[wasm_bindgen(catch, constructor, js_class = "KeyframeEffect")]
    ///The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)
    ///
    ///*This API requires the following crate features to be activated: `Element`, `KeyframeEffect`, `KeyframeEffectOptions`*
    pub fn new_with_opt_element_and_keyframes_and_keyframe_effect_options(
        target: Option<&Element>,
        keyframes: Option<&::js_sys::Object>,
        options: &KeyframeEffectOptions,
    ) -> Result<KeyframeEffect, JsValue>;

    #[cfg(all(feature = "CssPseudoElement", feature = "KeyframeEffectOptions",))]
    #[wasm_bindgen(catch, constructor, js_class = "KeyframeEffect")]
    ///The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)
    ///
    ///*This API requires the following crate features to be activated: `CssPseudoElement`, `KeyframeEffect`, `KeyframeEffectOptions`*
    pub fn new_with_opt_css_pseudo_element_and_keyframes_and_keyframe_effect_options(
        target: Option<&CssPseudoElement>,
        keyframes: Option<&::js_sys::Object>,
        options: &KeyframeEffectOptions,
    ) -> Result<KeyframeEffect, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "KeyframeEffect")]
    ///The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)
    ///
    ///*This API requires the following crate features to be activated: `KeyframeEffect`*
    pub fn new_with_source(source: &KeyframeEffect) -> Result<KeyframeEffect, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "KeyframeEffect" , js_name = getKeyframes ) ]
    ///The `getKeyframes()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/getKeyframes)
    ///
    ///*This API requires the following crate features to be activated: `KeyframeEffect`*
    pub fn get_keyframes(this: &KeyframeEffect) -> Result<::js_sys::Array, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "KeyframeEffect" , js_name = setKeyframes ) ]
    ///The `setKeyframes()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/setKeyframes)
    ///
    ///*This API requires the following crate features to be activated: `KeyframeEffect`*
    pub fn set_keyframes(
        this: &KeyframeEffect,
        keyframes: Option<&::js_sys::Object>,
    ) -> Result<(), JsValue>;

}
