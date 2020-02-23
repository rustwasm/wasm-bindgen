use super::*;
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_6b728e4850bbfc28: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
pub mod css {
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_escape_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&str as WasmDescribe>::describe();
        <String as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `css.escape()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/css/escape)\n\n*This API requires the following crate features to be activated: `css`*"]
    #[allow(clippy::all)]
    pub fn escape(ident: &str) -> String {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_escape_(
                ident: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_escape_(
            ident: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(ident);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let ident = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ident);
                __widl_f_escape_(ident)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_supports_with_value_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&str as WasmDescribe>::describe();
        <&str as WasmDescribe>::describe();
        <bool as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `css.supports()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/css/supports)\n\n*This API requires the following crate features to be activated: `css`*"]
    #[allow(clippy::all)]
    pub fn supports_with_value(
        property: &str,
        value: &str,
    ) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_supports_with_value_(
                property: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_supports_with_value_(
            property: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(property);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let property = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(property);
                let value = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_supports_with_value_(property, value)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_supports_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&str as WasmDescribe>::describe();
        <bool as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `css.supports()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/css/supports)\n\n*This API requires the following crate features to be activated: `css`*"]
    #[allow(clippy::all)]
    pub fn supports(condition_text: &str) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_supports_(
                condition_text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_supports_(
            condition_text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(condition_text);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let condition_text =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(condition_text);
                __widl_f_supports_(condition_text)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
    #[allow(non_upper_case_globals)]
    #[cfg(target_arch = "wasm32")]
    #[link_section = "__wasm_bindgen_unstable"]
    #[doc(hidden)]
    #[allow(clippy::all)]
    pub static __WASM_BINDGEN_GENERATED_b865ce8c115c92e3: [u8; 271usize] = {
        static _INCLUDED_FILES: &[&str] = &[];
        * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xCD\0\0\0\0\0\x03\0\x01\x03css\0\x10__widl_f_escape_\0\0\0\0\x01\x01\x05ident\x06escape\0\x01\x03css\0\x1D__widl_f_supports_with_value_\x01\0\0\0\x01\x02\x08property\x05value\x08supports\0\x01\x03css\0\x12__widl_f_supports_\x01\0\0\0\x01\x01\x0Econdition_text\x08supports\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
    };
}
