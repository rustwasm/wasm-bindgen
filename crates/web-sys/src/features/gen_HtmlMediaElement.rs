use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLMediaElement , typescript_name = HTMLMediaElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlMediaElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub type HtmlMediaElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = error ) ]
    #[cfg(feature = "MediaError")]
    #[doc = "Getter for the `error` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/error)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaError`*"]
    pub fn error(this: &HtmlMediaElement) -> Option<MediaError>;
    # [ wasm_bindgen ( structural , method , getter , js_name = src ) ]
    #[doc = "Getter for the `src` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/src)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn src(this: &HtmlMediaElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = src ) ]
    #[doc = "Setter for the `src` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/src)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn set_src(this: &HtmlMediaElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = currentSrc ) ]
    #[doc = "Getter for the `currentSrc` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/currentSrc)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn current_src(this: &HtmlMediaElement) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = srcObject ) ]
    #[cfg(feature = "MediaStream")]
    #[doc = "Getter for the `srcObject` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/srcObject)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaStream`*"]
    pub fn src_object(this: &HtmlMediaElement) -> Option<MediaStream>;
    # [ wasm_bindgen ( structural , method , setter , js_name = srcObject ) ]
    #[cfg(feature = "MediaStream")]
    #[doc = "Setter for the `srcObject` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/srcObject)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaStream`*"]
    pub fn set_src_object(this: &HtmlMediaElement, value: Option<&MediaStream>);
    # [ wasm_bindgen ( structural , method , getter , js_name = crossOrigin ) ]
    #[doc = "Getter for the `crossOrigin` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/crossOrigin)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn cross_origin(this: &HtmlMediaElement) -> Option<String>;
    # [ wasm_bindgen ( structural , method , setter , js_name = crossOrigin ) ]
    #[doc = "Setter for the `crossOrigin` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/crossOrigin)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn set_cross_origin(this: &HtmlMediaElement, value: Option<&str>);
    # [ wasm_bindgen ( structural , method , getter , js_name = networkState ) ]
    #[doc = "Getter for the `networkState` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/networkState)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn network_state(this: &HtmlMediaElement) -> u16;
    # [ wasm_bindgen ( structural , method , getter , js_name = preload ) ]
    #[doc = "Getter for the `preload` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/preload)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn preload(this: &HtmlMediaElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = preload ) ]
    #[doc = "Setter for the `preload` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/preload)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn set_preload(this: &HtmlMediaElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = buffered ) ]
    #[cfg(feature = "TimeRanges")]
    #[doc = "Getter for the `buffered` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/buffered)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`, `TimeRanges`*"]
    pub fn buffered(this: &HtmlMediaElement) -> TimeRanges;
    # [ wasm_bindgen ( structural , method , getter , js_name = readyState ) ]
    #[doc = "Getter for the `readyState` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/readyState)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn ready_state(this: &HtmlMediaElement) -> u16;
    # [ wasm_bindgen ( structural , method , getter , js_name = seeking ) ]
    #[doc = "Getter for the `seeking` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/seeking)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn seeking(this: &HtmlMediaElement) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = currentTime ) ]
    #[doc = "Getter for the `currentTime` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/currentTime)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn current_time(this: &HtmlMediaElement) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = currentTime ) ]
    #[doc = "Setter for the `currentTime` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/currentTime)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn set_current_time(this: &HtmlMediaElement, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = duration ) ]
    #[doc = "Getter for the `duration` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/duration)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn duration(this: &HtmlMediaElement) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = paused ) ]
    #[doc = "Getter for the `paused` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/paused)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn paused(this: &HtmlMediaElement) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = defaultPlaybackRate ) ]
    #[doc = "Getter for the `defaultPlaybackRate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/defaultPlaybackRate)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn default_playback_rate(this: &HtmlMediaElement) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = defaultPlaybackRate ) ]
    #[doc = "Setter for the `defaultPlaybackRate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/defaultPlaybackRate)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn set_default_playback_rate(this: &HtmlMediaElement, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = playbackRate ) ]
    #[doc = "Getter for the `playbackRate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/playbackRate)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn playback_rate(this: &HtmlMediaElement) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = playbackRate ) ]
    #[doc = "Setter for the `playbackRate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/playbackRate)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn set_playback_rate(this: &HtmlMediaElement, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = played ) ]
    #[cfg(feature = "TimeRanges")]
    #[doc = "Getter for the `played` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/played)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`, `TimeRanges`*"]
    pub fn played(this: &HtmlMediaElement) -> TimeRanges;
    # [ wasm_bindgen ( structural , method , getter , js_name = seekable ) ]
    #[cfg(feature = "TimeRanges")]
    #[doc = "Getter for the `seekable` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/seekable)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`, `TimeRanges`*"]
    pub fn seekable(this: &HtmlMediaElement) -> TimeRanges;
    # [ wasm_bindgen ( structural , method , getter , js_name = ended ) ]
    #[doc = "Getter for the `ended` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/ended)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn ended(this: &HtmlMediaElement) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = autoplay ) ]
    #[doc = "Getter for the `autoplay` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/autoplay)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn autoplay(this: &HtmlMediaElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = autoplay ) ]
    #[doc = "Setter for the `autoplay` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/autoplay)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn set_autoplay(this: &HtmlMediaElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = loop ) ]
    #[doc = "Getter for the `loop` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loop)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn loop_(this: &HtmlMediaElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = loop ) ]
    #[doc = "Setter for the `loop` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loop)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn set_loop(this: &HtmlMediaElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = controls ) ]
    #[doc = "Getter for the `controls` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/controls)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn controls(this: &HtmlMediaElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = controls ) ]
    #[doc = "Setter for the `controls` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/controls)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn set_controls(this: &HtmlMediaElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = volume ) ]
    #[doc = "Getter for the `volume` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/volume)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn volume(this: &HtmlMediaElement) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = volume ) ]
    #[doc = "Setter for the `volume` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/volume)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn set_volume(this: &HtmlMediaElement, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = muted ) ]
    #[doc = "Getter for the `muted` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/muted)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn muted(this: &HtmlMediaElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = muted ) ]
    #[doc = "Setter for the `muted` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/muted)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn set_muted(this: &HtmlMediaElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = defaultMuted ) ]
    #[doc = "Getter for the `defaultMuted` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/defaultMuted)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn default_muted(this: &HtmlMediaElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = defaultMuted ) ]
    #[doc = "Setter for the `defaultMuted` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/defaultMuted)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn set_default_muted(this: &HtmlMediaElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = audioTracks ) ]
    #[cfg(feature = "AudioTrackList")]
    #[doc = "Getter for the `audioTracks` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/audioTracks)\n\n*This API requires the following crate features to be activated: `AudioTrackList`, `HtmlMediaElement`*"]
    pub fn audio_tracks(this: &HtmlMediaElement) -> AudioTrackList;
    # [ wasm_bindgen ( structural , method , getter , js_name = videoTracks ) ]
    #[cfg(feature = "VideoTrackList")]
    #[doc = "Getter for the `videoTracks` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/videoTracks)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`, `VideoTrackList`*"]
    pub fn video_tracks(this: &HtmlMediaElement) -> VideoTrackList;
    # [ wasm_bindgen ( structural , method , getter , js_name = textTracks ) ]
    #[cfg(feature = "TextTrackList")]
    #[doc = "Getter for the `textTracks` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/textTracks)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`, `TextTrackList`*"]
    pub fn text_tracks(this: &HtmlMediaElement) -> Option<TextTrackList>;
    # [ wasm_bindgen ( structural , method , getter , js_name = mediaKeys ) ]
    #[cfg(feature = "MediaKeys")]
    #[doc = "Getter for the `mediaKeys` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/mediaKeys)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaKeys`*"]
    pub fn media_keys(this: &HtmlMediaElement) -> Option<MediaKeys>;
    # [ wasm_bindgen ( structural , method , getter , js_name = onencrypted ) ]
    #[doc = "Getter for the `onencrypted` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/onencrypted)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn onencrypted(this: &HtmlMediaElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onencrypted ) ]
    #[doc = "Setter for the `onencrypted` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/onencrypted)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn set_onencrypted(this: &HtmlMediaElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onwaitingforkey ) ]
    #[doc = "Getter for the `onwaitingforkey` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/onwaitingforkey)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn onwaitingforkey(this: &HtmlMediaElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onwaitingforkey ) ]
    #[doc = "Setter for the `onwaitingforkey` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/onwaitingforkey)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn set_onwaitingforkey(this: &HtmlMediaElement, value: Option<&::js_sys::Function>);
    #[cfg(all(feature = "TextTrack", feature = "TextTrackKind",))]
    # [ wasm_bindgen ( method , structural , js_name = addTextTrack ) ]
    #[doc = "The `addTextTrack()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/addTextTrack)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`, `TextTrack`, `TextTrackKind`*"]
    pub fn add_text_track(this: &HtmlMediaElement, kind: TextTrackKind) -> TextTrack;
    #[cfg(all(feature = "TextTrack", feature = "TextTrackKind",))]
    # [ wasm_bindgen ( method , structural , js_name = addTextTrack ) ]
    #[doc = "The `addTextTrack()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/addTextTrack)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`, `TextTrack`, `TextTrackKind`*"]
    pub fn add_text_track_with_label(
        this: &HtmlMediaElement,
        kind: TextTrackKind,
        label: &str,
    ) -> TextTrack;
    #[cfg(all(feature = "TextTrack", feature = "TextTrackKind",))]
    # [ wasm_bindgen ( method , structural , js_name = addTextTrack ) ]
    #[doc = "The `addTextTrack()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/addTextTrack)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`, `TextTrack`, `TextTrackKind`*"]
    pub fn add_text_track_with_label_and_language(
        this: &HtmlMediaElement,
        kind: TextTrackKind,
        label: &str,
        language: &str,
    ) -> TextTrack;
    # [ wasm_bindgen ( method , structural , js_name = canPlayType ) ]
    #[doc = "The `canPlayType()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/canPlayType)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn can_play_type(this: &HtmlMediaElement, type_: &str) -> String;
    # [ wasm_bindgen ( catch , method , structural , js_name = fastSeek ) ]
    #[doc = "The `fastSeek()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/fastSeek)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn fast_seek(this: &HtmlMediaElement, time: f64) -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = hasSuspendTaint ) ]
    #[doc = "The `hasSuspendTaint()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/hasSuspendTaint)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn has_suspend_taint(this: &HtmlMediaElement) -> bool;
    # [ wasm_bindgen ( method , structural , js_name = load ) ]
    #[doc = "The `load()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/load)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn load(this: &HtmlMediaElement);
    # [ wasm_bindgen ( catch , method , structural , js_name = pause ) ]
    #[doc = "The `pause()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/pause)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn pause(this: &HtmlMediaElement) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = play ) ]
    #[doc = "The `play()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/play)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn play(this: &HtmlMediaElement) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = seekToNextFrame ) ]
    #[doc = "The `seekToNextFrame()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/seekToNextFrame)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn seek_to_next_frame(this: &HtmlMediaElement) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "MediaKeys")]
    # [ wasm_bindgen ( method , structural , js_name = setMediaKeys ) ]
    #[doc = "The `setMediaKeys()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/setMediaKeys)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaKeys`*"]
    pub fn set_media_keys(
        this: &HtmlMediaElement,
        media_keys: Option<&MediaKeys>,
    ) -> ::js_sys::Promise;
    # [ wasm_bindgen ( method , structural , js_name = setVisible ) ]
    #[doc = "The `setVisible()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/setVisible)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn set_visible(this: &HtmlMediaElement, a_visible: bool);
}
impl HtmlMediaElement {
    pub const NETWORK_EMPTY: u16 = 0i64 as u16;
    pub const NETWORK_IDLE: u16 = 1u64 as u16;
    pub const NETWORK_LOADING: u16 = 2u64 as u16;
    pub const NETWORK_NO_SOURCE: u16 = 3u64 as u16;
    pub const HAVE_NOTHING: u16 = 0i64 as u16;
    pub const HAVE_METADATA: u16 = 1u64 as u16;
    pub const HAVE_CURRENT_DATA: u16 = 2u64 as u16;
    pub const HAVE_FUTURE_DATA: u16 = 3u64 as u16;
    pub const HAVE_ENOUGH_DATA: u16 = 4u64 as u16;
}
