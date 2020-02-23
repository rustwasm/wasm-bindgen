use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `WEBGL_compressed_texture_pvrtc` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_compressed_texture_pvrtc)\n\n*This API requires the following crate features to be activated: `WebglCompressedTexturePvrtc`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct WebglCompressedTexturePvrtc {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_WebglCompressedTexturePvrtc: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for WebglCompressedTexturePvrtc {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(30u32);
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
            inform(112u32);
            inform(118u32);
            inform(114u32);
            inform(116u32);
            inform(99u32);
        }
    }
    impl core::ops::Deref for WebglCompressedTexturePvrtc {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for WebglCompressedTexturePvrtc {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for WebglCompressedTexturePvrtc {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a WebglCompressedTexturePvrtc {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for WebglCompressedTexturePvrtc {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            WebglCompressedTexturePvrtc {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for WebglCompressedTexturePvrtc {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a WebglCompressedTexturePvrtc {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for WebglCompressedTexturePvrtc {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<WebglCompressedTexturePvrtc>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(WebglCompressedTexturePvrtc {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for WebglCompressedTexturePvrtc {
        #[inline]
        fn from(obj: JsValue) -> WebglCompressedTexturePvrtc {
            WebglCompressedTexturePvrtc { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for WebglCompressedTexturePvrtc {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<WebglCompressedTexturePvrtc> for WebglCompressedTexturePvrtc {
        #[inline]
        fn as_ref(&self) -> &WebglCompressedTexturePvrtc {
            self
        }
    }
    impl From<WebglCompressedTexturePvrtc> for JsValue {
        #[inline]
        fn from(obj: WebglCompressedTexturePvrtc) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for WebglCompressedTexturePvrtc {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_WEBGL_compressed_texture_pvrtc(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_WEBGL_compressed_texture_pvrtc(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_WEBGL_compressed_texture_pvrtc(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            WebglCompressedTexturePvrtc { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const WebglCompressedTexturePvrtc) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<WebglCompressedTexturePvrtc> for ::js_sys::Object {
    #[inline]
    fn from(obj: WebglCompressedTexturePvrtc) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for WebglCompressedTexturePvrtc {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
impl WebglCompressedTexturePvrtc {
    pub const COMPRESSED_RGB_PVRTC_4BPPV1_IMG: u32 = 35840u64 as u32;
}
impl WebglCompressedTexturePvrtc {
    pub const COMPRESSED_RGB_PVRTC_2BPPV1_IMG: u32 = 35841u64 as u32;
}
impl WebglCompressedTexturePvrtc {
    pub const COMPRESSED_RGBA_PVRTC_4BPPV1_IMG: u32 = 35842u64 as u32;
}
impl WebglCompressedTexturePvrtc {
    pub const COMPRESSED_RGBA_PVRTC_2BPPV1_IMG: u32 = 35843u64 as u32;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_5f7a9655717f6e88: [u8; 189usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}{\0\0\0\0\0\x01\0\0\x02\x1EWEBGL_compressed_texture_pvrtc0__widl_instanceof_WEBGL_compressed_texture_pvrtc\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
