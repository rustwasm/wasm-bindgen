use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `EXT_sRGB` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_sRGB)\n\n*This API requires the following crate features to be activated: `ExtSRgb`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ExtSRgb {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ExtSRgb: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ExtSRgb {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(8u32);
            inform(69u32);
            inform(88u32);
            inform(84u32);
            inform(95u32);
            inform(115u32);
            inform(82u32);
            inform(71u32);
            inform(66u32);
        }
    }
    impl core::ops::Deref for ExtSRgb {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for ExtSRgb {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ExtSRgb {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ExtSRgb {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ExtSRgb {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ExtSRgb {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ExtSRgb {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ExtSRgb {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ExtSRgb {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ExtSRgb>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ExtSRgb {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ExtSRgb {
        #[inline]
        fn from(obj: JsValue) -> ExtSRgb {
            ExtSRgb { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ExtSRgb {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ExtSRgb> for ExtSRgb {
        #[inline]
        fn as_ref(&self) -> &ExtSRgb {
            self
        }
    }
    impl From<ExtSRgb> for JsValue {
        #[inline]
        fn from(obj: ExtSRgb) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ExtSRgb {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_EXT_sRGB(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_EXT_sRGB(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_EXT_sRGB(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ExtSRgb { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ExtSRgb) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ExtSRgb> for ::js_sys::Object {
    #[inline]
    fn from(obj: ExtSRgb) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ExtSRgb {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
impl ExtSRgb {
    pub const SRGB_EXT: u32 = 35904u64 as u32;
}
impl ExtSRgb {
    pub const SRGB_ALPHA_EXT: u32 = 35906u64 as u32;
}
impl ExtSRgb {
    pub const SRGB8_ALPHA8_EXT: u32 = 35907u64 as u32;
}
impl ExtSRgb {
    pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT: u32 = 33296u64 as u32;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_7e657b784489f103: [u8; 145usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}O\0\0\0\0\0\x01\0\0\x02\x08EXT_sRGB\x1A__widl_instanceof_EXT_sRGB\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
