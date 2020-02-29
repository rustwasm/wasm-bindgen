use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = MOZ_debug , typescript_type = "MOZ_debug" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MozDebug` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MOZ_debug)
    ///
    ///*This API requires the following crate features to be activated: `MozDebug`*
    pub type MozDebug;

    # [ wasm_bindgen ( catch , method , structural , js_class = "MOZ_debug" , js_name = getParameter ) ]
    ///The `getParameter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MOZ_debug/getParameter)
    ///
    ///*This API requires the following crate features to be activated: `MozDebug`*
    pub fn get_parameter(this: &MozDebug, pname: u32) -> Result<::wasm_bindgen::JsValue, JsValue>;

}

impl MozDebug {
    ///The `MOZ_debug.EXTENSIONS` const.
    ///
    ///*This API requires the following crate features to be activated: `MozDebug`*

    pub const EXTENSIONS: u32 = 7939u64 as u32;

    ///The `MOZ_debug.WSI_INFO` const.
    ///
    ///*This API requires the following crate features to be activated: `MozDebug`*

    pub const WSI_INFO: u32 = 65536u64 as u32;

    ///The `MOZ_debug.UNPACK_REQUIRE_FASTPATH` const.
    ///
    ///*This API requires the following crate features to be activated: `MozDebug`*

    pub const UNPACK_REQUIRE_FASTPATH: u32 = 65537u64 as u32;
}
