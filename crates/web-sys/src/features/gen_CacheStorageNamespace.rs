use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `CacheStorageNamespace` enum.
///
///*This API requires the following crate features to be activated: `CacheStorageNamespace`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum CacheStorageNamespace {
    Content = "content",
    Chrome = "chrome",
}
