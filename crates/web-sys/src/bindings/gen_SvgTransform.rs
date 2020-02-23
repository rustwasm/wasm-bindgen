use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGTransform` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform)\n\n*This API requires the following crate features to be activated: `SvgTransform`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgTransform {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgTransform: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgTransform {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(84u32);
            inform(114u32);
            inform(97u32);
            inform(110u32);
            inform(115u32);
            inform(102u32);
            inform(111u32);
            inform(114u32);
            inform(109u32);
        }
    }
    impl core::ops::Deref for SvgTransform {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgTransform {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgTransform {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgTransform {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgTransform {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgTransform {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgTransform {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgTransform {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgTransform {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgTransform>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgTransform {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgTransform {
        #[inline]
        fn from(obj: JsValue) -> SvgTransform {
            SvgTransform { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgTransform {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgTransform> for SvgTransform {
        #[inline]
        fn as_ref(&self) -> &SvgTransform {
            self
        }
    }
    impl From<SvgTransform> for JsValue {
        #[inline]
        fn from(obj: SvgTransform) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgTransform {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGTransform(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGTransform(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGTransform(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgTransform { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgTransform) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgTransform> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgTransform) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgTransform {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgMatrix", feature = "SvgTransform",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_matrix_SVGTransform() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgTransform as WasmDescribe>::describe();
    <&SvgMatrix as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgTransform {
    #[cfg(all(feature = "SvgMatrix", feature = "SvgTransform",))]
    #[allow(bad_style)]
    #[doc = "The `setMatrix()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setMatrix)\n\n*This API requires the following crate features to be activated: `SvgMatrix`, `SvgTransform`*"]
    #[allow(clippy::all)]
    pub fn set_matrix(&self, matrix: &SvgMatrix) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgMatrix", feature = "SvgTransform",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_matrix_SVGTransform(
                self_: <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                matrix: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_matrix_SVGTransform(
            self_: <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            matrix: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(matrix);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let matrix = <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(matrix);
                __widl_f_set_matrix_SVGTransform(self_, matrix)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SvgTransform",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_rotate_SVGTransform() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&SvgTransform as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgTransform {
    #[cfg(all(feature = "SvgTransform",))]
    #[allow(bad_style)]
    #[doc = "The `setRotate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setRotate)\n\n*This API requires the following crate features to be activated: `SvgTransform`*"]
    #[allow(clippy::all)]
    pub fn set_rotate(&self, angle: f32, cx: f32, cy: f32) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgTransform",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_rotate_SVGTransform(
                self_: <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                angle: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cx: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cy: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_rotate_SVGTransform(
            self_: <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            angle: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cx: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cy: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(angle);
            drop(cx);
            drop(cy);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let angle = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(angle);
                let cx = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cx);
                let cy = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cy);
                __widl_f_set_rotate_SVGTransform(self_, angle, cx, cy)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SvgTransform",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_scale_SVGTransform() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&SvgTransform as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgTransform {
    #[cfg(all(feature = "SvgTransform",))]
    #[allow(bad_style)]
    #[doc = "The `setScale()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setScale)\n\n*This API requires the following crate features to be activated: `SvgTransform`*"]
    #[allow(clippy::all)]
    pub fn set_scale(&self, sx: f32, sy: f32) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgTransform",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_scale_SVGTransform(
                self_: <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sx: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sy: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_scale_SVGTransform(
            self_: <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sx: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sy: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(sx);
            drop(sy);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let sx = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sx);
                let sy = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sy);
                __widl_f_set_scale_SVGTransform(self_, sx, sy)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SvgTransform",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_skew_x_SVGTransform() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgTransform as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgTransform {
    #[cfg(all(feature = "SvgTransform",))]
    #[allow(bad_style)]
    #[doc = "The `setSkewX()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setSkewX)\n\n*This API requires the following crate features to be activated: `SvgTransform`*"]
    #[allow(clippy::all)]
    pub fn set_skew_x(&self, angle: f32) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgTransform",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_skew_x_SVGTransform(
                self_: <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                angle: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_skew_x_SVGTransform(
            self_: <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let angle = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(angle);
                __widl_f_set_skew_x_SVGTransform(self_, angle)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SvgTransform",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_skew_y_SVGTransform() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgTransform as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgTransform {
    #[cfg(all(feature = "SvgTransform",))]
    #[allow(bad_style)]
    #[doc = "The `setSkewY()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setSkewY)\n\n*This API requires the following crate features to be activated: `SvgTransform`*"]
    #[allow(clippy::all)]
    pub fn set_skew_y(&self, angle: f32) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgTransform",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_skew_y_SVGTransform(
                self_: <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                angle: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_skew_y_SVGTransform(
            self_: <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let angle = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(angle);
                __widl_f_set_skew_y_SVGTransform(self_, angle)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SvgTransform",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_translate_SVGTransform() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&SvgTransform as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgTransform {
    #[cfg(all(feature = "SvgTransform",))]
    #[allow(bad_style)]
    #[doc = "The `setTranslate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setTranslate)\n\n*This API requires the following crate features to be activated: `SvgTransform`*"]
    #[allow(clippy::all)]
    pub fn set_translate(&self, tx: f32, ty: f32) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgTransform",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_translate_SVGTransform(
                self_: <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tx: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ty: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_translate_SVGTransform(
            self_: <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tx: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ty: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(tx);
            drop(ty);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let tx = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tx);
                let ty = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ty);
                __widl_f_set_translate_SVGTransform(self_, tx, ty)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SvgTransform",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_SVGTransform() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgTransform as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl SvgTransform {
    #[cfg(all(feature = "SvgTransform",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/type)\n\n*This API requires the following crate features to be activated: `SvgTransform`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> u16 {
        #[cfg(all(feature = "SvgTransform",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_SVGTransform(
                self_: <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_SVGTransform(
            self_: <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_SVGTransform(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgMatrix", feature = "SvgTransform",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_matrix_SVGTransform() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgTransform as WasmDescribe>::describe();
    <SvgMatrix as WasmDescribe>::describe();
}
impl SvgTransform {
    #[cfg(all(feature = "SvgMatrix", feature = "SvgTransform",))]
    #[allow(bad_style)]
    #[doc = "The `matrix` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/matrix)\n\n*This API requires the following crate features to be activated: `SvgMatrix`, `SvgTransform`*"]
    #[allow(clippy::all)]
    pub fn matrix(&self) -> SvgMatrix {
        #[cfg(all(feature = "SvgMatrix", feature = "SvgTransform",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_matrix_SVGTransform(
                self_: <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_matrix_SVGTransform(
            self_: <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_matrix_SVGTransform(self_)
            };
            <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgTransform",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_angle_SVGTransform() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgTransform as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SvgTransform {
    #[cfg(all(feature = "SvgTransform",))]
    #[allow(bad_style)]
    #[doc = "The `angle` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/angle)\n\n*This API requires the following crate features to be activated: `SvgTransform`*"]
    #[allow(clippy::all)]
    pub fn angle(&self) -> f32 {
        #[cfg(all(feature = "SvgTransform",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_angle_SVGTransform(
                self_: <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_angle_SVGTransform(
            self_: <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgTransform as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_angle_SVGTransform(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
impl SvgTransform {
    pub const SVG_TRANSFORM_UNKNOWN: u16 = 0i64 as u16;
}
impl SvgTransform {
    pub const SVG_TRANSFORM_MATRIX: u16 = 1u64 as u16;
}
impl SvgTransform {
    pub const SVG_TRANSFORM_TRANSLATE: u16 = 2u64 as u16;
}
impl SvgTransform {
    pub const SVG_TRANSFORM_SCALE: u16 = 3u64 as u16;
}
impl SvgTransform {
    pub const SVG_TRANSFORM_ROTATE: u16 = 4u64 as u16;
}
impl SvgTransform {
    pub const SVG_TRANSFORM_SKEWX: u16 = 5u64 as u16;
}
impl SvgTransform {
    pub const SVG_TRANSFORM_SKEWY: u16 = 6u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_5533e4c677e3ebca: [u8; 855usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x15\x03\0\0\0\0\n\0\0\x02\x0CSVGTransform\x1E__widl_instanceof_SVGTransform\0\0\0\0 __widl_f_set_matrix_SVGTransform\x01\0\0\x01\x0CSVGTransform\x01\0\0\x01\x02\x05self_\x06matrix\tsetMatrix\0\0\0 __widl_f_set_rotate_SVGTransform\x01\0\0\x01\x0CSVGTransform\x01\0\0\x01\x04\x05self_\x05angle\x02cx\x02cy\tsetRotate\0\0\0\x1F__widl_f_set_scale_SVGTransform\x01\0\0\x01\x0CSVGTransform\x01\0\0\x01\x03\x05self_\x02sx\x02sy\x08setScale\0\0\0 __widl_f_set_skew_x_SVGTransform\x01\0\0\x01\x0CSVGTransform\x01\0\0\x01\x02\x05self_\x05angle\x08setSkewX\0\0\0 __widl_f_set_skew_y_SVGTransform\x01\0\0\x01\x0CSVGTransform\x01\0\0\x01\x02\x05self_\x05angle\x08setSkewY\0\0\0#__widl_f_set_translate_SVGTransform\x01\0\0\x01\x0CSVGTransform\x01\0\0\x01\x03\x05self_\x02tx\x02ty\x0CsetTranslate\0\0\0\x1A__widl_f_type_SVGTransform\0\0\0\x01\x0CSVGTransform\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0\x1C__widl_f_matrix_SVGTransform\0\0\0\x01\x0CSVGTransform\x01\0\x01\x06matrix\x01\x01\x05self_\x06matrix\0\0\0\x1B__widl_f_angle_SVGTransform\0\0\0\x01\x0CSVGTransform\x01\0\x01\x05angle\x01\x01\x05self_\x05angle\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
