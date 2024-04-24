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
    #[wasm_bindgen(method, getter = "backupTo")]
    fn backup_to_shim(this: &NativeOsFileWriteAtomicOptions) -> Option<&str>;
    #[wasm_bindgen(method, setter = "backupTo")]
    fn set_backup_to_shim(this: &NativeOsFileWriteAtomicOptions, val: Option<&str>);
    #[wasm_bindgen(method, getter = "bytes")]
    fn bytes_shim(this: &NativeOsFileWriteAtomicOptions) -> Option<f64>;
    #[wasm_bindgen(method, setter = "bytes")]
    fn set_bytes_shim(this: &NativeOsFileWriteAtomicOptions, val: Option<f64>);
    #[wasm_bindgen(method, getter = "flush")]
    fn flush_shim(this: &NativeOsFileWriteAtomicOptions) -> bool;
    #[wasm_bindgen(method, setter = "flush")]
    fn set_flush_shim(this: &NativeOsFileWriteAtomicOptions, val: bool);
    #[wasm_bindgen(method, getter = "noOverwrite")]
    fn no_overwrite_shim(this: &NativeOsFileWriteAtomicOptions) -> bool;
    #[wasm_bindgen(method, setter = "noOverwrite")]
    fn set_no_overwrite_shim(this: &NativeOsFileWriteAtomicOptions, val: bool);
    #[wasm_bindgen(method, getter = "tmpPath")]
    fn tmp_path_shim(this: &NativeOsFileWriteAtomicOptions) -> Option<&str>;
    #[wasm_bindgen(method, setter = "tmpPath")]
    fn set_tmp_path_shim(this: &NativeOsFileWriteAtomicOptions, val: Option<&str>);
}
#[doc = "The trait to access properties on the `NativeOsFileWriteAtomicOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
pub trait NativeOsFileWriteAtomicOptionsGetters {
    #[doc = "Get the `backupTo` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
    fn backup_to(&self) -> Option<&str>;
    #[doc = "Get the `bytes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
    fn bytes(&self) -> Option<f64>;
    #[doc = "Get the `flush` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
    fn flush(&self) -> bool;
    #[doc = "Get the `noOverwrite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
    fn no_overwrite(&self) -> bool;
    #[doc = "Get the `tmpPath` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
    fn tmp_path(&self) -> Option<&str>;
}
impl NativeOsFileWriteAtomicOptionsGetters for NativeOsFileWriteAtomicOptions {
    fn backup_to(&self) -> Option<&str> {
        self.backup_to_shim()
    }
    fn bytes(&self) -> Option<f64> {
        self.bytes_shim()
    }
    fn flush(&self) -> bool {
        self.flush_shim()
    }
    fn no_overwrite(&self) -> bool {
        self.no_overwrite_shim()
    }
    fn tmp_path(&self) -> Option<&str> {
        self.tmp_path_shim()
    }
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
        self.set_backup_to_shim(val);
        self
    }
    #[doc = "Change the `bytes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
    pub fn bytes(&mut self, val: Option<f64>) -> &mut Self {
        self.set_bytes_shim(val);
        self
    }
    #[doc = "Change the `flush` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
    pub fn flush(&mut self, val: bool) -> &mut Self {
        self.set_flush_shim(val);
        self
    }
    #[doc = "Change the `noOverwrite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
    pub fn no_overwrite(&mut self, val: bool) -> &mut Self {
        self.set_no_overwrite_shim(val);
        self
    }
    #[doc = "Change the `tmpPath` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
    pub fn tmp_path(&mut self, val: Option<&str>) -> &mut Self {
        self.set_tmp_path_shim(val);
        self
    }
}
impl Default for NativeOsFileWriteAtomicOptions {
    fn default() -> Self {
        Self::new()
    }
}
