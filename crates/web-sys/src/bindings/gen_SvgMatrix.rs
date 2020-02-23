use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGMatrix` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgMatrix {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgMatrix: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgMatrix {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(9u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(77u32);
            inform(97u32);
            inform(116u32);
            inform(114u32);
            inform(105u32);
            inform(120u32);
        }
    }
    impl core::ops::Deref for SvgMatrix {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgMatrix {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgMatrix {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgMatrix {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgMatrix {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgMatrix {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgMatrix {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgMatrix {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgMatrix {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgMatrix>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgMatrix {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgMatrix {
        #[inline]
        fn from(obj: JsValue) -> SvgMatrix {
            SvgMatrix { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgMatrix {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgMatrix> for SvgMatrix {
        #[inline]
        fn as_ref(&self) -> &SvgMatrix {
            self
        }
    }
    impl From<SvgMatrix> for JsValue {
        #[inline]
        fn from(obj: SvgMatrix) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgMatrix {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGMatrix(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGMatrix(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGMatrix(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgMatrix { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgMatrix) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgMatrix> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgMatrix) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgMatrix {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_flip_x_SVGMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgMatrix as WasmDescribe>::describe();
    <SvgMatrix as WasmDescribe>::describe();
}
impl SvgMatrix {
    #[cfg(all(feature = "SvgMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `flipX()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/flipX)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    #[allow(clippy::all)]
    pub fn flip_x(&self) -> SvgMatrix {
        #[cfg(all(feature = "SvgMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_flip_x_SVGMatrix(
                self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_flip_x_SVGMatrix(
            self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_flip_x_SVGMatrix(self_)
            };
            <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_flip_y_SVGMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgMatrix as WasmDescribe>::describe();
    <SvgMatrix as WasmDescribe>::describe();
}
impl SvgMatrix {
    #[cfg(all(feature = "SvgMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `flipY()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/flipY)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    #[allow(clippy::all)]
    pub fn flip_y(&self) -> SvgMatrix {
        #[cfg(all(feature = "SvgMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_flip_y_SVGMatrix(
                self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_flip_y_SVGMatrix(
            self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_flip_y_SVGMatrix(self_)
            };
            <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_inverse_SVGMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgMatrix as WasmDescribe>::describe();
    <SvgMatrix as WasmDescribe>::describe();
}
impl SvgMatrix {
    #[cfg(all(feature = "SvgMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `inverse()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/inverse)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    #[allow(clippy::all)]
    pub fn inverse(&self) -> Result<SvgMatrix, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_inverse_SVGMatrix(
                self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_inverse_SVGMatrix(
            self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_inverse_SVGMatrix(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "SvgMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_multiply_SVGMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgMatrix as WasmDescribe>::describe();
    <&SvgMatrix as WasmDescribe>::describe();
    <SvgMatrix as WasmDescribe>::describe();
}
impl SvgMatrix {
    #[cfg(all(feature = "SvgMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `multiply()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/multiply)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    #[allow(clippy::all)]
    pub fn multiply(&self, second_matrix: &SvgMatrix) -> SvgMatrix {
        #[cfg(all(feature = "SvgMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_multiply_SVGMatrix(
                self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                second_matrix: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_multiply_SVGMatrix(
            self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            second_matrix: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(second_matrix);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let second_matrix =
                    <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(second_matrix);
                __widl_f_multiply_SVGMatrix(self_, second_matrix)
            };
            <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rotate_SVGMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgMatrix as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <SvgMatrix as WasmDescribe>::describe();
}
impl SvgMatrix {
    #[cfg(all(feature = "SvgMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `rotate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/rotate)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    #[allow(clippy::all)]
    pub fn rotate(&self, angle: f32) -> SvgMatrix {
        #[cfg(all(feature = "SvgMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rotate_SVGMatrix(
                self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                angle: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rotate_SVGMatrix(
            self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            angle: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(angle);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let angle = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(angle);
                __widl_f_rotate_SVGMatrix(self_, angle)
            };
            <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rotate_from_vector_SVGMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&SvgMatrix as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <SvgMatrix as WasmDescribe>::describe();
}
impl SvgMatrix {
    #[cfg(all(feature = "SvgMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `rotateFromVector()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/rotateFromVector)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    #[allow(clippy::all)]
    pub fn rotate_from_vector(&self, x: f32, y: f32) -> Result<SvgMatrix, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rotate_from_vector_SVGMatrix(
                self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rotate_from_vector_SVGMatrix(
            self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(x);
            drop(y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_rotate_from_vector_SVGMatrix(self_, x, y)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "SvgMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale_SVGMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgMatrix as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <SvgMatrix as WasmDescribe>::describe();
}
impl SvgMatrix {
    #[cfg(all(feature = "SvgMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `scale()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/scale)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    #[allow(clippy::all)]
    pub fn scale(&self, scale_factor: f32) -> SvgMatrix {
        #[cfg(all(feature = "SvgMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale_SVGMatrix(
                self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_factor: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale_SVGMatrix(
            self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_factor: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(scale_factor);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scale_factor =
                    <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_factor);
                __widl_f_scale_SVGMatrix(self_, scale_factor)
            };
            <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale_non_uniform_SVGMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&SvgMatrix as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <SvgMatrix as WasmDescribe>::describe();
}
impl SvgMatrix {
    #[cfg(all(feature = "SvgMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `scaleNonUniform()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/scaleNonUniform)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    #[allow(clippy::all)]
    pub fn scale_non_uniform(&self, scale_factor_x: f32, scale_factor_y: f32) -> SvgMatrix {
        #[cfg(all(feature = "SvgMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale_non_uniform_SVGMatrix(
                self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_factor_x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_factor_y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale_non_uniform_SVGMatrix(
            self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_factor_x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_factor_y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(scale_factor_x);
            drop(scale_factor_y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scale_factor_x =
                    <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_factor_x);
                let scale_factor_y =
                    <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_factor_y);
                __widl_f_scale_non_uniform_SVGMatrix(self_, scale_factor_x, scale_factor_y)
            };
            <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_skew_x_SVGMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgMatrix as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <SvgMatrix as WasmDescribe>::describe();
}
impl SvgMatrix {
    #[cfg(all(feature = "SvgMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `skewX()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/skewX)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    #[allow(clippy::all)]
    pub fn skew_x(&self, angle: f32) -> Result<SvgMatrix, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_skew_x_SVGMatrix(
                self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                angle: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_skew_x_SVGMatrix(
            self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            angle: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(angle);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let angle = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(angle);
                __widl_f_skew_x_SVGMatrix(self_, angle)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "SvgMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_skew_y_SVGMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgMatrix as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <SvgMatrix as WasmDescribe>::describe();
}
impl SvgMatrix {
    #[cfg(all(feature = "SvgMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `skewY()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/skewY)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    #[allow(clippy::all)]
    pub fn skew_y(&self, angle: f32) -> Result<SvgMatrix, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_skew_y_SVGMatrix(
                self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                angle: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_skew_y_SVGMatrix(
            self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            angle: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(angle);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let angle = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(angle);
                __widl_f_skew_y_SVGMatrix(self_, angle)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "SvgMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_translate_SVGMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&SvgMatrix as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <SvgMatrix as WasmDescribe>::describe();
}
impl SvgMatrix {
    #[cfg(all(feature = "SvgMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `translate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/translate)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    #[allow(clippy::all)]
    pub fn translate(&self, x: f32, y: f32) -> SvgMatrix {
        #[cfg(all(feature = "SvgMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_translate_SVGMatrix(
                self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_translate_SVGMatrix(
            self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(x);
            drop(y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_translate_SVGMatrix(self_, x, y)
            };
            <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_a_SVGMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgMatrix as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SvgMatrix {
    #[cfg(all(feature = "SvgMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `a` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/a)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    #[allow(clippy::all)]
    pub fn a(&self) -> f32 {
        #[cfg(all(feature = "SvgMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_a_SVGMatrix(
                self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_a_SVGMatrix(
            self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_a_SVGMatrix(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_a_SVGMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgMatrix as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgMatrix {
    #[cfg(all(feature = "SvgMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `a` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/a)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    #[allow(clippy::all)]
    pub fn set_a(&self, a: f32) {
        #[cfg(all(feature = "SvgMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_a_SVGMatrix(
                self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_a_SVGMatrix(
            self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(a);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a);
                __widl_f_set_a_SVGMatrix(self_, a)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_b_SVGMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgMatrix as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SvgMatrix {
    #[cfg(all(feature = "SvgMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `b` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/b)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    #[allow(clippy::all)]
    pub fn b(&self) -> f32 {
        #[cfg(all(feature = "SvgMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_b_SVGMatrix(
                self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_b_SVGMatrix(
            self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_b_SVGMatrix(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_b_SVGMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgMatrix as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgMatrix {
    #[cfg(all(feature = "SvgMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `b` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/b)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    #[allow(clippy::all)]
    pub fn set_b(&self, b: f32) {
        #[cfg(all(feature = "SvgMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_b_SVGMatrix(
                self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                b: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_b_SVGMatrix(
            self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            b: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(b);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let b = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(b);
                __widl_f_set_b_SVGMatrix(self_, b)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_c_SVGMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgMatrix as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SvgMatrix {
    #[cfg(all(feature = "SvgMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `c` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/c)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    #[allow(clippy::all)]
    pub fn c(&self) -> f32 {
        #[cfg(all(feature = "SvgMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_c_SVGMatrix(
                self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_c_SVGMatrix(
            self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_c_SVGMatrix(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_c_SVGMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgMatrix as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgMatrix {
    #[cfg(all(feature = "SvgMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `c` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/c)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    #[allow(clippy::all)]
    pub fn set_c(&self, c: f32) {
        #[cfg(all(feature = "SvgMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_c_SVGMatrix(
                self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                c: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_c_SVGMatrix(
            self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            c: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(c);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let c = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(c);
                __widl_f_set_c_SVGMatrix(self_, c)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_d_SVGMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgMatrix as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SvgMatrix {
    #[cfg(all(feature = "SvgMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `d` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/d)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    #[allow(clippy::all)]
    pub fn d(&self) -> f32 {
        #[cfg(all(feature = "SvgMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_d_SVGMatrix(
                self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_d_SVGMatrix(
            self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_d_SVGMatrix(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_d_SVGMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgMatrix as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgMatrix {
    #[cfg(all(feature = "SvgMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `d` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/d)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    #[allow(clippy::all)]
    pub fn set_d(&self, d: f32) {
        #[cfg(all(feature = "SvgMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_d_SVGMatrix(
                self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                d: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_d_SVGMatrix(
            self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            d: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(d);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let d = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(d);
                __widl_f_set_d_SVGMatrix(self_, d)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_e_SVGMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgMatrix as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SvgMatrix {
    #[cfg(all(feature = "SvgMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `e` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/e)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    #[allow(clippy::all)]
    pub fn e(&self) -> f32 {
        #[cfg(all(feature = "SvgMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_e_SVGMatrix(
                self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_e_SVGMatrix(
            self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_e_SVGMatrix(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_e_SVGMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgMatrix as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgMatrix {
    #[cfg(all(feature = "SvgMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `e` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/e)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    #[allow(clippy::all)]
    pub fn set_e(&self, e: f32) {
        #[cfg(all(feature = "SvgMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_e_SVGMatrix(
                self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                e: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_e_SVGMatrix(
            self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            e: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(e);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let e = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(e);
                __widl_f_set_e_SVGMatrix(self_, e)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_f_SVGMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgMatrix as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SvgMatrix {
    #[cfg(all(feature = "SvgMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `f` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/f)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    #[allow(clippy::all)]
    pub fn f(&self) -> f32 {
        #[cfg(all(feature = "SvgMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_f_SVGMatrix(
                self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_f_SVGMatrix(
            self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_f_SVGMatrix(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_f_SVGMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgMatrix as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgMatrix {
    #[cfg(all(feature = "SvgMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `f` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/f)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    #[allow(clippy::all)]
    pub fn set_f(&self, f: f32) {
        #[cfg(all(feature = "SvgMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_f_SVGMatrix(
                self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                f: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_f_SVGMatrix(
            self_: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            f: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(f);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let f = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(f);
                __widl_f_set_f_SVGMatrix(self_, f)
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
pub static __WASM_BINDGEN_GENERATED_985620e2d9091c1d: [u8; 1621usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x13\x06\0\0\0\0\x18\0\0\x02\tSVGMatrix\x1B__widl_instanceof_SVGMatrix\0\0\0\0\x19__widl_f_flip_x_SVGMatrix\0\0\0\x01\tSVGMatrix\x01\0\0\x01\x01\x05self_\x05flipX\0\0\0\x19__widl_f_flip_y_SVGMatrix\0\0\0\x01\tSVGMatrix\x01\0\0\x01\x01\x05self_\x05flipY\0\0\0\x1A__widl_f_inverse_SVGMatrix\x01\0\0\x01\tSVGMatrix\x01\0\0\x01\x01\x05self_\x07inverse\0\0\0\x1B__widl_f_multiply_SVGMatrix\0\0\0\x01\tSVGMatrix\x01\0\0\x01\x02\x05self_\rsecond_matrix\x08multiply\0\0\0\x19__widl_f_rotate_SVGMatrix\0\0\0\x01\tSVGMatrix\x01\0\0\x01\x02\x05self_\x05angle\x06rotate\0\0\0%__widl_f_rotate_from_vector_SVGMatrix\x01\0\0\x01\tSVGMatrix\x01\0\0\x01\x03\x05self_\x01x\x01y\x10rotateFromVector\0\0\0\x18__widl_f_scale_SVGMatrix\0\0\0\x01\tSVGMatrix\x01\0\0\x01\x02\x05self_\x0Cscale_factor\x05scale\0\0\0$__widl_f_scale_non_uniform_SVGMatrix\0\0\0\x01\tSVGMatrix\x01\0\0\x01\x03\x05self_\x0Escale_factor_x\x0Escale_factor_y\x0FscaleNonUniform\0\0\0\x19__widl_f_skew_x_SVGMatrix\x01\0\0\x01\tSVGMatrix\x01\0\0\x01\x02\x05self_\x05angle\x05skewX\0\0\0\x19__widl_f_skew_y_SVGMatrix\x01\0\0\x01\tSVGMatrix\x01\0\0\x01\x02\x05self_\x05angle\x05skewY\0\0\0\x1C__widl_f_translate_SVGMatrix\0\0\0\x01\tSVGMatrix\x01\0\0\x01\x03\x05self_\x01x\x01y\ttranslate\0\0\0\x14__widl_f_a_SVGMatrix\0\0\0\x01\tSVGMatrix\x01\0\x01\x01a\x01\x01\x05self_\x01a\0\0\0\x18__widl_f_set_a_SVGMatrix\0\0\0\x01\tSVGMatrix\x01\0\x02\x01a\x01\x02\x05self_\x01a\x01a\0\0\0\x14__widl_f_b_SVGMatrix\0\0\0\x01\tSVGMatrix\x01\0\x01\x01b\x01\x01\x05self_\x01b\0\0\0\x18__widl_f_set_b_SVGMatrix\0\0\0\x01\tSVGMatrix\x01\0\x02\x01b\x01\x02\x05self_\x01b\x01b\0\0\0\x14__widl_f_c_SVGMatrix\0\0\0\x01\tSVGMatrix\x01\0\x01\x01c\x01\x01\x05self_\x01c\0\0\0\x18__widl_f_set_c_SVGMatrix\0\0\0\x01\tSVGMatrix\x01\0\x02\x01c\x01\x02\x05self_\x01c\x01c\0\0\0\x14__widl_f_d_SVGMatrix\0\0\0\x01\tSVGMatrix\x01\0\x01\x01d\x01\x01\x05self_\x01d\0\0\0\x18__widl_f_set_d_SVGMatrix\0\0\0\x01\tSVGMatrix\x01\0\x02\x01d\x01\x02\x05self_\x01d\x01d\0\0\0\x14__widl_f_e_SVGMatrix\0\0\0\x01\tSVGMatrix\x01\0\x01\x01e\x01\x01\x05self_\x01e\0\0\0\x18__widl_f_set_e_SVGMatrix\0\0\0\x01\tSVGMatrix\x01\0\x02\x01e\x01\x02\x05self_\x01e\x01e\0\0\0\x14__widl_f_f_SVGMatrix\0\0\0\x01\tSVGMatrix\x01\0\x01\x01f\x01\x01\x05self_\x01f\0\0\0\x18__widl_f_set_f_SVGMatrix\0\0\0\x01\tSVGMatrix\x01\0\x02\x01f\x01\x02\x05self_\x01f\x01f\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
