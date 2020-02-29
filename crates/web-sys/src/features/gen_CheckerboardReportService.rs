use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = CheckerboardReportService , typescript_type = "CheckerboardReportService" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `CheckerboardReportService` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CheckerboardReportService)
    ///
    ///*This API requires the following crate features to be activated: `CheckerboardReportService`*
    pub type CheckerboardReportService;

    #[wasm_bindgen(catch, constructor, js_class = "CheckerboardReportService")]
    ///The `new CheckerboardReportService(..)` constructor, creating a new instance of `CheckerboardReportService`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CheckerboardReportService/CheckerboardReportService)
    ///
    ///*This API requires the following crate features to be activated: `CheckerboardReportService`*
    pub fn new() -> Result<CheckerboardReportService, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "CheckerboardReportService" , js_name = flushActiveReports ) ]
    ///The `flushActiveReports()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CheckerboardReportService/flushActiveReports)
    ///
    ///*This API requires the following crate features to be activated: `CheckerboardReportService`*
    pub fn flush_active_reports(this: &CheckerboardReportService);

    # [ wasm_bindgen ( method , structural , js_class = "CheckerboardReportService" , js_name = getReports ) ]
    ///The `getReports()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CheckerboardReportService/getReports)
    ///
    ///*This API requires the following crate features to be activated: `CheckerboardReportService`*
    pub fn get_reports(this: &CheckerboardReportService) -> ::js_sys::Array;

    # [ wasm_bindgen ( method , structural , js_class = "CheckerboardReportService" , js_name = isRecordingEnabled ) ]
    ///The `isRecordingEnabled()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CheckerboardReportService/isRecordingEnabled)
    ///
    ///*This API requires the following crate features to be activated: `CheckerboardReportService`*
    pub fn is_recording_enabled(this: &CheckerboardReportService) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "CheckerboardReportService" , js_name = setRecordingEnabled ) ]
    ///The `setRecordingEnabled()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CheckerboardReportService/setRecordingEnabled)
    ///
    ///*This API requires the following crate features to be activated: `CheckerboardReportService`*
    pub fn set_recording_enabled(this: &CheckerboardReportService, a_enabled: bool);

}
