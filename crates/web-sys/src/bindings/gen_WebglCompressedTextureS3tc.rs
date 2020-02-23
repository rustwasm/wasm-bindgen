use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `WEBGL_compressed_texture_s3tc` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_compressed_texture_s3tc)\n\n*This API requires the following crate features to be activated: `WebglCompressedTextureS3tc`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct WebglCompressedTextureS3tc {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_WebglCompressedTextureS3tc: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for WebglCompressedTextureS3tc {
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
            inform(115u32);
            inform(51u32);
            inform(116u32);
            inform(99u32);
        }
    }
    impl core::ops::Deref for WebglCompressedTextureS3tc {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for WebglCompressedTextureS3tc {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for WebglCompressedTextureS3tc {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a WebglCompressedTextureS3tc {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for WebglCompressedTextureS3tc {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            WebglCompressedTextureS3tc {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for WebglCompressedTextureS3tc {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a WebglCompressedTextureS3tc {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for WebglCompressedTextureS3tc {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<WebglCompressedTextureS3tc>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(WebglCompressedTextureS3tc {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for WebglCompressedTextureS3tc {
        #[inline]
        fn from(obj: JsValue) -> WebglCompressedTextureS3tc {
            WebglCompressedTextureS3tc { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for WebglCompressedTextureS3tc {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<WebglCompressedTextureS3tc> for WebglCompressedTextureS3tc {
        #[inline]
        fn as_ref(&self) -> &WebglCompressedTextureS3tc {
            self
        }
    }
    impl From<WebglCompressedTextureS3tc> for JsValue {
        #[inline]
        fn from(obj: WebglCompressedTextureS3tc) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for WebglCompressedTextureS3tc {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_WEBGL_compressed_texture_s3tc(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_WEBGL_compressed_texture_s3tc(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_WEBGL_compressed_texture_s3tc(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            WebglCompressedTextureS3tc { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const WebglCompressedTextureS3tc) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<WebglCompressedTextureS3tc> for ::js_sys::Object {
    #[inline]
    fn from(obj: WebglCompressedTextureS3tc) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for WebglCompressedTextureS3tc {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
impl WebglCompressedTextureS3tc {
    pub const COMPRESSED_RGB_S3TC_DXT1_EXT: u32 = 33776u64 as u32;
}
impl WebglCompressedTextureS3tc {
    pub const COMPRESSED_RGBA_S3TC_DXT1_EXT: u32 = 33777u64 as u32;
}
impl WebglCompressedTextureS3tc {
    pub const COMPRESSED_RGBA_S3TC_DXT3_EXT: u32 = 33778u64 as u32;
}
impl WebglCompressedTextureS3tc {
    pub const COMPRESSED_RGBA_S3TC_DXT5_EXT: u32 = 33779u64 as u32;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_d5532a451dd6ed4c: [u8; 187usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}y\0\0\0\0\0\x01\0\0\x02\x1DWEBGL_compressed_texture_s3tc/__widl_instanceof_WEBGL_compressed_texture_s3tc\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
