use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLMediaElement , typescript_type = "HTMLMediaElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlMediaElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub type HtmlMediaElement;

    #[cfg(feature = "MediaError")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMediaElement" , js_name = error ) ]
    ///Getter for the `error` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/error)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaError`*
    pub fn error(this: &HtmlMediaElement) -> Option<MediaError>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMediaElement" , js_name = src ) ]
    ///Getter for the `src` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/src)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn src(this: &HtmlMediaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMediaElement" , js_name = src ) ]
    ///Setter for the `src` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/src)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn set_src(this: &HtmlMediaElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMediaElement" , js_name = currentSrc ) ]
    ///Getter for the `currentSrc` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/currentSrc)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn current_src(this: &HtmlMediaElement) -> String;

    #[cfg(feature = "MediaStream")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMediaElement" , js_name = srcObject ) ]
    ///Getter for the `srcObject` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/srcObject)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaStream`*
    pub fn src_object(this: &HtmlMediaElement) -> Option<MediaStream>;

    #[cfg(feature = "MediaStream")]
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMediaElement" , js_name = srcObject ) ]
    ///Setter for the `srcObject` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/srcObject)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaStream`*
    pub fn set_src_object(this: &HtmlMediaElement, value: Option<&MediaStream>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMediaElement" , js_name = crossOrigin ) ]
    ///Getter for the `crossOrigin` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/crossOrigin)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn cross_origin(this: &HtmlMediaElement) -> Option<String>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMediaElement" , js_name = crossOrigin ) ]
    ///Setter for the `crossOrigin` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/crossOrigin)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn set_cross_origin(this: &HtmlMediaElement, value: Option<&str>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMediaElement" , js_name = networkState ) ]
    ///Getter for the `networkState` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/networkState)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn network_state(this: &HtmlMediaElement) -> u16;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMediaElement" , js_name = preload ) ]
    ///Getter for the `preload` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/preload)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn preload(this: &HtmlMediaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMediaElement" , js_name = preload ) ]
    ///Setter for the `preload` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/preload)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn set_preload(this: &HtmlMediaElement, value: &str);

    #[cfg(feature = "TimeRanges")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMediaElement" , js_name = buffered ) ]
    ///Getter for the `buffered` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/buffered)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`, `TimeRanges`*
    pub fn buffered(this: &HtmlMediaElement) -> TimeRanges;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMediaElement" , js_name = readyState ) ]
    ///Getter for the `readyState` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/readyState)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn ready_state(this: &HtmlMediaElement) -> u16;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMediaElement" , js_name = seeking ) ]
    ///Getter for the `seeking` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/seeking)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn seeking(this: &HtmlMediaElement) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMediaElement" , js_name = currentTime ) ]
    ///Getter for the `currentTime` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/currentTime)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn current_time(this: &HtmlMediaElement) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMediaElement" , js_name = currentTime ) ]
    ///Setter for the `currentTime` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/currentTime)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn set_current_time(this: &HtmlMediaElement, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMediaElement" , js_name = duration ) ]
    ///Getter for the `duration` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/duration)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn duration(this: &HtmlMediaElement) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMediaElement" , js_name = paused ) ]
    ///Getter for the `paused` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/paused)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn paused(this: &HtmlMediaElement) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMediaElement" , js_name = defaultPlaybackRate ) ]
    ///Getter for the `defaultPlaybackRate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/defaultPlaybackRate)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn default_playback_rate(this: &HtmlMediaElement) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMediaElement" , js_name = defaultPlaybackRate ) ]
    ///Setter for the `defaultPlaybackRate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/defaultPlaybackRate)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn set_default_playback_rate(this: &HtmlMediaElement, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMediaElement" , js_name = playbackRate ) ]
    ///Getter for the `playbackRate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/playbackRate)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn playback_rate(this: &HtmlMediaElement) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMediaElement" , js_name = playbackRate ) ]
    ///Setter for the `playbackRate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/playbackRate)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn set_playback_rate(this: &HtmlMediaElement, value: f64);

    #[cfg(feature = "TimeRanges")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMediaElement" , js_name = played ) ]
    ///Getter for the `played` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/played)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`, `TimeRanges`*
    pub fn played(this: &HtmlMediaElement) -> TimeRanges;

    #[cfg(feature = "TimeRanges")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMediaElement" , js_name = seekable ) ]
    ///Getter for the `seekable` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/seekable)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`, `TimeRanges`*
    pub fn seekable(this: &HtmlMediaElement) -> TimeRanges;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMediaElement" , js_name = ended ) ]
    ///Getter for the `ended` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/ended)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn ended(this: &HtmlMediaElement) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMediaElement" , js_name = autoplay ) ]
    ///Getter for the `autoplay` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/autoplay)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn autoplay(this: &HtmlMediaElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMediaElement" , js_name = autoplay ) ]
    ///Setter for the `autoplay` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/autoplay)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn set_autoplay(this: &HtmlMediaElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMediaElement" , js_name = loop ) ]
    ///Getter for the `loop` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loop)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn loop_(this: &HtmlMediaElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMediaElement" , js_name = loop ) ]
    ///Setter for the `loop` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loop)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn set_loop(this: &HtmlMediaElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMediaElement" , js_name = controls ) ]
    ///Getter for the `controls` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/controls)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn controls(this: &HtmlMediaElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMediaElement" , js_name = controls ) ]
    ///Setter for the `controls` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/controls)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn set_controls(this: &HtmlMediaElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMediaElement" , js_name = volume ) ]
    ///Getter for the `volume` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/volume)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn volume(this: &HtmlMediaElement) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMediaElement" , js_name = volume ) ]
    ///Setter for the `volume` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/volume)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn set_volume(this: &HtmlMediaElement, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMediaElement" , js_name = muted ) ]
    ///Getter for the `muted` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/muted)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn muted(this: &HtmlMediaElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMediaElement" , js_name = muted ) ]
    ///Setter for the `muted` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/muted)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn set_muted(this: &HtmlMediaElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMediaElement" , js_name = defaultMuted ) ]
    ///Getter for the `defaultMuted` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/defaultMuted)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn default_muted(this: &HtmlMediaElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMediaElement" , js_name = defaultMuted ) ]
    ///Setter for the `defaultMuted` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/defaultMuted)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn set_default_muted(this: &HtmlMediaElement, value: bool);

    #[cfg(feature = "AudioTrackList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMediaElement" , js_name = audioTracks ) ]
    ///Getter for the `audioTracks` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/audioTracks)
    ///
    ///*This API requires the following crate features to be activated: `AudioTrackList`, `HtmlMediaElement`*
    pub fn audio_tracks(this: &HtmlMediaElement) -> AudioTrackList;

    #[cfg(feature = "VideoTrackList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMediaElement" , js_name = videoTracks ) ]
    ///Getter for the `videoTracks` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/videoTracks)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`, `VideoTrackList`*
    pub fn video_tracks(this: &HtmlMediaElement) -> VideoTrackList;

    #[cfg(feature = "TextTrackList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMediaElement" , js_name = textTracks ) ]
    ///Getter for the `textTracks` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/textTracks)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`, `TextTrackList`*
    pub fn text_tracks(this: &HtmlMediaElement) -> Option<TextTrackList>;

    #[cfg(feature = "MediaKeys")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMediaElement" , js_name = mediaKeys ) ]
    ///Getter for the `mediaKeys` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/mediaKeys)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaKeys`*
    pub fn media_keys(this: &HtmlMediaElement) -> Option<MediaKeys>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMediaElement" , js_name = onencrypted ) ]
    ///Getter for the `onencrypted` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/onencrypted)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn onencrypted(this: &HtmlMediaElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMediaElement" , js_name = onencrypted ) ]
    ///Setter for the `onencrypted` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/onencrypted)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn set_onencrypted(this: &HtmlMediaElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMediaElement" , js_name = onwaitingforkey ) ]
    ///Getter for the `onwaitingforkey` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/onwaitingforkey)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn onwaitingforkey(this: &HtmlMediaElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMediaElement" , js_name = onwaitingforkey ) ]
    ///Setter for the `onwaitingforkey` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/onwaitingforkey)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn set_onwaitingforkey(this: &HtmlMediaElement, value: Option<&::js_sys::Function>);

    #[cfg(all(feature = "TextTrack", feature = "TextTrackKind",))]
    # [ wasm_bindgen ( method , structural , js_class = "HTMLMediaElement" , js_name = addTextTrack ) ]
    ///The `addTextTrack()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/addTextTrack)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`, `TextTrack`, `TextTrackKind`*
    pub fn add_text_track(this: &HtmlMediaElement, kind: TextTrackKind) -> TextTrack;

    #[cfg(all(feature = "TextTrack", feature = "TextTrackKind",))]
    # [ wasm_bindgen ( method , structural , js_class = "HTMLMediaElement" , js_name = addTextTrack ) ]
    ///The `addTextTrack()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/addTextTrack)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`, `TextTrack`, `TextTrackKind`*
    pub fn add_text_track_with_label(
        this: &HtmlMediaElement,
        kind: TextTrackKind,
        label: &str,
    ) -> TextTrack;

    #[cfg(all(feature = "TextTrack", feature = "TextTrackKind",))]
    # [ wasm_bindgen ( method , structural , js_class = "HTMLMediaElement" , js_name = addTextTrack ) ]
    ///The `addTextTrack()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/addTextTrack)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`, `TextTrack`, `TextTrackKind`*
    pub fn add_text_track_with_label_and_language(
        this: &HtmlMediaElement,
        kind: TextTrackKind,
        label: &str,
        language: &str,
    ) -> TextTrack;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLMediaElement" , js_name = canPlayType ) ]
    ///The `canPlayType()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/canPlayType)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn can_play_type(this: &HtmlMediaElement, type_: &str) -> String;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLMediaElement" , js_name = fastSeek ) ]
    ///The `fastSeek()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/fastSeek)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn fast_seek(this: &HtmlMediaElement, time: f64) -> Result<(), JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLMediaElement" , js_name = hasSuspendTaint ) ]
    ///The `hasSuspendTaint()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/hasSuspendTaint)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn has_suspend_taint(this: &HtmlMediaElement) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLMediaElement" , js_name = load ) ]
    ///The `load()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/load)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn load(this: &HtmlMediaElement);

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLMediaElement" , js_name = pause ) ]
    ///The `pause()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/pause)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn pause(this: &HtmlMediaElement) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLMediaElement" , js_name = play ) ]
    ///The `play()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/play)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn play(this: &HtmlMediaElement) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLMediaElement" , js_name = seekToNextFrame ) ]
    ///The `seekToNextFrame()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/seekToNextFrame)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn seek_to_next_frame(this: &HtmlMediaElement) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "MediaKeys")]
    # [ wasm_bindgen ( method , structural , js_class = "HTMLMediaElement" , js_name = setMediaKeys ) ]
    ///The `setMediaKeys()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/setMediaKeys)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaKeys`*
    pub fn set_media_keys(
        this: &HtmlMediaElement,
        media_keys: Option<&MediaKeys>,
    ) -> ::js_sys::Promise;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLMediaElement" , js_name = setVisible ) ]
    ///The `setVisible()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/setVisible)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*
    pub fn set_visible(this: &HtmlMediaElement, a_visible: bool);

}

