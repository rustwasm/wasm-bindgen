use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = Animation , typescript_name = Animation ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Animation` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    pub type Animation;
    # [ wasm_bindgen ( structural , method , getter , js_name = id ) ]
    #[doc = "Getter for the `id` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/id)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    pub fn id(this: &Animation) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = id ) ]
    #[doc = "Setter for the `id` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/id)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    pub fn set_id(this: &Animation, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = effect ) ]
    #[cfg(feature = "AnimationEffect")]
    #[doc = "Getter for the `effect` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/effect)\n\n*This API requires the following crate features to be activated: `Animation`, `AnimationEffect`*"]
    pub fn effect(this: &Animation) -> Option<AnimationEffect>;
    # [ wasm_bindgen ( structural , method , setter , js_name = effect ) ]
    #[cfg(feature = "AnimationEffect")]
    #[doc = "Setter for the `effect` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/effect)\n\n*This API requires the following crate features to be activated: `Animation`, `AnimationEffect`*"]
    pub fn set_effect(this: &Animation, value: Option<AnimationEffect>);
    # [ wasm_bindgen ( structural , method , getter , js_name = timeline ) ]
    #[cfg(feature = "AnimationTimeline")]
    #[doc = "Getter for the `timeline` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/timeline)\n\n*This API requires the following crate features to be activated: `Animation`, `AnimationTimeline`*"]
    pub fn timeline(this: &Animation) -> Option<AnimationTimeline>;
    # [ wasm_bindgen ( structural , method , setter , js_name = timeline ) ]
    #[cfg(feature = "AnimationTimeline")]
    #[doc = "Setter for the `timeline` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/timeline)\n\n*This API requires the following crate features to be activated: `Animation`, `AnimationTimeline`*"]
    pub fn set_timeline(this: &Animation, value: Option<AnimationTimeline>);
    # [ wasm_bindgen ( structural , method , getter , js_name = startTime ) ]
    #[doc = "Getter for the `startTime` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/startTime)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    pub fn start_time(this: &Animation) -> Option<f64>;
    # [ wasm_bindgen ( structural , method , setter , js_name = startTime ) ]
    #[doc = "Setter for the `startTime` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/startTime)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    pub fn set_start_time(this: &Animation, value: Option<f64>);
    # [ wasm_bindgen ( structural , method , getter , js_name = currentTime ) ]
    #[doc = "Getter for the `currentTime` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/currentTime)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    pub fn current_time(this: &Animation) -> Option<f64>;
    # [ wasm_bindgen ( structural , method , setter , js_name = currentTime ) ]
    #[doc = "Setter for the `currentTime` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/currentTime)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    pub fn set_current_time(this: &Animation, value: Option<f64>);
    # [ wasm_bindgen ( structural , method , getter , js_name = playbackRate ) ]
    #[doc = "Getter for the `playbackRate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/playbackRate)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    pub fn playback_rate(this: &Animation) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = playbackRate ) ]
    #[doc = "Setter for the `playbackRate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/playbackRate)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    pub fn set_playback_rate(this: &Animation, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = playState ) ]
    #[cfg(feature = "AnimationPlayState")]
    #[doc = "Getter for the `playState` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/playState)\n\n*This API requires the following crate features to be activated: `Animation`, `AnimationPlayState`*"]
    pub fn play_state(this: &Animation) -> AnimationPlayState;
    # [ wasm_bindgen ( structural , method , getter , js_name = pending ) ]
    #[doc = "Getter for the `pending` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/pending)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    pub fn pending(this: &Animation) -> bool;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = ready ) ]
    #[doc = "Getter for the `ready` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/ready)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    pub fn ready(this: &Animation) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = finished ) ]
    #[doc = "Getter for the `finished` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/finished)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    pub fn finished(this: &Animation) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = onfinish ) ]
    #[doc = "Getter for the `onfinish` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/onfinish)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    pub fn onfinish(this: &Animation) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onfinish ) ]
    #[doc = "Setter for the `onfinish` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/onfinish)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    pub fn set_onfinish(this: &Animation, value: Option<::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = oncancel ) ]
    #[doc = "Getter for the `oncancel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/oncancel)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    pub fn oncancel(this: &Animation) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = oncancel ) ]
    #[doc = "Setter for the `oncancel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/oncancel)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    pub fn set_oncancel(this: &Animation, value: Option<::js_sys::Function>);
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new Animation(..)` constructor, creating a new instance of `Animation`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/Animation)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    pub fn new(this: &Animation) -> Result<Animation, JsValue>;
    #[cfg(feature = "AnimationEffect")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new Animation(..)` constructor, creating a new instance of `Animation`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/Animation)\n\n*This API requires the following crate features to be activated: `Animation`, `AnimationEffect`*"]
    pub fn new_with_effect(
        this: &Animation,
        effect: Option<&AnimationEffect>,
    ) -> Result<Animation, JsValue>;
    #[cfg(all(feature = "AnimationEffect", feature = "AnimationTimeline",))]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new Animation(..)` constructor, creating a new instance of `Animation`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/Animation)\n\n*This API requires the following crate features to be activated: `Animation`, `AnimationEffect`, `AnimationTimeline`*"]
    pub fn new_with_effect_and_timeline(
        this: &Animation,
        effect: Option<&AnimationEffect>,
        timeline: Option<&AnimationTimeline>,
    ) -> Result<Animation, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = cancel ) ]
    #[doc = "The `cancel()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/cancel)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    pub fn cancel(this: &Animation);
    # [ wasm_bindgen ( catch , method , structural , js_name = finish ) ]
    #[doc = "The `finish()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/finish)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    pub fn finish(this: &Animation) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = pause ) ]
    #[doc = "The `pause()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/pause)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    pub fn pause(this: &Animation) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = play ) ]
    #[doc = "The `play()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/play)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    pub fn play(this: &Animation) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = reverse ) ]
    #[doc = "The `reverse()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/reverse)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    pub fn reverse(this: &Animation) -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = updatePlaybackRate ) ]
    #[doc = "The `updatePlaybackRate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/updatePlaybackRate)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    pub fn update_playback_rate(this: &Animation, playback_rate: f64);
}
