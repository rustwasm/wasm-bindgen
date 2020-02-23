use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MediaKeyStatusMap` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap)\n\n*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MediaKeyStatusMap {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MediaKeyStatusMap: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MediaKeyStatusMap {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(17u32);
            inform(77u32);
            inform(101u32);
            inform(100u32);
            inform(105u32);
            inform(97u32);
            inform(75u32);
            inform(101u32);
            inform(121u32);
            inform(83u32);
            inform(116u32);
            inform(97u32);
            inform(116u32);
            inform(117u32);
            inform(115u32);
            inform(77u32);
            inform(97u32);
            inform(112u32);
        }
    }
    impl core::ops::Deref for MediaKeyStatusMap {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for MediaKeyStatusMap {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MediaKeyStatusMap {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MediaKeyStatusMap {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MediaKeyStatusMap {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MediaKeyStatusMap {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MediaKeyStatusMap {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MediaKeyStatusMap {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MediaKeyStatusMap {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MediaKeyStatusMap>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MediaKeyStatusMap {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MediaKeyStatusMap {
        #[inline]
        fn from(obj: JsValue) -> MediaKeyStatusMap {
            MediaKeyStatusMap { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MediaKeyStatusMap {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MediaKeyStatusMap> for MediaKeyStatusMap {
        #[inline]
        fn as_ref(&self) -> &MediaKeyStatusMap {
            self
        }
    }
    impl From<MediaKeyStatusMap> for JsValue {
        #[inline]
        fn from(obj: MediaKeyStatusMap) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MediaKeyStatusMap {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MediaKeyStatusMap(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MediaKeyStatusMap(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MediaKeyStatusMap(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MediaKeyStatusMap { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MediaKeyStatusMap) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MediaKeyStatusMap> for ::js_sys::Object {
    #[inline]
    fn from(obj: MediaKeyStatusMap) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MediaKeyStatusMap {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MediaKeyStatusMap",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_with_buffer_source_MediaKeyStatusMap() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaKeyStatusMap as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl MediaKeyStatusMap {
    #[cfg(all(feature = "MediaKeyStatusMap",))]
    #[allow(bad_style)]
    #[doc = "The `get()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/get)\n\n*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
    #[allow(clippy::all)]
    pub fn get_with_buffer_source(
        &self,
        key_id: &::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaKeyStatusMap",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_with_buffer_source_MediaKeyStatusMap(
                self_: <&MediaKeyStatusMap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_id: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_with_buffer_source_MediaKeyStatusMap(
            self_: <&MediaKeyStatusMap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_id: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(key_id);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaKeyStatusMap as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let key_id =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key_id);
                __widl_f_get_with_buffer_source_MediaKeyStatusMap(self_, key_id)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MediaKeyStatusMap",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_with_u8_array_MediaKeyStatusMap() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaKeyStatusMap as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl MediaKeyStatusMap {
    #[cfg(all(feature = "MediaKeyStatusMap",))]
    #[allow(bad_style)]
    #[doc = "The `get()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/get)\n\n*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
    #[allow(clippy::all)]
    pub fn get_with_u8_array(
        &self,
        key_id: &mut [u8],
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaKeyStatusMap",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_with_u8_array_MediaKeyStatusMap(
                self_: <&MediaKeyStatusMap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_id: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_with_u8_array_MediaKeyStatusMap(
            self_: <&MediaKeyStatusMap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_id: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(key_id);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaKeyStatusMap as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let key_id = <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key_id);
                __widl_f_get_with_u8_array_MediaKeyStatusMap(self_, key_id)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MediaKeyStatusMap",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_has_with_buffer_source_MediaKeyStatusMap() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaKeyStatusMap as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl MediaKeyStatusMap {
    #[cfg(all(feature = "MediaKeyStatusMap",))]
    #[allow(bad_style)]
    #[doc = "The `has()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/has)\n\n*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
    #[allow(clippy::all)]
    pub fn has_with_buffer_source(&self, key_id: &::js_sys::Object) -> bool {
        #[cfg(all(feature = "MediaKeyStatusMap",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_has_with_buffer_source_MediaKeyStatusMap(
                self_: <&MediaKeyStatusMap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_id: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_has_with_buffer_source_MediaKeyStatusMap(
            self_: <&MediaKeyStatusMap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_id: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(key_id);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaKeyStatusMap as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let key_id =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key_id);
                __widl_f_has_with_buffer_source_MediaKeyStatusMap(self_, key_id)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaKeyStatusMap",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_has_with_u8_array_MediaKeyStatusMap() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaKeyStatusMap as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl MediaKeyStatusMap {
    #[cfg(all(feature = "MediaKeyStatusMap",))]
    #[allow(bad_style)]
    #[doc = "The `has()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/has)\n\n*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
    #[allow(clippy::all)]
    pub fn has_with_u8_array(&self, key_id: &mut [u8]) -> bool {
        #[cfg(all(feature = "MediaKeyStatusMap",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_has_with_u8_array_MediaKeyStatusMap(
                self_: <&MediaKeyStatusMap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_id: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_has_with_u8_array_MediaKeyStatusMap(
            self_: <&MediaKeyStatusMap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_id: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(key_id);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaKeyStatusMap as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let key_id = <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key_id);
                __widl_f_has_with_u8_array_MediaKeyStatusMap(self_, key_id)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaKeyStatusMap",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_size_MediaKeyStatusMap() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaKeyStatusMap as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl MediaKeyStatusMap {
    #[cfg(all(feature = "MediaKeyStatusMap",))]
    #[allow(bad_style)]
    #[doc = "The `size` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/size)\n\n*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
    #[allow(clippy::all)]
    pub fn size(&self) -> u32 {
        #[cfg(all(feature = "MediaKeyStatusMap",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_size_MediaKeyStatusMap(
                self_: <&MediaKeyStatusMap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_size_MediaKeyStatusMap(
            self_: <&MediaKeyStatusMap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaKeyStatusMap as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_size_MediaKeyStatusMap(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_44ef67b7b57a7e6b: [u8; 619usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"})\x02\0\0\0\0\x06\0\0\x02\x11MediaKeyStatusMap#__widl_instanceof_MediaKeyStatusMap\0\0\0\01__widl_f_get_with_buffer_source_MediaKeyStatusMap\x01\0\0\x01\x11MediaKeyStatusMap\x01\0\0\x01\x02\x05self_\x06key_id\x03get\0\0\0,__widl_f_get_with_u8_array_MediaKeyStatusMap\x01\0\0\x01\x11MediaKeyStatusMap\x01\0\0\x01\x02\x05self_\x06key_id\x03get\0\0\01__widl_f_has_with_buffer_source_MediaKeyStatusMap\0\0\0\x01\x11MediaKeyStatusMap\x01\0\0\x01\x02\x05self_\x06key_id\x03has\0\0\0,__widl_f_has_with_u8_array_MediaKeyStatusMap\0\0\0\x01\x11MediaKeyStatusMap\x01\0\0\x01\x02\x05self_\x06key_id\x03has\0\0\0\x1F__widl_f_size_MediaKeyStatusMap\0\0\0\x01\x11MediaKeyStatusMap\x01\0\x01\x04size\x01\x01\x05self_\x04size\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
