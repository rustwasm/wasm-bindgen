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
    #[wasm_bindgen(method, getter = "decodeIssue")]
    fn decode_issue_shim(this: &DecoderDoctorNotification) -> String;
    #[wasm_bindgen(method, setter = "decodeIssue")]
    fn set_decode_issue_shim(this: &DecoderDoctorNotification, val: &str);
    #[wasm_bindgen(method, getter = "decoderDoctorReportId")]
    fn decoder_doctor_report_id_shim(this: &DecoderDoctorNotification) -> String;
    #[wasm_bindgen(method, setter = "decoderDoctorReportId")]
    fn set_decoder_doctor_report_id_shim(this: &DecoderDoctorNotification, val: &str);
    #[wasm_bindgen(method, getter = "docURL")]
    fn doc_url_shim(this: &DecoderDoctorNotification) -> String;
    #[wasm_bindgen(method, setter = "docURL")]
    fn set_doc_url_shim(this: &DecoderDoctorNotification, val: &str);
    #[wasm_bindgen(method, getter = "formats")]
    fn formats_shim(this: &DecoderDoctorNotification) -> String;
    #[wasm_bindgen(method, setter = "formats")]
    fn set_formats_shim(this: &DecoderDoctorNotification, val: &str);
    #[wasm_bindgen(method, getter = "isSolved")]
    fn is_solved_shim(this: &DecoderDoctorNotification) -> bool;
    #[wasm_bindgen(method, setter = "isSolved")]
    fn set_is_solved_shim(this: &DecoderDoctorNotification, val: bool);
    #[wasm_bindgen(method, getter = "resourceURL")]
    fn resource_url_shim(this: &DecoderDoctorNotification) -> String;
    #[wasm_bindgen(method, setter = "resourceURL")]
    fn set_resource_url_shim(this: &DecoderDoctorNotification, val: &str);
    #[cfg(feature = "DecoderDoctorNotificationType")]
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &DecoderDoctorNotification) -> DecoderDoctorNotificationType;
    #[cfg(feature = "DecoderDoctorNotificationType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &DecoderDoctorNotification, val: DecoderDoctorNotificationType);
}
#[doc = "The trait to access properties on the `DecoderDoctorNotification` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
pub trait DecoderDoctorNotificationGetters {
    #[doc = "Get the `decodeIssue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    fn decode_issue(&self) -> String;
    #[doc = "Get the `decoderDoctorReportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    fn decoder_doctor_report_id(&self) -> String;
    #[doc = "Get the `docURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    fn doc_url(&self) -> String;
    #[doc = "Get the `formats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    fn formats(&self) -> String;
    #[doc = "Get the `isSolved` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    fn is_solved(&self) -> bool;
    #[doc = "Get the `resourceURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    fn resource_url(&self) -> String;
    #[cfg(feature = "DecoderDoctorNotificationType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`, `DecoderDoctorNotificationType`*"]
    fn type_(&self) -> DecoderDoctorNotificationType;
}
impl DecoderDoctorNotificationGetters for DecoderDoctorNotification {
    fn decode_issue(&self) -> String {
        self.decode_issue_shim()
    }
    fn decoder_doctor_report_id(&self) -> String {
        self.decoder_doctor_report_id_shim()
    }
    fn doc_url(&self) -> String {
        self.doc_url_shim()
    }
    fn formats(&self) -> String {
        self.formats_shim()
    }
    fn is_solved(&self) -> bool {
        self.is_solved_shim()
    }
    fn resource_url(&self) -> String {
        self.resource_url_shim()
    }
    #[cfg(feature = "DecoderDoctorNotificationType")]
    fn type_(&self) -> DecoderDoctorNotificationType {
        self.type__shim()
    }
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
        self.set_decode_issue_shim(val);
        self
    }
    #[doc = "Change the `decoderDoctorReportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    pub fn decoder_doctor_report_id(&mut self, val: &str) -> &mut Self {
        self.set_decoder_doctor_report_id_shim(val);
        self
    }
    #[doc = "Change the `docURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    pub fn doc_url(&mut self, val: &str) -> &mut Self {
        self.set_doc_url_shim(val);
        self
    }
    #[doc = "Change the `formats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    pub fn formats(&mut self, val: &str) -> &mut Self {
        self.set_formats_shim(val);
        self
    }
    #[doc = "Change the `isSolved` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    pub fn is_solved(&mut self, val: bool) -> &mut Self {
        self.set_is_solved_shim(val);
        self
    }
    #[doc = "Change the `resourceURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    pub fn resource_url(&mut self, val: &str) -> &mut Self {
        self.set_resource_url_shim(val);
        self
    }
    #[cfg(feature = "DecoderDoctorNotificationType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`, `DecoderDoctorNotificationType`*"]
    pub fn type_(&mut self, val: DecoderDoctorNotificationType) -> &mut Self {
        self.set_type__shim(val);
        self
    }
}
