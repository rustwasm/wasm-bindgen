use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGZoomAndPan` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGZoomAndPan)\n\n*This API requires the following crate features to be activated: `SvgZoomAndPan`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgZoomAndPan {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgZoomAndPan: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgZoomAndPan {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(90u32);
            inform(111u32);
            inform(111u32);
            inform(109u32);
            inform(65u32);
            inform(110u32);
            inform(100u32);
            inform(80u32);
            inform(97u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for SvgZoomAndPan {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgZoomAndPan {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgZoomAndPan {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgZoomAndPan {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgZoomAndPan {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgZoomAndPan {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgZoomAndPan {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgZoomAndPan {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgZoomAndPan {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgZoomAndPan>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgZoomAndPan {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgZoomAndPan {
        #[inline]
        fn from(obj: JsValue) -> SvgZoomAndPan {
            SvgZoomAndPan { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgZoomAndPan {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgZoomAndPan> for SvgZoomAndPan {
        #[inline]
        fn as_ref(&self) -> &SvgZoomAndPan {
            self
        }
    }
    impl From<SvgZoomAndPan> for JsValue {
        #[inline]
        fn from(obj: SvgZoomAndPan) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgZoomAndPan {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGZoomAndPan(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGZoomAndPan(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGZoomAndPan(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgZoomAndPan { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgZoomAndPan) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgZoomAndPan> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgZoomAndPan) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgZoomAndPan {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgZoomAndPan",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_zoom_and_pan_SVGZoomAndPan() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgZoomAndPan as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl SvgZoomAndPan {
    #[cfg(all(feature = "SvgZoomAndPan",))]
    #[allow(bad_style)]
    #[doc = "The `zoomAndPan` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGZoomAndPan/zoomAndPan)\n\n*This API requires the following crate features to be activated: `SvgZoomAndPan`*"]
    #[allow(clippy::all)]
    pub fn zoom_and_pan(&self) -> u16 {
        #[cfg(all(feature = "SvgZoomAndPan",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_zoom_and_pan_SVGZoomAndPan(
                self_: <&SvgZoomAndPan as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_zoom_and_pan_SVGZoomAndPan(
            self_: <&SvgZoomAndPan as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgZoomAndPan as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_zoom_and_pan_SVGZoomAndPan(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgZoomAndPan",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_zoom_and_pan_SVGZoomAndPan() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgZoomAndPan as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgZoomAndPan {
    #[cfg(all(feature = "SvgZoomAndPan",))]
    #[allow(bad_style)]
    #[doc = "The `zoomAndPan` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGZoomAndPan/zoomAndPan)\n\n*This API requires the following crate features to be activated: `SvgZoomAndPan`*"]
    #[allow(clippy::all)]
    pub fn set_zoom_and_pan(&self, zoom_and_pan: u16) {
        #[cfg(all(feature = "SvgZoomAndPan",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_zoom_and_pan_SVGZoomAndPan(
                self_: <&SvgZoomAndPan as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                zoom_and_pan: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_zoom_and_pan_SVGZoomAndPan(
            self_: <&SvgZoomAndPan as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            zoom_and_pan: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(zoom_and_pan);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgZoomAndPan as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let zoom_and_pan =
                    <u16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(zoom_and_pan);
                __widl_f_set_zoom_and_pan_SVGZoomAndPan(self_, zoom_and_pan)
            };
            ()
        }
    }
}
impl SvgZoomAndPan {
    pub const SVG_ZOOMANDPAN_UNKNOWN: u16 = 0i64 as u16;
}
impl SvgZoomAndPan {
    pub const SVG_ZOOMANDPAN_DISABLE: u16 = 1u64 as u16;
}
impl SvgZoomAndPan {
    pub const SVG_ZOOMANDPAN_MAGNIFY: u16 = 2u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_a4d70521c86b836b: [u8; 352usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x1E\x01\0\0\0\0\x03\0\0\x02\rSVGZoomAndPan\x1F__widl_instanceof_SVGZoomAndPan\0\0\0\0#__widl_f_zoom_and_pan_SVGZoomAndPan\0\0\0\x01\rSVGZoomAndPan\x01\0\x01\nzoomAndPan\x01\x01\x05self_\nzoomAndPan\0\0\0'__widl_f_set_zoom_and_pan_SVGZoomAndPan\0\0\0\x01\rSVGZoomAndPan\x01\0\x02\nzoomAndPan\x01\x02\x05self_\x0Czoom_and_pan\nzoomAndPan\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
