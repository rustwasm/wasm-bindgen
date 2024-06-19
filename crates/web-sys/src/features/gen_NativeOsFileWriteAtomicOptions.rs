#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = NativeOSFileWriteAtomicOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NativeOsFileWriteAtomicOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
    pub type NativeOsFileWriteAtomicOptions;
    #[doc = "Get the `backupTo` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
    #[wasm_bindgen(method, getter = "backupTo")]
    pub fn get_backup_to(this: &NativeOsFileWriteAtomicOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "backupTo")]
    fn set_backup_to(this: &NativeOsFileWriteAtomicOptions, val: Option<&str>);
    #[doc = "Get the `bytes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
    #[wasm_bindgen(method, getter = "bytes")]
    pub fn get_bytes(this: &NativeOsFileWriteAtomicOptions) -> Option<f64>;
    #[wasm_bindgen(method, setter = "bytes")]
    fn set_bytes(this: &NativeOsFileWriteAtomicOptions, val: Option<f64>);
    #[doc = "Get the `flush` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
    #[wasm_bindgen(method, getter = "flush")]
    pub fn get_flush(this: &NativeOsFileWriteAtomicOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter = "flush")]
    fn set_flush(this: &NativeOsFileWriteAtomicOptions, val: bool);
    #[doc = "Get the `noOverwrite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
    #[wasm_bindgen(method, getter = "noOverwrite")]
    pub fn get_no_overwrite(this: &NativeOsFileWriteAtomicOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter = "noOverwrite")]
    fn set_no_overwrite(this: &NativeOsFileWriteAtomicOptions, val: bool);
    #[doc = "Get the `tmpPath` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
    #[wasm_bindgen(method, getter = "tmpPath")]
    pub fn get_tmp_path(this: &NativeOsFileWriteAtomicOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "tmpPath")]
    fn set_tmp_path(this: &NativeOsFileWriteAtomicOptions, val: Option<&str>);
}
impl NativeOsFileWriteAtomicOptions {
    #[doc = "Construct a new `NativeOsFileWriteAtomicOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `backupTo` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
    pub fn backup_to(&mut self, val: Option<&str>) -> &mut Self {
        self.set_backup_to(val);
        self
    }
    #[doc = "Change the `bytes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
    pub fn bytes(&mut self, val: Option<f64>) -> &mut Self {
        self.set_bytes(val);
        self
    }
    #[doc = "Change the `flush` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
    pub fn flush(&mut self, val: bool) -> &mut Self {
        self.set_flush(val);
        self
    }
    #[doc = "Change the `noOverwrite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
    pub fn no_overwrite(&mut self, val: bool) -> &mut Self {
        self.set_no_overwrite(val);
        self
    }
    #[doc = "Change the `tmpPath` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
    pub fn tmp_path(&mut self, val: Option<&str>) -> &mut Self {
        self.set_tmp_path(val);
        self
    }
}
impl Default for NativeOsFileWriteAtomicOptions {
    fn default() -> Self {
        Self::new()
    }
}
