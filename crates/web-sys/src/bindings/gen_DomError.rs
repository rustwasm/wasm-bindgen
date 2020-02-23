use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `DOMError` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMError)\n\n*This API requires the following crate features to be activated: `DomError`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct DomError {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_DomError: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for DomError {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(8u32);
            inform(68u32);
            inform(79u32);
            inform(77u32);
            inform(69u32);
            inform(114u32);
            inform(114u32);
            inform(111u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for DomError {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for DomError {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for DomError {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a DomError {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for DomError {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            DomError {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for DomError {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a DomError {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for DomError {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<DomError>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(DomError {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for DomError {
        #[inline]
        fn from(obj: JsValue) -> DomError {
            DomError { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for DomError {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<DomError> for DomError {
        #[inline]
        fn as_ref(&self) -> &DomError {
            self
        }
    }
    impl From<DomError> for JsValue {
        #[inline]
        fn from(obj: DomError) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for DomError {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_DOMError(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_DOMError(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_DOMError(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            DomError { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const DomError) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<DomError> for ::js_sys::Object {
    #[inline]
    fn from(obj: DomError) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for DomError {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "DomError",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_DOMError() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <DomError as WasmDescribe>::describe();
}
impl DomError {
    #[cfg(all(feature = "DomError",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMError(..)` constructor, creating a new instance of `DOMError`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMError/DOMError)\n\n*This API requires the following crate features to be activated: `DomError`*"]
    #[allow(clippy::all)]
    pub fn new(name: &str) -> Result<DomError, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomError",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_DOMError(
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomError as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_DOMError(
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomError as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_new_DOMError(name)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomError as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomError",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_message_DOMError() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <DomError as WasmDescribe>::describe();
}
impl DomError {
    #[cfg(all(feature = "DomError",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMError(..)` constructor, creating a new instance of `DOMError`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMError/DOMError)\n\n*This API requires the following crate features to be activated: `DomError`*"]
    #[allow(clippy::all)]
    pub fn new_with_message(
        name: &str,
        message: &str,
    ) -> Result<DomError, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomError",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_message_DOMError(
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                message: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomError as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_message_DOMError(
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            message: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomError as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(name);
            drop(message);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                let message = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(message);
                __widl_f_new_with_message_DOMError(name, message)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomError as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomError",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_DOMError() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomError as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl DomError {
    #[cfg(all(feature = "DomError",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMError/name)\n\n*This API requires the following crate features to be activated: `DomError`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "DomError",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_DOMError(
                self_: <&DomError as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_DOMError(
            self_: <&DomError as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomError as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_DOMError(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomError",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_message_DOMError() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomError as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl DomError {
    #[cfg(all(feature = "DomError",))]
    #[allow(bad_style)]
    #[doc = "The `message` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMError/message)\n\n*This API requires the following crate features to be activated: `DomError`*"]
    #[allow(clippy::all)]
    pub fn message(&self) -> String {
        #[cfg(all(feature = "DomError",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_message_DOMError(
                self_: <&DomError as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_message_DOMError(
            self_: <&DomError as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomError as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_message_DOMError(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_2694ca398a77a26b: [u8; 395usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}I\x01\0\0\0\0\x05\0\0\x02\x08DOMError\x1A__widl_instanceof_DOMError\0\0\0\0\x15__widl_f_new_DOMError\x01\0\0\x01\x08DOMError\0\x01\x01\x04name\x03new\0\0\0\"__widl_f_new_with_message_DOMError\x01\0\0\x01\x08DOMError\0\x01\x02\x04name\x07message\x03new\0\0\0\x16__widl_f_name_DOMError\0\0\0\x01\x08DOMError\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0\x19__widl_f_message_DOMError\0\0\0\x01\x08DOMError\x01\0\x01\x07message\x01\x01\x05self_\x07message\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
