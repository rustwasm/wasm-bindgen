use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `IdbTransactionMode` enum.\n\n*This API requires the following crate features to be activated: `IdbTransactionMode`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum IdbTransactionMode {
    Readonly = "readonly",
    Readwrite = "readwrite",
    Readwriteflush = "readwriteflush",
    Cleanup = "cleanup",
    Versionchange = "versionchange",
}
