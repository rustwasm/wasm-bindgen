use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = VRServiceTest , typescript_type = "VRServiceTest" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `VrServiceTest` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRServiceTest)
    ///
    ///*This API requires the following crate features to be activated: `VrServiceTest`*
    pub type VrServiceTest;

    # [ wasm_bindgen ( catch , method , structural , js_class = "VRServiceTest" , js_name = attachVRController ) ]
    ///The `attachVRController()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRServiceTest/attachVRController)
    ///
    ///*This API requires the following crate features to be activated: `VrServiceTest`*
    pub fn attach_vr_controller(
        this: &VrServiceTest,
        id: &str,
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "VRServiceTest" , js_name = attachVRDisplay ) ]
    ///The `attachVRDisplay()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRServiceTest/attachVRDisplay)
    ///
    ///*This API requires the following crate features to be activated: `VrServiceTest`*
    pub fn attach_vr_display(this: &VrServiceTest, id: &str) -> Result<::js_sys::Promise, JsValue>;

}
