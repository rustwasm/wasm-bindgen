use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `DOMPoint` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint)\n\n*This API requires the following crate features to be activated: `DomPoint`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct DomPoint {
    obj: DomPointReadOnly,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_DomPoint: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for DomPoint {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(8u32);
            inform(68u32);
            inform(79u32);
            inform(77u32);
            inform(80u32);
            inform(111u32);
            inform(105u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for DomPoint {
        type Target = DomPointReadOnly;
        #[inline]
        fn deref(&self) -> &DomPointReadOnly {
            &self.obj
        }
    }
    impl IntoWasmAbi for DomPoint {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for DomPoint {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a DomPoint {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for DomPoint {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            DomPoint {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for DomPoint {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a DomPoint {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for DomPoint {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<DomPoint>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(DomPoint {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for DomPoint {
        #[inline]
        fn from(obj: JsValue) -> DomPoint {
            DomPoint { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for DomPoint {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<DomPoint> for DomPoint {
        #[inline]
        fn as_ref(&self) -> &DomPoint {
            self
        }
    }
    impl From<DomPoint> for JsValue {
        #[inline]
        fn from(obj: DomPoint) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for DomPoint {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_DOMPoint(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_DOMPoint(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_DOMPoint(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            DomPoint { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const DomPoint) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<DomPoint> for DomPointReadOnly {
    #[inline]
    fn from(obj: DomPoint) -> DomPointReadOnly {
        use wasm_bindgen::JsCast;
        DomPointReadOnly::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<DomPointReadOnly> for DomPoint {
    #[inline]
    fn as_ref(&self) -> &DomPointReadOnly {
        use wasm_bindgen::JsCast;
        DomPointReadOnly::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<DomPoint> for ::js_sys::Object {
    #[inline]
    fn from(obj: DomPoint) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for DomPoint {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "DomPoint",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_DOMPoint() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <DomPoint as WasmDescribe>::describe();
}
impl DomPoint {
    #[cfg(all(feature = "DomPoint",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMPoint(..)` constructor, creating a new instance of `DOMPoint`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/DOMPoint)\n\n*This API requires the following crate features to be activated: `DomPoint`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<DomPoint, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomPoint",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_DOMPoint() -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_DOMPoint() -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_DOMPoint() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomPoint as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomPoint",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_x_DOMPoint() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <f64 as WasmDescribe>::describe();
    <DomPoint as WasmDescribe>::describe();
}
impl DomPoint {
    #[cfg(all(feature = "DomPoint",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMPoint(..)` constructor, creating a new instance of `DOMPoint`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/DOMPoint)\n\n*This API requires the following crate features to be activated: `DomPoint`*"]
    #[allow(clippy::all)]
    pub fn new_with_x(x: f64) -> Result<DomPoint, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomPoint",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_x_DOMPoint(
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_x_DOMPoint(
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(x);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                __widl_f_new_with_x_DOMPoint(x)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomPoint as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomPoint",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_x_and_y_DOMPoint() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomPoint as WasmDescribe>::describe();
}
impl DomPoint {
    #[cfg(all(feature = "DomPoint",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMPoint(..)` constructor, creating a new instance of `DOMPoint`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/DOMPoint)\n\n*This API requires the following crate features to be activated: `DomPoint`*"]
    #[allow(clippy::all)]
    pub fn new_with_x_and_y(x: f64, y: f64) -> Result<DomPoint, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomPoint",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_x_and_y_DOMPoint(
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_x_and_y_DOMPoint(
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(x);
            drop(y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_new_with_x_and_y_DOMPoint(x, y)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomPoint as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomPoint",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_x_and_y_and_z_DOMPoint() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomPoint as WasmDescribe>::describe();
}
impl DomPoint {
    #[cfg(all(feature = "DomPoint",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMPoint(..)` constructor, creating a new instance of `DOMPoint`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/DOMPoint)\n\n*This API requires the following crate features to be activated: `DomPoint`*"]
    #[allow(clippy::all)]
    pub fn new_with_x_and_y_and_z(
        x: f64,
        y: f64,
        z: f64,
    ) -> Result<DomPoint, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomPoint",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_x_and_y_and_z_DOMPoint(
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_x_and_y_and_z_DOMPoint(
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(x);
            drop(y);
            drop(z);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let z = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(z);
                __widl_f_new_with_x_and_y_and_z_DOMPoint(x, y, z)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomPoint as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomPoint",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_x_and_y_and_z_and_w_DOMPoint() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomPoint as WasmDescribe>::describe();
}
impl DomPoint {
    #[cfg(all(feature = "DomPoint",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMPoint(..)` constructor, creating a new instance of `DOMPoint`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/DOMPoint)\n\n*This API requires the following crate features to be activated: `DomPoint`*"]
    #[allow(clippy::all)]
    pub fn new_with_x_and_y_and_z_and_w(
        x: f64,
        y: f64,
        z: f64,
        w: f64,
    ) -> Result<DomPoint, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomPoint",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_x_and_y_and_z_and_w_DOMPoint(
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                w: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_x_and_y_and_z_and_w_DOMPoint(
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            w: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(x);
            drop(y);
            drop(z);
            drop(w);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let z = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(z);
                let w = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(w);
                __widl_f_new_with_x_and_y_and_z_and_w_DOMPoint(x, y, z, w)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomPoint as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomPoint",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_from_point_DOMPoint() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <DomPoint as WasmDescribe>::describe();
}
impl DomPoint {
    #[cfg(all(feature = "DomPoint",))]
    #[allow(bad_style)]
    #[doc = "The `fromPoint()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/fromPoint)\n\n*This API requires the following crate features to be activated: `DomPoint`*"]
    #[allow(clippy::all)]
    pub fn from_point() -> DomPoint {
        #[cfg(all(feature = "DomPoint",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_from_point_DOMPoint(
            ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_from_point_DOMPoint(
        ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_from_point_DOMPoint() };
            <DomPoint as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomPoint", feature = "DomPointInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_from_point_with_other_DOMPoint() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomPointInit as WasmDescribe>::describe();
    <DomPoint as WasmDescribe>::describe();
}
impl DomPoint {
    #[cfg(all(feature = "DomPoint", feature = "DomPointInit",))]
    #[allow(bad_style)]
    #[doc = "The `fromPoint()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/fromPoint)\n\n*This API requires the following crate features to be activated: `DomPoint`, `DomPointInit`*"]
    #[allow(clippy::all)]
    pub fn from_point_with_other(other: &DomPointInit) -> DomPoint {
        #[cfg(all(feature = "DomPoint", feature = "DomPointInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_from_point_with_other_DOMPoint(
                other: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_from_point_with_other_DOMPoint(
            other: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(other);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let other = <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(other);
                __widl_f_from_point_with_other_DOMPoint(other)
            };
            <DomPoint as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomPoint",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_x_DOMPoint() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomPoint as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomPoint {
    #[cfg(all(feature = "DomPoint",))]
    #[allow(bad_style)]
    #[doc = "The `x` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/x)\n\n*This API requires the following crate features to be activated: `DomPoint`*"]
    #[allow(clippy::all)]
    pub fn x(&self) -> f64 {
        #[cfg(all(feature = "DomPoint",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_x_DOMPoint(
                self_: <&DomPoint as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_x_DOMPoint(
            self_: <&DomPoint as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomPoint as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_x_DOMPoint(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomPoint",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_x_DOMPoint() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomPoint as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomPoint {
    #[cfg(all(feature = "DomPoint",))]
    #[allow(bad_style)]
    #[doc = "The `x` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/x)\n\n*This API requires the following crate features to be activated: `DomPoint`*"]
    #[allow(clippy::all)]
    pub fn set_x(&self, x: f64) {
        #[cfg(all(feature = "DomPoint",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_x_DOMPoint(
                self_: <&DomPoint as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_x_DOMPoint(
            self_: <&DomPoint as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DomPoint as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                __widl_f_set_x_DOMPoint(self_, x)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomPoint",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_y_DOMPoint() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomPoint as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomPoint {
    #[cfg(all(feature = "DomPoint",))]
    #[allow(bad_style)]
    #[doc = "The `y` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/y)\n\n*This API requires the following crate features to be activated: `DomPoint`*"]
    #[allow(clippy::all)]
    pub fn y(&self) -> f64 {
        #[cfg(all(feature = "DomPoint",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_y_DOMPoint(
                self_: <&DomPoint as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_y_DOMPoint(
            self_: <&DomPoint as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomPoint as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_y_DOMPoint(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomPoint",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_y_DOMPoint() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomPoint as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomPoint {
    #[cfg(all(feature = "DomPoint",))]
    #[allow(bad_style)]
    #[doc = "The `y` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/y)\n\n*This API requires the following crate features to be activated: `DomPoint`*"]
    #[allow(clippy::all)]
    pub fn set_y(&self, y: f64) {
        #[cfg(all(feature = "DomPoint",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_y_DOMPoint(
                self_: <&DomPoint as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_y_DOMPoint(
            self_: <&DomPoint as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DomPoint as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_set_y_DOMPoint(self_, y)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomPoint",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_z_DOMPoint() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomPoint as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomPoint {
    #[cfg(all(feature = "DomPoint",))]
    #[allow(bad_style)]
    #[doc = "The `z` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/z)\n\n*This API requires the following crate features to be activated: `DomPoint`*"]
    #[allow(clippy::all)]
    pub fn z(&self) -> f64 {
        #[cfg(all(feature = "DomPoint",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_z_DOMPoint(
                self_: <&DomPoint as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_z_DOMPoint(
            self_: <&DomPoint as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomPoint as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_z_DOMPoint(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomPoint",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_z_DOMPoint() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomPoint as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomPoint {
    #[cfg(all(feature = "DomPoint",))]
    #[allow(bad_style)]
    #[doc = "The `z` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/z)\n\n*This API requires the following crate features to be activated: `DomPoint`*"]
    #[allow(clippy::all)]
    pub fn set_z(&self, z: f64) {
        #[cfg(all(feature = "DomPoint",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_z_DOMPoint(
                self_: <&DomPoint as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_z_DOMPoint(
            self_: <&DomPoint as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(z);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomPoint as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let z = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(z);
                __widl_f_set_z_DOMPoint(self_, z)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomPoint",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_w_DOMPoint() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomPoint as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomPoint {
    #[cfg(all(feature = "DomPoint",))]
    #[allow(bad_style)]
    #[doc = "The `w` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/w)\n\n*This API requires the following crate features to be activated: `DomPoint`*"]
    #[allow(clippy::all)]
    pub fn w(&self) -> f64 {
        #[cfg(all(feature = "DomPoint",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_w_DOMPoint(
                self_: <&DomPoint as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_w_DOMPoint(
            self_: <&DomPoint as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomPoint as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_w_DOMPoint(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomPoint",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_w_DOMPoint() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomPoint as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomPoint {
    #[cfg(all(feature = "DomPoint",))]
    #[allow(bad_style)]
    #[doc = "The `w` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/w)\n\n*This API requires the following crate features to be activated: `DomPoint`*"]
    #[allow(clippy::all)]
    pub fn set_w(&self, w: f64) {
        #[cfg(all(feature = "DomPoint",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_w_DOMPoint(
                self_: <&DomPoint as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                w: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_w_DOMPoint(
            self_: <&DomPoint as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            w: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(w);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomPoint as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let w = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(w);
                __widl_f_set_w_DOMPoint(self_, w)
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
pub static __WASM_BINDGEN_GENERATED_a191607707108ee4: [u8; 1023usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xBD\x03\0\0\0\0\x10\0\0\x02\x08DOMPoint\x1A__widl_instanceof_DOMPoint\0\0\0\0\x15__widl_f_new_DOMPoint\x01\0\0\x01\x08DOMPoint\0\x01\0\x03new\0\0\0\x1C__widl_f_new_with_x_DOMPoint\x01\0\0\x01\x08DOMPoint\0\x01\x01\x01x\x03new\0\0\0\"__widl_f_new_with_x_and_y_DOMPoint\x01\0\0\x01\x08DOMPoint\0\x01\x02\x01x\x01y\x03new\0\0\0(__widl_f_new_with_x_and_y_and_z_DOMPoint\x01\0\0\x01\x08DOMPoint\0\x01\x03\x01x\x01y\x01z\x03new\0\0\0.__widl_f_new_with_x_and_y_and_z_and_w_DOMPoint\x01\0\0\x01\x08DOMPoint\0\x01\x04\x01x\x01y\x01z\x01w\x03new\0\0\0\x1C__widl_f_from_point_DOMPoint\0\0\0\x01\x08DOMPoint\x01\x01\0\x01\0\tfromPoint\0\0\0'__widl_f_from_point_with_other_DOMPoint\0\0\0\x01\x08DOMPoint\x01\x01\0\x01\x01\x05other\tfromPoint\0\0\0\x13__widl_f_x_DOMPoint\0\0\0\x01\x08DOMPoint\x01\0\x01\x01x\x01\x01\x05self_\x01x\0\0\0\x17__widl_f_set_x_DOMPoint\0\0\0\x01\x08DOMPoint\x01\0\x02\x01x\x01\x02\x05self_\x01x\x01x\0\0\0\x13__widl_f_y_DOMPoint\0\0\0\x01\x08DOMPoint\x01\0\x01\x01y\x01\x01\x05self_\x01y\0\0\0\x17__widl_f_set_y_DOMPoint\0\0\0\x01\x08DOMPoint\x01\0\x02\x01y\x01\x02\x05self_\x01y\x01y\0\0\0\x13__widl_f_z_DOMPoint\0\0\0\x01\x08DOMPoint\x01\0\x01\x01z\x01\x01\x05self_\x01z\0\0\0\x17__widl_f_set_z_DOMPoint\0\0\0\x01\x08DOMPoint\x01\0\x02\x01z\x01\x02\x05self_\x01z\x01z\0\0\0\x13__widl_f_w_DOMPoint\0\0\0\x01\x08DOMPoint\x01\0\x01\x01w\x01\x01\x05self_\x01w\0\0\0\x17__widl_f_set_w_DOMPoint\0\0\0\x01\x08DOMPoint\x01\0\x02\x01w\x01\x02\x05self_\x01w\x01w\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
