use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = CheckerboardReportService , typescript_name = CheckerboardReportService ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CheckerboardReportService` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CheckerboardReportService)\n\n*This API requires the following crate features to be activated: `CheckerboardReportService`*"]
    pub type CheckerboardReportService;
    #[wasm_bindgen(catch, js_class = "CheckerboardReportService", constructor)]
    #[doc = "The `new CheckerboardReportService(..)` constructor, creating a new instance of `CheckerboardReportService`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CheckerboardReportService/CheckerboardReportService)\n\n*This API requires the following crate features to be activated: `CheckerboardReportService`*"]
    pub fn new(this: &CheckerboardReportService) -> Result<CheckerboardReportService, JsValue>;
    # [ wasm_bindgen ( method , structural , js_class = "CheckerboardReportService" , js_name = flushActiveReports ) ]
    #[doc = "The `flushActiveReports()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CheckerboardReportService/flushActiveReports)\n\n*This API requires the following crate features to be activated: `CheckerboardReportService`*"]
    pub fn flush_active_reports(this: &CheckerboardReportService);
    # [ wasm_bindgen ( method , structural , js_class = "CheckerboardReportService" , js_name = getReports ) ]
    #[doc = "The `getReports()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CheckerboardReportService/getReports)\n\n*This API requires the following crate features to be activated: `CheckerboardReportService`*"]
    pub fn get_reports(this: &CheckerboardReportService) -> ::js_sys::Array;
    # [ wasm_bindgen ( method , structural , js_class = "CheckerboardReportService" , js_name = isRecordingEnabled ) ]
    #[doc = "The `isRecordingEnabled()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CheckerboardReportService/isRecordingEnabled)\n\n*This API requires the following crate features to be activated: `CheckerboardReportService`*"]
    pub fn is_recording_enabled(this: &CheckerboardReportService) -> bool;
    # [ wasm_bindgen ( method , structural , js_class = "CheckerboardReportService" , js_name = setRecordingEnabled ) ]
    #[doc = "The `setRecordingEnabled()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CheckerboardReportService/setRecordingEnabled)\n\n*This API requires the following crate features to be activated: `CheckerboardReportService`*"]
    pub fn set_recording_enabled(this: &CheckerboardReportService, a_enabled: bool);
}
