use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Path2D` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D)\n\n*This API requires the following crate features to be activated: `Path2d`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Path2d {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Path2d: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Path2d {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(6u32);
            inform(80u32);
            inform(97u32);
            inform(116u32);
            inform(104u32);
            inform(50u32);
            inform(68u32);
        }
    }
    impl core::ops::Deref for Path2d {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for Path2d {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Path2d {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Path2d {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Path2d {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Path2d {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Path2d {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Path2d {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Path2d {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Path2d>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Path2d {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Path2d {
        #[inline]
        fn from(obj: JsValue) -> Path2d {
            Path2d { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Path2d {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Path2d> for Path2d {
        #[inline]
        fn as_ref(&self) -> &Path2d {
            self
        }
    }
    impl From<Path2d> for JsValue {
        #[inline]
        fn from(obj: Path2d) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Path2d {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Path2D(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Path2D(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Path2D(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Path2d { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Path2d) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Path2d> for ::js_sys::Object {
    #[inline]
    fn from(obj: Path2d) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Path2d {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Path2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_Path2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <Path2d as WasmDescribe>::describe();
}
impl Path2d {
    #[cfg(all(feature = "Path2d",))]
    #[allow(bad_style)]
    #[doc = "The `new Path2D(..)` constructor, creating a new instance of `Path2D`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/Path2D)\n\n*This API requires the following crate features to be activated: `Path2d`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<Path2d, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Path2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_Path2D() -> <Path2d as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_Path2D() -> <Path2d as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_Path2D() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Path2d as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Path2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_other_Path2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Path2d as WasmDescribe>::describe();
    <Path2d as WasmDescribe>::describe();
}
impl Path2d {
    #[cfg(all(feature = "Path2d",))]
    #[allow(bad_style)]
    #[doc = "The `new Path2D(..)` constructor, creating a new instance of `Path2D`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/Path2D)\n\n*This API requires the following crate features to be activated: `Path2d`*"]
    #[allow(clippy::all)]
    pub fn new_with_other(other: &Path2d) -> Result<Path2d, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Path2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_other_Path2D(
                other: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Path2d as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_other_Path2D(
            other: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Path2d as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(other);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let other = <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(other);
                __widl_f_new_with_other_Path2D(other)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Path2d as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Path2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_path_string_Path2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <Path2d as WasmDescribe>::describe();
}
impl Path2d {
    #[cfg(all(feature = "Path2d",))]
    #[allow(bad_style)]
    #[doc = "The `new Path2D(..)` constructor, creating a new instance of `Path2D`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/Path2D)\n\n*This API requires the following crate features to be activated: `Path2d`*"]
    #[allow(clippy::all)]
    pub fn new_with_path_string(path_string: &str) -> Result<Path2d, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Path2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_path_string_Path2D(
                path_string: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Path2d as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_path_string_Path2D(
            path_string: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Path2d as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(path_string);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let path_string =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(path_string);
                __widl_f_new_with_path_string_Path2D(path_string)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Path2d as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Path2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_path_Path2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Path2d as WasmDescribe>::describe();
    <&Path2d as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Path2d {
    #[cfg(all(feature = "Path2d",))]
    #[allow(bad_style)]
    #[doc = "The `addPath()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/addPath)\n\n*This API requires the following crate features to be activated: `Path2d`*"]
    #[allow(clippy::all)]
    pub fn add_path(&self, path: &Path2d) {
        #[cfg(all(feature = "Path2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_path_Path2D(
                self_: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                path: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_path_Path2D(
            self_: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            path: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(path);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let path = <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(path);
                __widl_f_add_path_Path2D(self_, path)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Path2d", feature = "SvgMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_path_with_transformation_Path2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Path2d as WasmDescribe>::describe();
    <&Path2d as WasmDescribe>::describe();
    <&SvgMatrix as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Path2d {
    #[cfg(all(feature = "Path2d", feature = "SvgMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `addPath()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/addPath)\n\n*This API requires the following crate features to be activated: `Path2d`, `SvgMatrix`*"]
    #[allow(clippy::all)]
    pub fn add_path_with_transformation(&self, path: &Path2d, transformation: &SvgMatrix) {
        #[cfg(all(feature = "Path2d", feature = "SvgMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_path_with_transformation_Path2D(
                self_: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                path: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                transformation: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_path_with_transformation_Path2D(
            self_: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            path: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            transformation: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(path);
            drop(transformation);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let path = <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(path);
                let transformation =
                    <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(transformation);
                __widl_f_add_path_with_transformation_Path2D(self_, path, transformation)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Path2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_arc_Path2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&Path2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Path2d {
    #[cfg(all(feature = "Path2d",))]
    #[allow(bad_style)]
    #[doc = "The `arc()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/arc)\n\n*This API requires the following crate features to be activated: `Path2d`*"]
    #[allow(clippy::all)]
    pub fn arc(
        &self,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Path2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_arc_Path2D(
                self_: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                radius: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_arc_Path2D(
            self_: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            radius: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            end_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            drop(radius);
            drop(start_angle);
            drop(end_angle);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let radius = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(radius);
                let start_angle =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start_angle);
                let end_angle = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end_angle);
                __widl_f_arc_Path2D(self_, x, y, radius, start_angle, end_angle)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Path2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_arc_with_anticlockwise_Path2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&Path2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Path2d {
    #[cfg(all(feature = "Path2d",))]
    #[allow(bad_style)]
    #[doc = "The `arc()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/arc)\n\n*This API requires the following crate features to be activated: `Path2d`*"]
    #[allow(clippy::all)]
    pub fn arc_with_anticlockwise(
        &self,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Path2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_arc_with_anticlockwise_Path2D(
                self_: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                radius: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                anticlockwise: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_arc_with_anticlockwise_Path2D(
            self_: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            radius: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            end_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            anticlockwise: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            drop(radius);
            drop(start_angle);
            drop(end_angle);
            drop(anticlockwise);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let radius = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(radius);
                let start_angle =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start_angle);
                let end_angle = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end_angle);
                let anticlockwise =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(anticlockwise);
                __widl_f_arc_with_anticlockwise_Path2D(
                    self_,
                    x,
                    y,
                    radius,
                    start_angle,
                    end_angle,
                    anticlockwise,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Path2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_arc_to_Path2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&Path2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Path2d {
    #[cfg(all(feature = "Path2d",))]
    #[allow(bad_style)]
    #[doc = "The `arcTo()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/arcTo)\n\n*This API requires the following crate features to be activated: `Path2d`*"]
    #[allow(clippy::all)]
    pub fn arc_to(
        &self,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        radius: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Path2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_arc_to_Path2D(
                self_: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x1: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y1: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x2: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y2: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                radius: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_arc_to_Path2D(
            self_: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x1: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y1: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x2: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y2: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            radius: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x1);
            drop(y1);
            drop(x2);
            drop(y2);
            drop(radius);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x1 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x1);
                let y1 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y1);
                let x2 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x2);
                let y2 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y2);
                let radius = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(radius);
                __widl_f_arc_to_Path2D(self_, x1, y1, x2, y2, radius)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Path2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_bezier_curve_to_Path2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&Path2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Path2d {
    #[cfg(all(feature = "Path2d",))]
    #[allow(bad_style)]
    #[doc = "The `bezierCurveTo()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/bezierCurveTo)\n\n*This API requires the following crate features to be activated: `Path2d`*"]
    #[allow(clippy::all)]
    pub fn bezier_curve_to(&self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        #[cfg(all(feature = "Path2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_bezier_curve_to_Path2D(
                self_: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cp1x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cp1y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cp2x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cp2y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_bezier_curve_to_Path2D(
            self_: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cp1x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cp1y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cp2x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cp2y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(cp1x);
            drop(cp1y);
            drop(cp2x);
            drop(cp2y);
            drop(x);
            drop(y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let cp1x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cp1x);
                let cp1y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cp1y);
                let cp2x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cp2x);
                let cp2y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cp2y);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_bezier_curve_to_Path2D(self_, cp1x, cp1y, cp2x, cp2y, x, y)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Path2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_close_path_Path2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Path2d as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Path2d {
    #[cfg(all(feature = "Path2d",))]
    #[allow(bad_style)]
    #[doc = "The `closePath()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/closePath)\n\n*This API requires the following crate features to be activated: `Path2d`*"]
    #[allow(clippy::all)]
    pub fn close_path(&self) {
        #[cfg(all(feature = "Path2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_close_path_Path2D(
                self_: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_close_path_Path2D(
            self_: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_close_path_Path2D(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Path2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ellipse_Path2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&Path2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Path2d {
    #[cfg(all(feature = "Path2d",))]
    #[allow(bad_style)]
    #[doc = "The `ellipse()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/ellipse)\n\n*This API requires the following crate features to be activated: `Path2d`*"]
    #[allow(clippy::all)]
    pub fn ellipse(
        &self,
        x: f64,
        y: f64,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Path2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ellipse_Path2D(
                self_: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                radius_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                radius_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rotation: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ellipse_Path2D(
            self_: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            radius_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            radius_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rotation: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            end_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            drop(radius_x);
            drop(radius_y);
            drop(rotation);
            drop(start_angle);
            drop(end_angle);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let radius_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(radius_x);
                let radius_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(radius_y);
                let rotation = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rotation);
                let start_angle =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start_angle);
                let end_angle = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end_angle);
                __widl_f_ellipse_Path2D(
                    self_,
                    x,
                    y,
                    radius_x,
                    radius_y,
                    rotation,
                    start_angle,
                    end_angle,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Path2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ellipse_with_anticlockwise_Path2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(9u32);
    <&Path2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Path2d {
    #[cfg(all(feature = "Path2d",))]
    #[allow(bad_style)]
    #[doc = "The `ellipse()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/ellipse)\n\n*This API requires the following crate features to be activated: `Path2d`*"]
    #[allow(clippy::all)]
    pub fn ellipse_with_anticlockwise(
        &self,
        x: f64,
        y: f64,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Path2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ellipse_with_anticlockwise_Path2D(
                self_: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                radius_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                radius_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rotation: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                anticlockwise: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ellipse_with_anticlockwise_Path2D(
            self_: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            radius_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            radius_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rotation: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            end_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            anticlockwise: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            drop(radius_x);
            drop(radius_y);
            drop(rotation);
            drop(start_angle);
            drop(end_angle);
            drop(anticlockwise);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let radius_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(radius_x);
                let radius_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(radius_y);
                let rotation = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rotation);
                let start_angle =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start_angle);
                let end_angle = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end_angle);
                let anticlockwise =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(anticlockwise);
                __widl_f_ellipse_with_anticlockwise_Path2D(
                    self_,
                    x,
                    y,
                    radius_x,
                    radius_y,
                    rotation,
                    start_angle,
                    end_angle,
                    anticlockwise,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Path2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_line_to_Path2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Path2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Path2d {
    #[cfg(all(feature = "Path2d",))]
    #[allow(bad_style)]
    #[doc = "The `lineTo()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/lineTo)\n\n*This API requires the following crate features to be activated: `Path2d`*"]
    #[allow(clippy::all)]
    pub fn line_to(&self, x: f64, y: f64) {
        #[cfg(all(feature = "Path2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_line_to_Path2D(
                self_: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_line_to_Path2D(
            self_: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
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
                let self_ = <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_line_to_Path2D(self_, x, y)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Path2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_move_to_Path2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Path2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Path2d {
    #[cfg(all(feature = "Path2d",))]
    #[allow(bad_style)]
    #[doc = "The `moveTo()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/moveTo)\n\n*This API requires the following crate features to be activated: `Path2d`*"]
    #[allow(clippy::all)]
    pub fn move_to(&self, x: f64, y: f64) {
        #[cfg(all(feature = "Path2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_move_to_Path2D(
                self_: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_move_to_Path2D(
            self_: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
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
                let self_ = <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_move_to_Path2D(self_, x, y)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Path2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_quadratic_curve_to_Path2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&Path2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Path2d {
    #[cfg(all(feature = "Path2d",))]
    #[allow(bad_style)]
    #[doc = "The `quadraticCurveTo()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/quadraticCurveTo)\n\n*This API requires the following crate features to be activated: `Path2d`*"]
    #[allow(clippy::all)]
    pub fn quadratic_curve_to(&self, cpx: f64, cpy: f64, x: f64, y: f64) {
        #[cfg(all(feature = "Path2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_quadratic_curve_to_Path2D(
                self_: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cpx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cpy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_quadratic_curve_to_Path2D(
            self_: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cpx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cpy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(cpx);
            drop(cpy);
            drop(x);
            drop(y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let cpx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cpx);
                let cpy = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cpy);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_quadratic_curve_to_Path2D(self_, cpx, cpy, x, y)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Path2d",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rect_Path2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&Path2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Path2d {
    #[cfg(all(feature = "Path2d",))]
    #[allow(bad_style)]
    #[doc = "The `rect()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/rect)\n\n*This API requires the following crate features to be activated: `Path2d`*"]
    #[allow(clippy::all)]
    pub fn rect(&self, x: f64, y: f64, w: f64, h: f64) {
        #[cfg(all(feature = "Path2d",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rect_Path2D(
                self_: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                w: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                h: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rect_Path2D(
            self_: <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            w: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            h: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            drop(w);
            drop(h);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Path2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let w = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(w);
                let h = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(h);
                __widl_f_rect_Path2D(self_, x, y, w, h)
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
pub static __WASM_BINDGEN_GENERATED_460c1a471d1c0aed: [u8; 1419usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}I\x05\0\0\0\0\x11\0\0\x02\x06Path2D\x18__widl_instanceof_Path2D\0\0\0\0\x13__widl_f_new_Path2D\x01\0\0\x01\x06Path2D\0\x01\0\x03new\0\0\0\x1E__widl_f_new_with_other_Path2D\x01\0\0\x01\x06Path2D\0\x01\x01\x05other\x03new\0\0\0$__widl_f_new_with_path_string_Path2D\x01\0\0\x01\x06Path2D\0\x01\x01\x0Bpath_string\x03new\0\0\0\x18__widl_f_add_path_Path2D\0\0\0\x01\x06Path2D\x01\0\0\x01\x02\x05self_\x04path\x07addPath\0\0\0,__widl_f_add_path_with_transformation_Path2D\0\0\0\x01\x06Path2D\x01\0\0\x01\x03\x05self_\x04path\x0Etransformation\x07addPath\0\0\0\x13__widl_f_arc_Path2D\x01\0\0\x01\x06Path2D\x01\0\0\x01\x06\x05self_\x01x\x01y\x06radius\x0Bstart_angle\tend_angle\x03arc\0\0\0&__widl_f_arc_with_anticlockwise_Path2D\x01\0\0\x01\x06Path2D\x01\0\0\x01\x07\x05self_\x01x\x01y\x06radius\x0Bstart_angle\tend_angle\ranticlockwise\x03arc\0\0\0\x16__widl_f_arc_to_Path2D\x01\0\0\x01\x06Path2D\x01\0\0\x01\x06\x05self_\x02x1\x02y1\x02x2\x02y2\x06radius\x05arcTo\0\0\0\x1F__widl_f_bezier_curve_to_Path2D\0\0\0\x01\x06Path2D\x01\0\0\x01\x07\x05self_\x04cp1x\x04cp1y\x04cp2x\x04cp2y\x01x\x01y\rbezierCurveTo\0\0\0\x1A__widl_f_close_path_Path2D\0\0\0\x01\x06Path2D\x01\0\0\x01\x01\x05self_\tclosePath\0\0\0\x17__widl_f_ellipse_Path2D\x01\0\0\x01\x06Path2D\x01\0\0\x01\x08\x05self_\x01x\x01y\x08radius_x\x08radius_y\x08rotation\x0Bstart_angle\tend_angle\x07ellipse\0\0\0*__widl_f_ellipse_with_anticlockwise_Path2D\x01\0\0\x01\x06Path2D\x01\0\0\x01\t\x05self_\x01x\x01y\x08radius_x\x08radius_y\x08rotation\x0Bstart_angle\tend_angle\ranticlockwise\x07ellipse\0\0\0\x17__widl_f_line_to_Path2D\0\0\0\x01\x06Path2D\x01\0\0\x01\x03\x05self_\x01x\x01y\x06lineTo\0\0\0\x17__widl_f_move_to_Path2D\0\0\0\x01\x06Path2D\x01\0\0\x01\x03\x05self_\x01x\x01y\x06moveTo\0\0\0\"__widl_f_quadratic_curve_to_Path2D\0\0\0\x01\x06Path2D\x01\0\0\x01\x05\x05self_\x03cpx\x03cpy\x01x\x01y\x10quadraticCurveTo\0\0\0\x14__widl_f_rect_Path2D\0\0\0\x01\x06Path2D\x01\0\0\x01\x05\x05self_\x01x\x01y\x01w\x01h\x04rect\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
