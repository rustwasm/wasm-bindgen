use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MediaKeySystemAccess` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySystemAccess)\n\n*This API requires the following crate features to be activated: `MediaKeySystemAccess`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MediaKeySystemAccess {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MediaKeySystemAccess: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MediaKeySystemAccess {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(20u32);
            inform(77u32);
            inform(101u32);
            inform(100u32);
            inform(105u32);
            inform(97u32);
            inform(75u32);
            inform(101u32);
            inform(121u32);
            inform(83u32);
            inform(121u32);
            inform(115u32);
            inform(116u32);
            inform(101u32);
            inform(109u32);
            inform(65u32);
            inform(99u32);
            inform(99u32);
            inform(101u32);
            inform(115u32);
            inform(115u32);
        }
    }
    impl core::ops::Deref for MediaKeySystemAccess {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for MediaKeySystemAccess {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MediaKeySystemAccess {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MediaKeySystemAccess {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MediaKeySystemAccess {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MediaKeySystemAccess {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MediaKeySystemAccess {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MediaKeySystemAccess {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MediaKeySystemAccess {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MediaKeySystemAccess>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MediaKeySystemAccess {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MediaKeySystemAccess {
        #[inline]
        fn from(obj: JsValue) -> MediaKeySystemAccess {
            MediaKeySystemAccess { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MediaKeySystemAccess {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MediaKeySystemAccess> for MediaKeySystemAccess {
        #[inline]
        fn as_ref(&self) -> &MediaKeySystemAccess {
            self
        }
    }
    impl From<MediaKeySystemAccess> for JsValue {
        #[inline]
        fn from(obj: MediaKeySystemAccess) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MediaKeySystemAccess {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MediaKeySystemAccess(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MediaKeySystemAccess(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MediaKeySystemAccess(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MediaKeySystemAccess { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MediaKeySystemAccess) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MediaKeySystemAccess> for ::js_sys::Object {
    #[inline]
    fn from(obj: MediaKeySystemAccess) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MediaKeySystemAccess {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MediaKeySystemAccess",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_media_keys_MediaKeySystemAccess() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaKeySystemAccess as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl MediaKeySystemAccess {
    #[cfg(all(feature = "MediaKeySystemAccess",))]
    #[allow(bad_style)]
    #[doc = "The `createMediaKeys()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySystemAccess/createMediaKeys)\n\n*This API requires the following crate features to be activated: `MediaKeySystemAccess`*"]
    #[allow(clippy::all)]
    pub fn create_media_keys(&self) -> ::js_sys::Promise {
        #[cfg(all(feature = "MediaKeySystemAccess",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_media_keys_MediaKeySystemAccess(
                self_: <&MediaKeySystemAccess as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_media_keys_MediaKeySystemAccess(
            self_: <&MediaKeySystemAccess as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaKeySystemAccess as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_media_keys_MediaKeySystemAccess(self_)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "MediaKeySystemAccess",
    feature = "MediaKeySystemConfiguration",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_configuration_MediaKeySystemAccess() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaKeySystemAccess as WasmDescribe>::describe();
    <MediaKeySystemConfiguration as WasmDescribe>::describe();
}
impl MediaKeySystemAccess {
    #[cfg(all(
        feature = "MediaKeySystemAccess",
        feature = "MediaKeySystemConfiguration",
    ))]
    #[allow(bad_style)]
    #[doc = "The `getConfiguration()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySystemAccess/getConfiguration)\n\n*This API requires the following crate features to be activated: `MediaKeySystemAccess`, `MediaKeySystemConfiguration`*"]
    #[allow(clippy::all)]
    pub fn get_configuration(&self) -> MediaKeySystemConfiguration {
        #[cfg(all(
            feature = "MediaKeySystemAccess",
            feature = "MediaKeySystemConfiguration",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_configuration_MediaKeySystemAccess(
                self_: <&MediaKeySystemAccess as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaKeySystemConfiguration as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_configuration_MediaKeySystemAccess(
            self_: <&MediaKeySystemAccess as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaKeySystemConfiguration as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaKeySystemAccess as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_configuration_MediaKeySystemAccess(self_)
            };
            <MediaKeySystemConfiguration as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaKeySystemAccess",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_key_system_MediaKeySystemAccess() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaKeySystemAccess as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl MediaKeySystemAccess {
    #[cfg(all(feature = "MediaKeySystemAccess",))]
    #[allow(bad_style)]
    #[doc = "The `keySystem` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySystemAccess/keySystem)\n\n*This API requires the following crate features to be activated: `MediaKeySystemAccess`*"]
    #[allow(clippy::all)]
    pub fn key_system(&self) -> String {
        #[cfg(all(feature = "MediaKeySystemAccess",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_key_system_MediaKeySystemAccess(
                self_: <&MediaKeySystemAccess as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_key_system_MediaKeySystemAccess(
            self_: <&MediaKeySystemAccess as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&MediaKeySystemAccess as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_key_system_MediaKeySystemAccess(self_)
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
pub static __WASM_BINDGEN_GENERATED_1d5813026f580e2f: [u8; 476usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x9A\x01\0\0\0\0\x04\0\0\x02\x14MediaKeySystemAccess&__widl_instanceof_MediaKeySystemAccess\0\0\0\0/__widl_f_create_media_keys_MediaKeySystemAccess\0\0\0\x01\x14MediaKeySystemAccess\x01\0\0\x01\x01\x05self_\x0FcreateMediaKeys\0\0\0/__widl_f_get_configuration_MediaKeySystemAccess\0\0\0\x01\x14MediaKeySystemAccess\x01\0\0\x01\x01\x05self_\x10getConfiguration\0\0\0(__widl_f_key_system_MediaKeySystemAccess\0\0\0\x01\x14MediaKeySystemAccess\x01\0\x01\tkeySystem\x01\x01\x05self_\tkeySystem\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
