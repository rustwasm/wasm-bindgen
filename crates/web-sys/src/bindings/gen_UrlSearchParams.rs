use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `URLSearchParams` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams)\n\n*This API requires the following crate features to be activated: `UrlSearchParams`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct UrlSearchParams {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_UrlSearchParams: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for UrlSearchParams {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(85u32);
            inform(82u32);
            inform(76u32);
            inform(83u32);
            inform(101u32);
            inform(97u32);
            inform(114u32);
            inform(99u32);
            inform(104u32);
            inform(80u32);
            inform(97u32);
            inform(114u32);
            inform(97u32);
            inform(109u32);
            inform(115u32);
        }
    }
    impl core::ops::Deref for UrlSearchParams {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for UrlSearchParams {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for UrlSearchParams {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a UrlSearchParams {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for UrlSearchParams {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            UrlSearchParams {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for UrlSearchParams {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a UrlSearchParams {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for UrlSearchParams {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<UrlSearchParams>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(UrlSearchParams {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for UrlSearchParams {
        #[inline]
        fn from(obj: JsValue) -> UrlSearchParams {
            UrlSearchParams { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for UrlSearchParams {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<UrlSearchParams> for UrlSearchParams {
        #[inline]
        fn as_ref(&self) -> &UrlSearchParams {
            self
        }
    }
    impl From<UrlSearchParams> for JsValue {
        #[inline]
        fn from(obj: UrlSearchParams) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for UrlSearchParams {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_URLSearchParams(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_URLSearchParams(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_URLSearchParams(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            UrlSearchParams { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const UrlSearchParams) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<UrlSearchParams> for ::js_sys::Object {
    #[inline]
    fn from(obj: UrlSearchParams) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for UrlSearchParams {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "UrlSearchParams",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_URLSearchParams() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <UrlSearchParams as WasmDescribe>::describe();
}
impl UrlSearchParams {
    #[cfg(all(feature = "UrlSearchParams",))]
    #[allow(bad_style)]
    #[doc = "The `new URLSearchParams(..)` constructor, creating a new instance of `URLSearchParams`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/URLSearchParams)\n\n*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<UrlSearchParams, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "UrlSearchParams",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_URLSearchParams(
            ) -> <UrlSearchParams as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_URLSearchParams(
        ) -> <UrlSearchParams as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_URLSearchParams() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<UrlSearchParams as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "UrlSearchParams",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_str_sequence_sequence_URLSearchParams() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <UrlSearchParams as WasmDescribe>::describe();
}
impl UrlSearchParams {
    #[cfg(all(feature = "UrlSearchParams",))]
    #[allow(bad_style)]
    #[doc = "The `new URLSearchParams(..)` constructor, creating a new instance of `URLSearchParams`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/URLSearchParams)\n\n*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    #[allow(clippy::all)]
    pub fn new_with_str_sequence_sequence(
        init: &::wasm_bindgen::JsValue,
    ) -> Result<UrlSearchParams, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "UrlSearchParams",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_str_sequence_sequence_URLSearchParams(
                init: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <UrlSearchParams as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_str_sequence_sequence_URLSearchParams(
            init: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <UrlSearchParams as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(init);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let init =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        init,
                    );
                __widl_f_new_with_str_sequence_sequence_URLSearchParams(init)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<UrlSearchParams as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "UrlSearchParams",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_str_URLSearchParams() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <UrlSearchParams as WasmDescribe>::describe();
}
impl UrlSearchParams {
    #[cfg(all(feature = "UrlSearchParams",))]
    #[allow(bad_style)]
    #[doc = "The `new URLSearchParams(..)` constructor, creating a new instance of `URLSearchParams`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/URLSearchParams)\n\n*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    #[allow(clippy::all)]
    pub fn new_with_str(init: &str) -> Result<UrlSearchParams, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "UrlSearchParams",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_str_URLSearchParams(
                init: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <UrlSearchParams as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_str_URLSearchParams(
            init: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <UrlSearchParams as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(init);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let init = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(init);
                __widl_f_new_with_str_URLSearchParams(init)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<UrlSearchParams as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "UrlSearchParams",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_URLSearchParams() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&UrlSearchParams as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl UrlSearchParams {
    #[cfg(all(feature = "UrlSearchParams",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/append)\n\n*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    #[allow(clippy::all)]
    pub fn append(&self, name: &str, value: &str) {
        #[cfg(all(feature = "UrlSearchParams",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_URLSearchParams(
                self_: <&UrlSearchParams as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_URLSearchParams(
            self_: <&UrlSearchParams as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(name);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&UrlSearchParams as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                let value = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_append_URLSearchParams(self_, name, value)
            };
            ()
        }
    }
}
#[cfg(all(feature = "UrlSearchParams",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_URLSearchParams() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&UrlSearchParams as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl UrlSearchParams {
    #[cfg(all(feature = "UrlSearchParams",))]
    #[allow(bad_style)]
    #[doc = "The `delete()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/delete)\n\n*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    #[allow(clippy::all)]
    pub fn delete(&self, name: &str) {
        #[cfg(all(feature = "UrlSearchParams",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_URLSearchParams(
                self_: <&UrlSearchParams as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_URLSearchParams(
            self_: <&UrlSearchParams as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&UrlSearchParams as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_delete_URLSearchParams(self_, name)
            };
            ()
        }
    }
}
#[cfg(all(feature = "UrlSearchParams",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_URLSearchParams() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&UrlSearchParams as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl UrlSearchParams {
    #[cfg(all(feature = "UrlSearchParams",))]
    #[allow(bad_style)]
    #[doc = "The `get()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/get)\n\n*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    #[allow(clippy::all)]
    pub fn get(&self, name: &str) -> Option<String> {
        #[cfg(all(feature = "UrlSearchParams",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_URLSearchParams(
                self_: <&UrlSearchParams as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_URLSearchParams(
            self_: <&UrlSearchParams as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&UrlSearchParams as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_get_URLSearchParams(self_, name)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "UrlSearchParams",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_all_URLSearchParams() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&UrlSearchParams as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl UrlSearchParams {
    #[cfg(all(feature = "UrlSearchParams",))]
    #[allow(bad_style)]
    #[doc = "The `getAll()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/getAll)\n\n*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    #[allow(clippy::all)]
    pub fn get_all(&self, name: &str) -> ::js_sys::Array {
        #[cfg(all(feature = "UrlSearchParams",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_all_URLSearchParams(
                self_: <&UrlSearchParams as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_all_URLSearchParams(
            self_: <&UrlSearchParams as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&UrlSearchParams as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_get_all_URLSearchParams(self_, name)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "UrlSearchParams",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_has_URLSearchParams() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&UrlSearchParams as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl UrlSearchParams {
    #[cfg(all(feature = "UrlSearchParams",))]
    #[allow(bad_style)]
    #[doc = "The `has()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/has)\n\n*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    #[allow(clippy::all)]
    pub fn has(&self, name: &str) -> bool {
        #[cfg(all(feature = "UrlSearchParams",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_has_URLSearchParams(
                self_: <&UrlSearchParams as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_has_URLSearchParams(
            self_: <&UrlSearchParams as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&UrlSearchParams as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_has_URLSearchParams(self_, name)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "UrlSearchParams",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_URLSearchParams() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&UrlSearchParams as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl UrlSearchParams {
    #[cfg(all(feature = "UrlSearchParams",))]
    #[allow(bad_style)]
    #[doc = "The `set()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/set)\n\n*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    #[allow(clippy::all)]
    pub fn set(&self, name: &str, value: &str) {
        #[cfg(all(feature = "UrlSearchParams",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_URLSearchParams(
                self_: <&UrlSearchParams as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_URLSearchParams(
            self_: <&UrlSearchParams as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(name);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&UrlSearchParams as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                let value = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_set_URLSearchParams(self_, name, value)
            };
            ()
        }
    }
}
#[cfg(all(feature = "UrlSearchParams",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_sort_URLSearchParams() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&UrlSearchParams as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl UrlSearchParams {
    #[cfg(all(feature = "UrlSearchParams",))]
    #[allow(bad_style)]
    #[doc = "The `sort()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/sort)\n\n*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    #[allow(clippy::all)]
    pub fn sort(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "UrlSearchParams",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_sort_URLSearchParams(
                self_: <&UrlSearchParams as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_sort_URLSearchParams(
            self_: <&UrlSearchParams as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&UrlSearchParams as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_sort_URLSearchParams(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_25a4a8ab1a929ac4: [u8; 914usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}P\x03\0\0\0\0\x0B\0\0\x02\x0FURLSearchParams!__widl_instanceof_URLSearchParams\0\0\0\0\x1C__widl_f_new_URLSearchParams\x01\0\0\x01\x0FURLSearchParams\0\x01\0\x03new\0\0\07__widl_f_new_with_str_sequence_sequence_URLSearchParams\x01\0\0\x01\x0FURLSearchParams\0\x01\x01\x04init\x03new\0\0\0%__widl_f_new_with_str_URLSearchParams\x01\0\0\x01\x0FURLSearchParams\0\x01\x01\x04init\x03new\0\0\0\x1F__widl_f_append_URLSearchParams\0\0\0\x01\x0FURLSearchParams\x01\0\0\x01\x03\x05self_\x04name\x05value\x06append\0\0\0\x1F__widl_f_delete_URLSearchParams\0\0\0\x01\x0FURLSearchParams\x01\0\0\x01\x02\x05self_\x04name\x06delete\0\0\0\x1C__widl_f_get_URLSearchParams\0\0\0\x01\x0FURLSearchParams\x01\0\0\x01\x02\x05self_\x04name\x03get\0\0\0 __widl_f_get_all_URLSearchParams\0\0\0\x01\x0FURLSearchParams\x01\0\0\x01\x02\x05self_\x04name\x06getAll\0\0\0\x1C__widl_f_has_URLSearchParams\0\0\0\x01\x0FURLSearchParams\x01\0\0\x01\x02\x05self_\x04name\x03has\0\0\0\x1C__widl_f_set_URLSearchParams\0\0\0\x01\x0FURLSearchParams\x01\0\0\x01\x03\x05self_\x04name\x05value\x03set\0\0\0\x1D__widl_f_sort_URLSearchParams\x01\0\0\x01\x0FURLSearchParams\x01\0\0\x01\x01\x05self_\x04sort\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
