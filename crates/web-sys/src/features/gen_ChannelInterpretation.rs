use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `ChannelInterpretation` enum.
///
///*This API requires the following crate features to be activated: `ChannelInterpretation`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum ChannelInterpretation {
    Speakers = "speakers",
    Discrete = "discrete",
}
