use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = VideoPlaybackQuality , typescript_name = VideoPlaybackQuality ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoPlaybackQuality` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoPlaybackQuality)\n\n*This API requires the following crate features to be activated: `VideoPlaybackQuality`*"]
    pub type VideoPlaybackQuality;
    # [ wasm_bindgen ( structural , method , getter , js_name = creationTime ) ]
    #[doc = "Getter for the `creationTime` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoPlaybackQuality/creationTime)\n\n*This API requires the following crate features to be activated: `VideoPlaybackQuality`*"]
    pub fn creation_time(this: &VideoPlaybackQuality) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = totalVideoFrames ) ]
    #[doc = "Getter for the `totalVideoFrames` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoPlaybackQuality/totalVideoFrames)\n\n*This API requires the following crate features to be activated: `VideoPlaybackQuality`*"]
    pub fn total_video_frames(this: &VideoPlaybackQuality) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_name = droppedVideoFrames ) ]
    #[doc = "Getter for the `droppedVideoFrames` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoPlaybackQuality/droppedVideoFrames)\n\n*This API requires the following crate features to be activated: `VideoPlaybackQuality`*"]
    pub fn dropped_video_frames(this: &VideoPlaybackQuality) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_name = corruptedVideoFrames ) ]
    #[doc = "Getter for the `corruptedVideoFrames` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoPlaybackQuality/corruptedVideoFrames)\n\n*This API requires the following crate features to be activated: `VideoPlaybackQuality`*"]
    pub fn corrupted_video_frames(this: &VideoPlaybackQuality) -> u32;
}
