use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `WEBGL_compressed_texture_s3tc_srgb` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_compressed_texture_s3tc_srgb)\n\n*This API requires the following crate features to be activated: `WebglCompressedTextureS3tcSrgb`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct WebglCompressedTextureS3tcSrgb {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_WebglCompressedTextureS3tcSrgb: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for WebglCompressedTextureS3tcSrgb {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(34u32);
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
            inform(115u32);
            inform(51u32);
            inform(116u32);
            inform(99u32);
            inform(95u32);
            inform(115u32);
            inform(114u32);
            inform(103u32);
            inform(98u32);
        }
    }
    impl core::ops::Deref for WebglCompressedTextureS3tcSrgb {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for WebglCompressedTextureS3tcSrgb {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for WebglCompressedTextureS3tcSrgb {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a WebglCompressedTextureS3tcSrgb {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for WebglCompressedTextureS3tcSrgb {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            WebglCompressedTextureS3tcSrgb {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for WebglCompressedTextureS3tcSrgb {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a WebglCompressedTextureS3tcSrgb {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for WebglCompressedTextureS3tcSrgb {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<WebglCompressedTextureS3tcSrgb>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(WebglCompressedTextureS3tcSrgb {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for WebglCompressedTextureS3tcSrgb {
        #[inline]
        fn from(obj: JsValue) -> WebglCompressedTextureS3tcSrgb {
            WebglCompressedTextureS3tcSrgb { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for WebglCompressedTextureS3tcSrgb {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<WebglCompressedTextureS3tcSrgb> for WebglCompressedTextureS3tcSrgb {
        #[inline]
        fn as_ref(&self) -> &WebglCompressedTextureS3tcSrgb {
            self
        }
    }
    impl From<WebglCompressedTextureS3tcSrgb> for JsValue {
        #[inline]
        fn from(obj: WebglCompressedTextureS3tcSrgb) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for WebglCompressedTextureS3tcSrgb {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_WEBGL_compressed_texture_s3tc_srgb(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_WEBGL_compressed_texture_s3tc_srgb(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_WEBGL_compressed_texture_s3tc_srgb(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            WebglCompressedTextureS3tcSrgb { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const WebglCompressedTextureS3tcSrgb) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<WebglCompressedTextureS3tcSrgb> for ::js_sys::Object {
    #[inline]
    fn from(obj: WebglCompressedTextureS3tcSrgb) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for WebglCompressedTextureS3tcSrgb {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
impl WebglCompressedTextureS3tcSrgb {
    pub const COMPRESSED_SRGB_S3TC_DXT1_EXT: u32 = 35916u64 as u32;
}
impl WebglCompressedTextureS3tcSrgb {
    pub const COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT: u32 = 35917u64 as u32;
}
impl WebglCompressedTextureS3tcSrgb {
    pub const COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT: u32 = 35918u64 as u32;
}
impl WebglCompressedTextureS3tcSrgb {
    pub const COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT: u32 = 35919u64 as u32;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_89c0205659c19381: [u8; 197usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x83\0\0\0\0\0\x01\0\0\x02\"WEBGL_compressed_texture_s3tc_srgb4__widl_instanceof_WEBGL_compressed_texture_s3tc_srgb\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
