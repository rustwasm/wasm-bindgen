use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGPathSegCurvetoCubicRel` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgPathSegCurvetoCubicRel {
    obj: SvgPathSeg,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgPathSegCurvetoCubicRel: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgPathSegCurvetoCubicRel {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(25u32);
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
            inform(67u32);
            inform(117u32);
            inform(114u32);
            inform(118u32);
            inform(101u32);
            inform(116u32);
            inform(111u32);
            inform(67u32);
            inform(117u32);
            inform(98u32);
            inform(105u32);
            inform(99u32);
            inform(82u32);
            inform(101u32);
            inform(108u32);
        }
    }
    impl core::ops::Deref for SvgPathSegCurvetoCubicRel {
        type Target = SvgPathSeg;
        #[inline]
        fn deref(&self) -> &SvgPathSeg {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgPathSegCurvetoCubicRel {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgPathSegCurvetoCubicRel {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgPathSegCurvetoCubicRel {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgPathSegCurvetoCubicRel {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgPathSegCurvetoCubicRel {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgPathSegCurvetoCubicRel {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgPathSegCurvetoCubicRel {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgPathSegCurvetoCubicRel {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgPathSegCurvetoCubicRel>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgPathSegCurvetoCubicRel {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgPathSegCurvetoCubicRel {
        #[inline]
        fn from(obj: JsValue) -> SvgPathSegCurvetoCubicRel {
            SvgPathSegCurvetoCubicRel { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgPathSegCurvetoCubicRel {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgPathSegCurvetoCubicRel> for SvgPathSegCurvetoCubicRel {
        #[inline]
        fn as_ref(&self) -> &SvgPathSegCurvetoCubicRel {
            self
        }
    }
    impl From<SvgPathSegCurvetoCubicRel> for JsValue {
        #[inline]
        fn from(obj: SvgPathSegCurvetoCubicRel) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgPathSegCurvetoCubicRel {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGPathSegCurvetoCubicRel(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGPathSegCurvetoCubicRel(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGPathSegCurvetoCubicRel(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgPathSegCurvetoCubicRel { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgPathSegCurvetoCubicRel) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgPathSegCurvetoCubicRel> for SvgPathSeg {
    #[inline]
    fn from(obj: SvgPathSegCurvetoCubicRel) -> SvgPathSeg {
        use wasm_bindgen::JsCast;
        SvgPathSeg::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgPathSeg> for SvgPathSegCurvetoCubicRel {
    #[inline]
    fn as_ref(&self) -> &SvgPathSeg {
        use wasm_bindgen::JsCast;
        SvgPathSeg::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgPathSegCurvetoCubicRel> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgPathSegCurvetoCubicRel) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgPathSegCurvetoCubicRel {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_x_SVGPathSegCurvetoCubicRel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgPathSegCurvetoCubicRel as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SvgPathSegCurvetoCubicRel {
    #[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
    #[allow(bad_style)]
    #[doc = "The `x` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/x)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*"]
    #[allow(clippy::all)]
    pub fn x(&self) -> f32 {
        #[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_x_SVGPathSegCurvetoCubicRel(
                self_: <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_x_SVGPathSegCurvetoCubicRel(
            self_: <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_x_SVGPathSegCurvetoCubicRel(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_x_SVGPathSegCurvetoCubicRel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgPathSegCurvetoCubicRel as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgPathSegCurvetoCubicRel {
    #[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
    #[allow(bad_style)]
    #[doc = "The `x` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/x)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*"]
    #[allow(clippy::all)]
    pub fn set_x(&self, x: f32) {
        #[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_x_SVGPathSegCurvetoCubicRel(
                self_: <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_x_SVGPathSegCurvetoCubicRel(
            self_: <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                __widl_f_set_x_SVGPathSegCurvetoCubicRel(self_, x)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_y_SVGPathSegCurvetoCubicRel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgPathSegCurvetoCubicRel as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SvgPathSegCurvetoCubicRel {
    #[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
    #[allow(bad_style)]
    #[doc = "The `y` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/y)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*"]
    #[allow(clippy::all)]
    pub fn y(&self) -> f32 {
        #[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_y_SVGPathSegCurvetoCubicRel(
                self_: <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_y_SVGPathSegCurvetoCubicRel(
            self_: <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_y_SVGPathSegCurvetoCubicRel(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_y_SVGPathSegCurvetoCubicRel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgPathSegCurvetoCubicRel as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgPathSegCurvetoCubicRel {
    #[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
    #[allow(bad_style)]
    #[doc = "The `y` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/y)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*"]
    #[allow(clippy::all)]
    pub fn set_y(&self, y: f32) {
        #[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_y_SVGPathSegCurvetoCubicRel(
                self_: <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_y_SVGPathSegCurvetoCubicRel(
            self_: <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let y = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_set_y_SVGPathSegCurvetoCubicRel(self_, y)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_x1_SVGPathSegCurvetoCubicRel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgPathSegCurvetoCubicRel as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SvgPathSegCurvetoCubicRel {
    #[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
    #[allow(bad_style)]
    #[doc = "The `x1` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/x1)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*"]
    #[allow(clippy::all)]
    pub fn x1(&self) -> f32 {
        #[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_x1_SVGPathSegCurvetoCubicRel(
                self_: <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_x1_SVGPathSegCurvetoCubicRel(
            self_: <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_x1_SVGPathSegCurvetoCubicRel(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_x1_SVGPathSegCurvetoCubicRel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgPathSegCurvetoCubicRel as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgPathSegCurvetoCubicRel {
    #[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
    #[allow(bad_style)]
    #[doc = "The `x1` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/x1)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*"]
    #[allow(clippy::all)]
    pub fn set_x1(&self, x1: f32) {
        #[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_x1_SVGPathSegCurvetoCubicRel(
                self_: <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x1: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_x1_SVGPathSegCurvetoCubicRel(
            self_: <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x1: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x1 = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x1);
                __widl_f_set_x1_SVGPathSegCurvetoCubicRel(self_, x1)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_y1_SVGPathSegCurvetoCubicRel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgPathSegCurvetoCubicRel as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SvgPathSegCurvetoCubicRel {
    #[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
    #[allow(bad_style)]
    #[doc = "The `y1` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/y1)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*"]
    #[allow(clippy::all)]
    pub fn y1(&self) -> f32 {
        #[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_y1_SVGPathSegCurvetoCubicRel(
                self_: <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_y1_SVGPathSegCurvetoCubicRel(
            self_: <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_y1_SVGPathSegCurvetoCubicRel(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_y1_SVGPathSegCurvetoCubicRel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgPathSegCurvetoCubicRel as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgPathSegCurvetoCubicRel {
    #[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
    #[allow(bad_style)]
    #[doc = "The `y1` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/y1)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*"]
    #[allow(clippy::all)]
    pub fn set_y1(&self, y1: f32) {
        #[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_y1_SVGPathSegCurvetoCubicRel(
                self_: <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y1: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_y1_SVGPathSegCurvetoCubicRel(
            self_: <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y1: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(y1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let y1 = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y1);
                __widl_f_set_y1_SVGPathSegCurvetoCubicRel(self_, y1)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_x2_SVGPathSegCurvetoCubicRel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgPathSegCurvetoCubicRel as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SvgPathSegCurvetoCubicRel {
    #[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
    #[allow(bad_style)]
    #[doc = "The `x2` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/x2)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*"]
    #[allow(clippy::all)]
    pub fn x2(&self) -> f32 {
        #[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_x2_SVGPathSegCurvetoCubicRel(
                self_: <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_x2_SVGPathSegCurvetoCubicRel(
            self_: <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_x2_SVGPathSegCurvetoCubicRel(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_x2_SVGPathSegCurvetoCubicRel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgPathSegCurvetoCubicRel as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgPathSegCurvetoCubicRel {
    #[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
    #[allow(bad_style)]
    #[doc = "The `x2` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/x2)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*"]
    #[allow(clippy::all)]
    pub fn set_x2(&self, x2: f32) {
        #[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_x2_SVGPathSegCurvetoCubicRel(
                self_: <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x2: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_x2_SVGPathSegCurvetoCubicRel(
            self_: <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x2: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x2 = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x2);
                __widl_f_set_x2_SVGPathSegCurvetoCubicRel(self_, x2)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_y2_SVGPathSegCurvetoCubicRel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgPathSegCurvetoCubicRel as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SvgPathSegCurvetoCubicRel {
    #[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
    #[allow(bad_style)]
    #[doc = "The `y2` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/y2)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*"]
    #[allow(clippy::all)]
    pub fn y2(&self) -> f32 {
        #[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_y2_SVGPathSegCurvetoCubicRel(
                self_: <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_y2_SVGPathSegCurvetoCubicRel(
            self_: <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_y2_SVGPathSegCurvetoCubicRel(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_y2_SVGPathSegCurvetoCubicRel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgPathSegCurvetoCubicRel as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgPathSegCurvetoCubicRel {
    #[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
    #[allow(bad_style)]
    #[doc = "The `y2` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/y2)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*"]
    #[allow(clippy::all)]
    pub fn set_y2(&self, y2: f32) {
        #[cfg(all(feature = "SvgPathSegCurvetoCubicRel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_y2_SVGPathSegCurvetoCubicRel(
                self_: <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y2: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_y2_SVGPathSegCurvetoCubicRel(
            self_: <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y2: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(y2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgPathSegCurvetoCubicRel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let y2 = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y2);
                __widl_f_set_y2_SVGPathSegCurvetoCubicRel(self_, y2)
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
pub static __WASM_BINDGEN_GENERATED_40c3ca12f4b9f106: [u8; 1263usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xAD\x04\0\0\0\0\r\0\0\x02\x19SVGPathSegCurvetoCubicRel+__widl_instanceof_SVGPathSegCurvetoCubicRel\0\0\0\0$__widl_f_x_SVGPathSegCurvetoCubicRel\0\0\0\x01\x19SVGPathSegCurvetoCubicRel\x01\0\x01\x01x\x01\x01\x05self_\x01x\0\0\0(__widl_f_set_x_SVGPathSegCurvetoCubicRel\0\0\0\x01\x19SVGPathSegCurvetoCubicRel\x01\0\x02\x01x\x01\x02\x05self_\x01x\x01x\0\0\0$__widl_f_y_SVGPathSegCurvetoCubicRel\0\0\0\x01\x19SVGPathSegCurvetoCubicRel\x01\0\x01\x01y\x01\x01\x05self_\x01y\0\0\0(__widl_f_set_y_SVGPathSegCurvetoCubicRel\0\0\0\x01\x19SVGPathSegCurvetoCubicRel\x01\0\x02\x01y\x01\x02\x05self_\x01y\x01y\0\0\0%__widl_f_x1_SVGPathSegCurvetoCubicRel\0\0\0\x01\x19SVGPathSegCurvetoCubicRel\x01\0\x01\x02x1\x01\x01\x05self_\x02x1\0\0\0)__widl_f_set_x1_SVGPathSegCurvetoCubicRel\0\0\0\x01\x19SVGPathSegCurvetoCubicRel\x01\0\x02\x02x1\x01\x02\x05self_\x02x1\x02x1\0\0\0%__widl_f_y1_SVGPathSegCurvetoCubicRel\0\0\0\x01\x19SVGPathSegCurvetoCubicRel\x01\0\x01\x02y1\x01\x01\x05self_\x02y1\0\0\0)__widl_f_set_y1_SVGPathSegCurvetoCubicRel\0\0\0\x01\x19SVGPathSegCurvetoCubicRel\x01\0\x02\x02y1\x01\x02\x05self_\x02y1\x02y1\0\0\0%__widl_f_x2_SVGPathSegCurvetoCubicRel\0\0\0\x01\x19SVGPathSegCurvetoCubicRel\x01\0\x01\x02x2\x01\x01\x05self_\x02x2\0\0\0)__widl_f_set_x2_SVGPathSegCurvetoCubicRel\0\0\0\x01\x19SVGPathSegCurvetoCubicRel\x01\0\x02\x02x2\x01\x02\x05self_\x02x2\x02x2\0\0\0%__widl_f_y2_SVGPathSegCurvetoCubicRel\0\0\0\x01\x19SVGPathSegCurvetoCubicRel\x01\0\x01\x02y2\x01\x01\x05self_\x02y2\0\0\0)__widl_f_set_y2_SVGPathSegCurvetoCubicRel\0\0\0\x01\x19SVGPathSegCurvetoCubicRel\x01\0\x02\x02y2\x01\x02\x05self_\x02y2\x02y2\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
