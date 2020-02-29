use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = VideoPlaybackQuality , typescript_name = VideoPlaybackQuality ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `VideoPlaybackQuality` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoPlaybackQuality)
    ///
    ///*This API requires the following crate features to be activated: `VideoPlaybackQuality`*
    pub type VideoPlaybackQuality;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VideoPlaybackQuality" , js_name = creationTime ) ]
    ///Getter for the `creationTime` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoPlaybackQuality/creationTime)
    ///
    ///*This API requires the following crate features to be activated: `VideoPlaybackQuality`*
    pub fn creation_time(this: &VideoPlaybackQuality) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VideoPlaybackQuality" , js_name = totalVideoFrames ) ]
    ///Getter for the `totalVideoFrames` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoPlaybackQuality/totalVideoFrames)
    ///
    ///*This API requires the following crate features to be activated: `VideoPlaybackQuality`*
    pub fn total_video_frames(this: &VideoPlaybackQuality) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VideoPlaybackQuality" , js_name = droppedVideoFrames ) ]
    ///Getter for the `droppedVideoFrames` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoPlaybackQuality/droppedVideoFrames)
    ///
    ///*This API requires the following crate features to be activated: `VideoPlaybackQuality`*
    pub fn dropped_video_frames(this: &VideoPlaybackQuality) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VideoPlaybackQuality" , js_name = corruptedVideoFrames ) ]
    ///Getter for the `corruptedVideoFrames` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoPlaybackQuality/corruptedVideoFrames)
    ///
    ///*This API requires the following crate features to be activated: `VideoPlaybackQuality`*
    pub fn corrupted_video_frames(this: &VideoPlaybackQuality) -> u32;

}
