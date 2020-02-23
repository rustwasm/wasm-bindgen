use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `DOMRectReadOnly` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct DomRectReadOnly {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_DomRectReadOnly: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for DomRectReadOnly {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(68u32);
            inform(79u32);
            inform(77u32);
            inform(82u32);
            inform(101u32);
            inform(99u32);
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
    impl core::ops::Deref for DomRectReadOnly {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for DomRectReadOnly {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for DomRectReadOnly {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a DomRectReadOnly {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for DomRectReadOnly {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            DomRectReadOnly {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for DomRectReadOnly {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a DomRectReadOnly {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for DomRectReadOnly {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<DomRectReadOnly>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(DomRectReadOnly {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for DomRectReadOnly {
        #[inline]
        fn from(obj: JsValue) -> DomRectReadOnly {
            DomRectReadOnly { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for DomRectReadOnly {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<DomRectReadOnly> for DomRectReadOnly {
        #[inline]
        fn as_ref(&self) -> &DomRectReadOnly {
            self
        }
    }
    impl From<DomRectReadOnly> for JsValue {
        #[inline]
        fn from(obj: DomRectReadOnly) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for DomRectReadOnly {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_DOMRectReadOnly(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_DOMRectReadOnly(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_DOMRectReadOnly(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            DomRectReadOnly { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const DomRectReadOnly) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<DomRectReadOnly> for ::js_sys::Object {
    #[inline]
    fn from(obj: DomRectReadOnly) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for DomRectReadOnly {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "DomRectReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_DOMRectReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <DomRectReadOnly as WasmDescribe>::describe();
}
impl DomRectReadOnly {
    #[cfg(all(feature = "DomRectReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMRectReadOnly(..)` constructor, creating a new instance of `DOMRectReadOnly`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/DOMRectReadOnly)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<DomRectReadOnly, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomRectReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_DOMRectReadOnly(
            ) -> <DomRectReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_DOMRectReadOnly(
        ) -> <DomRectReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_DOMRectReadOnly() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomRectReadOnly as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DomRectReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_x_DOMRectReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <f64 as WasmDescribe>::describe();
    <DomRectReadOnly as WasmDescribe>::describe();
}
impl DomRectReadOnly {
    #[cfg(all(feature = "DomRectReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMRectReadOnly(..)` constructor, creating a new instance of `DOMRectReadOnly`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/DOMRectReadOnly)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    #[allow(clippy::all)]
    pub fn new_with_x(x: f64) -> Result<DomRectReadOnly, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomRectReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_x_DOMRectReadOnly(
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomRectReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_x_DOMRectReadOnly(
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomRectReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(x);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                __widl_f_new_with_x_DOMRectReadOnly(x)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomRectReadOnly as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DomRectReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_x_and_y_DOMRectReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomRectReadOnly as WasmDescribe>::describe();
}
impl DomRectReadOnly {
    #[cfg(all(feature = "DomRectReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMRectReadOnly(..)` constructor, creating a new instance of `DOMRectReadOnly`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/DOMRectReadOnly)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    #[allow(clippy::all)]
    pub fn new_with_x_and_y(x: f64, y: f64) -> Result<DomRectReadOnly, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomRectReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_x_and_y_DOMRectReadOnly(
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomRectReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_x_and_y_DOMRectReadOnly(
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomRectReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                __widl_f_new_with_x_and_y_DOMRectReadOnly(x, y)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomRectReadOnly as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DomRectReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_x_and_y_and_width_DOMRectReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomRectReadOnly as WasmDescribe>::describe();
}
impl DomRectReadOnly {
    #[cfg(all(feature = "DomRectReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMRectReadOnly(..)` constructor, creating a new instance of `DOMRectReadOnly`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/DOMRectReadOnly)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    #[allow(clippy::all)]
    pub fn new_with_x_and_y_and_width(
        x: f64,
        y: f64,
        width: f64,
    ) -> Result<DomRectReadOnly, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomRectReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_x_and_y_and_width_DOMRectReadOnly(
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomRectReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_x_and_y_and_width_DOMRectReadOnly(
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            width: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomRectReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(x);
            drop(y);
            drop(width);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let width = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                __widl_f_new_with_x_and_y_and_width_DOMRectReadOnly(x, y, width)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomRectReadOnly as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DomRectReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_x_and_y_and_width_and_height_DOMRectReadOnly(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomRectReadOnly as WasmDescribe>::describe();
}
impl DomRectReadOnly {
    #[cfg(all(feature = "DomRectReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMRectReadOnly(..)` constructor, creating a new instance of `DOMRectReadOnly`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/DOMRectReadOnly)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    #[allow(clippy::all)]
    pub fn new_with_x_and_y_and_width_and_height(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> Result<DomRectReadOnly, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomRectReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_x_and_y_and_width_and_height_DOMRectReadOnly(
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                height: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomRectReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_x_and_y_and_width_and_height_DOMRectReadOnly(
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            width: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            height: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomRectReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(x);
            drop(y);
            drop(width);
            drop(height);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let width = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                let height = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(height);
                __widl_f_new_with_x_and_y_and_width_and_height_DOMRectReadOnly(x, y, width, height)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomRectReadOnly as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DomRectReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_to_json_DOMRectReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomRectReadOnly as WasmDescribe>::describe();
    <::js_sys::Object as WasmDescribe>::describe();
}
impl DomRectReadOnly {
    #[cfg(all(feature = "DomRectReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `toJSON()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/toJSON)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    #[allow(clippy::all)]
    pub fn to_json(&self) -> ::js_sys::Object {
        #[cfg(all(feature = "DomRectReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_to_json_DOMRectReadOnly(
                self_: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_to_json_DOMRectReadOnly(
            self_: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_to_json_DOMRectReadOnly(self_)
            };
            <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomRectReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_x_DOMRectReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomRectReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomRectReadOnly {
    #[cfg(all(feature = "DomRectReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `x` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/x)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    #[allow(clippy::all)]
    pub fn x(&self) -> f64 {
        #[cfg(all(feature = "DomRectReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_x_DOMRectReadOnly(
                self_: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_x_DOMRectReadOnly(
            self_: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_x_DOMRectReadOnly(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomRectReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_y_DOMRectReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomRectReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomRectReadOnly {
    #[cfg(all(feature = "DomRectReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `y` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/y)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    #[allow(clippy::all)]
    pub fn y(&self) -> f64 {
        #[cfg(all(feature = "DomRectReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_y_DOMRectReadOnly(
                self_: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_y_DOMRectReadOnly(
            self_: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_y_DOMRectReadOnly(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomRectReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_width_DOMRectReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomRectReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomRectReadOnly {
    #[cfg(all(feature = "DomRectReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `width` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/width)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    #[allow(clippy::all)]
    pub fn width(&self) -> f64 {
        #[cfg(all(feature = "DomRectReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_width_DOMRectReadOnly(
                self_: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_width_DOMRectReadOnly(
            self_: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_width_DOMRectReadOnly(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomRectReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_height_DOMRectReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomRectReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomRectReadOnly {
    #[cfg(all(feature = "DomRectReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `height` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/height)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    #[allow(clippy::all)]
    pub fn height(&self) -> f64 {
        #[cfg(all(feature = "DomRectReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_height_DOMRectReadOnly(
                self_: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_height_DOMRectReadOnly(
            self_: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_height_DOMRectReadOnly(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomRectReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_top_DOMRectReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomRectReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomRectReadOnly {
    #[cfg(all(feature = "DomRectReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `top` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/top)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    #[allow(clippy::all)]
    pub fn top(&self) -> f64 {
        #[cfg(all(feature = "DomRectReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_top_DOMRectReadOnly(
                self_: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_top_DOMRectReadOnly(
            self_: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_top_DOMRectReadOnly(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomRectReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_right_DOMRectReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomRectReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomRectReadOnly {
    #[cfg(all(feature = "DomRectReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `right` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/right)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    #[allow(clippy::all)]
    pub fn right(&self) -> f64 {
        #[cfg(all(feature = "DomRectReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_right_DOMRectReadOnly(
                self_: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_right_DOMRectReadOnly(
            self_: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_right_DOMRectReadOnly(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomRectReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_bottom_DOMRectReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomRectReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomRectReadOnly {
    #[cfg(all(feature = "DomRectReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `bottom` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/bottom)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    #[allow(clippy::all)]
    pub fn bottom(&self) -> f64 {
        #[cfg(all(feature = "DomRectReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_bottom_DOMRectReadOnly(
                self_: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_bottom_DOMRectReadOnly(
            self_: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_bottom_DOMRectReadOnly(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomRectReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_left_DOMRectReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomRectReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomRectReadOnly {
    #[cfg(all(feature = "DomRectReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `left` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/left)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    #[allow(clippy::all)]
    pub fn left(&self) -> f64 {
        #[cfg(all(feature = "DomRectReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_left_DOMRectReadOnly(
                self_: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_left_DOMRectReadOnly(
            self_: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_left_DOMRectReadOnly(self_)
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
pub static __WASM_BINDGEN_GENERATED_3a6f15b298946a41: [u8; 1227usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x89\x04\0\0\0\0\x0F\0\0\x02\x0FDOMRectReadOnly!__widl_instanceof_DOMRectReadOnly\0\0\0\0\x1C__widl_f_new_DOMRectReadOnly\x01\0\0\x01\x0FDOMRectReadOnly\0\x01\0\x03new\0\0\0#__widl_f_new_with_x_DOMRectReadOnly\x01\0\0\x01\x0FDOMRectReadOnly\0\x01\x01\x01x\x03new\0\0\0)__widl_f_new_with_x_and_y_DOMRectReadOnly\x01\0\0\x01\x0FDOMRectReadOnly\0\x01\x02\x01x\x01y\x03new\0\0\03__widl_f_new_with_x_and_y_and_width_DOMRectReadOnly\x01\0\0\x01\x0FDOMRectReadOnly\0\x01\x03\x01x\x01y\x05width\x03new\0\0\0>__widl_f_new_with_x_and_y_and_width_and_height_DOMRectReadOnly\x01\0\0\x01\x0FDOMRectReadOnly\0\x01\x04\x01x\x01y\x05width\x06height\x03new\0\0\0 __widl_f_to_json_DOMRectReadOnly\0\0\0\x01\x0FDOMRectReadOnly\x01\0\0\x01\x01\x05self_\x06toJSON\0\0\0\x1A__widl_f_x_DOMRectReadOnly\0\0\0\x01\x0FDOMRectReadOnly\x01\0\x01\x01x\x01\x01\x05self_\x01x\0\0\0\x1A__widl_f_y_DOMRectReadOnly\0\0\0\x01\x0FDOMRectReadOnly\x01\0\x01\x01y\x01\x01\x05self_\x01y\0\0\0\x1E__widl_f_width_DOMRectReadOnly\0\0\0\x01\x0FDOMRectReadOnly\x01\0\x01\x05width\x01\x01\x05self_\x05width\0\0\0\x1F__widl_f_height_DOMRectReadOnly\0\0\0\x01\x0FDOMRectReadOnly\x01\0\x01\x06height\x01\x01\x05self_\x06height\0\0\0\x1C__widl_f_top_DOMRectReadOnly\0\0\0\x01\x0FDOMRectReadOnly\x01\0\x01\x03top\x01\x01\x05self_\x03top\0\0\0\x1E__widl_f_right_DOMRectReadOnly\0\0\0\x01\x0FDOMRectReadOnly\x01\0\x01\x05right\x01\x01\x05self_\x05right\0\0\0\x1F__widl_f_bottom_DOMRectReadOnly\0\0\0\x01\x0FDOMRectReadOnly\x01\0\x01\x06bottom\x01\x01\x05self_\x06bottom\0\0\0\x1D__widl_f_left_DOMRectReadOnly\0\0\0\x01\x0FDOMRectReadOnly\x01\0\x01\x04left\x01\x01\x05self_\x04left\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
