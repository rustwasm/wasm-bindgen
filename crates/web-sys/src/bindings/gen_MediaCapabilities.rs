use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MediaCapabilities` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilities)\n\n*This API requires the following crate features to be activated: `MediaCapabilities`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MediaCapabilities {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MediaCapabilities: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MediaCapabilities {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(17u32);
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
        }
    }
    impl core::ops::Deref for MediaCapabilities {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for MediaCapabilities {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MediaCapabilities {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MediaCapabilities {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MediaCapabilities {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MediaCapabilities {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MediaCapabilities {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MediaCapabilities {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MediaCapabilities {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MediaCapabilities>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MediaCapabilities {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MediaCapabilities {
        #[inline]
        fn from(obj: JsValue) -> MediaCapabilities {
            MediaCapabilities { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MediaCapabilities {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MediaCapabilities> for MediaCapabilities {
        #[inline]
        fn as_ref(&self) -> &MediaCapabilities {
            self
        }
    }
    impl From<MediaCapabilities> for JsValue {
        #[inline]
        fn from(obj: MediaCapabilities) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MediaCapabilities {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MediaCapabilities(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MediaCapabilities(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MediaCapabilities(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MediaCapabilities { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MediaCapabilities) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MediaCapabilities> for ::js_sys::Object {
    #[inline]
    fn from(obj: MediaCapabilities) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MediaCapabilities {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MediaCapabilities", feature = "MediaDecodingConfiguration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_decoding_info_MediaCapabilities() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaCapabilities as WasmDescribe>::describe();
    <&MediaDecodingConfiguration as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl MediaCapabilities {
    #[cfg(all(feature = "MediaCapabilities", feature = "MediaDecodingConfiguration",))]
    #[allow(bad_style)]
    #[doc = "The `decodingInfo()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilities/decodingInfo)\n\n*This API requires the following crate features to be activated: `MediaCapabilities`, `MediaDecodingConfiguration`*"]
    #[allow(clippy::all)]
    pub fn decoding_info(&self, configuration: &MediaDecodingConfiguration) -> ::js_sys::Promise {
        #[cfg(all(feature = "MediaCapabilities", feature = "MediaDecodingConfiguration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_decoding_info_MediaCapabilities(
                self_: <&MediaCapabilities as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                configuration : < & MediaDecodingConfiguration as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_decoding_info_MediaCapabilities(
            self_: <&MediaCapabilities as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            configuration: <&MediaDecodingConfiguration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(configuration);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaCapabilities as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let configuration =
                    <&MediaDecodingConfiguration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        configuration,
                    );
                __widl_f_decoding_info_MediaCapabilities(self_, configuration)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaCapabilities", feature = "MediaEncodingConfiguration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_encoding_info_MediaCapabilities() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaCapabilities as WasmDescribe>::describe();
    <&MediaEncodingConfiguration as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl MediaCapabilities {
    #[cfg(all(feature = "MediaCapabilities", feature = "MediaEncodingConfiguration",))]
    #[allow(bad_style)]
    #[doc = "The `encodingInfo()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilities/encodingInfo)\n\n*This API requires the following crate features to be activated: `MediaCapabilities`, `MediaEncodingConfiguration`*"]
    #[allow(clippy::all)]
    pub fn encoding_info(&self, configuration: &MediaEncodingConfiguration) -> ::js_sys::Promise {
        #[cfg(all(feature = "MediaCapabilities", feature = "MediaEncodingConfiguration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_encoding_info_MediaCapabilities(
                self_: <&MediaCapabilities as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                configuration : < & MediaEncodingConfiguration as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_encoding_info_MediaCapabilities(
            self_: <&MediaCapabilities as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            configuration: <&MediaEncodingConfiguration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(configuration);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaCapabilities as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let configuration =
                    <&MediaEncodingConfiguration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        configuration,
                    );
                __widl_f_encoding_info_MediaCapabilities(self_, configuration)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_7e904df7fa5e89d8: [u8; 371usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}1\x01\0\0\0\0\x03\0\0\x02\x11MediaCapabilities#__widl_instanceof_MediaCapabilities\0\0\0\0(__widl_f_decoding_info_MediaCapabilities\0\0\0\x01\x11MediaCapabilities\x01\0\0\x01\x02\x05self_\rconfiguration\x0CdecodingInfo\0\0\0(__widl_f_encoding_info_MediaCapabilities\0\0\0\x01\x11MediaCapabilities\x01\0\0\x01\x02\x05self_\rconfiguration\x0CencodingInfo\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
