use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGPathSeg` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSeg)\n\n*This API requires the following crate features to be activated: `SvgPathSeg`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgPathSeg {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgPathSeg: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgPathSeg {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(10u32);
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
        }
    }
    impl core::ops::Deref for SvgPathSeg {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgPathSeg {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgPathSeg {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgPathSeg {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgPathSeg {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgPathSeg {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgPathSeg {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgPathSeg {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgPathSeg {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgPathSeg>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgPathSeg {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgPathSeg {
        #[inline]
        fn from(obj: JsValue) -> SvgPathSeg {
            SvgPathSeg { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgPathSeg {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgPathSeg> for SvgPathSeg {
        #[inline]
        fn as_ref(&self) -> &SvgPathSeg {
            self
        }
    }
    impl From<SvgPathSeg> for JsValue {
        #[inline]
        fn from(obj: SvgPathSeg) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgPathSeg {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGPathSeg(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGPathSeg(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGPathSeg(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgPathSeg { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgPathSeg) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgPathSeg> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgPathSeg) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgPathSeg {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgPathSeg",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_path_seg_type_SVGPathSeg() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgPathSeg as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl SvgPathSeg {
    #[cfg(all(feature = "SvgPathSeg",))]
    #[allow(bad_style)]
    #[doc = "The `pathSegType` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSeg/pathSegType)\n\n*This API requires the following crate features to be activated: `SvgPathSeg`*"]
    #[allow(clippy::all)]
    pub fn path_seg_type(&self) -> u16 {
        #[cfg(all(feature = "SvgPathSeg",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_path_seg_type_SVGPathSeg(
                self_: <&SvgPathSeg as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_path_seg_type_SVGPathSeg(
            self_: <&SvgPathSeg as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgPathSeg as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_path_seg_type_SVGPathSeg(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgPathSeg",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_path_seg_type_as_letter_SVGPathSeg() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgPathSeg as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl SvgPathSeg {
    #[cfg(all(feature = "SvgPathSeg",))]
    #[allow(bad_style)]
    #[doc = "The `pathSegTypeAsLetter` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSeg/pathSegTypeAsLetter)\n\n*This API requires the following crate features to be activated: `SvgPathSeg`*"]
    #[allow(clippy::all)]
    pub fn path_seg_type_as_letter(&self) -> String {
        #[cfg(all(feature = "SvgPathSeg",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_path_seg_type_as_letter_SVGPathSeg(
                self_: <&SvgPathSeg as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_path_seg_type_as_letter_SVGPathSeg(
            self_: <&SvgPathSeg as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgPathSeg as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_path_seg_type_as_letter_SVGPathSeg(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
impl SvgPathSeg {
    pub const PATHSEG_UNKNOWN: u16 = 0i64 as u16;
}
impl SvgPathSeg {
    pub const PATHSEG_CLOSEPATH: u16 = 1u64 as u16;
}
impl SvgPathSeg {
    pub const PATHSEG_MOVETO_ABS: u16 = 2u64 as u16;
}
impl SvgPathSeg {
    pub const PATHSEG_MOVETO_REL: u16 = 3u64 as u16;
}
impl SvgPathSeg {
    pub const PATHSEG_LINETO_ABS: u16 = 4u64 as u16;
}
impl SvgPathSeg {
    pub const PATHSEG_LINETO_REL: u16 = 5u64 as u16;
}
impl SvgPathSeg {
    pub const PATHSEG_CURVETO_CUBIC_ABS: u16 = 6u64 as u16;
}
impl SvgPathSeg {
    pub const PATHSEG_CURVETO_CUBIC_REL: u16 = 7u64 as u16;
}
impl SvgPathSeg {
    pub const PATHSEG_CURVETO_QUADRATIC_ABS: u16 = 8u64 as u16;
}
impl SvgPathSeg {
    pub const PATHSEG_CURVETO_QUADRATIC_REL: u16 = 9u64 as u16;
}
impl SvgPathSeg {
    pub const PATHSEG_ARC_ABS: u16 = 10u64 as u16;
}
impl SvgPathSeg {
    pub const PATHSEG_ARC_REL: u16 = 11u64 as u16;
}
impl SvgPathSeg {
    pub const PATHSEG_LINETO_HORIZONTAL_ABS: u16 = 12u64 as u16;
}
impl SvgPathSeg {
    pub const PATHSEG_LINETO_HORIZONTAL_REL: u16 = 13u64 as u16;
}
impl SvgPathSeg {
    pub const PATHSEG_LINETO_VERTICAL_ABS: u16 = 14u64 as u16;
}
impl SvgPathSeg {
    pub const PATHSEG_LINETO_VERTICAL_REL: u16 = 15u64 as u16;
}
impl SvgPathSeg {
    pub const PATHSEG_CURVETO_CUBIC_SMOOTH_ABS: u16 = 16u64 as u16;
}
impl SvgPathSeg {
    pub const PATHSEG_CURVETO_CUBIC_SMOOTH_REL: u16 = 17u64 as u16;
}
impl SvgPathSeg {
    pub const PATHSEG_CURVETO_QUADRATIC_SMOOTH_ABS: u16 = 18u64 as u16;
}
impl SvgPathSeg {
    pub const PATHSEG_CURVETO_QUADRATIC_SMOOTH_REL: u16 = 19u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_824b7317b97bd607: [u8; 349usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x1B\x01\0\0\0\0\x03\0\0\x02\nSVGPathSeg\x1C__widl_instanceof_SVGPathSeg\0\0\0\0!__widl_f_path_seg_type_SVGPathSeg\0\0\0\x01\nSVGPathSeg\x01\0\x01\x0BpathSegType\x01\x01\x05self_\x0BpathSegType\0\0\0+__widl_f_path_seg_type_as_letter_SVGPathSeg\0\0\0\x01\nSVGPathSeg\x01\0\x01\x13pathSegTypeAsLetter\x01\x01\x05self_\x13pathSegTypeAsLetter\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
