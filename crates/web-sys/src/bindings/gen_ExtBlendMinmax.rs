use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `EXT_blend_minmax` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_blend_minmax)\n\n*This API requires the following crate features to be activated: `ExtBlendMinmax`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ExtBlendMinmax {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ExtBlendMinmax: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ExtBlendMinmax {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(69u32);
            inform(88u32);
            inform(84u32);
            inform(95u32);
            inform(98u32);
            inform(108u32);
            inform(101u32);
            inform(110u32);
            inform(100u32);
            inform(95u32);
            inform(109u32);
            inform(105u32);
            inform(110u32);
            inform(109u32);
            inform(97u32);
            inform(120u32);
        }
    }
    impl core::ops::Deref for ExtBlendMinmax {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for ExtBlendMinmax {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ExtBlendMinmax {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ExtBlendMinmax {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ExtBlendMinmax {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ExtBlendMinmax {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ExtBlendMinmax {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ExtBlendMinmax {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ExtBlendMinmax {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ExtBlendMinmax>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ExtBlendMinmax {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ExtBlendMinmax {
        #[inline]
        fn from(obj: JsValue) -> ExtBlendMinmax {
            ExtBlendMinmax { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ExtBlendMinmax {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ExtBlendMinmax> for ExtBlendMinmax {
        #[inline]
        fn as_ref(&self) -> &ExtBlendMinmax {
            self
        }
    }
    impl From<ExtBlendMinmax> for JsValue {
        #[inline]
        fn from(obj: ExtBlendMinmax) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ExtBlendMinmax {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_EXT_blend_minmax(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_EXT_blend_minmax(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_EXT_blend_minmax(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ExtBlendMinmax { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ExtBlendMinmax) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ExtBlendMinmax> for ::js_sys::Object {
    #[inline]
    fn from(obj: ExtBlendMinmax) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ExtBlendMinmax {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
impl ExtBlendMinmax {
    pub const MIN_EXT: u32 = 32775u64 as u32;
}
impl ExtBlendMinmax {
    pub const MAX_EXT: u32 = 32776u64 as u32;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_6ba5549d79197e95: [u8; 161usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}_\0\0\0\0\0\x01\0\0\x02\x10EXT_blend_minmax\"__widl_instanceof_EXT_blend_minmax\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
