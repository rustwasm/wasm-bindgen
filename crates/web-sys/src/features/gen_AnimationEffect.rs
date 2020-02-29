use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = AnimationEffect , typescript_name = AnimationEffect ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `AnimationEffect` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEffect)
    ///
    ///*This API requires the following crate features to be activated: `AnimationEffect`*
    pub type AnimationEffect;

    #[cfg(feature = "ComputedEffectTiming")]
    # [ wasm_bindgen ( method , structural , js_class = "AnimationEffect" , js_name = getComputedTiming ) ]
    ///The `getComputedTiming()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEffect/getComputedTiming)
    ///
    ///*This API requires the following crate features to be activated: `AnimationEffect`, `ComputedEffectTiming`*
    pub fn get_computed_timing(this: &AnimationEffect) -> ComputedEffectTiming;

    #[cfg(feature = "EffectTiming")]
    # [ wasm_bindgen ( method , structural , js_class = "AnimationEffect" , js_name = getTiming ) ]
    ///The `getTiming()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEffect/getTiming)
    ///
    ///*This API requires the following crate features to be activated: `AnimationEffect`, `EffectTiming`*
    pub fn get_timing(this: &AnimationEffect) -> EffectTiming;

    # [ wasm_bindgen ( catch , method , structural , js_class = "AnimationEffect" , js_name = updateTiming ) ]
    ///The `updateTiming()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEffect/updateTiming)
    ///
    ///*This API requires the following crate features to be activated: `AnimationEffect`*
    pub fn update_timing(this: &AnimationEffect) -> Result<(), JsValue>;

    #[cfg(feature = "OptionalEffectTiming")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "AnimationEffect" , js_name = updateTiming ) ]
    ///The `updateTiming()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEffect/updateTiming)
    ///
    ///*This API requires the following crate features to be activated: `AnimationEffect`, `OptionalEffectTiming`*
    pub fn update_timing_with_timing(
        this: &AnimationEffect,
        timing: &OptionalEffectTiming,
    ) -> Result<(), JsValue>;

}
