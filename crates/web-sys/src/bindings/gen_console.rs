use super::*;
use js_sys::Object;
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_7ec6bc8882a41415: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
pub mod console {
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_assert_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.assert()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn assert() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_assert_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_assert_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_assert_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_assert_with_condition_and_data_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <bool as WasmDescribe>::describe();
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.assert()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn assert_with_condition_and_data(condition: bool, data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_assert_with_condition_and_data_(
                condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_assert_with_condition_and_data_(
            condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(condition);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let condition = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(condition);
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_assert_with_condition_and_data_(condition, data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_assert_with_condition_and_data_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <bool as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.assert()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn assert_with_condition_and_data_0(condition: bool) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_assert_with_condition_and_data_0_(
                condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_assert_with_condition_and_data_0_(
            condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(condition);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let condition = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(condition);
                __widl_f_assert_with_condition_and_data_0_(condition)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_assert_with_condition_and_data_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <bool as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.assert()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn assert_with_condition_and_data_1(condition: bool, data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_assert_with_condition_and_data_1_(
                condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_assert_with_condition_and_data_1_(
            condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(condition);
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let condition = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(condition);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_assert_with_condition_and_data_1_(condition, data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_assert_with_condition_and_data_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <bool as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.assert()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn assert_with_condition_and_data_2(
        condition: bool,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_assert_with_condition_and_data_2_(
                condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_assert_with_condition_and_data_2_(
            condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(condition);
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let condition = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(condition);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_assert_with_condition_and_data_2_(condition, data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_assert_with_condition_and_data_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <bool as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.assert()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn assert_with_condition_and_data_3(
        condition: bool,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_assert_with_condition_and_data_3_(
                condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_assert_with_condition_and_data_3_(
            condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(condition);
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let condition = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(condition);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_assert_with_condition_and_data_3_(condition, data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_assert_with_condition_and_data_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <bool as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.assert()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn assert_with_condition_and_data_4(
        condition: bool,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_assert_with_condition_and_data_4_(
                condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_assert_with_condition_and_data_4_(
            condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(condition);
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let condition = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(condition);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_assert_with_condition_and_data_4_(
                    condition, data_1, data_2, data_3, data_4,
                )
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_assert_with_condition_and_data_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <bool as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.assert()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn assert_with_condition_and_data_5(
        condition: bool,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_assert_with_condition_and_data_5_(
                condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_assert_with_condition_and_data_5_(
            condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(condition);
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let condition = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(condition);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_assert_with_condition_and_data_5_(
                    condition, data_1, data_2, data_3, data_4, data_5,
                )
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_assert_with_condition_and_data_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <bool as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.assert()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn assert_with_condition_and_data_6(
        condition: bool,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_assert_with_condition_and_data_6_(
                condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_assert_with_condition_and_data_6_(
            condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(condition);
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let condition = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(condition);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_assert_with_condition_and_data_6_(
                    condition, data_1, data_2, data_3, data_4, data_5, data_6,
                )
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_assert_with_condition_and_data_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(8u32);
        <bool as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.assert()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn assert_with_condition_and_data_7(
        condition: bool,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_assert_with_condition_and_data_7_(
                condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_assert_with_condition_and_data_7_(
            condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(condition);
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let condition = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(condition);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_assert_with_condition_and_data_7_(
                    condition, data_1, data_2, data_3, data_4, data_5, data_6, data_7,
                )
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_clear_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.clear()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/clear)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn clear() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_clear_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_count_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.count()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/count)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn count() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_count_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_count_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_count_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_count_with_label_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&str as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.count()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/count)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn count_with_label(label: &str) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_count_with_label_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_count_with_label_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                __widl_f_count_with_label_(label)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_count_reset_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.countReset()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/countReset)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn count_reset() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_count_reset_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_count_reset_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_count_reset_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_count_reset_with_label_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&str as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.countReset()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/countReset)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn count_reset_with_label(label: &str) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_count_reset_with_label_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_count_reset_with_label_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                __widl_f_count_reset_with_label_(label)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_debug_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.debug()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn debug(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_debug_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_debug_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_debug_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_debug_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.debug()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn debug_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_debug_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_debug_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_debug_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_debug_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.debug()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn debug_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_debug_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_debug_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_debug_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_debug_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.debug()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn debug_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_debug_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_debug_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_debug_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_debug_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.debug()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn debug_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_debug_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_debug_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_debug_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_debug_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.debug()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn debug_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_debug_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_debug_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_debug_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_debug_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.debug()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn debug_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_debug_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_debug_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_debug_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_debug_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.debug()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn debug_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_debug_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_debug_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_debug_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_debug_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.debug()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn debug_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_debug_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_debug_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_debug_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dir_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dir()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dir(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dir_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dir_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_dir_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dir_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dir()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dir_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dir_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dir_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_dir_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dir_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dir()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dir_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dir_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dir_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_dir_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dir_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dir()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dir_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dir_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dir_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_dir_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dir_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dir()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dir_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dir_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dir_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_dir_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dir_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dir()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dir_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dir_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dir_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_dir_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dir_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dir()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dir_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dir_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dir_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_dir_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dir_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dir()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dir_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dir_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dir_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_dir_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dir_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dir()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dir_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dir_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dir_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_dir_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dirxml_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dirxml()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dirxml(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dirxml_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dirxml_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_dirxml_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dirxml_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dirxml()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dirxml_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dirxml_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dirxml_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_dirxml_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dirxml_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dirxml()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dirxml_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dirxml_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dirxml_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_dirxml_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dirxml_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dirxml()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dirxml_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dirxml_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dirxml_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_dirxml_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dirxml_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dirxml()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dirxml_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dirxml_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dirxml_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_dirxml_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dirxml_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dirxml()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dirxml_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dirxml_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dirxml_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_dirxml_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dirxml_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dirxml()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dirxml_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dirxml_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dirxml_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_dirxml_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dirxml_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dirxml()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dirxml_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dirxml_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dirxml_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_dirxml_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dirxml_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dirxml()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dirxml_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dirxml_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dirxml_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_dirxml_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_error_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.error()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn error(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_error_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_error_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.error()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn error_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_error_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_error_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.error()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn error_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_error_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_error_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.error()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn error_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_error_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_error_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.error()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn error_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_error_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_error_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.error()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn error_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_error_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_error_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.error()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn error_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_error_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_error_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.error()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn error_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_error_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_error_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.error()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn error_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_error_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_exception_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.exception()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn exception(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exception_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exception_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_exception_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_exception_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.exception()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn exception_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exception_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exception_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_exception_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_exception_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.exception()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn exception_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exception_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exception_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_exception_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_exception_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.exception()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn exception_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exception_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exception_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_exception_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_exception_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.exception()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn exception_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exception_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exception_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_exception_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_exception_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.exception()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn exception_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exception_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exception_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_exception_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_exception_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.exception()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn exception_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exception_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exception_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_exception_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_exception_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.exception()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn exception_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exception_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exception_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_exception_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_exception_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.exception()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn exception_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exception_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exception_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_exception_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.group()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_group_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.group()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_group_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.group()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_group_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.group()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_group_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.group()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_group_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.group()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_group_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.group()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_group_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.group()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_group_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.group()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_group_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_collapsed_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.groupCollapsed()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_collapsed(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_collapsed_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_collapsed_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_group_collapsed_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_collapsed_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.groupCollapsed()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_collapsed_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_collapsed_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_collapsed_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_group_collapsed_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_collapsed_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.groupCollapsed()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_collapsed_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_collapsed_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_collapsed_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_group_collapsed_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_collapsed_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.groupCollapsed()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_collapsed_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_collapsed_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_collapsed_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_group_collapsed_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_collapsed_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.groupCollapsed()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_collapsed_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_collapsed_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_collapsed_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_group_collapsed_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_collapsed_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.groupCollapsed()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_collapsed_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_collapsed_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_collapsed_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_group_collapsed_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_collapsed_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.groupCollapsed()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_collapsed_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_collapsed_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_collapsed_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_group_collapsed_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_collapsed_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.groupCollapsed()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_collapsed_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_collapsed_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_collapsed_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_group_collapsed_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_collapsed_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.groupCollapsed()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_collapsed_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_collapsed_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_collapsed_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_group_collapsed_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_end_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.groupEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_end() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_end_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_end_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_group_end_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_info_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.info()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn info(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_info_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_info_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_info_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_info_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.info()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn info_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_info_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_info_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_info_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_info_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.info()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn info_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_info_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_info_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_info_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_info_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.info()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn info_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_info_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_info_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_info_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_info_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.info()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn info_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_info_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_info_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_info_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_info_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.info()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn info_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_info_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_info_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_info_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_info_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.info()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn info_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_info_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_info_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_info_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_info_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.info()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn info_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_info_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_info_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_info_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_info_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.info()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn info_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_info_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_info_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_info_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_log_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.log()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn log(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_log_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_log_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_log_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_log_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.log()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn log_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_log_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_log_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_log_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_log_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.log()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn log_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_log_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_log_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_log_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_log_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.log()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn log_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_log_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_log_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_log_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_log_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.log()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn log_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_log_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_log_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_log_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_log_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.log()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn log_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_log_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_log_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_log_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_log_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.log()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn log_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_log_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_log_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_log_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_log_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.log()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn log_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_log_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_log_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_log_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_log_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.log()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn log_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_log_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_log_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_log_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profile()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_profile_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profile()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_profile_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profile()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_profile_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profile()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_profile_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profile()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_profile_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profile()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_profile_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profile()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_profile_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profile()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_profile_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profile()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_profile_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_end_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profileEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_end(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_end_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_end_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_profile_end_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_end_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profileEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_end_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_end_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_end_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_profile_end_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_end_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profileEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_end_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_end_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_end_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_profile_end_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_end_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profileEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_end_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_end_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_end_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_profile_end_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_end_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profileEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_end_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_end_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_end_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_profile_end_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_end_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profileEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_end_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_end_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_end_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_profile_end_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_end_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profileEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_end_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_end_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_end_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_profile_end_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_end_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profileEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_end_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_end_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_end_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_profile_end_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_end_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profileEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_end_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_end_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_end_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_profile_end_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_table_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.table()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn table(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_table_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_table_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_table_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_table_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.table()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn table_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_table_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_table_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_table_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_table_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.table()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn table_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_table_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_table_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_table_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_table_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.table()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn table_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_table_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_table_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_table_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_table_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.table()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn table_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_table_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_table_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_table_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_table_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.table()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn table_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_table_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_table_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_table_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_table_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.table()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn table_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_table_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_table_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_table_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_table_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.table()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn table_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_table_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_table_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_table_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_table_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.table()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn table_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_table_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_table_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_table_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.time()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/time)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_time_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_with_label_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&str as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.time()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/time)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_with_label(label: &str) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_with_label_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_with_label_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                __widl_f_time_with_label_(label)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_end_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_end() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_end_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_end_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_time_end_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_end_with_label_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&str as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_end_with_label(label: &str) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_end_with_label_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_end_with_label_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                __widl_f_time_end_with_label_(label)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_log_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeLog()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_log() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_log_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_log_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_time_log_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_log_with_label_and_data_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&str as WasmDescribe>::describe();
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeLog()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_log_with_label_and_data(label: &str, data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_log_with_label_and_data_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_log_with_label_and_data_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_time_log_with_label_and_data_(label, data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_log_with_label_and_data_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&str as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeLog()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_log_with_label_and_data_0(label: &str) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_log_with_label_and_data_0_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_log_with_label_and_data_0_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                __widl_f_time_log_with_label_and_data_0_(label)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_log_with_label_and_data_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&str as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeLog()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_log_with_label_and_data_1(label: &str, data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_log_with_label_and_data_1_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_log_with_label_and_data_1_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_time_log_with_label_and_data_1_(label, data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_log_with_label_and_data_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&str as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeLog()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_log_with_label_and_data_2(
        label: &str,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_log_with_label_and_data_2_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_log_with_label_and_data_2_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_time_log_with_label_and_data_2_(label, data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_log_with_label_and_data_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&str as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeLog()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_log_with_label_and_data_3(
        label: &str,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_log_with_label_and_data_3_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_log_with_label_and_data_3_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_time_log_with_label_and_data_3_(label, data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_log_with_label_and_data_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&str as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeLog()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_log_with_label_and_data_4(
        label: &str,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_log_with_label_and_data_4_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_log_with_label_and_data_4_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_time_log_with_label_and_data_4_(label, data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_log_with_label_and_data_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&str as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeLog()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_log_with_label_and_data_5(
        label: &str,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_log_with_label_and_data_5_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_log_with_label_and_data_5_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_time_log_with_label_and_data_5_(
                    label, data_1, data_2, data_3, data_4, data_5,
                )
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_log_with_label_and_data_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&str as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeLog()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_log_with_label_and_data_6(
        label: &str,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_log_with_label_and_data_6_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_log_with_label_and_data_6_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_time_log_with_label_and_data_6_(
                    label, data_1, data_2, data_3, data_4, data_5, data_6,
                )
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_log_with_label_and_data_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(8u32);
        <&str as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeLog()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_log_with_label_and_data_7(
        label: &str,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_log_with_label_and_data_7_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_log_with_label_and_data_7_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_time_log_with_label_and_data_7_(
                    label, data_1, data_2, data_3, data_4, data_5, data_6, data_7,
                )
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_stamp_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeStamp()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeStamp)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_stamp() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_stamp_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_stamp_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_time_stamp_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_stamp_with_data_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeStamp()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeStamp)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_stamp_with_data(data: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_stamp_with_data_(
                data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_stamp_with_data_(
            data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                __widl_f_time_stamp_with_data_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_trace_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.trace()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn trace(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_trace_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_trace_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_trace_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_trace_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.trace()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn trace_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_trace_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_trace_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_trace_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_trace_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.trace()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn trace_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_trace_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_trace_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_trace_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_trace_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.trace()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn trace_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_trace_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_trace_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_trace_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_trace_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.trace()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn trace_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_trace_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_trace_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_trace_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_trace_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.trace()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn trace_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_trace_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_trace_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_trace_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_trace_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.trace()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn trace_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_trace_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_trace_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_trace_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_trace_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.trace()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn trace_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_trace_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_trace_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_trace_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_trace_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.trace()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn trace_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_trace_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_trace_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_trace_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_warn_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.warn()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn warn(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_warn_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_warn_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_warn_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_warn_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.warn()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn warn_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_warn_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_warn_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_warn_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_warn_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.warn()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn warn_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_warn_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_warn_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_warn_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_warn_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.warn()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn warn_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_warn_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_warn_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_warn_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_warn_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.warn()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn warn_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_warn_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_warn_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_warn_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_warn_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.warn()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn warn_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_warn_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_warn_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_warn_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_warn_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.warn()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn warn_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_warn_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_warn_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_warn_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_warn_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.warn()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn warn_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_warn_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_warn_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_warn_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_warn_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.warn()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn warn_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_warn_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_warn_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_warn_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[allow(non_upper_case_globals)]
    #[cfg(target_arch = "wasm32")]
    #[link_section = "__wasm_bindgen_unstable"]
    #[doc(hidden)]
    #[allow(clippy::all)]
    pub static __WASM_BINDGEN_GENERATED_bf0b75a0eee8b17e: [u8; 10774usize] = {
        static _INCLUDED_FILES: &[&str] = &[];
        * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xD4)\0\0\0\0\x9E\x01\0\x01\x07console\0\x10__widl_f_assert_\0\0\0\0\x01\0\x06assert\0\x01\x07console\0(__widl_f_assert_with_condition_and_data_\0\x01\0\0\x01\x02\tcondition\x04data\x06assert\0\x01\x07console\0*__widl_f_assert_with_condition_and_data_0_\0\0\0\0\x01\x01\tcondition\x06assert\0\x01\x07console\0*__widl_f_assert_with_condition_and_data_1_\0\0\0\0\x01\x02\tcondition\x06data_1\x06assert\0\x01\x07console\0*__widl_f_assert_with_condition_and_data_2_\0\0\0\0\x01\x03\tcondition\x06data_1\x06data_2\x06assert\0\x01\x07console\0*__widl_f_assert_with_condition_and_data_3_\0\0\0\0\x01\x04\tcondition\x06data_1\x06data_2\x06data_3\x06assert\0\x01\x07console\0*__widl_f_assert_with_condition_and_data_4_\0\0\0\0\x01\x05\tcondition\x06data_1\x06data_2\x06data_3\x06data_4\x06assert\0\x01\x07console\0*__widl_f_assert_with_condition_and_data_5_\0\0\0\0\x01\x06\tcondition\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06assert\0\x01\x07console\0*__widl_f_assert_with_condition_and_data_6_\0\0\0\0\x01\x07\tcondition\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06assert\0\x01\x07console\0*__widl_f_assert_with_condition_and_data_7_\0\0\0\0\x01\x08\tcondition\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x06assert\0\x01\x07console\0\x0F__widl_f_clear_\0\0\0\0\x01\0\x05clear\0\x01\x07console\0\x0F__widl_f_count_\0\0\0\0\x01\0\x05count\0\x01\x07console\0\x1A__widl_f_count_with_label_\0\0\0\0\x01\x01\x05label\x05count\0\x01\x07console\0\x15__widl_f_count_reset_\0\0\0\0\x01\0\ncountReset\0\x01\x07console\0 __widl_f_count_reset_with_label_\0\0\0\0\x01\x01\x05label\ncountReset\0\x01\x07console\0\x0F__widl_f_debug_\0\x01\0\0\x01\x01\x04data\x05debug\0\x01\x07console\0\x11__widl_f_debug_0_\0\0\0\0\x01\0\x05debug\0\x01\x07console\0\x11__widl_f_debug_1_\0\0\0\0\x01\x01\x06data_1\x05debug\0\x01\x07console\0\x11__widl_f_debug_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x05debug\0\x01\x07console\0\x11__widl_f_debug_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x05debug\0\x01\x07console\0\x11__widl_f_debug_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x05debug\0\x01\x07console\0\x11__widl_f_debug_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x05debug\0\x01\x07console\0\x11__widl_f_debug_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x05debug\0\x01\x07console\0\x11__widl_f_debug_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x05debug\0\x01\x07console\0\r__widl_f_dir_\0\x01\0\0\x01\x01\x04data\x03dir\0\x01\x07console\0\x0F__widl_f_dir_0_\0\0\0\0\x01\0\x03dir\0\x01\x07console\0\x0F__widl_f_dir_1_\0\0\0\0\x01\x01\x06data_1\x03dir\0\x01\x07console\0\x0F__widl_f_dir_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x03dir\0\x01\x07console\0\x0F__widl_f_dir_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x03dir\0\x01\x07console\0\x0F__widl_f_dir_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x03dir\0\x01\x07console\0\x0F__widl_f_dir_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x03dir\0\x01\x07console\0\x0F__widl_f_dir_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x03dir\0\x01\x07console\0\x0F__widl_f_dir_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x03dir\0\x01\x07console\0\x10__widl_f_dirxml_\0\x01\0\0\x01\x01\x04data\x06dirxml\0\x01\x07console\0\x12__widl_f_dirxml_0_\0\0\0\0\x01\0\x06dirxml\0\x01\x07console\0\x12__widl_f_dirxml_1_\0\0\0\0\x01\x01\x06data_1\x06dirxml\0\x01\x07console\0\x12__widl_f_dirxml_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x06dirxml\0\x01\x07console\0\x12__widl_f_dirxml_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x06dirxml\0\x01\x07console\0\x12__widl_f_dirxml_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x06dirxml\0\x01\x07console\0\x12__widl_f_dirxml_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06dirxml\0\x01\x07console\0\x12__widl_f_dirxml_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06dirxml\0\x01\x07console\0\x12__widl_f_dirxml_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x06dirxml\0\x01\x07console\0\x0F__widl_f_error_\0\x01\0\0\x01\x01\x04data\x05error\0\x01\x07console\0\x11__widl_f_error_0_\0\0\0\0\x01\0\x05error\0\x01\x07console\0\x11__widl_f_error_1_\0\0\0\0\x01\x01\x06data_1\x05error\0\x01\x07console\0\x11__widl_f_error_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x05error\0\x01\x07console\0\x11__widl_f_error_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x05error\0\x01\x07console\0\x11__widl_f_error_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x05error\0\x01\x07console\0\x11__widl_f_error_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x05error\0\x01\x07console\0\x11__widl_f_error_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x05error\0\x01\x07console\0\x11__widl_f_error_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x05error\0\x01\x07console\0\x13__widl_f_exception_\0\x01\0\0\x01\x01\x04data\texception\0\x01\x07console\0\x15__widl_f_exception_0_\0\0\0\0\x01\0\texception\0\x01\x07console\0\x15__widl_f_exception_1_\0\0\0\0\x01\x01\x06data_1\texception\0\x01\x07console\0\x15__widl_f_exception_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\texception\0\x01\x07console\0\x15__widl_f_exception_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\texception\0\x01\x07console\0\x15__widl_f_exception_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\texception\0\x01\x07console\0\x15__widl_f_exception_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\texception\0\x01\x07console\0\x15__widl_f_exception_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\texception\0\x01\x07console\0\x15__widl_f_exception_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\texception\0\x01\x07console\0\x0F__widl_f_group_\0\x01\0\0\x01\x01\x04data\x05group\0\x01\x07console\0\x11__widl_f_group_0_\0\0\0\0\x01\0\x05group\0\x01\x07console\0\x11__widl_f_group_1_\0\0\0\0\x01\x01\x06data_1\x05group\0\x01\x07console\0\x11__widl_f_group_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x05group\0\x01\x07console\0\x11__widl_f_group_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x05group\0\x01\x07console\0\x11__widl_f_group_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x05group\0\x01\x07console\0\x11__widl_f_group_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x05group\0\x01\x07console\0\x11__widl_f_group_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x05group\0\x01\x07console\0\x11__widl_f_group_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x05group\0\x01\x07console\0\x19__widl_f_group_collapsed_\0\x01\0\0\x01\x01\x04data\x0EgroupCollapsed\0\x01\x07console\0\x1B__widl_f_group_collapsed_0_\0\0\0\0\x01\0\x0EgroupCollapsed\0\x01\x07console\0\x1B__widl_f_group_collapsed_1_\0\0\0\0\x01\x01\x06data_1\x0EgroupCollapsed\0\x01\x07console\0\x1B__widl_f_group_collapsed_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x0EgroupCollapsed\0\x01\x07console\0\x1B__widl_f_group_collapsed_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x0EgroupCollapsed\0\x01\x07console\0\x1B__widl_f_group_collapsed_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x0EgroupCollapsed\0\x01\x07console\0\x1B__widl_f_group_collapsed_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x0EgroupCollapsed\0\x01\x07console\0\x1B__widl_f_group_collapsed_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x0EgroupCollapsed\0\x01\x07console\0\x1B__widl_f_group_collapsed_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x0EgroupCollapsed\0\x01\x07console\0\x13__widl_f_group_end_\0\0\0\0\x01\0\x08groupEnd\0\x01\x07console\0\x0E__widl_f_info_\0\x01\0\0\x01\x01\x04data\x04info\0\x01\x07console\0\x10__widl_f_info_0_\0\0\0\0\x01\0\x04info\0\x01\x07console\0\x10__widl_f_info_1_\0\0\0\0\x01\x01\x06data_1\x04info\0\x01\x07console\0\x10__widl_f_info_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x04info\0\x01\x07console\0\x10__widl_f_info_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x04info\0\x01\x07console\0\x10__widl_f_info_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x04info\0\x01\x07console\0\x10__widl_f_info_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x04info\0\x01\x07console\0\x10__widl_f_info_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x04info\0\x01\x07console\0\x10__widl_f_info_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x04info\0\x01\x07console\0\r__widl_f_log_\0\x01\0\0\x01\x01\x04data\x03log\0\x01\x07console\0\x0F__widl_f_log_0_\0\0\0\0\x01\0\x03log\0\x01\x07console\0\x0F__widl_f_log_1_\0\0\0\0\x01\x01\x06data_1\x03log\0\x01\x07console\0\x0F__widl_f_log_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x03log\0\x01\x07console\0\x0F__widl_f_log_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x03log\0\x01\x07console\0\x0F__widl_f_log_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x03log\0\x01\x07console\0\x0F__widl_f_log_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x03log\0\x01\x07console\0\x0F__widl_f_log_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x03log\0\x01\x07console\0\x0F__widl_f_log_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x03log\0\x01\x07console\0\x11__widl_f_profile_\0\x01\0\0\x01\x01\x04data\x07profile\0\x01\x07console\0\x13__widl_f_profile_0_\0\0\0\0\x01\0\x07profile\0\x01\x07console\0\x13__widl_f_profile_1_\0\0\0\0\x01\x01\x06data_1\x07profile\0\x01\x07console\0\x13__widl_f_profile_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x07profile\0\x01\x07console\0\x13__widl_f_profile_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x07profile\0\x01\x07console\0\x13__widl_f_profile_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x07profile\0\x01\x07console\0\x13__widl_f_profile_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x07profile\0\x01\x07console\0\x13__widl_f_profile_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x07profile\0\x01\x07console\0\x13__widl_f_profile_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x07profile\0\x01\x07console\0\x15__widl_f_profile_end_\0\x01\0\0\x01\x01\x04data\nprofileEnd\0\x01\x07console\0\x17__widl_f_profile_end_0_\0\0\0\0\x01\0\nprofileEnd\0\x01\x07console\0\x17__widl_f_profile_end_1_\0\0\0\0\x01\x01\x06data_1\nprofileEnd\0\x01\x07console\0\x17__widl_f_profile_end_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\nprofileEnd\0\x01\x07console\0\x17__widl_f_profile_end_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\nprofileEnd\0\x01\x07console\0\x17__widl_f_profile_end_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\nprofileEnd\0\x01\x07console\0\x17__widl_f_profile_end_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\nprofileEnd\0\x01\x07console\0\x17__widl_f_profile_end_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\nprofileEnd\0\x01\x07console\0\x17__widl_f_profile_end_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\nprofileEnd\0\x01\x07console\0\x0F__widl_f_table_\0\x01\0\0\x01\x01\x04data\x05table\0\x01\x07console\0\x11__widl_f_table_0_\0\0\0\0\x01\0\x05table\0\x01\x07console\0\x11__widl_f_table_1_\0\0\0\0\x01\x01\x06data_1\x05table\0\x01\x07console\0\x11__widl_f_table_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x05table\0\x01\x07console\0\x11__widl_f_table_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x05table\0\x01\x07console\0\x11__widl_f_table_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x05table\0\x01\x07console\0\x11__widl_f_table_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x05table\0\x01\x07console\0\x11__widl_f_table_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x05table\0\x01\x07console\0\x11__widl_f_table_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x05table\0\x01\x07console\0\x0E__widl_f_time_\0\0\0\0\x01\0\x04time\0\x01\x07console\0\x19__widl_f_time_with_label_\0\0\0\0\x01\x01\x05label\x04time\0\x01\x07console\0\x12__widl_f_time_end_\0\0\0\0\x01\0\x07timeEnd\0\x01\x07console\0\x1D__widl_f_time_end_with_label_\0\0\0\0\x01\x01\x05label\x07timeEnd\0\x01\x07console\0\x12__widl_f_time_log_\0\0\0\0\x01\0\x07timeLog\0\x01\x07console\0&__widl_f_time_log_with_label_and_data_\0\x01\0\0\x01\x02\x05label\x04data\x07timeLog\0\x01\x07console\0(__widl_f_time_log_with_label_and_data_0_\0\0\0\0\x01\x01\x05label\x07timeLog\0\x01\x07console\0(__widl_f_time_log_with_label_and_data_1_\0\0\0\0\x01\x02\x05label\x06data_1\x07timeLog\0\x01\x07console\0(__widl_f_time_log_with_label_and_data_2_\0\0\0\0\x01\x03\x05label\x06data_1\x06data_2\x07timeLog\0\x01\x07console\0(__widl_f_time_log_with_label_and_data_3_\0\0\0\0\x01\x04\x05label\x06data_1\x06data_2\x06data_3\x07timeLog\0\x01\x07console\0(__widl_f_time_log_with_label_and_data_4_\0\0\0\0\x01\x05\x05label\x06data_1\x06data_2\x06data_3\x06data_4\x07timeLog\0\x01\x07console\0(__widl_f_time_log_with_label_and_data_5_\0\0\0\0\x01\x06\x05label\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x07timeLog\0\x01\x07console\0(__widl_f_time_log_with_label_and_data_6_\0\0\0\0\x01\x07\x05label\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x07timeLog\0\x01\x07console\0(__widl_f_time_log_with_label_and_data_7_\0\0\0\0\x01\x08\x05label\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x07timeLog\0\x01\x07console\0\x14__widl_f_time_stamp_\0\0\0\0\x01\0\ttimeStamp\0\x01\x07console\0\x1E__widl_f_time_stamp_with_data_\0\0\0\0\x01\x01\x04data\ttimeStamp\0\x01\x07console\0\x0F__widl_f_trace_\0\x01\0\0\x01\x01\x04data\x05trace\0\x01\x07console\0\x11__widl_f_trace_0_\0\0\0\0\x01\0\x05trace\0\x01\x07console\0\x11__widl_f_trace_1_\0\0\0\0\x01\x01\x06data_1\x05trace\0\x01\x07console\0\x11__widl_f_trace_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x05trace\0\x01\x07console\0\x11__widl_f_trace_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x05trace\0\x01\x07console\0\x11__widl_f_trace_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x05trace\0\x01\x07console\0\x11__widl_f_trace_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x05trace\0\x01\x07console\0\x11__widl_f_trace_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x05trace\0\x01\x07console\0\x11__widl_f_trace_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x05trace\0\x01\x07console\0\x0E__widl_f_warn_\0\x01\0\0\x01\x01\x04data\x04warn\0\x01\x07console\0\x10__widl_f_warn_0_\0\0\0\0\x01\0\x04warn\0\x01\x07console\0\x10__widl_f_warn_1_\0\0\0\0\x01\x01\x06data_1\x04warn\0\x01\x07console\0\x10__widl_f_warn_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x04warn\0\x01\x07console\0\x10__widl_f_warn_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x04warn\0\x01\x07console\0\x10__widl_f_warn_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x04warn\0\x01\x07console\0\x10__widl_f_warn_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x04warn\0\x01\x07console\0\x10__widl_f_warn_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x04warn\0\x01\x07console\0\x10__widl_f_warn_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x04warn\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
    };
}
