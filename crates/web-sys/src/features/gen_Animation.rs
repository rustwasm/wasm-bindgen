use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = Animation , typescript_type = "Animation" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Animation` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation)
    ///
    ///*This API requires the following crate features to be activated: `Animation`*
    pub type Animation;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Animation" , js_name = id ) ]
    ///Getter for the `id` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/id)
    ///
    ///*This API requires the following crate features to be activated: `Animation`*
    pub fn id(this: &Animation) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Animation" , js_name = id ) ]
    ///Setter for the `id` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/id)
    ///
    ///*This API requires the following crate features to be activated: `Animation`*
    pub fn set_id(this: &Animation, value: &str);

    #[cfg(feature = "AnimationEffect")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Animation" , js_name = effect ) ]
    ///Getter for the `effect` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/effect)
    ///
    ///*This API requires the following crate features to be activated: `Animation`, `AnimationEffect`*
    pub fn effect(this: &Animation) -> Option<AnimationEffect>;

    #[cfg(feature = "AnimationEffect")]
    # [ wasm_bindgen ( structural , method , setter , js_class = "Animation" , js_name = effect ) ]
    ///Setter for the `effect` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/effect)
    ///
    ///*This API requires the following crate features to be activated: `Animation`, `AnimationEffect`*
    pub fn set_effect(this: &Animation, value: Option<&AnimationEffect>);

    #[cfg(feature = "AnimationTimeline")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Animation" , js_name = timeline ) ]
    ///Getter for the `timeline` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/timeline)
    ///
    ///*This API requires the following crate features to be activated: `Animation`, `AnimationTimeline`*
    pub fn timeline(this: &Animation) -> Option<AnimationTimeline>;

    #[cfg(feature = "AnimationTimeline")]
    # [ wasm_bindgen ( structural , method , setter , js_class = "Animation" , js_name = timeline ) ]
    ///Setter for the `timeline` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/timeline)
    ///
    ///*This API requires the following crate features to be activated: `Animation`, `AnimationTimeline`*
    pub fn set_timeline(this: &Animation, value: Option<&AnimationTimeline>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Animation" , js_name = startTime ) ]
    ///Getter for the `startTime` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/startTime)
    ///
    ///*This API requires the following crate features to be activated: `Animation`*
    pub fn start_time(this: &Animation) -> Option<f64>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Animation" , js_name = startTime ) ]
    ///Setter for the `startTime` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/startTime)
    ///
    ///*This API requires the following crate features to be activated: `Animation`*
    pub fn set_start_time(this: &Animation, value: Option<f64>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Animation" , js_name = currentTime ) ]
    ///Getter for the `currentTime` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/currentTime)
    ///
    ///*This API requires the following crate features to be activated: `Animation`*
    pub fn current_time(this: &Animation) -> Option<f64>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Animation" , js_name = currentTime ) ]
    ///Setter for the `currentTime` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/currentTime)
    ///
    ///*This API requires the following crate features to be activated: `Animation`*
    pub fn set_current_time(this: &Animation, value: Option<f64>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Animation" , js_name = playbackRate ) ]
    ///Getter for the `playbackRate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/playbackRate)
    ///
    ///*This API requires the following crate features to be activated: `Animation`*
    pub fn playback_rate(this: &Animation) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Animation" , js_name = playbackRate ) ]
    ///Setter for the `playbackRate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/playbackRate)
    ///
    ///*This API requires the following crate features to be activated: `Animation`*
    pub fn set_playback_rate(this: &Animation, value: f64);

    #[cfg(feature = "AnimationPlayState")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Animation" , js_name = playState ) ]
    ///Getter for the `playState` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/playState)
    ///
    ///*This API requires the following crate features to be activated: `Animation`, `AnimationPlayState`*
    pub fn play_state(this: &Animation) -> AnimationPlayState;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Animation" , js_name = pending ) ]
    ///Getter for the `pending` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/pending)
    ///
    ///*This API requires the following crate features to be activated: `Animation`*
    pub fn pending(this: &Animation) -> bool;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Animation" , js_name = ready ) ]
    ///Getter for the `ready` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/ready)
    ///
    ///*This API requires the following crate features to be activated: `Animation`*
    pub fn ready(this: &Animation) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Animation" , js_name = finished ) ]
    ///Getter for the `finished` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/finished)
    ///
    ///*This API requires the following crate features to be activated: `Animation`*
    pub fn finished(this: &Animation) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Animation" , js_name = onfinish ) ]
    ///Getter for the `onfinish` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/onfinish)
    ///
    ///*This API requires the following crate features to be activated: `Animation`*
    pub fn onfinish(this: &Animation) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Animation" , js_name = onfinish ) ]
    ///Setter for the `onfinish` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/onfinish)
    ///
    ///*This API requires the following crate features to be activated: `Animation`*
    pub fn set_onfinish(this: &Animation, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Animation" , js_name = oncancel ) ]
    ///Getter for the `oncancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/oncancel)
    ///
    ///*This API requires the following crate features to be activated: `Animation`*
    pub fn oncancel(this: &Animation) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Animation" , js_name = oncancel ) ]
    ///Setter for the `oncancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/oncancel)
    ///
    ///*This API requires the following crate features to be activated: `Animation`*
    pub fn set_oncancel(this: &Animation, value: Option<&::js_sys::Function>);

    #[wasm_bindgen(catch, constructor, js_class = "Animation")]
    ///The `new Animation(..)` constructor, creating a new instance of `Animation`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/Animation)
    ///
    ///*This API requires the following crate features to be activated: `Animation`*
    pub fn new() -> Result<Animation, JsValue>;

    #[cfg(feature = "AnimationEffect")]
    #[wasm_bindgen(catch, constructor, js_class = "Animation")]
    ///The `new Animation(..)` constructor, creating a new instance of `Animation`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/Animation)
    ///
    ///*This API requires the following crate features to be activated: `Animation`, `AnimationEffect`*
    pub fn new_with_effect(effect: Option<&AnimationEffect>) -> Result<Animation, JsValue>;

    #[cfg(all(feature = "AnimationEffect", feature = "AnimationTimeline",))]
    #[wasm_bindgen(catch, constructor, js_class = "Animation")]
    ///The `new Animation(..)` constructor, creating a new instance of `Animation`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/Animation)
    ///
    ///*This API requires the following crate features to be activated: `Animation`, `AnimationEffect`, `AnimationTimeline`*
    pub fn new_with_effect_and_timeline(
        effect: Option<&AnimationEffect>,
        timeline: Option<&AnimationTimeline>,
    ) -> Result<Animation, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "Animation" , js_name = cancel ) ]
    ///The `cancel()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/cancel)
    ///
    ///*This API requires the following crate features to be activated: `Animation`*
    pub fn cancel(this: &Animation);

    # [ wasm_bindgen ( catch , method , structural , js_class = "Animation" , js_name = finish ) ]
    ///The `finish()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/finish)
    ///
    ///*This API requires the following crate features to be activated: `Animation`*
    pub fn finish(this: &Animation) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Animation" , js_name = pause ) ]
    ///The `pause()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/pause)
    ///
    ///*This API requires the following crate features to be activated: `Animation`*
    pub fn pause(this: &Animation) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Animation" , js_name = play ) ]
    ///The `play()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/play)
    ///
    ///*This API requires the following crate features to be activated: `Animation`*
    pub fn play(this: &Animation) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Animation" , js_name = reverse ) ]
    ///The `reverse()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/reverse)
    ///
    ///*This API requires the following crate features to be activated: `Animation`*
    pub fn reverse(this: &Animation) -> Result<(), JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "Animation" , js_name = updatePlaybackRate ) ]
    ///The `updatePlaybackRate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/updatePlaybackRate)
    ///
    ///*This API requires the following crate features to be activated: `Animation`*
    pub fn update_playback_rate(this: &Animation, playback_rate: f64);

}
