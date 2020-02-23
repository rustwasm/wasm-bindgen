use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGAnimatedLength` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedLength)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgAnimatedLength {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgAnimatedLength: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgAnimatedLength {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(17u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(65u32);
            inform(110u32);
            inform(105u32);
            inform(109u32);
            inform(97u32);
            inform(116u32);
            inform(101u32);
            inform(100u32);
            inform(76u32);
            inform(101u32);
            inform(110u32);
            inform(103u32);
            inform(116u32);
            inform(104u32);
        }
    }
    impl core::ops::Deref for SvgAnimatedLength {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgAnimatedLength {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgAnimatedLength {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgAnimatedLength {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgAnimatedLength {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgAnimatedLength {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgAnimatedLength {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgAnimatedLength {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgAnimatedLength {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgAnimatedLength>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgAnimatedLength {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgAnimatedLength {
        #[inline]
        fn from(obj: JsValue) -> SvgAnimatedLength {
            SvgAnimatedLength { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgAnimatedLength {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgAnimatedLength> for SvgAnimatedLength {
        #[inline]
        fn as_ref(&self) -> &SvgAnimatedLength {
            self
        }
    }
    impl From<SvgAnimatedLength> for JsValue {
        #[inline]
        fn from(obj: SvgAnimatedLength) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgAnimatedLength {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGAnimatedLength(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGAnimatedLength(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGAnimatedLength(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgAnimatedLength { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgAnimatedLength) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgAnimatedLength> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgAnimatedLength) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgAnimatedLength {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgLength",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_base_val_SVGAnimatedLength() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgAnimatedLength as WasmDescribe>::describe();
    <SvgLength as WasmDescribe>::describe();
}
impl SvgAnimatedLength {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgLength",))]
    #[allow(bad_style)]
    #[doc = "The `baseVal` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedLength/baseVal)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgLength`*"]
    #[allow(clippy::all)]
    pub fn base_val(&self) -> SvgLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgLength",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_base_val_SVGAnimatedLength(
                self_: <&SvgAnimatedLength as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_base_val_SVGAnimatedLength(
            self_: <&SvgAnimatedLength as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgLength as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgAnimatedLength as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_base_val_SVGAnimatedLength(self_)
            };
            <SvgLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgLength",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_anim_val_SVGAnimatedLength() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgAnimatedLength as WasmDescribe>::describe();
    <SvgLength as WasmDescribe>::describe();
}
impl SvgAnimatedLength {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgLength",))]
    #[allow(bad_style)]
    #[doc = "The `animVal` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedLength/animVal)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgLength`*"]
    #[allow(clippy::all)]
    pub fn anim_val(&self) -> SvgLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgLength",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_anim_val_SVGAnimatedLength(
                self_: <&SvgAnimatedLength as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_anim_val_SVGAnimatedLength(
            self_: <&SvgAnimatedLength as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgLength as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgAnimatedLength as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_anim_val_SVGAnimatedLength(self_)
            };
            <SvgLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_14f1f5b777db6e04: [u8; 339usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x11\x01\0\0\0\0\x03\0\0\x02\x11SVGAnimatedLength#__widl_instanceof_SVGAnimatedLength\0\0\0\0#__widl_f_base_val_SVGAnimatedLength\0\0\0\x01\x11SVGAnimatedLength\x01\0\x01\x07baseVal\x01\x01\x05self_\x07baseVal\0\0\0#__widl_f_anim_val_SVGAnimatedLength\0\0\0\x01\x11SVGAnimatedLength\x01\0\x01\x07animVal\x01\x01\x05self_\x07animVal\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
