use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGPathSegLinetoHorizontalAbs` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoHorizontalAbs)\n\n*This API requires the following crate features to be activated: `SvgPathSegLinetoHorizontalAbs`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgPathSegLinetoHorizontalAbs {
    obj: SvgPathSeg,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgPathSegLinetoHorizontalAbs: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgPathSegLinetoHorizontalAbs {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(29u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(80u32);
            inform(97u32);
            inform(116u32);
            inform(104u32);
            inform(83u32);
            inform(101u32);
            inform(103u32);
            inform(76u32);
            inform(105u32);
            inform(110u32);
            inform(101u32);
            inform(116u32);
            inform(111u32);
            inform(72u32);
            inform(111u32);
            inform(114u32);
            inform(105u32);
            inform(122u32);
            inform(111u32);
            inform(110u32);
            inform(116u32);
            inform(97u32);
            inform(108u32);
            inform(65u32);
            inform(98u32);
            inform(115u32);
        }
    }
    impl core::ops::Deref for SvgPathSegLinetoHorizontalAbs {
        type Target = SvgPathSeg;
        #[inline]
        fn deref(&self) -> &SvgPathSeg {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgPathSegLinetoHorizontalAbs {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgPathSegLinetoHorizontalAbs {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgPathSegLinetoHorizontalAbs {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgPathSegLinetoHorizontalAbs {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgPathSegLinetoHorizontalAbs {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgPathSegLinetoHorizontalAbs {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgPathSegLinetoHorizontalAbs {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgPathSegLinetoHorizontalAbs {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgPathSegLinetoHorizontalAbs>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgPathSegLinetoHorizontalAbs {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgPathSegLinetoHorizontalAbs {
        #[inline]
        fn from(obj: JsValue) -> SvgPathSegLinetoHorizontalAbs {
            SvgPathSegLinetoHorizontalAbs { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgPathSegLinetoHorizontalAbs {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgPathSegLinetoHorizontalAbs> for SvgPathSegLinetoHorizontalAbs {
        #[inline]
        fn as_ref(&self) -> &SvgPathSegLinetoHorizontalAbs {
            self
        }
    }
    impl From<SvgPathSegLinetoHorizontalAbs> for JsValue {
        #[inline]
        fn from(obj: SvgPathSegLinetoHorizontalAbs) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgPathSegLinetoHorizontalAbs {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGPathSegLinetoHorizontalAbs(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGPathSegLinetoHorizontalAbs(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGPathSegLinetoHorizontalAbs(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgPathSegLinetoHorizontalAbs { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgPathSegLinetoHorizontalAbs) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgPathSegLinetoHorizontalAbs> for SvgPathSeg {
    #[inline]
    fn from(obj: SvgPathSegLinetoHorizontalAbs) -> SvgPathSeg {
        use wasm_bindgen::JsCast;
        SvgPathSeg::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgPathSeg> for SvgPathSegLinetoHorizontalAbs {
    #[inline]
    fn as_ref(&self) -> &SvgPathSeg {
        use wasm_bindgen::JsCast;
        SvgPathSeg::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgPathSegLinetoHorizontalAbs> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgPathSegLinetoHorizontalAbs) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgPathSegLinetoHorizontalAbs {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgPathSegLinetoHorizontalAbs",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_x_SVGPathSegLinetoHorizontalAbs() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgPathSegLinetoHorizontalAbs as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SvgPathSegLinetoHorizontalAbs {
    #[cfg(all(feature = "SvgPathSegLinetoHorizontalAbs",))]
    #[allow(bad_style)]
    #[doc = "The `x` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoHorizontalAbs/x)\n\n*This API requires the following crate features to be activated: `SvgPathSegLinetoHorizontalAbs`*"]
    #[allow(clippy::all)]
    pub fn x(&self) -> f32 {
        #[cfg(all(feature = "SvgPathSegLinetoHorizontalAbs",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_x_SVGPathSegLinetoHorizontalAbs(
                self_: <&SvgPathSegLinetoHorizontalAbs as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_x_SVGPathSegLinetoHorizontalAbs(
            self_: <&SvgPathSegLinetoHorizontalAbs as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = < & SvgPathSegLinetoHorizontalAbs as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( self ) ;
                __widl_f_x_SVGPathSegLinetoHorizontalAbs(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgPathSegLinetoHorizontalAbs",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_x_SVGPathSegLinetoHorizontalAbs() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgPathSegLinetoHorizontalAbs as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgPathSegLinetoHorizontalAbs {
    #[cfg(all(feature = "SvgPathSegLinetoHorizontalAbs",))]
    #[allow(bad_style)]
    #[doc = "The `x` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoHorizontalAbs/x)\n\n*This API requires the following crate features to be activated: `SvgPathSegLinetoHorizontalAbs`*"]
    #[allow(clippy::all)]
    pub fn set_x(&self, x: f32) {
        #[cfg(all(feature = "SvgPathSegLinetoHorizontalAbs",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_x_SVGPathSegLinetoHorizontalAbs(
                self_: <&SvgPathSegLinetoHorizontalAbs as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_x_SVGPathSegLinetoHorizontalAbs(
            self_: <&SvgPathSegLinetoHorizontalAbs as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = < & SvgPathSegLinetoHorizontalAbs as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( self ) ;
                let x = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                __widl_f_set_x_SVGPathSegLinetoHorizontalAbs(self_, x)
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
pub static __WASM_BINDGEN_GENERATED_9934f83275d14a64: [u8; 379usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}9\x01\0\0\0\0\x03\0\0\x02\x1DSVGPathSegLinetoHorizontalAbs/__widl_instanceof_SVGPathSegLinetoHorizontalAbs\0\0\0\0(__widl_f_x_SVGPathSegLinetoHorizontalAbs\0\0\0\x01\x1DSVGPathSegLinetoHorizontalAbs\x01\0\x01\x01x\x01\x01\x05self_\x01x\0\0\0,__widl_f_set_x_SVGPathSegLinetoHorizontalAbs\0\0\0\x01\x1DSVGPathSegLinetoHorizontalAbs\x01\0\x02\x01x\x01\x02\x05self_\x01x\x01x\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
