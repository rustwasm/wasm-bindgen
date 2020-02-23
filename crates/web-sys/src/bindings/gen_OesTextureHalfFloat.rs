use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `OES_texture_half_float` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_texture_half_float)\n\n*This API requires the following crate features to be activated: `OesTextureHalfFloat`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct OesTextureHalfFloat {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_OesTextureHalfFloat: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for OesTextureHalfFloat {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(22u32);
            inform(79u32);
            inform(69u32);
            inform(83u32);
            inform(95u32);
            inform(116u32);
            inform(101u32);
            inform(120u32);
            inform(116u32);
            inform(117u32);
            inform(114u32);
            inform(101u32);
            inform(95u32);
            inform(104u32);
            inform(97u32);
            inform(108u32);
            inform(102u32);
            inform(95u32);
            inform(102u32);
            inform(108u32);
            inform(111u32);
            inform(97u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for OesTextureHalfFloat {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for OesTextureHalfFloat {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for OesTextureHalfFloat {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a OesTextureHalfFloat {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for OesTextureHalfFloat {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            OesTextureHalfFloat {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for OesTextureHalfFloat {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a OesTextureHalfFloat {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for OesTextureHalfFloat {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<OesTextureHalfFloat>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(OesTextureHalfFloat {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for OesTextureHalfFloat {
        #[inline]
        fn from(obj: JsValue) -> OesTextureHalfFloat {
            OesTextureHalfFloat { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for OesTextureHalfFloat {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<OesTextureHalfFloat> for OesTextureHalfFloat {
        #[inline]
        fn as_ref(&self) -> &OesTextureHalfFloat {
            self
        }
    }
    impl From<OesTextureHalfFloat> for JsValue {
        #[inline]
        fn from(obj: OesTextureHalfFloat) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for OesTextureHalfFloat {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_OES_texture_half_float(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_OES_texture_half_float(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_OES_texture_half_float(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            OesTextureHalfFloat { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const OesTextureHalfFloat) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<OesTextureHalfFloat> for ::js_sys::Object {
    #[inline]
    fn from(obj: OesTextureHalfFloat) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for OesTextureHalfFloat {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
impl OesTextureHalfFloat {
    pub const HALF_FLOAT_OES: u32 = 36193u64 as u32;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_89c0f10b1dfabbeb: [u8; 173usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}k\0\0\0\0\0\x01\0\0\x02\x16OES_texture_half_float(__widl_instanceof_OES_texture_half_float\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
