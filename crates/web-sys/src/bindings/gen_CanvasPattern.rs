use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `CanvasPattern` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasPattern)\n\n*This API requires the following crate features to be activated: `CanvasPattern`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct CanvasPattern {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CanvasPattern: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for CanvasPattern {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
            inform(67u32);
            inform(97u32);
            inform(110u32);
            inform(118u32);
            inform(97u32);
            inform(115u32);
            inform(80u32);
            inform(97u32);
            inform(116u32);
            inform(116u32);
            inform(101u32);
            inform(114u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for CanvasPattern {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for CanvasPattern {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for CanvasPattern {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CanvasPattern {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for CanvasPattern {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            CanvasPattern {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for CanvasPattern {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a CanvasPattern {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for CanvasPattern {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<CanvasPattern>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(CanvasPattern {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for CanvasPattern {
        #[inline]
        fn from(obj: JsValue) -> CanvasPattern {
            CanvasPattern { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for CanvasPattern {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<CanvasPattern> for CanvasPattern {
        #[inline]
        fn as_ref(&self) -> &CanvasPattern {
            self
        }
    }
    impl From<CanvasPattern> for JsValue {
        #[inline]
        fn from(obj: CanvasPattern) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for CanvasPattern {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_CanvasPattern(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_CanvasPattern(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_CanvasPattern(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CanvasPattern { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CanvasPattern) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<CanvasPattern> for ::js_sys::Object {
    #[inline]
    fn from(obj: CanvasPattern) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for CanvasPattern {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "CanvasPattern", feature = "SvgMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_transform_CanvasPattern() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasPattern as WasmDescribe>::describe();
    <&SvgMatrix as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasPattern {
    #[cfg(all(feature = "CanvasPattern", feature = "SvgMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `setTransform()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasPattern/setTransform)\n\n*This API requires the following crate features to be activated: `CanvasPattern`, `SvgMatrix`*"]
    #[allow(clippy::all)]
    pub fn set_transform(&self, matrix: &SvgMatrix) {
        #[cfg(all(feature = "CanvasPattern", feature = "SvgMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_transform_CanvasPattern(
                self_: <&CanvasPattern as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                matrix: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_transform_CanvasPattern(
            self_: <&CanvasPattern as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            matrix: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(matrix);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CanvasPattern as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let matrix = <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(matrix);
                __widl_f_set_transform_CanvasPattern(self_, matrix)
            };
            ()
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_95c723bc702f6334: [u8; 244usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xB2\0\0\0\0\0\x02\0\0\x02\rCanvasPattern\x1F__widl_instanceof_CanvasPattern\0\0\0\0$__widl_f_set_transform_CanvasPattern\0\0\0\x01\rCanvasPattern\x01\0\0\x01\x02\x05self_\x06matrix\x0CsetTransform\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
