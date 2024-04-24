#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaMetadataInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaMetadataInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaMetadataInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type MediaMetadataInit;
    #[wasm_bindgen(method, getter = "album")]
    fn album_shim(this: &MediaMetadataInit) -> &str;
    #[wasm_bindgen(method, setter = "album")]
    fn set_album_shim(this: &MediaMetadataInit, val: &str);
    #[wasm_bindgen(method, getter = "artist")]
    fn artist_shim(this: &MediaMetadataInit) -> &str;
    #[wasm_bindgen(method, setter = "artist")]
    fn set_artist_shim(this: &MediaMetadataInit, val: &str);
    #[wasm_bindgen(method, getter = "artwork")]
    fn artwork_shim(this: &MediaMetadataInit) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "artwork")]
    fn set_artwork_shim(this: &MediaMetadataInit, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "title")]
    fn title_shim(this: &MediaMetadataInit) -> &str;
    #[wasm_bindgen(method, setter = "title")]
    fn set_title_shim(this: &MediaMetadataInit, val: &str);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `MediaMetadataInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaMetadataInit`*"]
pub trait MediaMetadataInitGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `album` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaMetadataInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn album(&self) -> &str;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `artist` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaMetadataInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn artist(&self) -> &str;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `artwork` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaMetadataInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn artwork(&self) -> &::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `title` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaMetadataInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn title(&self) -> &str;
}
#[cfg(web_sys_unstable_apis)]
impl MediaMetadataInitGetters for MediaMetadataInit {
    #[cfg(web_sys_unstable_apis)]
    fn album(&self) -> &str {
        self.album_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn artist(&self) -> &str {
        self.artist_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn artwork(&self) -> &::wasm_bindgen::JsValue {
        self.artwork_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn title(&self) -> &str {
        self.title_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl MediaMetadataInit {
    #[doc = "Construct a new `MediaMetadataInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaMetadataInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `album` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaMetadataInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn album(&mut self, val: &str) -> &mut Self {
        self.set_album_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `artist` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaMetadataInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn artist(&mut self, val: &str) -> &mut Self {
        self.set_artist_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `artwork` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaMetadataInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn artwork(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_artwork_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `title` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaMetadataInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn title(&mut self, val: &str) -> &mut Self {
        self.set_title_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for MediaMetadataInit {
    fn default() -> Self {
        Self::new()
    }
}
