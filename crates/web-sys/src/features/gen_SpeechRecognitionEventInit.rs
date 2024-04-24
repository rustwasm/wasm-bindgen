#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = SpeechRecognitionEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechRecognitionEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
    pub type SpeechRecognitionEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &SpeechRecognitionEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &SpeechRecognitionEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &SpeechRecognitionEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &SpeechRecognitionEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &SpeechRecognitionEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &SpeechRecognitionEventInit, val: bool);
    #[cfg(feature = "Document")]
    #[wasm_bindgen(method, getter = "emma")]
    fn emma_shim(this: &SpeechRecognitionEventInit) -> Option<Document>;
    #[cfg(feature = "Document")]
    #[wasm_bindgen(method, setter = "emma")]
    fn set_emma_shim(this: &SpeechRecognitionEventInit, val: Option<&Document>);
    #[wasm_bindgen(method, getter = "interpretation")]
    fn interpretation_shim(this: &SpeechRecognitionEventInit) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "interpretation")]
    fn set_interpretation_shim(this: &SpeechRecognitionEventInit, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "resultIndex")]
    fn result_index_shim(this: &SpeechRecognitionEventInit) -> u32;
    #[wasm_bindgen(method, setter = "resultIndex")]
    fn set_result_index_shim(this: &SpeechRecognitionEventInit, val: u32);
    #[cfg(feature = "SpeechRecognitionResultList")]
    #[wasm_bindgen(method, getter = "results")]
    fn results_shim(this: &SpeechRecognitionEventInit) -> Option<SpeechRecognitionResultList>;
    #[cfg(feature = "SpeechRecognitionResultList")]
    #[wasm_bindgen(method, setter = "results")]
    fn set_results_shim(
        this: &SpeechRecognitionEventInit,
        val: Option<&SpeechRecognitionResultList>,
    );
}
#[doc = "The trait to access properties on the `SpeechRecognitionEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
pub trait SpeechRecognitionEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
    fn composed(&self) -> bool;
    #[cfg(feature = "Document")]
    #[doc = "Get the `emma` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `SpeechRecognitionEventInit`*"]
    fn emma(&self) -> Option<Document>;
    #[doc = "Get the `interpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
    fn interpretation(&self) -> ::wasm_bindgen::JsValue;
    #[doc = "Get the `resultIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
    fn result_index(&self) -> u32;
    #[cfg(feature = "SpeechRecognitionResultList")]
    #[doc = "Get the `results` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`, `SpeechRecognitionResultList`*"]
    fn results(&self) -> Option<SpeechRecognitionResultList>;
}
impl SpeechRecognitionEventInitGetters for SpeechRecognitionEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    #[cfg(feature = "Document")]
    fn emma(&self) -> Option<Document> {
        self.emma_shim()
    }
    fn interpretation(&self) -> ::wasm_bindgen::JsValue {
        self.interpretation_shim()
    }
    fn result_index(&self) -> u32 {
        self.result_index_shim()
    }
    #[cfg(feature = "SpeechRecognitionResultList")]
    fn results(&self) -> Option<SpeechRecognitionResultList> {
        self.results_shim()
    }
}
impl SpeechRecognitionEventInit {
    #[doc = "Construct a new `SpeechRecognitionEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[cfg(feature = "Document")]
    #[doc = "Change the `emma` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `SpeechRecognitionEventInit`*"]
    pub fn emma(&mut self, val: Option<&Document>) -> &mut Self {
        self.set_emma_shim(val);
        self
    }
    #[doc = "Change the `interpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
    pub fn interpretation(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_interpretation_shim(val);
        self
    }
    #[doc = "Change the `resultIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
    pub fn result_index(&mut self, val: u32) -> &mut Self {
        self.set_result_index_shim(val);
        self
    }
    #[cfg(feature = "SpeechRecognitionResultList")]
    #[doc = "Change the `results` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`, `SpeechRecognitionResultList`*"]
    pub fn results(&mut self, val: Option<&SpeechRecognitionResultList>) -> &mut Self {
        self.set_results_shim(val);
        self
    }
}
impl Default for SpeechRecognitionEventInit {
    fn default() -> Self {
        Self::new()
    }
}
