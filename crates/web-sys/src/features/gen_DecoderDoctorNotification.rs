#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DecoderDoctorNotification)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DecoderDoctorNotification` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    pub type DecoderDoctorNotification;
    #[doc = "Get the `decodeIssue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    #[wasm_bindgen(method, getter = "decodeIssue")]
    pub fn get_decode_issue(this: &DecoderDoctorNotification) -> Option<String>;
    #[wasm_bindgen(method, setter = "decodeIssue")]
    fn set_decode_issue(this: &DecoderDoctorNotification, val: &str);
    #[doc = "Get the `decoderDoctorReportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    #[wasm_bindgen(method, getter = "decoderDoctorReportId")]
    pub fn get_decoder_doctor_report_id(this: &DecoderDoctorNotification) -> String;
    #[wasm_bindgen(method, setter = "decoderDoctorReportId")]
    fn set_decoder_doctor_report_id(this: &DecoderDoctorNotification, val: &str);
    #[doc = "Get the `docURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    #[wasm_bindgen(method, getter = "docURL")]
    pub fn get_doc_url(this: &DecoderDoctorNotification) -> Option<String>;
    #[wasm_bindgen(method, setter = "docURL")]
    fn set_doc_url(this: &DecoderDoctorNotification, val: &str);
    #[doc = "Get the `formats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    #[wasm_bindgen(method, getter = "formats")]
    pub fn get_formats(this: &DecoderDoctorNotification) -> Option<String>;
    #[wasm_bindgen(method, setter = "formats")]
    fn set_formats(this: &DecoderDoctorNotification, val: &str);
    #[doc = "Get the `isSolved` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    #[wasm_bindgen(method, getter = "isSolved")]
    pub fn get_is_solved(this: &DecoderDoctorNotification) -> bool;
    #[wasm_bindgen(method, setter = "isSolved")]
    fn set_is_solved(this: &DecoderDoctorNotification, val: bool);
    #[doc = "Get the `resourceURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    #[wasm_bindgen(method, getter = "resourceURL")]
    pub fn get_resource_url(this: &DecoderDoctorNotification) -> Option<String>;
    #[wasm_bindgen(method, setter = "resourceURL")]
    fn set_resource_url(this: &DecoderDoctorNotification, val: &str);
    #[cfg(feature = "DecoderDoctorNotificationType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`, `DecoderDoctorNotificationType`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &DecoderDoctorNotification) -> DecoderDoctorNotificationType;
    #[cfg(feature = "DecoderDoctorNotificationType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type(this: &DecoderDoctorNotification, val: DecoderDoctorNotificationType);
}
impl DecoderDoctorNotification {
    #[cfg(feature = "DecoderDoctorNotificationType")]
    #[doc = "Construct a new `DecoderDoctorNotification`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`, `DecoderDoctorNotificationType`*"]
    pub fn new(
        decoder_doctor_report_id: &str,
        is_solved: bool,
        type_: DecoderDoctorNotificationType,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.decoder_doctor_report_id(decoder_doctor_report_id);
        ret.is_solved(is_solved);
        ret.type_(type_);
        ret
    }
    #[doc = "Change the `decodeIssue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    pub fn decode_issue(&mut self, val: &str) -> &mut Self {
        self.set_decode_issue(val);
        self
    }
    #[doc = "Change the `decoderDoctorReportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    pub fn decoder_doctor_report_id(&mut self, val: &str) -> &mut Self {
        self.set_decoder_doctor_report_id(val);
        self
    }
    #[doc = "Change the `docURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    pub fn doc_url(&mut self, val: &str) -> &mut Self {
        self.set_doc_url(val);
        self
    }
    #[doc = "Change the `formats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    pub fn formats(&mut self, val: &str) -> &mut Self {
        self.set_formats(val);
        self
    }
    #[doc = "Change the `isSolved` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    pub fn is_solved(&mut self, val: bool) -> &mut Self {
        self.set_is_solved(val);
        self
    }
    #[doc = "Change the `resourceURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    pub fn resource_url(&mut self, val: &str) -> &mut Self {
        self.set_resource_url(val);
        self
    }
    #[cfg(feature = "DecoderDoctorNotificationType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`, `DecoderDoctorNotificationType`*"]
    pub fn type_(&mut self, val: DecoderDoctorNotificationType) -> &mut Self {
        self.set_type(val);
        self
    }
}
