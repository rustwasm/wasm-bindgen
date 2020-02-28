use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `ProfileTimelineWorkerOperationType` enum.\n\n*This API requires the following crate features to be activated: `ProfileTimelineWorkerOperationType`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ProfileTimelineWorkerOperationType {
    SerializeDataOffMainThread = "serializeDataOffMainThread",
    SerializeDataOnMainThread = "serializeDataOnMainThread",
    DeserializeDataOffMainThread = "deserializeDataOffMainThread",
    DeserializeDataOnMainThread = "deserializeDataOnMainThread",
}
