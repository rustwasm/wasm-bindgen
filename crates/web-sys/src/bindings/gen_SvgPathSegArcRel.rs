use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGPathSegArcRel` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel)\n\n*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgPathSegArcRel {
    obj: SvgPathSeg,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgPathSegArcRel: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgPathSegArcRel {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
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
            inform(65u32);
            inform(114u32);
            inform(99u32);
            inform(82u32);
            inform(101u32);
            inform(108u32);
        }
    }
    impl core::ops::Deref for SvgPathSegArcRel {
        type Target = SvgPathSeg;
        #[inline]
        fn deref(&self) -> &SvgPathSeg {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgPathSegArcRel {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgPathSegArcRel {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgPathSegArcRel {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgPathSegArcRel {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgPathSegArcRel {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgPathSegArcRel {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgPathSegArcRel {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgPathSegArcRel {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgPathSegArcRel>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgPathSegArcRel {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgPathSegArcRel {
        #[inline]
        fn from(obj: JsValue) -> SvgPathSegArcRel {
            SvgPathSegArcRel { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgPathSegArcRel {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgPathSegArcRel> for SvgPathSegArcRel {
        #[inline]
        fn as_ref(&self) -> &SvgPathSegArcRel {
            self
        }
    }
    impl From<SvgPathSegArcRel> for JsValue {
        #[inline]
        fn from(obj: SvgPathSegArcRel) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgPathSegArcRel {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGPathSegArcRel(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGPathSegArcRel(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGPathSegArcRel(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgPathSegArcRel { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgPathSegArcRel) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgPathSegArcRel> for SvgPathSeg {
    #[inline]
    fn from(obj: SvgPathSegArcRel) -> SvgPathSeg {
        use wasm_bindgen::JsCast;
        SvgPathSeg::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgPathSeg> for SvgPathSegArcRel {
    #[inline]
    fn as_ref(&self) -> &SvgPathSeg {
        use wasm_bindgen::JsCast;
        SvgPathSeg::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgPathSegArcRel> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgPathSegArcRel) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgPathSegArcRel {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgPathSegArcRel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_x_SVGPathSegArcRel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgPathSegArcRel as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SvgPathSegArcRel {
    #[cfg(all(feature = "SvgPathSegArcRel",))]
    #[allow(bad_style)]
    #[doc = "The `x` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/x)\n\n*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    #[allow(clippy::all)]
    pub fn x(&self) -> f32 {
        #[cfg(all(feature = "SvgPathSegArcRel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_x_SVGPathSegArcRel(
                self_: <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_x_SVGPathSegArcRel(
            self_: <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_x_SVGPathSegArcRel(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgPathSegArcRel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_x_SVGPathSegArcRel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgPathSegArcRel as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgPathSegArcRel {
    #[cfg(all(feature = "SvgPathSegArcRel",))]
    #[allow(bad_style)]
    #[doc = "The `x` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/x)\n\n*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    #[allow(clippy::all)]
    pub fn set_x(&self, x: f32) {
        #[cfg(all(feature = "SvgPathSegArcRel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_x_SVGPathSegArcRel(
                self_: <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_x_SVGPathSegArcRel(
            self_: <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ =
                    <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                __widl_f_set_x_SVGPathSegArcRel(self_, x)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgPathSegArcRel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_y_SVGPathSegArcRel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgPathSegArcRel as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SvgPathSegArcRel {
    #[cfg(all(feature = "SvgPathSegArcRel",))]
    #[allow(bad_style)]
    #[doc = "The `y` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/y)\n\n*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    #[allow(clippy::all)]
    pub fn y(&self) -> f32 {
        #[cfg(all(feature = "SvgPathSegArcRel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_y_SVGPathSegArcRel(
                self_: <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_y_SVGPathSegArcRel(
            self_: <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_y_SVGPathSegArcRel(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgPathSegArcRel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_y_SVGPathSegArcRel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgPathSegArcRel as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgPathSegArcRel {
    #[cfg(all(feature = "SvgPathSegArcRel",))]
    #[allow(bad_style)]
    #[doc = "The `y` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/y)\n\n*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    #[allow(clippy::all)]
    pub fn set_y(&self, y: f32) {
        #[cfg(all(feature = "SvgPathSegArcRel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_y_SVGPathSegArcRel(
                self_: <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_y_SVGPathSegArcRel(
            self_: <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let y = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_set_y_SVGPathSegArcRel(self_, y)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgPathSegArcRel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_r1_SVGPathSegArcRel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgPathSegArcRel as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SvgPathSegArcRel {
    #[cfg(all(feature = "SvgPathSegArcRel",))]
    #[allow(bad_style)]
    #[doc = "The `r1` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/r1)\n\n*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    #[allow(clippy::all)]
    pub fn r1(&self) -> f32 {
        #[cfg(all(feature = "SvgPathSegArcRel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_r1_SVGPathSegArcRel(
                self_: <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_r1_SVGPathSegArcRel(
            self_: <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_r1_SVGPathSegArcRel(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgPathSegArcRel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_r1_SVGPathSegArcRel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgPathSegArcRel as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgPathSegArcRel {
    #[cfg(all(feature = "SvgPathSegArcRel",))]
    #[allow(bad_style)]
    #[doc = "The `r1` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/r1)\n\n*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    #[allow(clippy::all)]
    pub fn set_r1(&self, r1: f32) {
        #[cfg(all(feature = "SvgPathSegArcRel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_r1_SVGPathSegArcRel(
                self_: <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                r1: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_r1_SVGPathSegArcRel(
            self_: <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            r1: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(r1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let r1 = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(r1);
                __widl_f_set_r1_SVGPathSegArcRel(self_, r1)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgPathSegArcRel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_r2_SVGPathSegArcRel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgPathSegArcRel as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SvgPathSegArcRel {
    #[cfg(all(feature = "SvgPathSegArcRel",))]
    #[allow(bad_style)]
    #[doc = "The `r2` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/r2)\n\n*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    #[allow(clippy::all)]
    pub fn r2(&self) -> f32 {
        #[cfg(all(feature = "SvgPathSegArcRel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_r2_SVGPathSegArcRel(
                self_: <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_r2_SVGPathSegArcRel(
            self_: <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_r2_SVGPathSegArcRel(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgPathSegArcRel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_r2_SVGPathSegArcRel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgPathSegArcRel as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgPathSegArcRel {
    #[cfg(all(feature = "SvgPathSegArcRel",))]
    #[allow(bad_style)]
    #[doc = "The `r2` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/r2)\n\n*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    #[allow(clippy::all)]
    pub fn set_r2(&self, r2: f32) {
        #[cfg(all(feature = "SvgPathSegArcRel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_r2_SVGPathSegArcRel(
                self_: <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                r2: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_r2_SVGPathSegArcRel(
            self_: <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            r2: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(r2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let r2 = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(r2);
                __widl_f_set_r2_SVGPathSegArcRel(self_, r2)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgPathSegArcRel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_angle_SVGPathSegArcRel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgPathSegArcRel as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SvgPathSegArcRel {
    #[cfg(all(feature = "SvgPathSegArcRel",))]
    #[allow(bad_style)]
    #[doc = "The `angle` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/angle)\n\n*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    #[allow(clippy::all)]
    pub fn angle(&self) -> f32 {
        #[cfg(all(feature = "SvgPathSegArcRel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_angle_SVGPathSegArcRel(
                self_: <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_angle_SVGPathSegArcRel(
            self_: <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_angle_SVGPathSegArcRel(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgPathSegArcRel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_angle_SVGPathSegArcRel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgPathSegArcRel as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgPathSegArcRel {
    #[cfg(all(feature = "SvgPathSegArcRel",))]
    #[allow(bad_style)]
    #[doc = "The `angle` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/angle)\n\n*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    #[allow(clippy::all)]
    pub fn set_angle(&self, angle: f32) {
        #[cfg(all(feature = "SvgPathSegArcRel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_angle_SVGPathSegArcRel(
                self_: <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                angle: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_angle_SVGPathSegArcRel(
            self_: <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            angle: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(angle);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let angle = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(angle);
                __widl_f_set_angle_SVGPathSegArcRel(self_, angle)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgPathSegArcRel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_large_arc_flag_SVGPathSegArcRel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgPathSegArcRel as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl SvgPathSegArcRel {
    #[cfg(all(feature = "SvgPathSegArcRel",))]
    #[allow(bad_style)]
    #[doc = "The `largeArcFlag` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/largeArcFlag)\n\n*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    #[allow(clippy::all)]
    pub fn large_arc_flag(&self) -> bool {
        #[cfg(all(feature = "SvgPathSegArcRel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_large_arc_flag_SVGPathSegArcRel(
                self_: <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_large_arc_flag_SVGPathSegArcRel(
            self_: <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_large_arc_flag_SVGPathSegArcRel(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgPathSegArcRel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_large_arc_flag_SVGPathSegArcRel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgPathSegArcRel as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgPathSegArcRel {
    #[cfg(all(feature = "SvgPathSegArcRel",))]
    #[allow(bad_style)]
    #[doc = "The `largeArcFlag` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/largeArcFlag)\n\n*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    #[allow(clippy::all)]
    pub fn set_large_arc_flag(&self, large_arc_flag: bool) {
        #[cfg(all(feature = "SvgPathSegArcRel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_large_arc_flag_SVGPathSegArcRel(
                self_: <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                large_arc_flag: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_large_arc_flag_SVGPathSegArcRel(
            self_: <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            large_arc_flag: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(large_arc_flag);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let large_arc_flag =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(large_arc_flag);
                __widl_f_set_large_arc_flag_SVGPathSegArcRel(self_, large_arc_flag)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgPathSegArcRel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_sweep_flag_SVGPathSegArcRel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgPathSegArcRel as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl SvgPathSegArcRel {
    #[cfg(all(feature = "SvgPathSegArcRel",))]
    #[allow(bad_style)]
    #[doc = "The `sweepFlag` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/sweepFlag)\n\n*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    #[allow(clippy::all)]
    pub fn sweep_flag(&self) -> bool {
        #[cfg(all(feature = "SvgPathSegArcRel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_sweep_flag_SVGPathSegArcRel(
                self_: <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_sweep_flag_SVGPathSegArcRel(
            self_: <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_sweep_flag_SVGPathSegArcRel(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgPathSegArcRel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_sweep_flag_SVGPathSegArcRel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgPathSegArcRel as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgPathSegArcRel {
    #[cfg(all(feature = "SvgPathSegArcRel",))]
    #[allow(bad_style)]
    #[doc = "The `sweepFlag` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/sweepFlag)\n\n*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    #[allow(clippy::all)]
    pub fn set_sweep_flag(&self, sweep_flag: bool) {
        #[cfg(all(feature = "SvgPathSegArcRel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_sweep_flag_SVGPathSegArcRel(
                self_: <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sweep_flag: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_sweep_flag_SVGPathSegArcRel(
            self_: <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sweep_flag: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(sweep_flag);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgPathSegArcRel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let sweep_flag = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sweep_flag);
                __widl_f_set_sweep_flag_SVGPathSegArcRel(self_, sweep_flag)
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
pub static __WASM_BINDGEN_GENERATED_e3a694eaf360faa7: [u8; 1325usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xEB\x04\0\0\0\0\x0F\0\0\x02\x10SVGPathSegArcRel\"__widl_instanceof_SVGPathSegArcRel\0\0\0\0\x1B__widl_f_x_SVGPathSegArcRel\0\0\0\x01\x10SVGPathSegArcRel\x01\0\x01\x01x\x01\x01\x05self_\x01x\0\0\0\x1F__widl_f_set_x_SVGPathSegArcRel\0\0\0\x01\x10SVGPathSegArcRel\x01\0\x02\x01x\x01\x02\x05self_\x01x\x01x\0\0\0\x1B__widl_f_y_SVGPathSegArcRel\0\0\0\x01\x10SVGPathSegArcRel\x01\0\x01\x01y\x01\x01\x05self_\x01y\0\0\0\x1F__widl_f_set_y_SVGPathSegArcRel\0\0\0\x01\x10SVGPathSegArcRel\x01\0\x02\x01y\x01\x02\x05self_\x01y\x01y\0\0\0\x1C__widl_f_r1_SVGPathSegArcRel\0\0\0\x01\x10SVGPathSegArcRel\x01\0\x01\x02r1\x01\x01\x05self_\x02r1\0\0\0 __widl_f_set_r1_SVGPathSegArcRel\0\0\0\x01\x10SVGPathSegArcRel\x01\0\x02\x02r1\x01\x02\x05self_\x02r1\x02r1\0\0\0\x1C__widl_f_r2_SVGPathSegArcRel\0\0\0\x01\x10SVGPathSegArcRel\x01\0\x01\x02r2\x01\x01\x05self_\x02r2\0\0\0 __widl_f_set_r2_SVGPathSegArcRel\0\0\0\x01\x10SVGPathSegArcRel\x01\0\x02\x02r2\x01\x02\x05self_\x02r2\x02r2\0\0\0\x1F__widl_f_angle_SVGPathSegArcRel\0\0\0\x01\x10SVGPathSegArcRel\x01\0\x01\x05angle\x01\x01\x05self_\x05angle\0\0\0#__widl_f_set_angle_SVGPathSegArcRel\0\0\0\x01\x10SVGPathSegArcRel\x01\0\x02\x05angle\x01\x02\x05self_\x05angle\x05angle\0\0\0(__widl_f_large_arc_flag_SVGPathSegArcRel\0\0\0\x01\x10SVGPathSegArcRel\x01\0\x01\x0ClargeArcFlag\x01\x01\x05self_\x0ClargeArcFlag\0\0\0,__widl_f_set_large_arc_flag_SVGPathSegArcRel\0\0\0\x01\x10SVGPathSegArcRel\x01\0\x02\x0ClargeArcFlag\x01\x02\x05self_\x0Elarge_arc_flag\x0ClargeArcFlag\0\0\0$__widl_f_sweep_flag_SVGPathSegArcRel\0\0\0\x01\x10SVGPathSegArcRel\x01\0\x01\tsweepFlag\x01\x01\x05self_\tsweepFlag\0\0\0(__widl_f_set_sweep_flag_SVGPathSegArcRel\0\0\0\x01\x10SVGPathSegArcRel\x01\0\x02\tsweepFlag\x01\x02\x05self_\nsweep_flag\tsweepFlag\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
