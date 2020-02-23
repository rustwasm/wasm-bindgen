use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MediaCapabilitiesInfo` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilitiesInfo)\n\n*This API requires the following crate features to be activated: `MediaCapabilitiesInfo`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MediaCapabilitiesInfo {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MediaCapabilitiesInfo: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MediaCapabilitiesInfo {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(21u32);
            inform(77u32);
            inform(101u32);
            inform(100u32);
            inform(105u32);
            inform(97u32);
            inform(67u32);
            inform(97u32);
            inform(112u32);
            inform(97u32);
            inform(98u32);
            inform(105u32);
            inform(108u32);
            inform(105u32);
            inform(116u32);
            inform(105u32);
            inform(101u32);
            inform(115u32);
            inform(73u32);
            inform(110u32);
            inform(102u32);
            inform(111u32);
        }
    }
    impl core::ops::Deref for MediaCapabilitiesInfo {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for MediaCapabilitiesInfo {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MediaCapabilitiesInfo {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MediaCapabilitiesInfo {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MediaCapabilitiesInfo {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MediaCapabilitiesInfo {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MediaCapabilitiesInfo {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MediaCapabilitiesInfo {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MediaCapabilitiesInfo {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MediaCapabilitiesInfo>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MediaCapabilitiesInfo {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MediaCapabilitiesInfo {
        #[inline]
        fn from(obj: JsValue) -> MediaCapabilitiesInfo {
            MediaCapabilitiesInfo { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MediaCapabilitiesInfo {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MediaCapabilitiesInfo> for MediaCapabilitiesInfo {
        #[inline]
        fn as_ref(&self) -> &MediaCapabilitiesInfo {
            self
        }
    }
    impl From<MediaCapabilitiesInfo> for JsValue {
        #[inline]
        fn from(obj: MediaCapabilitiesInfo) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MediaCapabilitiesInfo {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MediaCapabilitiesInfo(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MediaCapabilitiesInfo(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MediaCapabilitiesInfo(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MediaCapabilitiesInfo { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MediaCapabilitiesInfo) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MediaCapabilitiesInfo> for ::js_sys::Object {
    #[inline]
    fn from(obj: MediaCapabilitiesInfo) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MediaCapabilitiesInfo {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MediaCapabilitiesInfo",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_supported_MediaCapabilitiesInfo() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaCapabilitiesInfo as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl MediaCapabilitiesInfo {
    #[cfg(all(feature = "MediaCapabilitiesInfo",))]
    #[allow(bad_style)]
    #[doc = "The `supported` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilitiesInfo/supported)\n\n*This API requires the following crate features to be activated: `MediaCapabilitiesInfo`*"]
    #[allow(clippy::all)]
    pub fn supported(&self) -> bool {
        #[cfg(all(feature = "MediaCapabilitiesInfo",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_supported_MediaCapabilitiesInfo(
                self_: <&MediaCapabilitiesInfo as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_supported_MediaCapabilitiesInfo(
            self_: <&MediaCapabilitiesInfo as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaCapabilitiesInfo as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_supported_MediaCapabilitiesInfo(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaCapabilitiesInfo",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_smooth_MediaCapabilitiesInfo() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaCapabilitiesInfo as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl MediaCapabilitiesInfo {
    #[cfg(all(feature = "MediaCapabilitiesInfo",))]
    #[allow(bad_style)]
    #[doc = "The `smooth` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilitiesInfo/smooth)\n\n*This API requires the following crate features to be activated: `MediaCapabilitiesInfo`*"]
    #[allow(clippy::all)]
    pub fn smooth(&self) -> bool {
        #[cfg(all(feature = "MediaCapabilitiesInfo",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_smooth_MediaCapabilitiesInfo(
                self_: <&MediaCapabilitiesInfo as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_smooth_MediaCapabilitiesInfo(
            self_: <&MediaCapabilitiesInfo as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaCapabilitiesInfo as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_smooth_MediaCapabilitiesInfo(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaCapabilitiesInfo",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_power_efficient_MediaCapabilitiesInfo() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaCapabilitiesInfo as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl MediaCapabilitiesInfo {
    #[cfg(all(feature = "MediaCapabilitiesInfo",))]
    #[allow(bad_style)]
    #[doc = "The `powerEfficient` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilitiesInfo/powerEfficient)\n\n*This API requires the following crate features to be activated: `MediaCapabilitiesInfo`*"]
    #[allow(clippy::all)]
    pub fn power_efficient(&self) -> bool {
        #[cfg(all(feature = "MediaCapabilitiesInfo",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_power_efficient_MediaCapabilitiesInfo(
                self_: <&MediaCapabilitiesInfo as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_power_efficient_MediaCapabilitiesInfo(
            self_: <&MediaCapabilitiesInfo as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaCapabilitiesInfo as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_power_efficient_MediaCapabilitiesInfo(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_7aacd8b0602561d2: [u8; 481usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x9F\x01\0\0\0\0\x04\0\0\x02\x15MediaCapabilitiesInfo'__widl_instanceof_MediaCapabilitiesInfo\0\0\0\0(__widl_f_supported_MediaCapabilitiesInfo\0\0\0\x01\x15MediaCapabilitiesInfo\x01\0\x01\tsupported\x01\x01\x05self_\tsupported\0\0\0%__widl_f_smooth_MediaCapabilitiesInfo\0\0\0\x01\x15MediaCapabilitiesInfo\x01\0\x01\x06smooth\x01\x01\x05self_\x06smooth\0\0\0.__widl_f_power_efficient_MediaCapabilitiesInfo\0\0\0\x01\x15MediaCapabilitiesInfo\x01\0\x01\x0EpowerEfficient\x01\x01\x05self_\x0EpowerEfficient\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
