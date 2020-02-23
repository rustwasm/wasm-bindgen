use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGPreserveAspectRatio` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPreserveAspectRatio)\n\n*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgPreserveAspectRatio {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgPreserveAspectRatio: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgPreserveAspectRatio {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(22u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(80u32);
            inform(114u32);
            inform(101u32);
            inform(115u32);
            inform(101u32);
            inform(114u32);
            inform(118u32);
            inform(101u32);
            inform(65u32);
            inform(115u32);
            inform(112u32);
            inform(101u32);
            inform(99u32);
            inform(116u32);
            inform(82u32);
            inform(97u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
        }
    }
    impl core::ops::Deref for SvgPreserveAspectRatio {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgPreserveAspectRatio {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgPreserveAspectRatio {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgPreserveAspectRatio {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgPreserveAspectRatio {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgPreserveAspectRatio {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgPreserveAspectRatio {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgPreserveAspectRatio {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgPreserveAspectRatio {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgPreserveAspectRatio>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgPreserveAspectRatio {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgPreserveAspectRatio {
        #[inline]
        fn from(obj: JsValue) -> SvgPreserveAspectRatio {
            SvgPreserveAspectRatio { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgPreserveAspectRatio {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgPreserveAspectRatio> for SvgPreserveAspectRatio {
        #[inline]
        fn as_ref(&self) -> &SvgPreserveAspectRatio {
            self
        }
    }
    impl From<SvgPreserveAspectRatio> for JsValue {
        #[inline]
        fn from(obj: SvgPreserveAspectRatio) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgPreserveAspectRatio {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGPreserveAspectRatio(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGPreserveAspectRatio(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGPreserveAspectRatio(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgPreserveAspectRatio { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgPreserveAspectRatio) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgPreserveAspectRatio> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgPreserveAspectRatio) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgPreserveAspectRatio {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgPreserveAspectRatio",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_align_SVGPreserveAspectRatio() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgPreserveAspectRatio as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl SvgPreserveAspectRatio {
    #[cfg(all(feature = "SvgPreserveAspectRatio",))]
    #[allow(bad_style)]
    #[doc = "The `align` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPreserveAspectRatio/align)\n\n*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    #[allow(clippy::all)]
    pub fn align(&self) -> u16 {
        #[cfg(all(feature = "SvgPreserveAspectRatio",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_align_SVGPreserveAspectRatio(
                self_: <&SvgPreserveAspectRatio as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_align_SVGPreserveAspectRatio(
            self_: <&SvgPreserveAspectRatio as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgPreserveAspectRatio as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_align_SVGPreserveAspectRatio(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgPreserveAspectRatio",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_align_SVGPreserveAspectRatio() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgPreserveAspectRatio as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgPreserveAspectRatio {
    #[cfg(all(feature = "SvgPreserveAspectRatio",))]
    #[allow(bad_style)]
    #[doc = "The `align` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPreserveAspectRatio/align)\n\n*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    #[allow(clippy::all)]
    pub fn set_align(&self, align: u16) {
        #[cfg(all(feature = "SvgPreserveAspectRatio",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_align_SVGPreserveAspectRatio(
                self_: <&SvgPreserveAspectRatio as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                align: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_align_SVGPreserveAspectRatio(
            self_: <&SvgPreserveAspectRatio as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            align: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(align);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgPreserveAspectRatio as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let align = <u16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(align);
                __widl_f_set_align_SVGPreserveAspectRatio(self_, align)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgPreserveAspectRatio",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_meet_or_slice_SVGPreserveAspectRatio() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgPreserveAspectRatio as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl SvgPreserveAspectRatio {
    #[cfg(all(feature = "SvgPreserveAspectRatio",))]
    #[allow(bad_style)]
    #[doc = "The `meetOrSlice` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPreserveAspectRatio/meetOrSlice)\n\n*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    #[allow(clippy::all)]
    pub fn meet_or_slice(&self) -> u16 {
        #[cfg(all(feature = "SvgPreserveAspectRatio",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_meet_or_slice_SVGPreserveAspectRatio(
                self_: <&SvgPreserveAspectRatio as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_meet_or_slice_SVGPreserveAspectRatio(
            self_: <&SvgPreserveAspectRatio as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgPreserveAspectRatio as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_meet_or_slice_SVGPreserveAspectRatio(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgPreserveAspectRatio",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_meet_or_slice_SVGPreserveAspectRatio() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgPreserveAspectRatio as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgPreserveAspectRatio {
    #[cfg(all(feature = "SvgPreserveAspectRatio",))]
    #[allow(bad_style)]
    #[doc = "The `meetOrSlice` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPreserveAspectRatio/meetOrSlice)\n\n*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    #[allow(clippy::all)]
    pub fn set_meet_or_slice(&self, meet_or_slice: u16) {
        #[cfg(all(feature = "SvgPreserveAspectRatio",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_meet_or_slice_SVGPreserveAspectRatio(
                self_: <&SvgPreserveAspectRatio as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                meet_or_slice: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_meet_or_slice_SVGPreserveAspectRatio(
            self_: <&SvgPreserveAspectRatio as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            meet_or_slice: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(meet_or_slice);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgPreserveAspectRatio as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let meet_or_slice =
                    <u16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(meet_or_slice);
                __widl_f_set_meet_or_slice_SVGPreserveAspectRatio(self_, meet_or_slice)
            };
            ()
        }
    }
}
impl SvgPreserveAspectRatio {
    pub const SVG_PRESERVEASPECTRATIO_UNKNOWN: u16 = 0i64 as u16;
}
impl SvgPreserveAspectRatio {
    pub const SVG_PRESERVEASPECTRATIO_NONE: u16 = 1u64 as u16;
}
impl SvgPreserveAspectRatio {
    pub const SVG_PRESERVEASPECTRATIO_XMINYMIN: u16 = 2u64 as u16;
}
impl SvgPreserveAspectRatio {
    pub const SVG_PRESERVEASPECTRATIO_XMIDYMIN: u16 = 3u64 as u16;
}
impl SvgPreserveAspectRatio {
    pub const SVG_PRESERVEASPECTRATIO_XMAXYMIN: u16 = 4u64 as u16;
}
impl SvgPreserveAspectRatio {
    pub const SVG_PRESERVEASPECTRATIO_XMINYMID: u16 = 5u64 as u16;
}
impl SvgPreserveAspectRatio {
    pub const SVG_PRESERVEASPECTRATIO_XMIDYMID: u16 = 6u64 as u16;
}
impl SvgPreserveAspectRatio {
    pub const SVG_PRESERVEASPECTRATIO_XMAXYMID: u16 = 7u64 as u16;
}
impl SvgPreserveAspectRatio {
    pub const SVG_PRESERVEASPECTRATIO_XMINYMAX: u16 = 8u64 as u16;
}
impl SvgPreserveAspectRatio {
    pub const SVG_PRESERVEASPECTRATIO_XMIDYMAX: u16 = 9u64 as u16;
}
impl SvgPreserveAspectRatio {
    pub const SVG_PRESERVEASPECTRATIO_XMAXYMAX: u16 = 10u64 as u16;
}
impl SvgPreserveAspectRatio {
    pub const SVG_MEETORSLICE_UNKNOWN: u16 = 0i64 as u16;
}
impl SvgPreserveAspectRatio {
    pub const SVG_MEETORSLICE_MEET: u16 = 1u64 as u16;
}
impl SvgPreserveAspectRatio {
    pub const SVG_MEETORSLICE_SLICE: u16 = 2u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_c95ba95235870b4b: [u8; 605usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x1B\x02\0\0\0\0\x05\0\0\x02\x16SVGPreserveAspectRatio(__widl_instanceof_SVGPreserveAspectRatio\0\0\0\0%__widl_f_align_SVGPreserveAspectRatio\0\0\0\x01\x16SVGPreserveAspectRatio\x01\0\x01\x05align\x01\x01\x05self_\x05align\0\0\0)__widl_f_set_align_SVGPreserveAspectRatio\0\0\0\x01\x16SVGPreserveAspectRatio\x01\0\x02\x05align\x01\x02\x05self_\x05align\x05align\0\0\0-__widl_f_meet_or_slice_SVGPreserveAspectRatio\0\0\0\x01\x16SVGPreserveAspectRatio\x01\0\x01\x0BmeetOrSlice\x01\x01\x05self_\x0BmeetOrSlice\0\0\01__widl_f_set_meet_or_slice_SVGPreserveAspectRatio\0\0\0\x01\x16SVGPreserveAspectRatio\x01\0\x02\x0BmeetOrSlice\x01\x02\x05self_\rmeet_or_slice\x0BmeetOrSlice\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
