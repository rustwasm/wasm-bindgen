use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `WebKitCSSMatrix` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct WebKitCssMatrix {
    obj: DomMatrix,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_WebKitCssMatrix: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for WebKitCssMatrix {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(87u32);
            inform(101u32);
            inform(98u32);
            inform(75u32);
            inform(105u32);
            inform(116u32);
            inform(67u32);
            inform(83u32);
            inform(83u32);
            inform(77u32);
            inform(97u32);
            inform(116u32);
            inform(114u32);
            inform(105u32);
            inform(120u32);
        }
    }
    impl core::ops::Deref for WebKitCssMatrix {
        type Target = DomMatrix;
        #[inline]
        fn deref(&self) -> &DomMatrix {
            &self.obj
        }
    }
    impl IntoWasmAbi for WebKitCssMatrix {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for WebKitCssMatrix {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a WebKitCssMatrix {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for WebKitCssMatrix {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            WebKitCssMatrix {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for WebKitCssMatrix {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a WebKitCssMatrix {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for WebKitCssMatrix {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<WebKitCssMatrix>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(WebKitCssMatrix {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for WebKitCssMatrix {
        #[inline]
        fn from(obj: JsValue) -> WebKitCssMatrix {
            WebKitCssMatrix { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for WebKitCssMatrix {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<WebKitCssMatrix> for WebKitCssMatrix {
        #[inline]
        fn as_ref(&self) -> &WebKitCssMatrix {
            self
        }
    }
    impl From<WebKitCssMatrix> for JsValue {
        #[inline]
        fn from(obj: WebKitCssMatrix) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for WebKitCssMatrix {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_WebKitCSSMatrix(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_WebKitCSSMatrix(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_WebKitCSSMatrix(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            WebKitCssMatrix { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const WebKitCssMatrix) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<WebKitCssMatrix> for DomMatrix {
    #[inline]
    fn from(obj: WebKitCssMatrix) -> DomMatrix {
        use wasm_bindgen::JsCast;
        DomMatrix::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<DomMatrix> for WebKitCssMatrix {
    #[inline]
    fn as_ref(&self) -> &DomMatrix {
        use wasm_bindgen::JsCast;
        DomMatrix::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<WebKitCssMatrix> for DomMatrixReadOnly {
    #[inline]
    fn from(obj: WebKitCssMatrix) -> DomMatrixReadOnly {
        use wasm_bindgen::JsCast;
        DomMatrixReadOnly::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<DomMatrixReadOnly> for WebKitCssMatrix {
    #[inline]
    fn as_ref(&self) -> &DomMatrixReadOnly {
        use wasm_bindgen::JsCast;
        DomMatrixReadOnly::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<WebKitCssMatrix> for ::js_sys::Object {
    #[inline]
    fn from(obj: WebKitCssMatrix) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for WebKitCssMatrix {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "WebKitCssMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_WebKitCSSMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <WebKitCssMatrix as WasmDescribe>::describe();
}
impl WebKitCssMatrix {
    #[cfg(all(feature = "WebKitCssMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `new WebKitCSSMatrix(..)` constructor, creating a new instance of `WebKitCSSMatrix`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/WebKitCSSMatrix)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<WebKitCssMatrix, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WebKitCssMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_WebKitCSSMatrix(
            ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_WebKitCSSMatrix(
        ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_WebKitCSSMatrix() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WebKitCssMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_transform_list_WebKitCSSMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <WebKitCssMatrix as WasmDescribe>::describe();
}
impl WebKitCssMatrix {
    #[cfg(all(feature = "WebKitCssMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `new WebKitCSSMatrix(..)` constructor, creating a new instance of `WebKitCSSMatrix`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/WebKitCSSMatrix)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    #[allow(clippy::all)]
    pub fn new_with_transform_list(
        transform_list: &str,
    ) -> Result<WebKitCssMatrix, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WebKitCssMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_transform_list_WebKitCSSMatrix(
                transform_list: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_transform_list_WebKitCSSMatrix(
            transform_list: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(transform_list);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let transform_list =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(transform_list);
                __widl_f_new_with_transform_list_WebKitCSSMatrix(transform_list)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WebKitCssMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_other_WebKitCSSMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebKitCssMatrix as WasmDescribe>::describe();
    <WebKitCssMatrix as WasmDescribe>::describe();
}
impl WebKitCssMatrix {
    #[cfg(all(feature = "WebKitCssMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `new WebKitCSSMatrix(..)` constructor, creating a new instance of `WebKitCSSMatrix`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/WebKitCSSMatrix)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    #[allow(clippy::all)]
    pub fn new_with_other(
        other: &WebKitCssMatrix,
    ) -> Result<WebKitCssMatrix, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WebKitCssMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_other_WebKitCSSMatrix(
                other: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_other_WebKitCSSMatrix(
            other: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(other);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let other =
                    <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(other);
                __widl_f_new_with_other_WebKitCSSMatrix(other)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WebKitCssMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_inverse_WebKitCSSMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebKitCssMatrix as WasmDescribe>::describe();
    <WebKitCssMatrix as WasmDescribe>::describe();
}
impl WebKitCssMatrix {
    #[cfg(all(feature = "WebKitCssMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `inverse()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/inverse)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    #[allow(clippy::all)]
    pub fn inverse(&self) -> Result<WebKitCssMatrix, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WebKitCssMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_inverse_WebKitCSSMatrix(
                self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_inverse_WebKitCSSMatrix(
            self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_inverse_WebKitCSSMatrix(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WebKitCssMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_multiply_WebKitCSSMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebKitCssMatrix as WasmDescribe>::describe();
    <&WebKitCssMatrix as WasmDescribe>::describe();
    <WebKitCssMatrix as WasmDescribe>::describe();
}
impl WebKitCssMatrix {
    #[cfg(all(feature = "WebKitCssMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `multiply()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/multiply)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    #[allow(clippy::all)]
    pub fn multiply(&self, other: &WebKitCssMatrix) -> WebKitCssMatrix {
        #[cfg(all(feature = "WebKitCssMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_multiply_WebKitCSSMatrix(
                self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                other: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_multiply_WebKitCSSMatrix(
            self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            other: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(other);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let other =
                    <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(other);
                __widl_f_multiply_WebKitCSSMatrix(self_, other)
            };
            <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebKitCssMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rotate_WebKitCSSMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebKitCssMatrix as WasmDescribe>::describe();
    <WebKitCssMatrix as WasmDescribe>::describe();
}
impl WebKitCssMatrix {
    #[cfg(all(feature = "WebKitCssMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `rotate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotate)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    #[allow(clippy::all)]
    pub fn rotate(&self) -> WebKitCssMatrix {
        #[cfg(all(feature = "WebKitCssMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rotate_WebKitCSSMatrix(
                self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rotate_WebKitCSSMatrix(
            self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_rotate_WebKitCSSMatrix(self_)
            };
            <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebKitCssMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rotate_with_rot_x_WebKitCSSMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebKitCssMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <WebKitCssMatrix as WasmDescribe>::describe();
}
impl WebKitCssMatrix {
    #[cfg(all(feature = "WebKitCssMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `rotate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotate)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    #[allow(clippy::all)]
    pub fn rotate_with_rot_x(&self, rot_x: f64) -> WebKitCssMatrix {
        #[cfg(all(feature = "WebKitCssMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rotate_with_rot_x_WebKitCSSMatrix(
                self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rot_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rotate_with_rot_x_WebKitCSSMatrix(
            self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rot_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(rot_x);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rot_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rot_x);
                __widl_f_rotate_with_rot_x_WebKitCSSMatrix(self_, rot_x)
            };
            <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebKitCssMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rotate_with_rot_x_and_rot_y_WebKitCSSMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebKitCssMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <WebKitCssMatrix as WasmDescribe>::describe();
}
impl WebKitCssMatrix {
    #[cfg(all(feature = "WebKitCssMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `rotate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotate)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    #[allow(clippy::all)]
    pub fn rotate_with_rot_x_and_rot_y(&self, rot_x: f64, rot_y: f64) -> WebKitCssMatrix {
        #[cfg(all(feature = "WebKitCssMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rotate_with_rot_x_and_rot_y_WebKitCSSMatrix(
                self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rot_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rot_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rotate_with_rot_x_and_rot_y_WebKitCSSMatrix(
            self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rot_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rot_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(rot_x);
            drop(rot_y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rot_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rot_x);
                let rot_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rot_y);
                __widl_f_rotate_with_rot_x_and_rot_y_WebKitCSSMatrix(self_, rot_x, rot_y)
            };
            <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebKitCssMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rotate_with_rot_x_and_rot_y_and_rot_z_WebKitCSSMatrix(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WebKitCssMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <WebKitCssMatrix as WasmDescribe>::describe();
}
impl WebKitCssMatrix {
    #[cfg(all(feature = "WebKitCssMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `rotate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotate)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    #[allow(clippy::all)]
    pub fn rotate_with_rot_x_and_rot_y_and_rot_z(
        &self,
        rot_x: f64,
        rot_y: f64,
        rot_z: f64,
    ) -> WebKitCssMatrix {
        #[cfg(all(feature = "WebKitCssMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rotate_with_rot_x_and_rot_y_and_rot_z_WebKitCSSMatrix(
                self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rot_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rot_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rot_z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rotate_with_rot_x_and_rot_y_and_rot_z_WebKitCSSMatrix(
            self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rot_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rot_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rot_z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(rot_x);
            drop(rot_y);
            drop(rot_z);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rot_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rot_x);
                let rot_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rot_y);
                let rot_z = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rot_z);
                __widl_f_rotate_with_rot_x_and_rot_y_and_rot_z_WebKitCSSMatrix(
                    self_, rot_x, rot_y, rot_z,
                )
            };
            <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebKitCssMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rotate_axis_angle_WebKitCSSMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebKitCssMatrix as WasmDescribe>::describe();
    <WebKitCssMatrix as WasmDescribe>::describe();
}
impl WebKitCssMatrix {
    #[cfg(all(feature = "WebKitCssMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `rotateAxisAngle()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotateAxisAngle)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    #[allow(clippy::all)]
    pub fn rotate_axis_angle(&self) -> WebKitCssMatrix {
        #[cfg(all(feature = "WebKitCssMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rotate_axis_angle_WebKitCSSMatrix(
                self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rotate_axis_angle_WebKitCSSMatrix(
            self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_rotate_axis_angle_WebKitCSSMatrix(self_)
            };
            <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebKitCssMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rotate_axis_angle_with_x_WebKitCSSMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebKitCssMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <WebKitCssMatrix as WasmDescribe>::describe();
}
impl WebKitCssMatrix {
    #[cfg(all(feature = "WebKitCssMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `rotateAxisAngle()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotateAxisAngle)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    #[allow(clippy::all)]
    pub fn rotate_axis_angle_with_x(&self, x: f64) -> WebKitCssMatrix {
        #[cfg(all(feature = "WebKitCssMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rotate_axis_angle_with_x_WebKitCSSMatrix(
                self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rotate_axis_angle_with_x_WebKitCSSMatrix(
            self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                __widl_f_rotate_axis_angle_with_x_WebKitCSSMatrix(self_, x)
            };
            <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebKitCssMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rotate_axis_angle_with_x_and_y_WebKitCSSMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebKitCssMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <WebKitCssMatrix as WasmDescribe>::describe();
}
impl WebKitCssMatrix {
    #[cfg(all(feature = "WebKitCssMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `rotateAxisAngle()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotateAxisAngle)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    #[allow(clippy::all)]
    pub fn rotate_axis_angle_with_x_and_y(&self, x: f64, y: f64) -> WebKitCssMatrix {
        #[cfg(all(feature = "WebKitCssMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rotate_axis_angle_with_x_and_y_WebKitCSSMatrix(
                self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rotate_axis_angle_with_x_and_y_WebKitCSSMatrix(
            self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                let self_ =
                    <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_rotate_axis_angle_with_x_and_y_WebKitCSSMatrix(self_, x, y)
            };
            <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebKitCssMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rotate_axis_angle_with_x_and_y_and_z_WebKitCSSMatrix(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WebKitCssMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <WebKitCssMatrix as WasmDescribe>::describe();
}
impl WebKitCssMatrix {
    #[cfg(all(feature = "WebKitCssMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `rotateAxisAngle()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotateAxisAngle)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    #[allow(clippy::all)]
    pub fn rotate_axis_angle_with_x_and_y_and_z(&self, x: f64, y: f64, z: f64) -> WebKitCssMatrix {
        #[cfg(all(feature = "WebKitCssMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rotate_axis_angle_with_x_and_y_and_z_WebKitCSSMatrix(
                self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rotate_axis_angle_with_x_and_y_and_z_WebKitCSSMatrix(
            self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
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
                let self_ =
                    <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let z = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(z);
                __widl_f_rotate_axis_angle_with_x_and_y_and_z_WebKitCSSMatrix(self_, x, y, z)
            };
            <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebKitCssMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rotate_axis_angle_with_x_and_y_and_z_and_angle_WebKitCSSMatrix(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&WebKitCssMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <WebKitCssMatrix as WasmDescribe>::describe();
}
impl WebKitCssMatrix {
    #[cfg(all(feature = "WebKitCssMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `rotateAxisAngle()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotateAxisAngle)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    #[allow(clippy::all)]
    pub fn rotate_axis_angle_with_x_and_y_and_z_and_angle(
        &self,
        x: f64,
        y: f64,
        z: f64,
        angle: f64,
    ) -> WebKitCssMatrix {
        #[cfg(all(feature = "WebKitCssMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rotate_axis_angle_with_x_and_y_and_z_and_angle_WebKitCSSMatrix(
                self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rotate_axis_angle_with_x_and_y_and_z_and_angle_WebKitCSSMatrix(
            self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(x);
            drop(y);
            drop(z);
            drop(angle);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let z = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(z);
                let angle = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(angle);
                __widl_f_rotate_axis_angle_with_x_and_y_and_z_and_angle_WebKitCSSMatrix(
                    self_, x, y, z, angle,
                )
            };
            <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebKitCssMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale_WebKitCSSMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebKitCssMatrix as WasmDescribe>::describe();
    <WebKitCssMatrix as WasmDescribe>::describe();
}
impl WebKitCssMatrix {
    #[cfg(all(feature = "WebKitCssMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `scale()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/scale)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    #[allow(clippy::all)]
    pub fn scale(&self) -> WebKitCssMatrix {
        #[cfg(all(feature = "WebKitCssMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale_WebKitCSSMatrix(
                self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale_WebKitCSSMatrix(
            self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_scale_WebKitCSSMatrix(self_)
            };
            <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebKitCssMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale_with_scale_x_WebKitCSSMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebKitCssMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <WebKitCssMatrix as WasmDescribe>::describe();
}
impl WebKitCssMatrix {
    #[cfg(all(feature = "WebKitCssMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `scale()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/scale)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    #[allow(clippy::all)]
    pub fn scale_with_scale_x(&self, scale_x: f64) -> WebKitCssMatrix {
        #[cfg(all(feature = "WebKitCssMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale_with_scale_x_WebKitCSSMatrix(
                self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale_with_scale_x_WebKitCSSMatrix(
            self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(scale_x);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scale_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_x);
                __widl_f_scale_with_scale_x_WebKitCSSMatrix(self_, scale_x)
            };
            <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebKitCssMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale_with_scale_x_and_scale_y_WebKitCSSMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebKitCssMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <WebKitCssMatrix as WasmDescribe>::describe();
}
impl WebKitCssMatrix {
    #[cfg(all(feature = "WebKitCssMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `scale()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/scale)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    #[allow(clippy::all)]
    pub fn scale_with_scale_x_and_scale_y(&self, scale_x: f64, scale_y: f64) -> WebKitCssMatrix {
        #[cfg(all(feature = "WebKitCssMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale_with_scale_x_and_scale_y_WebKitCSSMatrix(
                self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale_with_scale_x_and_scale_y_WebKitCSSMatrix(
            self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(scale_x);
            drop(scale_y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scale_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_x);
                let scale_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_y);
                __widl_f_scale_with_scale_x_and_scale_y_WebKitCSSMatrix(self_, scale_x, scale_y)
            };
            <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebKitCssMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale_with_scale_x_and_scale_y_and_scale_z_WebKitCSSMatrix(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WebKitCssMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <WebKitCssMatrix as WasmDescribe>::describe();
}
impl WebKitCssMatrix {
    #[cfg(all(feature = "WebKitCssMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `scale()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/scale)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    #[allow(clippy::all)]
    pub fn scale_with_scale_x_and_scale_y_and_scale_z(
        &self,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
    ) -> WebKitCssMatrix {
        #[cfg(all(feature = "WebKitCssMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale_with_scale_x_and_scale_y_and_scale_z_WebKitCSSMatrix(
                self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale_with_scale_x_and_scale_y_and_scale_z_WebKitCSSMatrix(
            self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(scale_x);
            drop(scale_y);
            drop(scale_z);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scale_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_x);
                let scale_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_y);
                let scale_z = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_z);
                __widl_f_scale_with_scale_x_and_scale_y_and_scale_z_WebKitCSSMatrix(
                    self_, scale_x, scale_y, scale_z,
                )
            };
            <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebKitCssMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_matrix_value_WebKitCSSMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebKitCssMatrix as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <WebKitCssMatrix as WasmDescribe>::describe();
}
impl WebKitCssMatrix {
    #[cfg(all(feature = "WebKitCssMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `setMatrixValue()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/setMatrixValue)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    #[allow(clippy::all)]
    pub fn set_matrix_value(
        &self,
        transform_list: &str,
    ) -> Result<WebKitCssMatrix, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WebKitCssMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_matrix_value_WebKitCSSMatrix(
                self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                transform_list: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_matrix_value_WebKitCSSMatrix(
            self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            transform_list: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(transform_list);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let transform_list =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(transform_list);
                __widl_f_set_matrix_value_WebKitCSSMatrix(self_, transform_list)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WebKitCssMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_skew_x_WebKitCSSMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebKitCssMatrix as WasmDescribe>::describe();
    <WebKitCssMatrix as WasmDescribe>::describe();
}
impl WebKitCssMatrix {
    #[cfg(all(feature = "WebKitCssMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `skewX()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/skewX)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    #[allow(clippy::all)]
    pub fn skew_x(&self) -> WebKitCssMatrix {
        #[cfg(all(feature = "WebKitCssMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_skew_x_WebKitCSSMatrix(
                self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_skew_x_WebKitCSSMatrix(
            self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_skew_x_WebKitCSSMatrix(self_)
            };
            <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebKitCssMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_skew_x_with_sx_WebKitCSSMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebKitCssMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <WebKitCssMatrix as WasmDescribe>::describe();
}
impl WebKitCssMatrix {
    #[cfg(all(feature = "WebKitCssMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `skewX()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/skewX)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    #[allow(clippy::all)]
    pub fn skew_x_with_sx(&self, sx: f64) -> WebKitCssMatrix {
        #[cfg(all(feature = "WebKitCssMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_skew_x_with_sx_WebKitCSSMatrix(
                self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_skew_x_with_sx_WebKitCSSMatrix(
            self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(sx);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let sx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sx);
                __widl_f_skew_x_with_sx_WebKitCSSMatrix(self_, sx)
            };
            <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebKitCssMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_skew_y_WebKitCSSMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebKitCssMatrix as WasmDescribe>::describe();
    <WebKitCssMatrix as WasmDescribe>::describe();
}
impl WebKitCssMatrix {
    #[cfg(all(feature = "WebKitCssMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `skewY()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/skewY)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    #[allow(clippy::all)]
    pub fn skew_y(&self) -> WebKitCssMatrix {
        #[cfg(all(feature = "WebKitCssMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_skew_y_WebKitCSSMatrix(
                self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_skew_y_WebKitCSSMatrix(
            self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_skew_y_WebKitCSSMatrix(self_)
            };
            <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebKitCssMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_skew_y_with_sy_WebKitCSSMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebKitCssMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <WebKitCssMatrix as WasmDescribe>::describe();
}
impl WebKitCssMatrix {
    #[cfg(all(feature = "WebKitCssMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `skewY()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/skewY)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    #[allow(clippy::all)]
    pub fn skew_y_with_sy(&self, sy: f64) -> WebKitCssMatrix {
        #[cfg(all(feature = "WebKitCssMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_skew_y_with_sy_WebKitCSSMatrix(
                self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_skew_y_with_sy_WebKitCSSMatrix(
            self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(sy);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let sy = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sy);
                __widl_f_skew_y_with_sy_WebKitCSSMatrix(self_, sy)
            };
            <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebKitCssMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_translate_WebKitCSSMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebKitCssMatrix as WasmDescribe>::describe();
    <WebKitCssMatrix as WasmDescribe>::describe();
}
impl WebKitCssMatrix {
    #[cfg(all(feature = "WebKitCssMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `translate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/translate)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    #[allow(clippy::all)]
    pub fn translate(&self) -> WebKitCssMatrix {
        #[cfg(all(feature = "WebKitCssMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_translate_WebKitCSSMatrix(
                self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_translate_WebKitCSSMatrix(
            self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_translate_WebKitCSSMatrix(self_)
            };
            <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebKitCssMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_translate_with_tx_WebKitCSSMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebKitCssMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <WebKitCssMatrix as WasmDescribe>::describe();
}
impl WebKitCssMatrix {
    #[cfg(all(feature = "WebKitCssMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `translate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/translate)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    #[allow(clippy::all)]
    pub fn translate_with_tx(&self, tx: f64) -> WebKitCssMatrix {
        #[cfg(all(feature = "WebKitCssMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_translate_with_tx_WebKitCSSMatrix(
                self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_translate_with_tx_WebKitCSSMatrix(
            self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(tx);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let tx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tx);
                __widl_f_translate_with_tx_WebKitCSSMatrix(self_, tx)
            };
            <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebKitCssMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_translate_with_tx_and_ty_WebKitCSSMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebKitCssMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <WebKitCssMatrix as WasmDescribe>::describe();
}
impl WebKitCssMatrix {
    #[cfg(all(feature = "WebKitCssMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `translate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/translate)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    #[allow(clippy::all)]
    pub fn translate_with_tx_and_ty(&self, tx: f64, ty: f64) -> WebKitCssMatrix {
        #[cfg(all(feature = "WebKitCssMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_translate_with_tx_and_ty_WebKitCSSMatrix(
                self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ty: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_translate_with_tx_and_ty_WebKitCSSMatrix(
            self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ty: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                let self_ =
                    <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let tx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tx);
                let ty = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ty);
                __widl_f_translate_with_tx_and_ty_WebKitCSSMatrix(self_, tx, ty)
            };
            <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebKitCssMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_translate_with_tx_and_ty_and_tz_WebKitCSSMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WebKitCssMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <WebKitCssMatrix as WasmDescribe>::describe();
}
impl WebKitCssMatrix {
    #[cfg(all(feature = "WebKitCssMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `translate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/translate)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    #[allow(clippy::all)]
    pub fn translate_with_tx_and_ty_and_tz(&self, tx: f64, ty: f64, tz: f64) -> WebKitCssMatrix {
        #[cfg(all(feature = "WebKitCssMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_translate_with_tx_and_ty_and_tz_WebKitCSSMatrix(
                self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ty: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tz: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_translate_with_tx_and_ty_and_tz_WebKitCSSMatrix(
            self_: <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ty: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tz: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(tx);
            drop(ty);
            drop(tz);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebKitCssMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let tx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tx);
                let ty = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ty);
                let tz = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tz);
                __widl_f_translate_with_tx_and_ty_and_tz_WebKitCSSMatrix(self_, tx, ty, tz)
            };
            <WebKitCssMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_ace79beb93996cd9: [u8; 2696usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}F\n\0\0\0\0\x1C\0\0\x02\x0FWebKitCSSMatrix!__widl_instanceof_WebKitCSSMatrix\0\0\0\0\x1C__widl_f_new_WebKitCSSMatrix\x01\0\0\x01\x0FWebKitCSSMatrix\0\x01\0\x03new\0\0\00__widl_f_new_with_transform_list_WebKitCSSMatrix\x01\0\0\x01\x0FWebKitCSSMatrix\0\x01\x01\x0Etransform_list\x03new\0\0\0'__widl_f_new_with_other_WebKitCSSMatrix\x01\0\0\x01\x0FWebKitCSSMatrix\0\x01\x01\x05other\x03new\0\0\0 __widl_f_inverse_WebKitCSSMatrix\x01\0\0\x01\x0FWebKitCSSMatrix\x01\0\0\x01\x01\x05self_\x07inverse\0\0\0!__widl_f_multiply_WebKitCSSMatrix\0\0\0\x01\x0FWebKitCSSMatrix\x01\0\0\x01\x02\x05self_\x05other\x08multiply\0\0\0\x1F__widl_f_rotate_WebKitCSSMatrix\0\0\0\x01\x0FWebKitCSSMatrix\x01\0\0\x01\x01\x05self_\x06rotate\0\0\0*__widl_f_rotate_with_rot_x_WebKitCSSMatrix\0\0\0\x01\x0FWebKitCSSMatrix\x01\0\0\x01\x02\x05self_\x05rot_x\x06rotate\0\0\04__widl_f_rotate_with_rot_x_and_rot_y_WebKitCSSMatrix\0\0\0\x01\x0FWebKitCSSMatrix\x01\0\0\x01\x03\x05self_\x05rot_x\x05rot_y\x06rotate\0\0\0>__widl_f_rotate_with_rot_x_and_rot_y_and_rot_z_WebKitCSSMatrix\0\0\0\x01\x0FWebKitCSSMatrix\x01\0\0\x01\x04\x05self_\x05rot_x\x05rot_y\x05rot_z\x06rotate\0\0\0*__widl_f_rotate_axis_angle_WebKitCSSMatrix\0\0\0\x01\x0FWebKitCSSMatrix\x01\0\0\x01\x01\x05self_\x0FrotateAxisAngle\0\0\01__widl_f_rotate_axis_angle_with_x_WebKitCSSMatrix\0\0\0\x01\x0FWebKitCSSMatrix\x01\0\0\x01\x02\x05self_\x01x\x0FrotateAxisAngle\0\0\07__widl_f_rotate_axis_angle_with_x_and_y_WebKitCSSMatrix\0\0\0\x01\x0FWebKitCSSMatrix\x01\0\0\x01\x03\x05self_\x01x\x01y\x0FrotateAxisAngle\0\0\0=__widl_f_rotate_axis_angle_with_x_and_y_and_z_WebKitCSSMatrix\0\0\0\x01\x0FWebKitCSSMatrix\x01\0\0\x01\x04\x05self_\x01x\x01y\x01z\x0FrotateAxisAngle\0\0\0G__widl_f_rotate_axis_angle_with_x_and_y_and_z_and_angle_WebKitCSSMatrix\0\0\0\x01\x0FWebKitCSSMatrix\x01\0\0\x01\x05\x05self_\x01x\x01y\x01z\x05angle\x0FrotateAxisAngle\0\0\0\x1E__widl_f_scale_WebKitCSSMatrix\0\0\0\x01\x0FWebKitCSSMatrix\x01\0\0\x01\x01\x05self_\x05scale\0\0\0+__widl_f_scale_with_scale_x_WebKitCSSMatrix\0\0\0\x01\x0FWebKitCSSMatrix\x01\0\0\x01\x02\x05self_\x07scale_x\x05scale\0\0\07__widl_f_scale_with_scale_x_and_scale_y_WebKitCSSMatrix\0\0\0\x01\x0FWebKitCSSMatrix\x01\0\0\x01\x03\x05self_\x07scale_x\x07scale_y\x05scale\0\0\0C__widl_f_scale_with_scale_x_and_scale_y_and_scale_z_WebKitCSSMatrix\0\0\0\x01\x0FWebKitCSSMatrix\x01\0\0\x01\x04\x05self_\x07scale_x\x07scale_y\x07scale_z\x05scale\0\0\0)__widl_f_set_matrix_value_WebKitCSSMatrix\x01\0\0\x01\x0FWebKitCSSMatrix\x01\0\0\x01\x02\x05self_\x0Etransform_list\x0EsetMatrixValue\0\0\0\x1F__widl_f_skew_x_WebKitCSSMatrix\0\0\0\x01\x0FWebKitCSSMatrix\x01\0\0\x01\x01\x05self_\x05skewX\0\0\0'__widl_f_skew_x_with_sx_WebKitCSSMatrix\0\0\0\x01\x0FWebKitCSSMatrix\x01\0\0\x01\x02\x05self_\x02sx\x05skewX\0\0\0\x1F__widl_f_skew_y_WebKitCSSMatrix\0\0\0\x01\x0FWebKitCSSMatrix\x01\0\0\x01\x01\x05self_\x05skewY\0\0\0'__widl_f_skew_y_with_sy_WebKitCSSMatrix\0\0\0\x01\x0FWebKitCSSMatrix\x01\0\0\x01\x02\x05self_\x02sy\x05skewY\0\0\0\"__widl_f_translate_WebKitCSSMatrix\0\0\0\x01\x0FWebKitCSSMatrix\x01\0\0\x01\x01\x05self_\ttranslate\0\0\0*__widl_f_translate_with_tx_WebKitCSSMatrix\0\0\0\x01\x0FWebKitCSSMatrix\x01\0\0\x01\x02\x05self_\x02tx\ttranslate\0\0\01__widl_f_translate_with_tx_and_ty_WebKitCSSMatrix\0\0\0\x01\x0FWebKitCSSMatrix\x01\0\0\x01\x03\x05self_\x02tx\x02ty\ttranslate\0\0\08__widl_f_translate_with_tx_and_ty_and_tz_WebKitCSSMatrix\0\0\0\x01\x0FWebKitCSSMatrix\x01\0\0\x01\x04\x05self_\x02tx\x02ty\x02tz\ttranslate\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
