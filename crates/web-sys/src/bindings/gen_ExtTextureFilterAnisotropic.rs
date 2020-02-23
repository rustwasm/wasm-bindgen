use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `EXT_texture_filter_anisotropic` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_texture_filter_anisotropic)\n\n*This API requires the following crate features to be activated: `ExtTextureFilterAnisotropic`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ExtTextureFilterAnisotropic {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ExtTextureFilterAnisotropic: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ExtTextureFilterAnisotropic {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(30u32);
            inform(69u32);
            inform(88u32);
            inform(84u32);
            inform(95u32);
            inform(116u32);
            inform(101u32);
            inform(120u32);
            inform(116u32);
            inform(117u32);
            inform(114u32);
            inform(101u32);
            inform(95u32);
            inform(102u32);
            inform(105u32);
            inform(108u32);
            inform(116u32);
            inform(101u32);
            inform(114u32);
            inform(95u32);
            inform(97u32);
            inform(110u32);
            inform(105u32);
            inform(115u32);
            inform(111u32);
            inform(116u32);
            inform(114u32);
            inform(111u32);
            inform(112u32);
            inform(105u32);
            inform(99u32);
        }
    }
    impl core::ops::Deref for ExtTextureFilterAnisotropic {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for ExtTextureFilterAnisotropic {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ExtTextureFilterAnisotropic {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ExtTextureFilterAnisotropic {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ExtTextureFilterAnisotropic {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ExtTextureFilterAnisotropic {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ExtTextureFilterAnisotropic {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ExtTextureFilterAnisotropic {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ExtTextureFilterAnisotropic {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ExtTextureFilterAnisotropic>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ExtTextureFilterAnisotropic {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ExtTextureFilterAnisotropic {
        #[inline]
        fn from(obj: JsValue) -> ExtTextureFilterAnisotropic {
            ExtTextureFilterAnisotropic { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ExtTextureFilterAnisotropic {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ExtTextureFilterAnisotropic> for ExtTextureFilterAnisotropic {
        #[inline]
        fn as_ref(&self) -> &ExtTextureFilterAnisotropic {
            self
        }
    }
    impl From<ExtTextureFilterAnisotropic> for JsValue {
        #[inline]
        fn from(obj: ExtTextureFilterAnisotropic) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ExtTextureFilterAnisotropic {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_EXT_texture_filter_anisotropic(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_EXT_texture_filter_anisotropic(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_EXT_texture_filter_anisotropic(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ExtTextureFilterAnisotropic { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ExtTextureFilterAnisotropic) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ExtTextureFilterAnisotropic> for ::js_sys::Object {
    #[inline]
    fn from(obj: ExtTextureFilterAnisotropic) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ExtTextureFilterAnisotropic {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
impl ExtTextureFilterAnisotropic {
    pub const TEXTURE_MAX_ANISOTROPY_EXT: u32 = 34046u64 as u32;
}
impl ExtTextureFilterAnisotropic {
    pub const MAX_TEXTURE_MAX_ANISOTROPY_EXT: u32 = 34047u64 as u32;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_0b8c4cbbb948049d: [u8; 189usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}{\0\0\0\0\0\x01\0\0\x02\x1EEXT_texture_filter_anisotropic0__widl_instanceof_EXT_texture_filter_anisotropic\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
