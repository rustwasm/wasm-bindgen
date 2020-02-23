use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MimeType` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeType)\n\n*This API requires the following crate features to be activated: `MimeType`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MimeType {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MimeType: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MimeType {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(8u32);
            inform(77u32);
            inform(105u32);
            inform(109u32);
            inform(101u32);
            inform(84u32);
            inform(121u32);
            inform(112u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for MimeType {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for MimeType {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MimeType {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MimeType {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MimeType {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MimeType {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MimeType {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MimeType {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MimeType {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MimeType>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MimeType {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MimeType {
        #[inline]
        fn from(obj: JsValue) -> MimeType {
            MimeType { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MimeType {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MimeType> for MimeType {
        #[inline]
        fn as_ref(&self) -> &MimeType {
            self
        }
    }
    impl From<MimeType> for JsValue {
        #[inline]
        fn from(obj: MimeType) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MimeType {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MimeType(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MimeType(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MimeType(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MimeType { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MimeType) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MimeType> for ::js_sys::Object {
    #[inline]
    fn from(obj: MimeType) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MimeType {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MimeType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_description_MimeType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MimeType as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl MimeType {
    #[cfg(all(feature = "MimeType",))]
    #[allow(bad_style)]
    #[doc = "The `description` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeType/description)\n\n*This API requires the following crate features to be activated: `MimeType`*"]
    #[allow(clippy::all)]
    pub fn description(&self) -> String {
        #[cfg(all(feature = "MimeType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_description_MimeType(
                self_: <&MimeType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_description_MimeType(
            self_: <&MimeType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MimeType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_description_MimeType(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MimeType", feature = "Plugin",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_enabled_plugin_MimeType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MimeType as WasmDescribe>::describe();
    <Option<Plugin> as WasmDescribe>::describe();
}
impl MimeType {
    #[cfg(all(feature = "MimeType", feature = "Plugin",))]
    #[allow(bad_style)]
    #[doc = "The `enabledPlugin` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeType/enabledPlugin)\n\n*This API requires the following crate features to be activated: `MimeType`, `Plugin`*"]
    #[allow(clippy::all)]
    pub fn enabled_plugin(&self) -> Option<Plugin> {
        #[cfg(all(feature = "MimeType", feature = "Plugin",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_enabled_plugin_MimeType(
                self_: <&MimeType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Plugin> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_enabled_plugin_MimeType(
            self_: <&MimeType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Plugin> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MimeType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_enabled_plugin_MimeType(self_)
            };
            <Option<Plugin> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MimeType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_suffixes_MimeType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MimeType as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl MimeType {
    #[cfg(all(feature = "MimeType",))]
    #[allow(bad_style)]
    #[doc = "The `suffixes` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeType/suffixes)\n\n*This API requires the following crate features to be activated: `MimeType`*"]
    #[allow(clippy::all)]
    pub fn suffixes(&self) -> String {
        #[cfg(all(feature = "MimeType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_suffixes_MimeType(
                self_: <&MimeType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_suffixes_MimeType(
            self_: <&MimeType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MimeType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_suffixes_MimeType(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MimeType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_MimeType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MimeType as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl MimeType {
    #[cfg(all(feature = "MimeType",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeType/type)\n\n*This API requires the following crate features to be activated: `MimeType`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> String {
        #[cfg(all(feature = "MimeType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_MimeType(
                self_: <&MimeType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_MimeType(
            self_: <&MimeType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MimeType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_MimeType(self_)
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
pub static __WASM_BINDGEN_GENERATED_9d1ad588f9f8662c: [u8; 446usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}|\x01\0\0\0\0\x05\0\0\x02\x08MimeType\x1A__widl_instanceof_MimeType\0\0\0\0\x1D__widl_f_description_MimeType\0\0\0\x01\x08MimeType\x01\0\x01\x0Bdescription\x01\x01\x05self_\x0Bdescription\0\0\0 __widl_f_enabled_plugin_MimeType\0\0\0\x01\x08MimeType\x01\0\x01\renabledPlugin\x01\x01\x05self_\renabledPlugin\0\0\0\x1A__widl_f_suffixes_MimeType\0\0\0\x01\x08MimeType\x01\0\x01\x08suffixes\x01\x01\x05self_\x08suffixes\0\0\0\x16__widl_f_type_MimeType\0\0\0\x01\x08MimeType\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
