use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGAnimatedNumberList` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedNumberList)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumberList`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgAnimatedNumberList {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgAnimatedNumberList: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgAnimatedNumberList {
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
            inform(78u32);
            inform(117u32);
            inform(109u32);
            inform(98u32);
            inform(101u32);
            inform(114u32);
            inform(76u32);
            inform(105u32);
            inform(115u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for SvgAnimatedNumberList {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgAnimatedNumberList {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgAnimatedNumberList {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgAnimatedNumberList {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgAnimatedNumberList {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgAnimatedNumberList {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgAnimatedNumberList {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgAnimatedNumberList {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgAnimatedNumberList {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgAnimatedNumberList>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgAnimatedNumberList {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgAnimatedNumberList {
        #[inline]
        fn from(obj: JsValue) -> SvgAnimatedNumberList {
            SvgAnimatedNumberList { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgAnimatedNumberList {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgAnimatedNumberList> for SvgAnimatedNumberList {
        #[inline]
        fn as_ref(&self) -> &SvgAnimatedNumberList {
            self
        }
    }
    impl From<SvgAnimatedNumberList> for JsValue {
        #[inline]
        fn from(obj: SvgAnimatedNumberList) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgAnimatedNumberList {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGAnimatedNumberList(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGAnimatedNumberList(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGAnimatedNumberList(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgAnimatedNumberList { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgAnimatedNumberList) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgAnimatedNumberList> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgAnimatedNumberList) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgAnimatedNumberList {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgAnimatedNumberList", feature = "SvgNumberList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_base_val_SVGAnimatedNumberList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgAnimatedNumberList as WasmDescribe>::describe();
    <SvgNumberList as WasmDescribe>::describe();
}
impl SvgAnimatedNumberList {
    #[cfg(all(feature = "SvgAnimatedNumberList", feature = "SvgNumberList",))]
    #[allow(bad_style)]
    #[doc = "The `baseVal` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedNumberList/baseVal)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumberList`, `SvgNumberList`*"]
    #[allow(clippy::all)]
    pub fn base_val(&self) -> SvgNumberList {
        #[cfg(all(feature = "SvgAnimatedNumberList", feature = "SvgNumberList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_base_val_SVGAnimatedNumberList(
                self_: <&SvgAnimatedNumberList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgNumberList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_base_val_SVGAnimatedNumberList(
            self_: <&SvgAnimatedNumberList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgNumberList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgAnimatedNumberList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_base_val_SVGAnimatedNumberList(self_)
            };
            <SvgNumberList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedNumberList", feature = "SvgNumberList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_anim_val_SVGAnimatedNumberList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgAnimatedNumberList as WasmDescribe>::describe();
    <SvgNumberList as WasmDescribe>::describe();
}
impl SvgAnimatedNumberList {
    #[cfg(all(feature = "SvgAnimatedNumberList", feature = "SvgNumberList",))]
    #[allow(bad_style)]
    #[doc = "The `animVal` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedNumberList/animVal)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumberList`, `SvgNumberList`*"]
    #[allow(clippy::all)]
    pub fn anim_val(&self) -> SvgNumberList {
        #[cfg(all(feature = "SvgAnimatedNumberList", feature = "SvgNumberList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_anim_val_SVGAnimatedNumberList(
                self_: <&SvgAnimatedNumberList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgNumberList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_anim_val_SVGAnimatedNumberList(
            self_: <&SvgAnimatedNumberList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgNumberList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgAnimatedNumberList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_anim_val_SVGAnimatedNumberList(self_)
            };
            <SvgNumberList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_617fdcd46e8f21b7: [u8; 363usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"})\x01\0\0\0\0\x03\0\0\x02\x15SVGAnimatedNumberList'__widl_instanceof_SVGAnimatedNumberList\0\0\0\0'__widl_f_base_val_SVGAnimatedNumberList\0\0\0\x01\x15SVGAnimatedNumberList\x01\0\x01\x07baseVal\x01\x01\x05self_\x07baseVal\0\0\0'__widl_f_anim_val_SVGAnimatedNumberList\0\0\0\x01\x15SVGAnimatedNumberList\x01\0\x01\x07animVal\x01\x01\x05self_\x07animVal\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
