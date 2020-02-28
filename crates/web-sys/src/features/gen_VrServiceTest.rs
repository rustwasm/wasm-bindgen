use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = VRServiceTest , typescript_name = VRServiceTest ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VrServiceTest` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRServiceTest)\n\n*This API requires the following crate features to be activated: `VrServiceTest`*"]
    pub type VrServiceTest;
    # [ wasm_bindgen ( catch , method , structural , js_class = "VRServiceTest" , js_name = attachVRController ) ]
    #[doc = "The `attachVRController()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRServiceTest/attachVRController)\n\n*This API requires the following crate features to be activated: `VrServiceTest`*"]
    pub fn attach_vr_controller(
        this: &VrServiceTest,
        id: &str,
    ) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "VRServiceTest" , js_name = attachVRDisplay ) ]
    #[doc = "The `attachVRDisplay()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRServiceTest/attachVRDisplay)\n\n*This API requires the following crate features to be activated: `VrServiceTest`*"]
    pub fn attach_vr_display(this: &VrServiceTest, id: &str) -> Result<::js_sys::Promise, JsValue>;
}
