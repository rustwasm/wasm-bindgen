use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `OES_texture_half_float_linear` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_texture_half_float_linear)\n\n*This API requires the following crate features to be activated: `OesTextureHalfFloatLinear`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct OesTextureHalfFloatLinear {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_OesTextureHalfFloatLinear: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for OesTextureHalfFloatLinear {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(29u32);
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
            inform(95u32);
            inform(108u32);
            inform(105u32);
            inform(110u32);
            inform(101u32);
            inform(97u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for OesTextureHalfFloatLinear {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for OesTextureHalfFloatLinear {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for OesTextureHalfFloatLinear {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a OesTextureHalfFloatLinear {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for OesTextureHalfFloatLinear {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            OesTextureHalfFloatLinear {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for OesTextureHalfFloatLinear {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a OesTextureHalfFloatLinear {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for OesTextureHalfFloatLinear {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<OesTextureHalfFloatLinear>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(OesTextureHalfFloatLinear {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for OesTextureHalfFloatLinear {
        #[inline]
        fn from(obj: JsValue) -> OesTextureHalfFloatLinear {
            OesTextureHalfFloatLinear { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for OesTextureHalfFloatLinear {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<OesTextureHalfFloatLinear> for OesTextureHalfFloatLinear {
        #[inline]
        fn as_ref(&self) -> &OesTextureHalfFloatLinear {
            self
        }
    }
    impl From<OesTextureHalfFloatLinear> for JsValue {
        #[inline]
        fn from(obj: OesTextureHalfFloatLinear) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for OesTextureHalfFloatLinear {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_OES_texture_half_float_linear(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_OES_texture_half_float_linear(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_OES_texture_half_float_linear(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            OesTextureHalfFloatLinear { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const OesTextureHalfFloatLinear) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<OesTextureHalfFloatLinear> for ::js_sys::Object {
    #[inline]
    fn from(obj: OesTextureHalfFloatLinear) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for OesTextureHalfFloatLinear {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_a58647e49957d1a4: [u8; 187usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}y\0\0\0\0\0\x01\0\0\x02\x1DOES_texture_half_float_linear/__widl_instanceof_OES_texture_half_float_linear\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
