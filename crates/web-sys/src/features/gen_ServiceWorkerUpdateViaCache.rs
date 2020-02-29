use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `ServiceWorkerUpdateViaCache` enum.
///
///*This API requires the following crate features to be activated: `ServiceWorkerUpdateViaCache`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum ServiceWorkerUpdateViaCache {
    Imports = "imports",
    All = "all",
    None = "none",
}
