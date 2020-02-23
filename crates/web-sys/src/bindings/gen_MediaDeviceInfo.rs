use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MediaDeviceInfo` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo)\n\n*This API requires the following crate features to be activated: `MediaDeviceInfo`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MediaDeviceInfo {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MediaDeviceInfo: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MediaDeviceInfo {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(77u32);
            inform(101u32);
            inform(100u32);
            inform(105u32);
            inform(97u32);
            inform(68u32);
            inform(101u32);
            inform(118u32);
            inform(105u32);
            inform(99u32);
            inform(101u32);
            inform(73u32);
            inform(110u32);
            inform(102u32);
            inform(111u32);
        }
    }
    impl core::ops::Deref for MediaDeviceInfo {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for MediaDeviceInfo {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MediaDeviceInfo {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MediaDeviceInfo {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MediaDeviceInfo {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MediaDeviceInfo {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MediaDeviceInfo {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MediaDeviceInfo {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MediaDeviceInfo {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MediaDeviceInfo>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MediaDeviceInfo {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MediaDeviceInfo {
        #[inline]
        fn from(obj: JsValue) -> MediaDeviceInfo {
            MediaDeviceInfo { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MediaDeviceInfo {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MediaDeviceInfo> for MediaDeviceInfo {
        #[inline]
        fn as_ref(&self) -> &MediaDeviceInfo {
            self
        }
    }
    impl From<MediaDeviceInfo> for JsValue {
        #[inline]
        fn from(obj: MediaDeviceInfo) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MediaDeviceInfo {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MediaDeviceInfo(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MediaDeviceInfo(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MediaDeviceInfo(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MediaDeviceInfo { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MediaDeviceInfo) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MediaDeviceInfo> for ::js_sys::Object {
    #[inline]
    fn from(obj: MediaDeviceInfo) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MediaDeviceInfo {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MediaDeviceInfo",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_to_json_MediaDeviceInfo() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaDeviceInfo as WasmDescribe>::describe();
    <::js_sys::Object as WasmDescribe>::describe();
}
impl MediaDeviceInfo {
    #[cfg(all(feature = "MediaDeviceInfo",))]
    #[allow(bad_style)]
    #[doc = "The `toJSON()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/toJSON)\n\n*This API requires the following crate features to be activated: `MediaDeviceInfo`*"]
    #[allow(clippy::all)]
    pub fn to_json(&self) -> ::js_sys::Object {
        #[cfg(all(feature = "MediaDeviceInfo",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_to_json_MediaDeviceInfo(
                self_: <&MediaDeviceInfo as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_to_json_MediaDeviceInfo(
            self_: <&MediaDeviceInfo as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaDeviceInfo as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_to_json_MediaDeviceInfo(self_)
            };
            <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaDeviceInfo",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_device_id_MediaDeviceInfo() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaDeviceInfo as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl MediaDeviceInfo {
    #[cfg(all(feature = "MediaDeviceInfo",))]
    #[allow(bad_style)]
    #[doc = "The `deviceId` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/deviceId)\n\n*This API requires the following crate features to be activated: `MediaDeviceInfo`*"]
    #[allow(clippy::all)]
    pub fn device_id(&self) -> String {
        #[cfg(all(feature = "MediaDeviceInfo",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_device_id_MediaDeviceInfo(
                self_: <&MediaDeviceInfo as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_device_id_MediaDeviceInfo(
            self_: <&MediaDeviceInfo as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaDeviceInfo as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_device_id_MediaDeviceInfo(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaDeviceInfo", feature = "MediaDeviceKind",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_kind_MediaDeviceInfo() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaDeviceInfo as WasmDescribe>::describe();
    <MediaDeviceKind as WasmDescribe>::describe();
}
impl MediaDeviceInfo {
    #[cfg(all(feature = "MediaDeviceInfo", feature = "MediaDeviceKind",))]
    #[allow(bad_style)]
    #[doc = "The `kind` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/kind)\n\n*This API requires the following crate features to be activated: `MediaDeviceInfo`, `MediaDeviceKind`*"]
    #[allow(clippy::all)]
    pub fn kind(&self) -> MediaDeviceKind {
        #[cfg(all(feature = "MediaDeviceInfo", feature = "MediaDeviceKind",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_kind_MediaDeviceInfo(
                self_: <&MediaDeviceInfo as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaDeviceKind as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_kind_MediaDeviceInfo(
            self_: <&MediaDeviceInfo as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaDeviceKind as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaDeviceInfo as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_kind_MediaDeviceInfo(self_)
            };
            <MediaDeviceKind as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaDeviceInfo",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_label_MediaDeviceInfo() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaDeviceInfo as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl MediaDeviceInfo {
    #[cfg(all(feature = "MediaDeviceInfo",))]
    #[allow(bad_style)]
    #[doc = "The `label` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/label)\n\n*This API requires the following crate features to be activated: `MediaDeviceInfo`*"]
    #[allow(clippy::all)]
    pub fn label(&self) -> String {
        #[cfg(all(feature = "MediaDeviceInfo",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_label_MediaDeviceInfo(
                self_: <&MediaDeviceInfo as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_label_MediaDeviceInfo(
            self_: <&MediaDeviceInfo as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaDeviceInfo as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_label_MediaDeviceInfo(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaDeviceInfo",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_group_id_MediaDeviceInfo() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaDeviceInfo as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl MediaDeviceInfo {
    #[cfg(all(feature = "MediaDeviceInfo",))]
    #[allow(bad_style)]
    #[doc = "The `groupId` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/groupId)\n\n*This API requires the following crate features to be activated: `MediaDeviceInfo`*"]
    #[allow(clippy::all)]
    pub fn group_id(&self) -> String {
        #[cfg(all(feature = "MediaDeviceInfo",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_id_MediaDeviceInfo(
                self_: <&MediaDeviceInfo as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_id_MediaDeviceInfo(
            self_: <&MediaDeviceInfo as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaDeviceInfo as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_group_id_MediaDeviceInfo(self_)
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
pub static __WASM_BINDGEN_GENERATED_d4025c9ba2aa9217: [u8; 555usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xE9\x01\0\0\0\0\x06\0\0\x02\x0FMediaDeviceInfo!__widl_instanceof_MediaDeviceInfo\0\0\0\0 __widl_f_to_json_MediaDeviceInfo\0\0\0\x01\x0FMediaDeviceInfo\x01\0\0\x01\x01\x05self_\x06toJSON\0\0\0\"__widl_f_device_id_MediaDeviceInfo\0\0\0\x01\x0FMediaDeviceInfo\x01\0\x01\x08deviceId\x01\x01\x05self_\x08deviceId\0\0\0\x1D__widl_f_kind_MediaDeviceInfo\0\0\0\x01\x0FMediaDeviceInfo\x01\0\x01\x04kind\x01\x01\x05self_\x04kind\0\0\0\x1E__widl_f_label_MediaDeviceInfo\0\0\0\x01\x0FMediaDeviceInfo\x01\0\x01\x05label\x01\x01\x05self_\x05label\0\0\0!__widl_f_group_id_MediaDeviceInfo\0\0\0\x01\x0FMediaDeviceInfo\x01\0\x01\x07groupId\x01\x01\x05self_\x07groupId\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
