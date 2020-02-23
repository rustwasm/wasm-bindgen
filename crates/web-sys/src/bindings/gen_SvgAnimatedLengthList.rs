use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGAnimatedLengthList` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedLengthList)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLengthList`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgAnimatedLengthList {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgAnimatedLengthList: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgAnimatedLengthList {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(21u32);
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
            inform(76u32);
            inform(105u32);
            inform(115u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for SvgAnimatedLengthList {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgAnimatedLengthList {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgAnimatedLengthList {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgAnimatedLengthList {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgAnimatedLengthList {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgAnimatedLengthList {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgAnimatedLengthList {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgAnimatedLengthList {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgAnimatedLengthList {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgAnimatedLengthList>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgAnimatedLengthList {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgAnimatedLengthList {
        #[inline]
        fn from(obj: JsValue) -> SvgAnimatedLengthList {
            SvgAnimatedLengthList { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgAnimatedLengthList {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgAnimatedLengthList> for SvgAnimatedLengthList {
        #[inline]
        fn as_ref(&self) -> &SvgAnimatedLengthList {
            self
        }
    }
    impl From<SvgAnimatedLengthList> for JsValue {
        #[inline]
        fn from(obj: SvgAnimatedLengthList) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgAnimatedLengthList {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGAnimatedLengthList(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGAnimatedLengthList(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGAnimatedLengthList(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgAnimatedLengthList { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgAnimatedLengthList) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgAnimatedLengthList> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgAnimatedLengthList) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgAnimatedLengthList {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgAnimatedLengthList", feature = "SvgLengthList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_base_val_SVGAnimatedLengthList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgAnimatedLengthList as WasmDescribe>::describe();
    <SvgLengthList as WasmDescribe>::describe();
}
impl SvgAnimatedLengthList {
    #[cfg(all(feature = "SvgAnimatedLengthList", feature = "SvgLengthList",))]
    #[allow(bad_style)]
    #[doc = "The `baseVal` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedLengthList/baseVal)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLengthList`, `SvgLengthList`*"]
    #[allow(clippy::all)]
    pub fn base_val(&self) -> SvgLengthList {
        #[cfg(all(feature = "SvgAnimatedLengthList", feature = "SvgLengthList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_base_val_SVGAnimatedLengthList(
                self_: <&SvgAnimatedLengthList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgLengthList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_base_val_SVGAnimatedLengthList(
            self_: <&SvgAnimatedLengthList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgLengthList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgAnimatedLengthList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_base_val_SVGAnimatedLengthList(self_)
            };
            <SvgLengthList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLengthList", feature = "SvgLengthList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_anim_val_SVGAnimatedLengthList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgAnimatedLengthList as WasmDescribe>::describe();
    <SvgLengthList as WasmDescribe>::describe();
}
impl SvgAnimatedLengthList {
    #[cfg(all(feature = "SvgAnimatedLengthList", feature = "SvgLengthList",))]
    #[allow(bad_style)]
    #[doc = "The `animVal` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedLengthList/animVal)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLengthList`, `SvgLengthList`*"]
    #[allow(clippy::all)]
    pub fn anim_val(&self) -> SvgLengthList {
        #[cfg(all(feature = "SvgAnimatedLengthList", feature = "SvgLengthList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_anim_val_SVGAnimatedLengthList(
                self_: <&SvgAnimatedLengthList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgLengthList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_anim_val_SVGAnimatedLengthList(
            self_: <&SvgAnimatedLengthList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgLengthList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgAnimatedLengthList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_anim_val_SVGAnimatedLengthList(self_)
            };
            <SvgLengthList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_7c37c7650577bb92: [u8; 363usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"})\x01\0\0\0\0\x03\0\0\x02\x15SVGAnimatedLengthList'__widl_instanceof_SVGAnimatedLengthList\0\0\0\0'__widl_f_base_val_SVGAnimatedLengthList\0\0\0\x01\x15SVGAnimatedLengthList\x01\0\x01\x07baseVal\x01\x01\x05self_\x07baseVal\0\0\0'__widl_f_anim_val_SVGAnimatedLengthList\0\0\0\x01\x15SVGAnimatedLengthList\x01\0\x01\x07animVal\x01\x01\x05self_\x07animVal\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
