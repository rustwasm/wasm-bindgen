use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `WEBGL_compressed_texture_etc` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_compressed_texture_etc)\n\n*This API requires the following crate features to be activated: `WebglCompressedTextureEtc`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct WebglCompressedTextureEtc {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_WebglCompressedTextureEtc: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for WebglCompressedTextureEtc {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(28u32);
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
            inform(101u32);
            inform(116u32);
            inform(99u32);
        }
    }
    impl core::ops::Deref for WebglCompressedTextureEtc {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for WebglCompressedTextureEtc {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for WebglCompressedTextureEtc {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a WebglCompressedTextureEtc {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for WebglCompressedTextureEtc {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            WebglCompressedTextureEtc {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for WebglCompressedTextureEtc {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a WebglCompressedTextureEtc {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for WebglCompressedTextureEtc {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<WebglCompressedTextureEtc>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(WebglCompressedTextureEtc {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for WebglCompressedTextureEtc {
        #[inline]
        fn from(obj: JsValue) -> WebglCompressedTextureEtc {
            WebglCompressedTextureEtc { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for WebglCompressedTextureEtc {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<WebglCompressedTextureEtc> for WebglCompressedTextureEtc {
        #[inline]
        fn as_ref(&self) -> &WebglCompressedTextureEtc {
            self
        }
    }
    impl From<WebglCompressedTextureEtc> for JsValue {
        #[inline]
        fn from(obj: WebglCompressedTextureEtc) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for WebglCompressedTextureEtc {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_WEBGL_compressed_texture_etc(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_WEBGL_compressed_texture_etc(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_WEBGL_compressed_texture_etc(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            WebglCompressedTextureEtc { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const WebglCompressedTextureEtc) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<WebglCompressedTextureEtc> for ::js_sys::Object {
    #[inline]
    fn from(obj: WebglCompressedTextureEtc) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for WebglCompressedTextureEtc {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
impl WebglCompressedTextureEtc {
    pub const COMPRESSED_R11_EAC: u32 = 37488u64 as u32;
}
impl WebglCompressedTextureEtc {
    pub const COMPRESSED_SIGNED_R11_EAC: u32 = 37489u64 as u32;
}
impl WebglCompressedTextureEtc {
    pub const COMPRESSED_RG11_EAC: u32 = 37490u64 as u32;
}
impl WebglCompressedTextureEtc {
    pub const COMPRESSED_SIGNED_RG11_EAC: u32 = 37491u64 as u32;
}
impl WebglCompressedTextureEtc {
    pub const COMPRESSED_RGB8_ETC2: u32 = 37492u64 as u32;
}
impl WebglCompressedTextureEtc {
    pub const COMPRESSED_SRGB8_ETC2: u32 = 37493u64 as u32;
}
impl WebglCompressedTextureEtc {
    pub const COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: u32 = 37494u64 as u32;
}
impl WebglCompressedTextureEtc {
    pub const COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: u32 = 37495u64 as u32;
}
impl WebglCompressedTextureEtc {
    pub const COMPRESSED_RGBA8_ETC2_EAC: u32 = 37496u64 as u32;
}
impl WebglCompressedTextureEtc {
    pub const COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: u32 = 37497u64 as u32;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_f684bb267f00eb88: [u8; 185usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}w\0\0\0\0\0\x01\0\0\x02\x1CWEBGL_compressed_texture_etc.__widl_instanceof_WEBGL_compressed_texture_etc\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
