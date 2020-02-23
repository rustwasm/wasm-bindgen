use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `DOMException` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException)\n\n*This API requires the following crate features to be activated: `DomException`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct DomException {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_DomException: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for DomException {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
            inform(68u32);
            inform(79u32);
            inform(77u32);
            inform(69u32);
            inform(120u32);
            inform(99u32);
            inform(101u32);
            inform(112u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for DomException {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for DomException {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for DomException {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a DomException {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for DomException {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            DomException {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for DomException {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a DomException {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for DomException {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<DomException>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(DomException {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for DomException {
        #[inline]
        fn from(obj: JsValue) -> DomException {
            DomException { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for DomException {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<DomException> for DomException {
        #[inline]
        fn as_ref(&self) -> &DomException {
            self
        }
    }
    impl From<DomException> for JsValue {
        #[inline]
        fn from(obj: DomException) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for DomException {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_DOMException(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_DOMException(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_DOMException(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            DomException { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const DomException) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<DomException> for ::js_sys::Object {
    #[inline]
    fn from(obj: DomException) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for DomException {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "DomException",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_DOMException() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <DomException as WasmDescribe>::describe();
}
impl DomException {
    #[cfg(all(feature = "DomException",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMException(..)` constructor, creating a new instance of `DOMException`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/DOMException)\n\n*This API requires the following crate features to be activated: `DomException`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<DomException, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomException",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_DOMException(
            ) -> <DomException as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_DOMException(
        ) -> <DomException as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_DOMException() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomException as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DomException",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_message_DOMException() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <DomException as WasmDescribe>::describe();
}
impl DomException {
    #[cfg(all(feature = "DomException",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMException(..)` constructor, creating a new instance of `DOMException`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/DOMException)\n\n*This API requires the following crate features to be activated: `DomException`*"]
    #[allow(clippy::all)]
    pub fn new_with_message(message: &str) -> Result<DomException, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomException",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_message_DOMException(
                message: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomException as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_message_DOMException(
            message: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomException as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(message);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let message = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(message);
                __widl_f_new_with_message_DOMException(message)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomException as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DomException",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_message_and_name_DOMException() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <DomException as WasmDescribe>::describe();
}
impl DomException {
    #[cfg(all(feature = "DomException",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMException(..)` constructor, creating a new instance of `DOMException`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/DOMException)\n\n*This API requires the following crate features to be activated: `DomException`*"]
    #[allow(clippy::all)]
    pub fn new_with_message_and_name(
        message: &str,
        name: &str,
    ) -> Result<DomException, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomException",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_message_and_name_DOMException(
                message: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomException as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_message_and_name_DOMException(
            message: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomException as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(message);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let message = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(message);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_new_with_message_and_name_DOMException(message, name)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomException as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DomException",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_DOMException() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomException as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl DomException {
    #[cfg(all(feature = "DomException",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/name)\n\n*This API requires the following crate features to be activated: `DomException`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "DomException",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_DOMException(
                self_: <&DomException as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_DOMException(
            self_: <&DomException as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomException as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_DOMException(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomException",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_message_DOMException() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomException as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl DomException {
    #[cfg(all(feature = "DomException",))]
    #[allow(bad_style)]
    #[doc = "The `message` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/message)\n\n*This API requires the following crate features to be activated: `DomException`*"]
    #[allow(clippy::all)]
    pub fn message(&self) -> String {
        #[cfg(all(feature = "DomException",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_message_DOMException(
                self_: <&DomException as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_message_DOMException(
            self_: <&DomException as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomException as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_message_DOMException(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomException",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_code_DOMException() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomException as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl DomException {
    #[cfg(all(feature = "DomException",))]
    #[allow(bad_style)]
    #[doc = "The `code` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/code)\n\n*This API requires the following crate features to be activated: `DomException`*"]
    #[allow(clippy::all)]
    pub fn code(&self) -> u16 {
        #[cfg(all(feature = "DomException",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_code_DOMException(
                self_: <&DomException as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_code_DOMException(
            self_: <&DomException as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomException as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_code_DOMException(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomException",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_result_DOMException() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomException as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl DomException {
    #[cfg(all(feature = "DomException",))]
    #[allow(bad_style)]
    #[doc = "The `result` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/result)\n\n*This API requires the following crate features to be activated: `DomException`*"]
    #[allow(clippy::all)]
    pub fn result(&self) -> u32 {
        #[cfg(all(feature = "DomException",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_result_DOMException(
                self_: <&DomException as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_result_DOMException(
            self_: <&DomException as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomException as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_result_DOMException(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomException",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_filename_DOMException() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomException as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl DomException {
    #[cfg(all(feature = "DomException",))]
    #[allow(bad_style)]
    #[doc = "The `filename` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/filename)\n\n*This API requires the following crate features to be activated: `DomException`*"]
    #[allow(clippy::all)]
    pub fn filename(&self) -> String {
        #[cfg(all(feature = "DomException",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_filename_DOMException(
                self_: <&DomException as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_filename_DOMException(
            self_: <&DomException as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomException as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_filename_DOMException(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomException",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_line_number_DOMException() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomException as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl DomException {
    #[cfg(all(feature = "DomException",))]
    #[allow(bad_style)]
    #[doc = "The `lineNumber` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/lineNumber)\n\n*This API requires the following crate features to be activated: `DomException`*"]
    #[allow(clippy::all)]
    pub fn line_number(&self) -> u32 {
        #[cfg(all(feature = "DomException",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_line_number_DOMException(
                self_: <&DomException as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_line_number_DOMException(
            self_: <&DomException as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomException as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_line_number_DOMException(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomException",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_column_number_DOMException() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomException as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl DomException {
    #[cfg(all(feature = "DomException",))]
    #[allow(bad_style)]
    #[doc = "The `columnNumber` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/columnNumber)\n\n*This API requires the following crate features to be activated: `DomException`*"]
    #[allow(clippy::all)]
    pub fn column_number(&self) -> u32 {
        #[cfg(all(feature = "DomException",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_column_number_DOMException(
                self_: <&DomException as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_column_number_DOMException(
            self_: <&DomException as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomException as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_column_number_DOMException(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomException",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_data_DOMException() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomException as WasmDescribe>::describe();
    <Option<::js_sys::Object> as WasmDescribe>::describe();
}
impl DomException {
    #[cfg(all(feature = "DomException",))]
    #[allow(bad_style)]
    #[doc = "The `data` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/data)\n\n*This API requires the following crate features to be activated: `DomException`*"]
    #[allow(clippy::all)]
    pub fn data(&self) -> Option<::js_sys::Object> {
        #[cfg(all(feature = "DomException",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_data_DOMException(
                self_: <&DomException as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_data_DOMException(
            self_: <&DomException as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomException as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_data_DOMException(self_)
            };
            <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomException",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stack_DOMException() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomException as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl DomException {
    #[cfg(all(feature = "DomException",))]
    #[allow(bad_style)]
    #[doc = "The `stack` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/stack)\n\n*This API requires the following crate features to be activated: `DomException`*"]
    #[allow(clippy::all)]
    pub fn stack(&self) -> String {
        #[cfg(all(feature = "DomException",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stack_DOMException(
                self_: <&DomException as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stack_DOMException(
            self_: <&DomException as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomException as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_stack_DOMException(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
impl DomException {
    pub const INDEX_SIZE_ERR: u16 = 1u64 as u16;
}
impl DomException {
    pub const DOMSTRING_SIZE_ERR: u16 = 2u64 as u16;
}
impl DomException {
    pub const HIERARCHY_REQUEST_ERR: u16 = 3u64 as u16;
}
impl DomException {
    pub const WRONG_DOCUMENT_ERR: u16 = 4u64 as u16;
}
impl DomException {
    pub const INVALID_CHARACTER_ERR: u16 = 5u64 as u16;
}
impl DomException {
    pub const NO_DATA_ALLOWED_ERR: u16 = 6u64 as u16;
}
impl DomException {
    pub const NO_MODIFICATION_ALLOWED_ERR: u16 = 7u64 as u16;
}
impl DomException {
    pub const NOT_FOUND_ERR: u16 = 8u64 as u16;
}
impl DomException {
    pub const NOT_SUPPORTED_ERR: u16 = 9u64 as u16;
}
impl DomException {
    pub const INUSE_ATTRIBUTE_ERR: u16 = 10u64 as u16;
}
impl DomException {
    pub const INVALID_STATE_ERR: u16 = 11u64 as u16;
}
impl DomException {
    pub const SYNTAX_ERR: u16 = 12u64 as u16;
}
impl DomException {
    pub const INVALID_MODIFICATION_ERR: u16 = 13u64 as u16;
}
impl DomException {
    pub const NAMESPACE_ERR: u16 = 14u64 as u16;
}
impl DomException {
    pub const INVALID_ACCESS_ERR: u16 = 15u64 as u16;
}
impl DomException {
    pub const VALIDATION_ERR: u16 = 16u64 as u16;
}
impl DomException {
    pub const TYPE_MISMATCH_ERR: u16 = 17u64 as u16;
}
impl DomException {
    pub const SECURITY_ERR: u16 = 18u64 as u16;
}
impl DomException {
    pub const NETWORK_ERR: u16 = 19u64 as u16;
}
impl DomException {
    pub const ABORT_ERR: u16 = 20u64 as u16;
}
impl DomException {
    pub const URL_MISMATCH_ERR: u16 = 21u64 as u16;
}
impl DomException {
    pub const QUOTA_EXCEEDED_ERR: u16 = 22u64 as u16;
}
impl DomException {
    pub const TIMEOUT_ERR: u16 = 23u64 as u16;
}
impl DomException {
    pub const INVALID_NODE_TYPE_ERR: u16 = 24u64 as u16;
}
impl DomException {
    pub const DATA_CLONE_ERR: u16 = 25u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_8dd8dd476125c9bb: [u8; 1054usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xDC\x03\0\0\0\0\r\0\0\x02\x0CDOMException\x1E__widl_instanceof_DOMException\0\0\0\0\x19__widl_f_new_DOMException\x01\0\0\x01\x0CDOMException\0\x01\0\x03new\0\0\0&__widl_f_new_with_message_DOMException\x01\0\0\x01\x0CDOMException\0\x01\x01\x07message\x03new\0\0\0/__widl_f_new_with_message_and_name_DOMException\x01\0\0\x01\x0CDOMException\0\x01\x02\x07message\x04name\x03new\0\0\0\x1A__widl_f_name_DOMException\0\0\0\x01\x0CDOMException\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0\x1D__widl_f_message_DOMException\0\0\0\x01\x0CDOMException\x01\0\x01\x07message\x01\x01\x05self_\x07message\0\0\0\x1A__widl_f_code_DOMException\0\0\0\x01\x0CDOMException\x01\0\x01\x04code\x01\x01\x05self_\x04code\0\0\0\x1C__widl_f_result_DOMException\0\0\0\x01\x0CDOMException\x01\0\x01\x06result\x01\x01\x05self_\x06result\0\0\0\x1E__widl_f_filename_DOMException\0\0\0\x01\x0CDOMException\x01\0\x01\x08filename\x01\x01\x05self_\x08filename\0\0\0!__widl_f_line_number_DOMException\0\0\0\x01\x0CDOMException\x01\0\x01\nlineNumber\x01\x01\x05self_\nlineNumber\0\0\0#__widl_f_column_number_DOMException\0\0\0\x01\x0CDOMException\x01\0\x01\x0CcolumnNumber\x01\x01\x05self_\x0CcolumnNumber\0\0\0\x1A__widl_f_data_DOMException\0\0\0\x01\x0CDOMException\x01\0\x01\x04data\x01\x01\x05self_\x04data\0\0\0\x1B__widl_f_stack_DOMException\0\0\0\x01\x0CDOMException\x01\0\x01\x05stack\x01\x01\x05self_\x05stack\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
