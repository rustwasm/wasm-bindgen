use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `DOMRect` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect)\n\n*This API requires the following crate features to be activated: `DomRect`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct DomRect {
    obj: DomRectReadOnly,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_DomRect: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for DomRect {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(7u32);
            inform(68u32);
            inform(79u32);
            inform(77u32);
            inform(82u32);
            inform(101u32);
            inform(99u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for DomRect {
        type Target = DomRectReadOnly;
        #[inline]
        fn deref(&self) -> &DomRectReadOnly {
            &self.obj
        }
    }
    impl IntoWasmAbi for DomRect {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for DomRect {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a DomRect {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for DomRect {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            DomRect {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for DomRect {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a DomRect {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for DomRect {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<DomRect>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(DomRect {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for DomRect {
        #[inline]
        fn from(obj: JsValue) -> DomRect {
            DomRect { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for DomRect {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<DomRect> for DomRect {
        #[inline]
        fn as_ref(&self) -> &DomRect {
            self
        }
    }
    impl From<DomRect> for JsValue {
        #[inline]
        fn from(obj: DomRect) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for DomRect {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_DOMRect(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_DOMRect(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_DOMRect(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            DomRect { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const DomRect) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<DomRect> for DomRectReadOnly {
    #[inline]
    fn from(obj: DomRect) -> DomRectReadOnly {
        use wasm_bindgen::JsCast;
        DomRectReadOnly::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<DomRectReadOnly> for DomRect {
    #[inline]
    fn as_ref(&self) -> &DomRectReadOnly {
        use wasm_bindgen::JsCast;
        DomRectReadOnly::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<DomRect> for ::js_sys::Object {
    #[inline]
    fn from(obj: DomRect) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for DomRect {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "DomRect",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_DOMRect() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <DomRect as WasmDescribe>::describe();
}
impl DomRect {
    #[cfg(all(feature = "DomRect",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMRect(..)` constructor, creating a new instance of `DOMRect`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/DOMRect)\n\n*This API requires the following crate features to be activated: `DomRect`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<DomRect, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomRect",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_DOMRect() -> <DomRect as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_DOMRect() -> <DomRect as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_DOMRect() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomRect as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomRect",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_x_DOMRect() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <f64 as WasmDescribe>::describe();
    <DomRect as WasmDescribe>::describe();
}
impl DomRect {
    #[cfg(all(feature = "DomRect",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMRect(..)` constructor, creating a new instance of `DOMRect`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/DOMRect)\n\n*This API requires the following crate features to be activated: `DomRect`*"]
    #[allow(clippy::all)]
    pub fn new_with_x(x: f64) -> Result<DomRect, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomRect",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_x_DOMRect(
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomRect as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_x_DOMRect(
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomRect as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(x);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                __widl_f_new_with_x_DOMRect(x)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomRect as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomRect",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_x_and_y_DOMRect() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomRect as WasmDescribe>::describe();
}
impl DomRect {
    #[cfg(all(feature = "DomRect",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMRect(..)` constructor, creating a new instance of `DOMRect`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/DOMRect)\n\n*This API requires the following crate features to be activated: `DomRect`*"]
    #[allow(clippy::all)]
    pub fn new_with_x_and_y(x: f64, y: f64) -> Result<DomRect, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomRect",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_x_and_y_DOMRect(
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomRect as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_x_and_y_DOMRect(
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomRect as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                __widl_f_new_with_x_and_y_DOMRect(x, y)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomRect as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomRect",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_x_and_y_and_width_DOMRect() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomRect as WasmDescribe>::describe();
}
impl DomRect {
    #[cfg(all(feature = "DomRect",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMRect(..)` constructor, creating a new instance of `DOMRect`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/DOMRect)\n\n*This API requires the following crate features to be activated: `DomRect`*"]
    #[allow(clippy::all)]
    pub fn new_with_x_and_y_and_width(
        x: f64,
        y: f64,
        width: f64,
    ) -> Result<DomRect, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomRect",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_x_and_y_and_width_DOMRect(
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomRect as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_x_and_y_and_width_DOMRect(
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            width: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomRect as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                __widl_f_new_with_x_and_y_and_width_DOMRect(x, y, width)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomRect as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomRect",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_x_and_y_and_width_and_height_DOMRect() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomRect as WasmDescribe>::describe();
}
impl DomRect {
    #[cfg(all(feature = "DomRect",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMRect(..)` constructor, creating a new instance of `DOMRect`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/DOMRect)\n\n*This API requires the following crate features to be activated: `DomRect`*"]
    #[allow(clippy::all)]
    pub fn new_with_x_and_y_and_width_and_height(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> Result<DomRect, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomRect",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_x_and_y_and_width_and_height_DOMRect(
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                height: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomRect as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_x_and_y_and_width_and_height_DOMRect(
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            width: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            height: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomRect as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                __widl_f_new_with_x_and_y_and_width_and_height_DOMRect(x, y, width, height)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomRect as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomRect",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_x_DOMRect() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomRect as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomRect {
    #[cfg(all(feature = "DomRect",))]
    #[allow(bad_style)]
    #[doc = "The `x` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/x)\n\n*This API requires the following crate features to be activated: `DomRect`*"]
    #[allow(clippy::all)]
    pub fn x(&self) -> f64 {
        #[cfg(all(feature = "DomRect",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_x_DOMRect(
                self_: <&DomRect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_x_DOMRect(
            self_: <&DomRect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomRect as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_x_DOMRect(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomRect",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_x_DOMRect() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomRect as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomRect {
    #[cfg(all(feature = "DomRect",))]
    #[allow(bad_style)]
    #[doc = "The `x` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/x)\n\n*This API requires the following crate features to be activated: `DomRect`*"]
    #[allow(clippy::all)]
    pub fn set_x(&self, x: f64) {
        #[cfg(all(feature = "DomRect",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_x_DOMRect(
                self_: <&DomRect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_x_DOMRect(
            self_: <&DomRect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DomRect as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                __widl_f_set_x_DOMRect(self_, x)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomRect",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_y_DOMRect() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomRect as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomRect {
    #[cfg(all(feature = "DomRect",))]
    #[allow(bad_style)]
    #[doc = "The `y` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/y)\n\n*This API requires the following crate features to be activated: `DomRect`*"]
    #[allow(clippy::all)]
    pub fn y(&self) -> f64 {
        #[cfg(all(feature = "DomRect",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_y_DOMRect(
                self_: <&DomRect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_y_DOMRect(
            self_: <&DomRect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomRect as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_y_DOMRect(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomRect",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_y_DOMRect() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomRect as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomRect {
    #[cfg(all(feature = "DomRect",))]
    #[allow(bad_style)]
    #[doc = "The `y` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/y)\n\n*This API requires the following crate features to be activated: `DomRect`*"]
    #[allow(clippy::all)]
    pub fn set_y(&self, y: f64) {
        #[cfg(all(feature = "DomRect",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_y_DOMRect(
                self_: <&DomRect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_y_DOMRect(
            self_: <&DomRect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DomRect as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_set_y_DOMRect(self_, y)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomRect",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_width_DOMRect() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomRect as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomRect {
    #[cfg(all(feature = "DomRect",))]
    #[allow(bad_style)]
    #[doc = "The `width` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/width)\n\n*This API requires the following crate features to be activated: `DomRect`*"]
    #[allow(clippy::all)]
    pub fn width(&self) -> f64 {
        #[cfg(all(feature = "DomRect",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_width_DOMRect(
                self_: <&DomRect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_width_DOMRect(
            self_: <&DomRect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomRect as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_width_DOMRect(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomRect",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_width_DOMRect() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomRect as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomRect {
    #[cfg(all(feature = "DomRect",))]
    #[allow(bad_style)]
    #[doc = "The `width` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/width)\n\n*This API requires the following crate features to be activated: `DomRect`*"]
    #[allow(clippy::all)]
    pub fn set_width(&self, width: f64) {
        #[cfg(all(feature = "DomRect",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_width_DOMRect(
                self_: <&DomRect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_width_DOMRect(
            self_: <&DomRect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            width: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(width);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomRect as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let width = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                __widl_f_set_width_DOMRect(self_, width)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomRect",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_height_DOMRect() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomRect as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomRect {
    #[cfg(all(feature = "DomRect",))]
    #[allow(bad_style)]
    #[doc = "The `height` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/height)\n\n*This API requires the following crate features to be activated: `DomRect`*"]
    #[allow(clippy::all)]
    pub fn height(&self) -> f64 {
        #[cfg(all(feature = "DomRect",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_height_DOMRect(
                self_: <&DomRect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_height_DOMRect(
            self_: <&DomRect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomRect as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_height_DOMRect(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomRect",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_height_DOMRect() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomRect as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomRect {
    #[cfg(all(feature = "DomRect",))]
    #[allow(bad_style)]
    #[doc = "The `height` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/height)\n\n*This API requires the following crate features to be activated: `DomRect`*"]
    #[allow(clippy::all)]
    pub fn set_height(&self, height: f64) {
        #[cfg(all(feature = "DomRect",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_height_DOMRect(
                self_: <&DomRect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                height: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_height_DOMRect(
            self_: <&DomRect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            height: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(height);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomRect as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let height = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(height);
                __widl_f_set_height_DOMRect(self_, height)
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
pub static __WASM_BINDGEN_GENERATED_c35c0411e39c8218: [u8; 947usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}q\x03\0\0\0\0\x0E\0\0\x02\x07DOMRect\x19__widl_instanceof_DOMRect\0\0\0\0\x14__widl_f_new_DOMRect\x01\0\0\x01\x07DOMRect\0\x01\0\x03new\0\0\0\x1B__widl_f_new_with_x_DOMRect\x01\0\0\x01\x07DOMRect\0\x01\x01\x01x\x03new\0\0\0!__widl_f_new_with_x_and_y_DOMRect\x01\0\0\x01\x07DOMRect\0\x01\x02\x01x\x01y\x03new\0\0\0+__widl_f_new_with_x_and_y_and_width_DOMRect\x01\0\0\x01\x07DOMRect\0\x01\x03\x01x\x01y\x05width\x03new\0\0\06__widl_f_new_with_x_and_y_and_width_and_height_DOMRect\x01\0\0\x01\x07DOMRect\0\x01\x04\x01x\x01y\x05width\x06height\x03new\0\0\0\x12__widl_f_x_DOMRect\0\0\0\x01\x07DOMRect\x01\0\x01\x01x\x01\x01\x05self_\x01x\0\0\0\x16__widl_f_set_x_DOMRect\0\0\0\x01\x07DOMRect\x01\0\x02\x01x\x01\x02\x05self_\x01x\x01x\0\0\0\x12__widl_f_y_DOMRect\0\0\0\x01\x07DOMRect\x01\0\x01\x01y\x01\x01\x05self_\x01y\0\0\0\x16__widl_f_set_y_DOMRect\0\0\0\x01\x07DOMRect\x01\0\x02\x01y\x01\x02\x05self_\x01y\x01y\0\0\0\x16__widl_f_width_DOMRect\0\0\0\x01\x07DOMRect\x01\0\x01\x05width\x01\x01\x05self_\x05width\0\0\0\x1A__widl_f_set_width_DOMRect\0\0\0\x01\x07DOMRect\x01\0\x02\x05width\x01\x02\x05self_\x05width\x05width\0\0\0\x17__widl_f_height_DOMRect\0\0\0\x01\x07DOMRect\x01\0\x01\x06height\x01\x01\x05self_\x06height\0\0\0\x1B__widl_f_set_height_DOMRect\0\0\0\x01\x07DOMRect\x01\0\x02\x06height\x01\x02\x05self_\x06height\x06height\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
