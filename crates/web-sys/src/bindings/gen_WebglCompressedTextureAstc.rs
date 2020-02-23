use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `WEBGL_compressed_texture_astc` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_compressed_texture_astc)\n\n*This API requires the following crate features to be activated: `WebglCompressedTextureAstc`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct WebglCompressedTextureAstc {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_WebglCompressedTextureAstc: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for WebglCompressedTextureAstc {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(29u32);
            inform(87u32);
            inform(69u32);
            inform(66u32);
            inform(71u32);
            inform(76u32);
            inform(95u32);
            inform(99u32);
            inform(111u32);
            inform(109u32);
            inform(112u32);
            inform(114u32);
            inform(101u32);
            inform(115u32);
            inform(115u32);
            inform(101u32);
            inform(100u32);
            inform(95u32);
            inform(116u32);
            inform(101u32);
            inform(120u32);
            inform(116u32);
            inform(117u32);
            inform(114u32);
            inform(101u32);
            inform(95u32);
            inform(97u32);
            inform(115u32);
            inform(116u32);
            inform(99u32);
        }
    }
    impl core::ops::Deref for WebglCompressedTextureAstc {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for WebglCompressedTextureAstc {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for WebglCompressedTextureAstc {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a WebglCompressedTextureAstc {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for WebglCompressedTextureAstc {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            WebglCompressedTextureAstc {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for WebglCompressedTextureAstc {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a WebglCompressedTextureAstc {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for WebglCompressedTextureAstc {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<WebglCompressedTextureAstc>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(WebglCompressedTextureAstc {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for WebglCompressedTextureAstc {
        #[inline]
        fn from(obj: JsValue) -> WebglCompressedTextureAstc {
            WebglCompressedTextureAstc { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for WebglCompressedTextureAstc {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<WebglCompressedTextureAstc> for WebglCompressedTextureAstc {
        #[inline]
        fn as_ref(&self) -> &WebglCompressedTextureAstc {
            self
        }
    }
    impl From<WebglCompressedTextureAstc> for JsValue {
        #[inline]
        fn from(obj: WebglCompressedTextureAstc) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for WebglCompressedTextureAstc {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_WEBGL_compressed_texture_astc(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_WEBGL_compressed_texture_astc(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_WEBGL_compressed_texture_astc(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            WebglCompressedTextureAstc { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const WebglCompressedTextureAstc) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<WebglCompressedTextureAstc> for ::js_sys::Object {
    #[inline]
    fn from(obj: WebglCompressedTextureAstc) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for WebglCompressedTextureAstc {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "WebglCompressedTextureAstc",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_supported_profiles_WEBGL_compressed_texture_astc(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebglCompressedTextureAstc as WasmDescribe>::describe();
    <Option<::js_sys::Array> as WasmDescribe>::describe();
}
impl WebglCompressedTextureAstc {
    #[cfg(all(feature = "WebglCompressedTextureAstc",))]
    #[allow(bad_style)]
    #[doc = "The `getSupportedProfiles()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_compressed_texture_astc/getSupportedProfiles)\n\n*This API requires the following crate features to be activated: `WebglCompressedTextureAstc`*"]
    #[allow(clippy::all)]
    pub fn get_supported_profiles(&self) -> Option<::js_sys::Array> {
        #[cfg(all(feature = "WebglCompressedTextureAstc",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_supported_profiles_WEBGL_compressed_texture_astc(
                self_: <&WebglCompressedTextureAstc as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Array> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_supported_profiles_WEBGL_compressed_texture_astc(
            self_: <&WebglCompressedTextureAstc as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Array> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebglCompressedTextureAstc as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_get_supported_profiles_WEBGL_compressed_texture_astc(self_)
            };
            <Option<::js_sys::Array> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
impl WebglCompressedTextureAstc {
    pub const COMPRESSED_RGBA_ASTC_4X4_KHR: u32 = 37808u64 as u32;
}
impl WebglCompressedTextureAstc {
    pub const COMPRESSED_RGBA_ASTC_5X4_KHR: u32 = 37809u64 as u32;
}
impl WebglCompressedTextureAstc {
    pub const COMPRESSED_RGBA_ASTC_5X5_KHR: u32 = 37810u64 as u32;
}
impl WebglCompressedTextureAstc {
    pub const COMPRESSED_RGBA_ASTC_6X5_KHR: u32 = 37811u64 as u32;
}
impl WebglCompressedTextureAstc {
    pub const COMPRESSED_RGBA_ASTC_6X6_KHR: u32 = 37812u64 as u32;
}
impl WebglCompressedTextureAstc {
    pub const COMPRESSED_RGBA_ASTC_8X5_KHR: u32 = 37813u64 as u32;
}
impl WebglCompressedTextureAstc {
    pub const COMPRESSED_RGBA_ASTC_8X6_KHR: u32 = 37814u64 as u32;
}
impl WebglCompressedTextureAstc {
    pub const COMPRESSED_RGBA_ASTC_8X8_KHR: u32 = 37815u64 as u32;
}
impl WebglCompressedTextureAstc {
    pub const COMPRESSED_RGBA_ASTC_10X5_KHR: u32 = 37816u64 as u32;
}
impl WebglCompressedTextureAstc {
    pub const COMPRESSED_RGBA_ASTC_10X6_KHR: u32 = 37817u64 as u32;
}
impl WebglCompressedTextureAstc {
    pub const COMPRESSED_RGBA_ASTC_10X8_KHR: u32 = 37818u64 as u32;
}
impl WebglCompressedTextureAstc {
    pub const COMPRESSED_RGBA_ASTC_10X10_KHR: u32 = 37819u64 as u32;
}
impl WebglCompressedTextureAstc {
    pub const COMPRESSED_RGBA_ASTC_12X10_KHR: u32 = 37820u64 as u32;
}
impl WebglCompressedTextureAstc {
    pub const COMPRESSED_RGBA_ASTC_12X12_KHR: u32 = 37821u64 as u32;
}
impl WebglCompressedTextureAstc {
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_4X4_KHR: u32 = 37840u64 as u32;
}
impl WebglCompressedTextureAstc {
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_5X4_KHR: u32 = 37841u64 as u32;
}
impl WebglCompressedTextureAstc {
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_5X5_KHR: u32 = 37842u64 as u32;
}
impl WebglCompressedTextureAstc {
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_6X5_KHR: u32 = 37843u64 as u32;
}
impl WebglCompressedTextureAstc {
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_6X6_KHR: u32 = 37844u64 as u32;
}
impl WebglCompressedTextureAstc {
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_8X5_KHR: u32 = 37845u64 as u32;
}
impl WebglCompressedTextureAstc {
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_8X6_KHR: u32 = 37846u64 as u32;
}
impl WebglCompressedTextureAstc {
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_8X8_KHR: u32 = 37847u64 as u32;
}
impl WebglCompressedTextureAstc {
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_10X5_KHR: u32 = 37848u64 as u32;
}
impl WebglCompressedTextureAstc {
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_10X6_KHR: u32 = 37849u64 as u32;
}
impl WebglCompressedTextureAstc {
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_10X8_KHR: u32 = 37850u64 as u32;
}
impl WebglCompressedTextureAstc {
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_10X10_KHR: u32 = 37851u64 as u32;
}
impl WebglCompressedTextureAstc {
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_12X10_KHR: u32 = 37852u64 as u32;
}
impl WebglCompressedTextureAstc {
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_12X12_KHR: u32 = 37853u64 as u32;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_592f15d9bfca6762: [u8; 318usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xFC\0\0\0\0\0\x02\0\0\x02\x1DWEBGL_compressed_texture_astc/__widl_instanceof_WEBGL_compressed_texture_astc\0\0\0\0=__widl_f_get_supported_profiles_WEBGL_compressed_texture_astc\0\0\0\x01\x1DWEBGL_compressed_texture_astc\x01\0\0\x01\x01\x05self_\x14getSupportedProfiles\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
