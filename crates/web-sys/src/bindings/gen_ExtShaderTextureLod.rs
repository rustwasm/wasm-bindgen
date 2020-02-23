use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `EXT_shader_texture_lod` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_shader_texture_lod)\n\n*This API requires the following crate features to be activated: `ExtShaderTextureLod`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ExtShaderTextureLod {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ExtShaderTextureLod: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ExtShaderTextureLod {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(22u32);
            inform(69u32);
            inform(88u32);
            inform(84u32);
            inform(95u32);
            inform(115u32);
            inform(104u32);
            inform(97u32);
            inform(100u32);
            inform(101u32);
            inform(114u32);
            inform(95u32);
            inform(116u32);
            inform(101u32);
            inform(120u32);
            inform(116u32);
            inform(117u32);
            inform(114u32);
            inform(101u32);
            inform(95u32);
            inform(108u32);
            inform(111u32);
            inform(100u32);
        }
    }
    impl core::ops::Deref for ExtShaderTextureLod {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for ExtShaderTextureLod {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ExtShaderTextureLod {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ExtShaderTextureLod {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ExtShaderTextureLod {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ExtShaderTextureLod {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ExtShaderTextureLod {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ExtShaderTextureLod {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ExtShaderTextureLod {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ExtShaderTextureLod>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ExtShaderTextureLod {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ExtShaderTextureLod {
        #[inline]
        fn from(obj: JsValue) -> ExtShaderTextureLod {
            ExtShaderTextureLod { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ExtShaderTextureLod {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ExtShaderTextureLod> for ExtShaderTextureLod {
        #[inline]
        fn as_ref(&self) -> &ExtShaderTextureLod {
            self
        }
    }
    impl From<ExtShaderTextureLod> for JsValue {
        #[inline]
        fn from(obj: ExtShaderTextureLod) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ExtShaderTextureLod {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_EXT_shader_texture_lod(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_EXT_shader_texture_lod(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_EXT_shader_texture_lod(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ExtShaderTextureLod { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ExtShaderTextureLod) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ExtShaderTextureLod> for ::js_sys::Object {
    #[inline]
    fn from(obj: ExtShaderTextureLod) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ExtShaderTextureLod {
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
pub static __WASM_BINDGEN_GENERATED_88d095e3616ad380: [u8; 173usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}k\0\0\0\0\0\x01\0\0\x02\x16EXT_shader_texture_lod(__widl_instanceof_EXT_shader_texture_lod\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
