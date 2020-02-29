use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `ProfileTimelineMessagePortOperationType` enum.
///
///*This API requires the following crate features to be activated: `ProfileTimelineMessagePortOperationType`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum ProfileTimelineMessagePortOperationType {
    SerializeData = "serializeData",
    DeserializeData = "deserializeData",
}
