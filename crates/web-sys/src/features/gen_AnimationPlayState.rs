use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `AnimationPlayState` enum.\n\n*This API requires the following crate features to be activated: `AnimationPlayState`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AnimationPlayState {
    Idle = "idle",
    Running = "running",
    Paused = "paused",
    Finished = "finished",
}
