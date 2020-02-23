use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `OES_texture_float_linear` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_texture_float_linear)\n\n*This API requires the following crate features to be activated: `OesTextureFloatLinear`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct OesTextureFloatLinear {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_OesTextureFloatLinear: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for OesTextureFloatLinear {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(24u32);
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
    impl core::ops::Deref for OesTextureFloatLinear {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for OesTextureFloatLinear {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for OesTextureFloatLinear {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a OesTextureFloatLinear {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for OesTextureFloatLinear {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            OesTextureFloatLinear {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for OesTextureFloatLinear {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a OesTextureFloatLinear {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for OesTextureFloatLinear {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<OesTextureFloatLinear>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(OesTextureFloatLinear {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for OesTextureFloatLinear {
        #[inline]
        fn from(obj: JsValue) -> OesTextureFloatLinear {
            OesTextureFloatLinear { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for OesTextureFloatLinear {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<OesTextureFloatLinear> for OesTextureFloatLinear {
        #[inline]
        fn as_ref(&self) -> &OesTextureFloatLinear {
            self
        }
    }
    impl From<OesTextureFloatLinear> for JsValue {
        #[inline]
        fn from(obj: OesTextureFloatLinear) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for OesTextureFloatLinear {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_OES_texture_float_linear(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_OES_texture_float_linear(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_OES_texture_float_linear(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            OesTextureFloatLinear { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const OesTextureFloatLinear) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<OesTextureFloatLinear> for ::js_sys::Object {
    #[inline]
    fn from(obj: OesTextureFloatLinear) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for OesTextureFloatLinear {
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
pub static __WASM_BINDGEN_GENERATED_b1cbc57fcff50841: [u8; 177usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}o\0\0\0\0\0\x01\0\0\x02\x18OES_texture_float_linear*__widl_instanceof_OES_texture_float_linear\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
