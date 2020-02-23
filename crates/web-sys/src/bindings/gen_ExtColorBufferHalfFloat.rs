use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `EXT_color_buffer_half_float` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_color_buffer_half_float)\n\n*This API requires the following crate features to be activated: `ExtColorBufferHalfFloat`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ExtColorBufferHalfFloat {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ExtColorBufferHalfFloat: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ExtColorBufferHalfFloat {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(27u32);
            inform(69u32);
            inform(88u32);
            inform(84u32);
            inform(95u32);
            inform(99u32);
            inform(111u32);
            inform(108u32);
            inform(111u32);
            inform(114u32);
            inform(95u32);
            inform(98u32);
            inform(117u32);
            inform(102u32);
            inform(102u32);
            inform(101u32);
            inform(114u32);
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
    impl core::ops::Deref for ExtColorBufferHalfFloat {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for ExtColorBufferHalfFloat {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ExtColorBufferHalfFloat {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ExtColorBufferHalfFloat {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ExtColorBufferHalfFloat {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ExtColorBufferHalfFloat {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ExtColorBufferHalfFloat {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ExtColorBufferHalfFloat {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ExtColorBufferHalfFloat {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ExtColorBufferHalfFloat>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ExtColorBufferHalfFloat {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ExtColorBufferHalfFloat {
        #[inline]
        fn from(obj: JsValue) -> ExtColorBufferHalfFloat {
            ExtColorBufferHalfFloat { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ExtColorBufferHalfFloat {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ExtColorBufferHalfFloat> for ExtColorBufferHalfFloat {
        #[inline]
        fn as_ref(&self) -> &ExtColorBufferHalfFloat {
            self
        }
    }
    impl From<ExtColorBufferHalfFloat> for JsValue {
        #[inline]
        fn from(obj: ExtColorBufferHalfFloat) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ExtColorBufferHalfFloat {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_EXT_color_buffer_half_float(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_EXT_color_buffer_half_float(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_EXT_color_buffer_half_float(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ExtColorBufferHalfFloat { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ExtColorBufferHalfFloat) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ExtColorBufferHalfFloat> for ::js_sys::Object {
    #[inline]
    fn from(obj: ExtColorBufferHalfFloat) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ExtColorBufferHalfFloat {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
impl ExtColorBufferHalfFloat {
    pub const RGBA16F_EXT: u32 = 34842u64 as u32;
}
impl ExtColorBufferHalfFloat {
    pub const RGB16F_EXT: u32 = 34843u64 as u32;
}
impl ExtColorBufferHalfFloat {
    pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT: u32 = 33297u64 as u32;
}
impl ExtColorBufferHalfFloat {
    pub const UNSIGNED_NORMALIZED_EXT: u32 = 35863u64 as u32;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_d81a0b04b9157746: [u8; 183usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}u\0\0\0\0\0\x01\0\0\x02\x1BEXT_color_buffer_half_float-__widl_instanceof_EXT_color_buffer_half_float\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
