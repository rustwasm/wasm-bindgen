use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `DOMPointReadOnly` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly)\n\n*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct DomPointReadOnly {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_DomPointReadOnly: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for DomPointReadOnly {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(68u32);
            inform(79u32);
            inform(77u32);
            inform(80u32);
            inform(111u32);
            inform(105u32);
            inform(110u32);
            inform(116u32);
            inform(82u32);
            inform(101u32);
            inform(97u32);
            inform(100u32);
            inform(79u32);
            inform(110u32);
            inform(108u32);
            inform(121u32);
        }
    }
    impl core::ops::Deref for DomPointReadOnly {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for DomPointReadOnly {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for DomPointReadOnly {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a DomPointReadOnly {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for DomPointReadOnly {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            DomPointReadOnly {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for DomPointReadOnly {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a DomPointReadOnly {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for DomPointReadOnly {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<DomPointReadOnly>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(DomPointReadOnly {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for DomPointReadOnly {
        #[inline]
        fn from(obj: JsValue) -> DomPointReadOnly {
            DomPointReadOnly { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for DomPointReadOnly {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<DomPointReadOnly> for DomPointReadOnly {
        #[inline]
        fn as_ref(&self) -> &DomPointReadOnly {
            self
        }
    }
    impl From<DomPointReadOnly> for JsValue {
        #[inline]
        fn from(obj: DomPointReadOnly) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for DomPointReadOnly {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_DOMPointReadOnly(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_DOMPointReadOnly(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_DOMPointReadOnly(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            DomPointReadOnly { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const DomPointReadOnly) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<DomPointReadOnly> for ::js_sys::Object {
    #[inline]
    fn from(obj: DomPointReadOnly) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for DomPointReadOnly {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "DomPointReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_DOMPointReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <DomPointReadOnly as WasmDescribe>::describe();
}
impl DomPointReadOnly {
    #[cfg(all(feature = "DomPointReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMPointReadOnly(..)` constructor, creating a new instance of `DOMPointReadOnly`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/DOMPointReadOnly)\n\n*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<DomPointReadOnly, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomPointReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_DOMPointReadOnly(
            ) -> <DomPointReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_DOMPointReadOnly(
        ) -> <DomPointReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_DOMPointReadOnly() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomPointReadOnly as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DomPointReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_x_DOMPointReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <f64 as WasmDescribe>::describe();
    <DomPointReadOnly as WasmDescribe>::describe();
}
impl DomPointReadOnly {
    #[cfg(all(feature = "DomPointReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMPointReadOnly(..)` constructor, creating a new instance of `DOMPointReadOnly`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/DOMPointReadOnly)\n\n*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    #[allow(clippy::all)]
    pub fn new_with_x(x: f64) -> Result<DomPointReadOnly, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomPointReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_x_DOMPointReadOnly(
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPointReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_x_DOMPointReadOnly(
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomPointReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(x);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                __widl_f_new_with_x_DOMPointReadOnly(x)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomPointReadOnly as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DomPointReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_x_and_y_DOMPointReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomPointReadOnly as WasmDescribe>::describe();
}
impl DomPointReadOnly {
    #[cfg(all(feature = "DomPointReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMPointReadOnly(..)` constructor, creating a new instance of `DOMPointReadOnly`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/DOMPointReadOnly)\n\n*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    #[allow(clippy::all)]
    pub fn new_with_x_and_y(x: f64, y: f64) -> Result<DomPointReadOnly, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomPointReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_x_and_y_DOMPointReadOnly(
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPointReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_x_and_y_DOMPointReadOnly(
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomPointReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                __widl_f_new_with_x_and_y_DOMPointReadOnly(x, y)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomPointReadOnly as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DomPointReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_x_and_y_and_z_DOMPointReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomPointReadOnly as WasmDescribe>::describe();
}
impl DomPointReadOnly {
    #[cfg(all(feature = "DomPointReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMPointReadOnly(..)` constructor, creating a new instance of `DOMPointReadOnly`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/DOMPointReadOnly)\n\n*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    #[allow(clippy::all)]
    pub fn new_with_x_and_y_and_z(
        x: f64,
        y: f64,
        z: f64,
    ) -> Result<DomPointReadOnly, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomPointReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_x_and_y_and_z_DOMPointReadOnly(
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPointReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_x_and_y_and_z_DOMPointReadOnly(
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomPointReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                __widl_f_new_with_x_and_y_and_z_DOMPointReadOnly(x, y, z)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomPointReadOnly as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DomPointReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_x_and_y_and_z_and_w_DOMPointReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomPointReadOnly as WasmDescribe>::describe();
}
impl DomPointReadOnly {
    #[cfg(all(feature = "DomPointReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMPointReadOnly(..)` constructor, creating a new instance of `DOMPointReadOnly`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/DOMPointReadOnly)\n\n*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    #[allow(clippy::all)]
    pub fn new_with_x_and_y_and_z_and_w(
        x: f64,
        y: f64,
        z: f64,
        w: f64,
    ) -> Result<DomPointReadOnly, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomPointReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_x_and_y_and_z_and_w_DOMPointReadOnly(
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                w: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPointReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_x_and_y_and_z_and_w_DOMPointReadOnly(
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            w: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomPointReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                __widl_f_new_with_x_and_y_and_z_and_w_DOMPointReadOnly(x, y, z, w)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomPointReadOnly as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DomPointReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_from_point_DOMPointReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <DomPointReadOnly as WasmDescribe>::describe();
}
impl DomPointReadOnly {
    #[cfg(all(feature = "DomPointReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `fromPoint()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/fromPoint)\n\n*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    #[allow(clippy::all)]
    pub fn from_point() -> DomPointReadOnly {
        #[cfg(all(feature = "DomPointReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_from_point_DOMPointReadOnly(
            ) -> <DomPointReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_from_point_DOMPointReadOnly(
        ) -> <DomPointReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_from_point_DOMPointReadOnly() };
            <DomPointReadOnly as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomPointInit", feature = "DomPointReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_from_point_with_other_DOMPointReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomPointInit as WasmDescribe>::describe();
    <DomPointReadOnly as WasmDescribe>::describe();
}
impl DomPointReadOnly {
    #[cfg(all(feature = "DomPointInit", feature = "DomPointReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `fromPoint()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/fromPoint)\n\n*This API requires the following crate features to be activated: `DomPointInit`, `DomPointReadOnly`*"]
    #[allow(clippy::all)]
    pub fn from_point_with_other(other: &DomPointInit) -> DomPointReadOnly {
        #[cfg(all(feature = "DomPointInit", feature = "DomPointReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_from_point_with_other_DOMPointReadOnly(
                other: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPointReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_from_point_with_other_DOMPointReadOnly(
            other: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomPointReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(other);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let other = <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(other);
                __widl_f_from_point_with_other_DOMPointReadOnly(other)
            };
            <DomPointReadOnly as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomPointReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_to_json_DOMPointReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomPointReadOnly as WasmDescribe>::describe();
    <::js_sys::Object as WasmDescribe>::describe();
}
impl DomPointReadOnly {
    #[cfg(all(feature = "DomPointReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `toJSON()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/toJSON)\n\n*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    #[allow(clippy::all)]
    pub fn to_json(&self) -> ::js_sys::Object {
        #[cfg(all(feature = "DomPointReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_to_json_DOMPointReadOnly(
                self_: <&DomPointReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_to_json_DOMPointReadOnly(
            self_: <&DomPointReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomPointReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_to_json_DOMPointReadOnly(self_)
            };
            <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomPointReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_x_DOMPointReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomPointReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomPointReadOnly {
    #[cfg(all(feature = "DomPointReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `x` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/x)\n\n*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    #[allow(clippy::all)]
    pub fn x(&self) -> f64 {
        #[cfg(all(feature = "DomPointReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_x_DOMPointReadOnly(
                self_: <&DomPointReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_x_DOMPointReadOnly(
            self_: <&DomPointReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomPointReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_x_DOMPointReadOnly(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomPointReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_y_DOMPointReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomPointReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomPointReadOnly {
    #[cfg(all(feature = "DomPointReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `y` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/y)\n\n*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    #[allow(clippy::all)]
    pub fn y(&self) -> f64 {
        #[cfg(all(feature = "DomPointReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_y_DOMPointReadOnly(
                self_: <&DomPointReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_y_DOMPointReadOnly(
            self_: <&DomPointReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomPointReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_y_DOMPointReadOnly(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomPointReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_z_DOMPointReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomPointReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomPointReadOnly {
    #[cfg(all(feature = "DomPointReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `z` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/z)\n\n*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    #[allow(clippy::all)]
    pub fn z(&self) -> f64 {
        #[cfg(all(feature = "DomPointReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_z_DOMPointReadOnly(
                self_: <&DomPointReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_z_DOMPointReadOnly(
            self_: <&DomPointReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomPointReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_z_DOMPointReadOnly(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomPointReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_w_DOMPointReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomPointReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomPointReadOnly {
    #[cfg(all(feature = "DomPointReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `w` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/w)\n\n*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    #[allow(clippy::all)]
    pub fn w(&self) -> f64 {
        #[cfg(all(feature = "DomPointReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_w_DOMPointReadOnly(
                self_: <&DomPointReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_w_DOMPointReadOnly(
            self_: <&DomPointReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomPointReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_w_DOMPointReadOnly(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_91a80918bc4e659e: [u8; 1063usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xE5\x03\0\0\0\0\r\0\0\x02\x10DOMPointReadOnly\"__widl_instanceof_DOMPointReadOnly\0\0\0\0\x1D__widl_f_new_DOMPointReadOnly\x01\0\0\x01\x10DOMPointReadOnly\0\x01\0\x03new\0\0\0$__widl_f_new_with_x_DOMPointReadOnly\x01\0\0\x01\x10DOMPointReadOnly\0\x01\x01\x01x\x03new\0\0\0*__widl_f_new_with_x_and_y_DOMPointReadOnly\x01\0\0\x01\x10DOMPointReadOnly\0\x01\x02\x01x\x01y\x03new\0\0\00__widl_f_new_with_x_and_y_and_z_DOMPointReadOnly\x01\0\0\x01\x10DOMPointReadOnly\0\x01\x03\x01x\x01y\x01z\x03new\0\0\06__widl_f_new_with_x_and_y_and_z_and_w_DOMPointReadOnly\x01\0\0\x01\x10DOMPointReadOnly\0\x01\x04\x01x\x01y\x01z\x01w\x03new\0\0\0$__widl_f_from_point_DOMPointReadOnly\0\0\0\x01\x10DOMPointReadOnly\x01\x01\0\x01\0\tfromPoint\0\0\0/__widl_f_from_point_with_other_DOMPointReadOnly\0\0\0\x01\x10DOMPointReadOnly\x01\x01\0\x01\x01\x05other\tfromPoint\0\0\0!__widl_f_to_json_DOMPointReadOnly\0\0\0\x01\x10DOMPointReadOnly\x01\0\0\x01\x01\x05self_\x06toJSON\0\0\0\x1B__widl_f_x_DOMPointReadOnly\0\0\0\x01\x10DOMPointReadOnly\x01\0\x01\x01x\x01\x01\x05self_\x01x\0\0\0\x1B__widl_f_y_DOMPointReadOnly\0\0\0\x01\x10DOMPointReadOnly\x01\0\x01\x01y\x01\x01\x05self_\x01y\0\0\0\x1B__widl_f_z_DOMPointReadOnly\0\0\0\x01\x10DOMPointReadOnly\x01\0\x01\x01z\x01\x01\x05self_\x01z\0\0\0\x1B__widl_f_w_DOMPointReadOnly\0\0\0\x01\x10DOMPointReadOnly\x01\0\x01\x01w\x01\x01\x05self_\x01w\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
