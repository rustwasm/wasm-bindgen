use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGAnimatedEnumeration` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedEnumeration)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgAnimatedEnumeration {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgAnimatedEnumeration: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgAnimatedEnumeration {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(22u32);
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
            inform(69u32);
            inform(110u32);
            inform(117u32);
            inform(109u32);
            inform(101u32);
            inform(114u32);
            inform(97u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for SvgAnimatedEnumeration {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgAnimatedEnumeration {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgAnimatedEnumeration {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgAnimatedEnumeration {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgAnimatedEnumeration {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgAnimatedEnumeration {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgAnimatedEnumeration {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgAnimatedEnumeration {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgAnimatedEnumeration {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgAnimatedEnumeration>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgAnimatedEnumeration {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgAnimatedEnumeration {
        #[inline]
        fn from(obj: JsValue) -> SvgAnimatedEnumeration {
            SvgAnimatedEnumeration { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgAnimatedEnumeration {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgAnimatedEnumeration> for SvgAnimatedEnumeration {
        #[inline]
        fn as_ref(&self) -> &SvgAnimatedEnumeration {
            self
        }
    }
    impl From<SvgAnimatedEnumeration> for JsValue {
        #[inline]
        fn from(obj: SvgAnimatedEnumeration) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgAnimatedEnumeration {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGAnimatedEnumeration(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGAnimatedEnumeration(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGAnimatedEnumeration(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgAnimatedEnumeration { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgAnimatedEnumeration) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgAnimatedEnumeration> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgAnimatedEnumeration) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgAnimatedEnumeration {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgAnimatedEnumeration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_base_val_SVGAnimatedEnumeration() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgAnimatedEnumeration as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl SvgAnimatedEnumeration {
    #[cfg(all(feature = "SvgAnimatedEnumeration",))]
    #[allow(bad_style)]
    #[doc = "The `baseVal` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedEnumeration/baseVal)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`*"]
    #[allow(clippy::all)]
    pub fn base_val(&self) -> u16 {
        #[cfg(all(feature = "SvgAnimatedEnumeration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_base_val_SVGAnimatedEnumeration(
                self_: <&SvgAnimatedEnumeration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_base_val_SVGAnimatedEnumeration(
            self_: <&SvgAnimatedEnumeration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgAnimatedEnumeration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_base_val_SVGAnimatedEnumeration(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedEnumeration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_base_val_SVGAnimatedEnumeration() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgAnimatedEnumeration as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgAnimatedEnumeration {
    #[cfg(all(feature = "SvgAnimatedEnumeration",))]
    #[allow(bad_style)]
    #[doc = "The `baseVal` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedEnumeration/baseVal)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`*"]
    #[allow(clippy::all)]
    pub fn set_base_val(&self, base_val: u16) {
        #[cfg(all(feature = "SvgAnimatedEnumeration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_base_val_SVGAnimatedEnumeration(
                self_: <&SvgAnimatedEnumeration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                base_val: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_base_val_SVGAnimatedEnumeration(
            self_: <&SvgAnimatedEnumeration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            base_val: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(base_val);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgAnimatedEnumeration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let base_val = <u16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(base_val);
                __widl_f_set_base_val_SVGAnimatedEnumeration(self_, base_val)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgAnimatedEnumeration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_anim_val_SVGAnimatedEnumeration() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgAnimatedEnumeration as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl SvgAnimatedEnumeration {
    #[cfg(all(feature = "SvgAnimatedEnumeration",))]
    #[allow(bad_style)]
    #[doc = "The `animVal` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedEnumeration/animVal)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`*"]
    #[allow(clippy::all)]
    pub fn anim_val(&self) -> u16 {
        #[cfg(all(feature = "SvgAnimatedEnumeration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_anim_val_SVGAnimatedEnumeration(
                self_: <&SvgAnimatedEnumeration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_anim_val_SVGAnimatedEnumeration(
            self_: <&SvgAnimatedEnumeration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgAnimatedEnumeration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_anim_val_SVGAnimatedEnumeration(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_f4388d708817af75: [u8; 480usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x9E\x01\0\0\0\0\x04\0\0\x02\x16SVGAnimatedEnumeration(__widl_instanceof_SVGAnimatedEnumeration\0\0\0\0(__widl_f_base_val_SVGAnimatedEnumeration\0\0\0\x01\x16SVGAnimatedEnumeration\x01\0\x01\x07baseVal\x01\x01\x05self_\x07baseVal\0\0\0,__widl_f_set_base_val_SVGAnimatedEnumeration\0\0\0\x01\x16SVGAnimatedEnumeration\x01\0\x02\x07baseVal\x01\x02\x05self_\x08base_val\x07baseVal\0\0\0(__widl_f_anim_val_SVGAnimatedEnumeration\0\0\0\x01\x16SVGAnimatedEnumeration\x01\0\x01\x07animVal\x01\x01\x05self_\x07animVal\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