impl HtmlMediaElement {
    ///The `HTMLMediaElement.NETWORK_EMPTY` const.
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*

    pub const NETWORK_EMPTY: u16 = 0i64 as u16;

    ///The `HTMLMediaElement.NETWORK_IDLE` const.
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*

    pub const NETWORK_IDLE: u16 = 1u64 as u16;

    ///The `HTMLMediaElement.NETWORK_LOADING` const.
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*

    pub const NETWORK_LOADING: u16 = 2u64 as u16;

    ///The `HTMLMediaElement.NETWORK_NO_SOURCE` const.
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*

    pub const NETWORK_NO_SOURCE: u16 = 3u64 as u16;

    ///The `HTMLMediaElement.HAVE_NOTHING` const.
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*

    pub const HAVE_NOTHING: u16 = 0i64 as u16;

    ///The `HTMLMediaElement.HAVE_METADATA` const.
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*

    pub const HAVE_METADATA: u16 = 1u64 as u16;

    ///The `HTMLMediaElement.HAVE_CURRENT_DATA` const.
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*

    pub const HAVE_CURRENT_DATA: u16 = 2u64 as u16;

    ///The `HTMLMediaElement.HAVE_FUTURE_DATA` const.
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*

    pub const HAVE_FUTURE_DATA: u16 = 3u64 as u16;

    ///The `HTMLMediaElement.HAVE_ENOUGH_DATA` const.
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`*

    pub const HAVE_ENOUGH_DATA: u16 = 4u64 as u16;
}
