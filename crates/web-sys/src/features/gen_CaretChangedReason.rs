use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `CaretChangedReason` enum.
///
///*This API requires the following crate features to be activated: `CaretChangedReason`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum CaretChangedReason {
    Visibilitychange = "visibilitychange",
    Updateposition = "updateposition",
    Longpressonemptycontent = "longpressonemptycontent",
    Taponcaret = "taponcaret",
    Presscaret = "presscaret",
    Releasecaret = "releasecaret",
    Scroll = "scroll",
}
