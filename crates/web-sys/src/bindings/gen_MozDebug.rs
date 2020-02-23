use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MOZ_debug` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MOZ_debug)\n\n*This API requires the following crate features to be activated: `MozDebug`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MozDebug {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MozDebug: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MozDebug {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(9u32);
            inform(77u32);
            inform(79u32);
            inform(90u32);
            inform(95u32);
            inform(100u32);
            inform(101u32);
            inform(98u32);
            inform(117u32);
            inform(103u32);
        }
    }
    impl core::ops::Deref for MozDebug {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for MozDebug {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MozDebug {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MozDebug {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MozDebug {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MozDebug {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MozDebug {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MozDebug {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MozDebug {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MozDebug>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MozDebug {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MozDebug {
        #[inline]
        fn from(obj: JsValue) -> MozDebug {
            MozDebug { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MozDebug {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MozDebug> for MozDebug {
        #[inline]
        fn as_ref(&self) -> &MozDebug {
            self
        }
    }
    impl From<MozDebug> for JsValue {
        #[inline]
        fn from(obj: MozDebug) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MozDebug {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MOZ_debug(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MOZ_debug(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MOZ_debug(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MozDebug { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MozDebug) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MozDebug> for ::js_sys::Object {
    #[inline]
    fn from(obj: MozDebug) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MozDebug {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MozDebug",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_parameter_MOZ_debug() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MozDebug as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl MozDebug {
    #[cfg(all(feature = "MozDebug",))]
    #[allow(bad_style)]
    #[doc = "The `getParameter()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MOZ_debug/getParameter)\n\n*This API requires the following crate features to be activated: `MozDebug`*"]
    #[allow(clippy::all)]
    pub fn get_parameter(
        &self,
        pname: u32,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MozDebug",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_parameter_MOZ_debug(
                self_: <&MozDebug as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pname: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_parameter_MOZ_debug(
            self_: <&MozDebug as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pname: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(pname);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MozDebug as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let pname = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pname);
                __widl_f_get_parameter_MOZ_debug(self_, pname)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
impl MozDebug {
    pub const EXTENSIONS: u32 = 7939u64 as u32;
}
impl MozDebug {
    pub const WSI_INFO: u32 = 65536u64 as u32;
}
impl MozDebug {
    pub const UNPACK_REQUIRE_FASTPATH: u32 = 65537u64 as u32;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_93ebc19c323d28d9: [u8; 227usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xA1\0\0\0\0\0\x02\0\0\x02\tMOZ_debug\x1B__widl_instanceof_MOZ_debug\0\0\0\0 __widl_f_get_parameter_MOZ_debug\x01\0\0\x01\tMOZ_debug\x01\0\0\x01\x02\x05self_\x05pname\x0CgetParameter\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
