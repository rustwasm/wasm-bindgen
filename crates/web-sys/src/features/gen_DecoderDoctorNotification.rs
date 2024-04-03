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
    #[wasm_bindgen(method, setter = "decodeIssue")]
    fn decode_issue_shim(this: &DecoderDoctorNotification, val: &str);
    #[wasm_bindgen(method, setter = "decoderDoctorReportId")]
    fn decoder_doctor_report_id_shim(this: &DecoderDoctorNotification, val: &str);
    #[wasm_bindgen(method, setter = "docURL")]
    fn doc_url_shim(this: &DecoderDoctorNotification, val: &str);
    #[wasm_bindgen(method, setter = "formats")]
    fn formats_shim(this: &DecoderDoctorNotification, val: &str);
    #[wasm_bindgen(method, setter = "isSolved")]
    fn is_solved_shim(this: &DecoderDoctorNotification, val: bool);
    #[wasm_bindgen(method, setter = "resourceURL")]
    fn resource_url_shim(this: &DecoderDoctorNotification, val: &str);
    #[cfg(feature = "DecoderDoctorNotificationType")]
    #[wasm_bindgen(method, setter = "type")]
    fn type__shim(this: &DecoderDoctorNotification, val: DecoderDoctorNotificationType);
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
        self.decode_issue_shim(val);
        self
    }
    #[doc = "Change the `decoderDoctorReportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    pub fn decoder_doctor_report_id(&mut self, val: &str) -> &mut Self {
        self.decoder_doctor_report_id_shim(val);
        self
    }
    #[doc = "Change the `docURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    pub fn doc_url(&mut self, val: &str) -> &mut Self {
        self.doc_url_shim(val);
        self
    }
    #[doc = "Change the `formats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    pub fn formats(&mut self, val: &str) -> &mut Self {
        self.formats_shim(val);
        self
    }
    #[doc = "Change the `isSolved` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    pub fn is_solved(&mut self, val: bool) -> &mut Self {
        self.is_solved_shim(val);
        self
    }
    #[doc = "Change the `resourceURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    pub fn resource_url(&mut self, val: &str) -> &mut Self {
        self.resource_url_shim(val);
        self
    }
    #[cfg(feature = "DecoderDoctorNotificationType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`, `DecoderDoctorNotificationType`*"]
    pub fn type_(&mut self, val: DecoderDoctorNotificationType) -> &mut Self {
        self.type__shim(val);
        self
    }
}
