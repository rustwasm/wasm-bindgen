use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGNumber` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumber)\n\n*This API requires the following crate features to be activated: `SvgNumber`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgNumber {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgNumber: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgNumber {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(9u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(78u32);
            inform(117u32);
            inform(109u32);
            inform(98u32);
            inform(101u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for SvgNumber {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgNumber {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgNumber {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgNumber {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgNumber {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgNumber {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgNumber {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgNumber {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgNumber {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgNumber>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgNumber {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgNumber {
        #[inline]
        fn from(obj: JsValue) -> SvgNumber {
            SvgNumber { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgNumber {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgNumber> for SvgNumber {
        #[inline]
        fn as_ref(&self) -> &SvgNumber {
            self
        }
    }
    impl From<SvgNumber> for JsValue {
        #[inline]
        fn from(obj: SvgNumber) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgNumber {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGNumber(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGNumber(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGNumber(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgNumber { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgNumber) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgNumber> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgNumber) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgNumber {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgNumber",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_value_SVGNumber() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgNumber as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SvgNumber {
    #[cfg(all(feature = "SvgNumber",))]
    #[allow(bad_style)]
    #[doc = "The `value` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumber/value)\n\n*This API requires the following crate features to be activated: `SvgNumber`*"]
    #[allow(clippy::all)]
    pub fn value(&self) -> f32 {
        #[cfg(all(feature = "SvgNumber",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_value_SVGNumber(
                self_: <&SvgNumber as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_value_SVGNumber(
            self_: <&SvgNumber as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgNumber as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_value_SVGNumber(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgNumber",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_value_SVGNumber() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgNumber as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgNumber {
    #[cfg(all(feature = "SvgNumber",))]
    #[allow(bad_style)]
    #[doc = "The `value` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumber/value)\n\n*This API requires the following crate features to be activated: `SvgNumber`*"]
    #[allow(clippy::all)]
    pub fn set_value(&self, value: f32) {
        #[cfg(all(feature = "SvgNumber",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_value_SVGNumber(
                self_: <&SvgNumber as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_value_SVGNumber(
            self_: <&SvgNumber as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgNumber as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_set_value_SVGNumber(self_, value)
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
pub static __WASM_BINDGEN_GENERATED_150b810005531037: [u8; 287usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xDD\0\0\0\0\0\x03\0\0\x02\tSVGNumber\x1B__widl_instanceof_SVGNumber\0\0\0\0\x18__widl_f_value_SVGNumber\0\0\0\x01\tSVGNumber\x01\0\x01\x05value\x01\x01\x05self_\x05value\0\0\0\x1C__widl_f_set_value_SVGNumber\0\0\0\x01\tSVGNumber\x01\0\x02\x05value\x01\x02\x05self_\x05value\x05value\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
