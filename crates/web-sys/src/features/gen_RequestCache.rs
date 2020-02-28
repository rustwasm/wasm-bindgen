use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RequestCache` enum.\n\n*This API requires the following crate features to be activated: `RequestCache`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum RequestCache {
    Default = "default",
    NoStore = "no-store",
    Reload = "reload",
    NoCache = "no-cache",
    ForceCache = "force-cache",
    OnlyIfCached = "only-if-cached",
}
