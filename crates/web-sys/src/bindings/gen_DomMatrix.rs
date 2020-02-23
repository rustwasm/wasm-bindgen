use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `DOMMatrix` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct DomMatrix {
    obj: DomMatrixReadOnly,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_DomMatrix: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for DomMatrix {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(9u32);
            inform(68u32);
            inform(79u32);
            inform(77u32);
            inform(77u32);
            inform(97u32);
            inform(116u32);
            inform(114u32);
            inform(105u32);
            inform(120u32);
        }
    }
    impl core::ops::Deref for DomMatrix {
        type Target = DomMatrixReadOnly;
        #[inline]
        fn deref(&self) -> &DomMatrixReadOnly {
            &self.obj
        }
    }
    impl IntoWasmAbi for DomMatrix {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for DomMatrix {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a DomMatrix {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for DomMatrix {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            DomMatrix {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for DomMatrix {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a DomMatrix {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for DomMatrix {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<DomMatrix>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(DomMatrix {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for DomMatrix {
        #[inline]
        fn from(obj: JsValue) -> DomMatrix {
            DomMatrix { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for DomMatrix {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<DomMatrix> for DomMatrix {
        #[inline]
        fn as_ref(&self) -> &DomMatrix {
            self
        }
    }
    impl From<DomMatrix> for JsValue {
        #[inline]
        fn from(obj: DomMatrix) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for DomMatrix {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_DOMMatrix(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_DOMMatrix(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_DOMMatrix(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            DomMatrix { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const DomMatrix) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<DomMatrix> for DomMatrixReadOnly {
    #[inline]
    fn from(obj: DomMatrix) -> DomMatrixReadOnly {
        use wasm_bindgen::JsCast;
        DomMatrixReadOnly::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<DomMatrixReadOnly> for DomMatrix {
    #[inline]
    fn as_ref(&self) -> &DomMatrixReadOnly {
        use wasm_bindgen::JsCast;
        DomMatrixReadOnly::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<DomMatrix> for ::js_sys::Object {
    #[inline]
    fn from(obj: DomMatrix) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for DomMatrix {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMMatrix(..)` constructor, creating a new instance of `DOMMatrix`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/DOMMatrix)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<DomMatrix, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_DOMMatrix() -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_DOMMatrix() -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi
        {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_DOMMatrix() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_transform_list_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMMatrix(..)` constructor, creating a new instance of `DOMMatrix`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/DOMMatrix)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn new_with_transform_list(
        transform_list: &str,
    ) -> Result<DomMatrix, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_transform_list_DOMMatrix(
                transform_list: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_transform_list_DOMMatrix(
            transform_list: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                __widl_f_new_with_transform_list_DOMMatrix(transform_list)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_other_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMMatrix(..)` constructor, creating a new instance of `DOMMatrix`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/DOMMatrix)\n\n*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn new_with_other(other: &DomMatrixReadOnly) -> Result<DomMatrix, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_other_DOMMatrix(
                other: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_other_DOMMatrix(
            other: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(other);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let other =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(other);
                __widl_f_new_with_other_DOMMatrix(other)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_array32_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&mut [f32] as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMMatrix(..)` constructor, creating a new instance of `DOMMatrix`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/DOMMatrix)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn new_with_array32(array32: &mut [f32]) -> Result<DomMatrix, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_array32_DOMMatrix(
                array32: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_array32_DOMMatrix(
            array32: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(array32);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let array32 = <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(array32);
                __widl_f_new_with_array32_DOMMatrix(array32)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_array64_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&mut [f64] as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMMatrix(..)` constructor, creating a new instance of `DOMMatrix`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/DOMMatrix)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn new_with_array64(array64: &mut [f64]) -> Result<DomMatrix, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_array64_DOMMatrix(
                array64: <&mut [f64] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_array64_DOMMatrix(
            array64: <&mut [f64] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(array64);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let array64 = <&mut [f64] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(array64);
                __widl_f_new_with_array64_DOMMatrix(array64)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_number_sequence_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMMatrix(..)` constructor, creating a new instance of `DOMMatrix`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/DOMMatrix)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn new_with_number_sequence(
        number_sequence: &::wasm_bindgen::JsValue,
    ) -> Result<DomMatrix, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_number_sequence_DOMMatrix(
                number_sequence : < & :: wasm_bindgen :: JsValue as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_number_sequence_DOMMatrix(
            number_sequence: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(number_sequence);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let number_sequence =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        number_sequence,
                    );
                __widl_f_new_with_number_sequence_DOMMatrix(number_sequence)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_invert_self_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrix as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `invertSelf()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/invertSelf)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn invert_self(&self) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_invert_self_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_invert_self_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_invert_self_DOMMatrix(self_)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_multiply_self_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrix as WasmDescribe>::describe();
    <&DomMatrix as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `multiplySelf()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/multiplySelf)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn multiply_self(&self, other: &DomMatrix) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_multiply_self_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                other: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_multiply_self_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            other: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(other);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let other = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(other);
                __widl_f_multiply_self_DOMMatrix(self_, other)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pre_multiply_self_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrix as WasmDescribe>::describe();
    <&DomMatrix as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `preMultiplySelf()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/preMultiplySelf)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn pre_multiply_self(&self, other: &DomMatrix) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pre_multiply_self_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                other: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pre_multiply_self_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            other: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(other);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let other = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(other);
                __widl_f_pre_multiply_self_DOMMatrix(self_, other)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rotate_axis_angle_self_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `rotateAxisAngleSelf()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/rotateAxisAngleSelf)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn rotate_axis_angle_self(&self, x: f64, y: f64, z: f64, angle: f64) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rotate_axis_angle_self_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rotate_axis_angle_self_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let z = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(z);
                let angle = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(angle);
                __widl_f_rotate_axis_angle_self_DOMMatrix(self_, x, y, z, angle)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rotate_from_vector_self_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `rotateFromVectorSelf()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/rotateFromVectorSelf)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn rotate_from_vector_self(&self, x: f64, y: f64) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rotate_from_vector_self_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rotate_from_vector_self_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_rotate_from_vector_self_DOMMatrix(self_, x, y)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rotate_self_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `rotateSelf()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/rotateSelf)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn rotate_self(&self, angle: f64) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rotate_self_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rotate_self_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(angle);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let angle = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(angle);
                __widl_f_rotate_self_DOMMatrix(self_, angle)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rotate_self_with_origin_x_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `rotateSelf()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/rotateSelf)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn rotate_self_with_origin_x(&self, angle: f64, origin_x: f64) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rotate_self_with_origin_x_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rotate_self_with_origin_x_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(angle);
            drop(origin_x);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let angle = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(angle);
                let origin_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_x);
                __widl_f_rotate_self_with_origin_x_DOMMatrix(self_, angle, origin_x)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rotate_self_with_origin_x_and_origin_y_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `rotateSelf()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/rotateSelf)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn rotate_self_with_origin_x_and_origin_y(
        &self,
        angle: f64,
        origin_x: f64,
        origin_y: f64,
    ) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rotate_self_with_origin_x_and_origin_y_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rotate_self_with_origin_x_and_origin_y_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(angle);
            drop(origin_x);
            drop(origin_y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let angle = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(angle);
                let origin_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_x);
                let origin_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_y);
                __widl_f_rotate_self_with_origin_x_and_origin_y_DOMMatrix(
                    self_, angle, origin_x, origin_y,
                )
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale3d_self_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `scale3dSelf()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scale3dSelf)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn scale3d_self(&self, scale: f64) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale3d_self_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale3d_self_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(scale);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scale = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale);
                __widl_f_scale3d_self_DOMMatrix(self_, scale)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale3d_self_with_origin_x_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `scale3dSelf()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scale3dSelf)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn scale3d_self_with_origin_x(&self, scale: f64, origin_x: f64) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale3d_self_with_origin_x_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale3d_self_with_origin_x_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(scale);
            drop(origin_x);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scale = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale);
                let origin_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_x);
                __widl_f_scale3d_self_with_origin_x_DOMMatrix(self_, scale, origin_x)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale3d_self_with_origin_x_and_origin_y_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `scale3dSelf()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scale3dSelf)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn scale3d_self_with_origin_x_and_origin_y(
        &self,
        scale: f64,
        origin_x: f64,
        origin_y: f64,
    ) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale3d_self_with_origin_x_and_origin_y_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale3d_self_with_origin_x_and_origin_y_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(scale);
            drop(origin_x);
            drop(origin_y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scale = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale);
                let origin_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_x);
                let origin_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_y);
                __widl_f_scale3d_self_with_origin_x_and_origin_y_DOMMatrix(
                    self_, scale, origin_x, origin_y,
                )
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale3d_self_with_origin_x_and_origin_y_and_origin_z_DOMMatrix(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `scale3dSelf()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scale3dSelf)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn scale3d_self_with_origin_x_and_origin_y_and_origin_z(
        &self,
        scale: f64,
        origin_x: f64,
        origin_y: f64,
        origin_z: f64,
    ) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale3d_self_with_origin_x_and_origin_y_and_origin_z_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale3d_self_with_origin_x_and_origin_y_and_origin_z_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(scale);
            drop(origin_x);
            drop(origin_y);
            drop(origin_z);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scale = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale);
                let origin_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_x);
                let origin_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_y);
                let origin_z = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_z);
                __widl_f_scale3d_self_with_origin_x_and_origin_y_and_origin_z_DOMMatrix(
                    self_, scale, origin_x, origin_y, origin_z,
                )
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale_non_uniform_self_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `scaleNonUniformSelf()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleNonUniformSelf)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn scale_non_uniform_self(&self, scale_x: f64) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale_non_uniform_self_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale_non_uniform_self_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(scale_x);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scale_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_x);
                __widl_f_scale_non_uniform_self_DOMMatrix(self_, scale_x)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale_non_uniform_self_with_scale_y_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `scaleNonUniformSelf()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleNonUniformSelf)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn scale_non_uniform_self_with_scale_y(&self, scale_x: f64, scale_y: f64) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale_non_uniform_self_with_scale_y_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale_non_uniform_self_with_scale_y_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scale_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_x);
                let scale_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_y);
                __widl_f_scale_non_uniform_self_with_scale_y_DOMMatrix(self_, scale_x, scale_y)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale_non_uniform_self_with_scale_y_and_scale_z_DOMMatrix(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `scaleNonUniformSelf()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleNonUniformSelf)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn scale_non_uniform_self_with_scale_y_and_scale_z(
        &self,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
    ) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale_non_uniform_self_with_scale_y_and_scale_z_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale_non_uniform_self_with_scale_y_and_scale_z_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scale_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_x);
                let scale_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_y);
                let scale_z = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_z);
                __widl_f_scale_non_uniform_self_with_scale_y_and_scale_z_DOMMatrix(
                    self_, scale_x, scale_y, scale_z,
                )
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale_non_uniform_self_with_scale_y_and_scale_z_and_origin_x_DOMMatrix(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `scaleNonUniformSelf()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleNonUniformSelf)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn scale_non_uniform_self_with_scale_y_and_scale_z_and_origin_x(
        &self,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
        origin_x: f64,
    ) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale_non_uniform_self_with_scale_y_and_scale_z_and_origin_x_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale_non_uniform_self_with_scale_y_and_scale_z_and_origin_x_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(scale_x);
            drop(scale_y);
            drop(scale_z);
            drop(origin_x);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scale_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_x);
                let scale_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_y);
                let scale_z = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_z);
                let origin_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_x);
                __widl_f_scale_non_uniform_self_with_scale_y_and_scale_z_and_origin_x_DOMMatrix(
                    self_, scale_x, scale_y, scale_z, origin_x,
                )
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale_non_uniform_self_with_scale_y_and_scale_z_and_origin_x_and_origin_y_DOMMatrix(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `scaleNonUniformSelf()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleNonUniformSelf)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn scale_non_uniform_self_with_scale_y_and_scale_z_and_origin_x_and_origin_y(
        &self,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
        origin_x: f64,
        origin_y: f64,
    ) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale_non_uniform_self_with_scale_y_and_scale_z_and_origin_x_and_origin_y_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale_non_uniform_self_with_scale_y_and_scale_z_and_origin_x_and_origin_y_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(scale_x);
            drop(scale_y);
            drop(scale_z);
            drop(origin_x);
            drop(origin_y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scale_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_x);
                let scale_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_y);
                let scale_z = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_z);
                let origin_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_x);
                let origin_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_y);
                __widl_f_scale_non_uniform_self_with_scale_y_and_scale_z_and_origin_x_and_origin_y_DOMMatrix ( self_ , scale_x , scale_y , scale_z , origin_x , origin_y )
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale_non_uniform_self_with_scale_y_and_scale_z_and_origin_x_and_origin_y_and_origin_z_DOMMatrix(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `scaleNonUniformSelf()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleNonUniformSelf)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn scale_non_uniform_self_with_scale_y_and_scale_z_and_origin_x_and_origin_y_and_origin_z(
        &self,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
        origin_x: f64,
        origin_y: f64,
        origin_z: f64,
    ) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale_non_uniform_self_with_scale_y_and_scale_z_and_origin_x_and_origin_y_and_origin_z_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale_non_uniform_self_with_scale_y_and_scale_z_and_origin_x_and_origin_y_and_origin_z_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(scale_x);
            drop(scale_y);
            drop(scale_z);
            drop(origin_x);
            drop(origin_y);
            drop(origin_z);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scale_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_x);
                let scale_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_y);
                let scale_z = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_z);
                let origin_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_x);
                let origin_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_y);
                let origin_z = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_z);
                __widl_f_scale_non_uniform_self_with_scale_y_and_scale_z_and_origin_x_and_origin_y_and_origin_z_DOMMatrix ( self_ , scale_x , scale_y , scale_z , origin_x , origin_y , origin_z )
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale_self_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `scaleSelf()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleSelf)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn scale_self(&self, scale: f64) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale_self_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale_self_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(scale);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scale = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale);
                __widl_f_scale_self_DOMMatrix(self_, scale)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale_self_with_origin_x_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `scaleSelf()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleSelf)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn scale_self_with_origin_x(&self, scale: f64, origin_x: f64) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale_self_with_origin_x_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale_self_with_origin_x_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(scale);
            drop(origin_x);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scale = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale);
                let origin_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_x);
                __widl_f_scale_self_with_origin_x_DOMMatrix(self_, scale, origin_x)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale_self_with_origin_x_and_origin_y_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `scaleSelf()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleSelf)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn scale_self_with_origin_x_and_origin_y(
        &self,
        scale: f64,
        origin_x: f64,
        origin_y: f64,
    ) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale_self_with_origin_x_and_origin_y_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale_self_with_origin_x_and_origin_y_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(scale);
            drop(origin_x);
            drop(origin_y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scale = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale);
                let origin_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_x);
                let origin_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_y);
                __widl_f_scale_self_with_origin_x_and_origin_y_DOMMatrix(
                    self_, scale, origin_x, origin_y,
                )
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_matrix_value_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrix as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `setMatrixValue()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/setMatrixValue)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn set_matrix_value(
        &self,
        transform_list: &str,
    ) -> Result<DomMatrix, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_matrix_value_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                transform_list: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_matrix_value_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            transform_list: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(transform_list);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let transform_list =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(transform_list);
                __widl_f_set_matrix_value_DOMMatrix(self_, transform_list)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_skew_x_self_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `skewXSelf()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/skewXSelf)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn skew_x_self(&self, sx: f64) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_skew_x_self_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_skew_x_self_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(sx);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let sx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sx);
                __widl_f_skew_x_self_DOMMatrix(self_, sx)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_skew_y_self_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `skewYSelf()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/skewYSelf)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn skew_y_self(&self, sy: f64) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_skew_y_self_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_skew_y_self_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(sy);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let sy = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sy);
                __widl_f_skew_y_self_DOMMatrix(self_, sy)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_translate_self_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `translateSelf()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/translateSelf)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn translate_self(&self, tx: f64, ty: f64) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_translate_self_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ty: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_translate_self_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ty: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let tx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tx);
                let ty = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ty);
                __widl_f_translate_self_DOMMatrix(self_, tx, ty)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_translate_self_with_tz_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `translateSelf()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/translateSelf)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn translate_self_with_tz(&self, tx: f64, ty: f64, tz: f64) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_translate_self_with_tz_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ty: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tz: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_translate_self_with_tz_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ty: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tz: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let tx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tx);
                let ty = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ty);
                let tz = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tz);
                __widl_f_translate_self_with_tz_DOMMatrix(self_, tx, ty, tz)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_a_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `a` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/a)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn a(&self) -> f64 {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_a_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_a_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_a_DOMMatrix(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_a_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `a` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/a)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn set_a(&self, a: f64) {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_a_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_a_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a);
                __widl_f_set_a_DOMMatrix(self_, a)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_b_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `b` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/b)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn b(&self) -> f64 {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_b_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_b_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_b_DOMMatrix(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_b_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `b` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/b)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn set_b(&self, b: f64) {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_b_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                b: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_b_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            b: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let b = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(b);
                __widl_f_set_b_DOMMatrix(self_, b)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_c_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `c` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/c)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn c(&self) -> f64 {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_c_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_c_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_c_DOMMatrix(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_c_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `c` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/c)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn set_c(&self, c: f64) {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_c_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                c: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_c_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            c: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let c = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(c);
                __widl_f_set_c_DOMMatrix(self_, c)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_d_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `d` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/d)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn d(&self) -> f64 {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_d_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_d_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_d_DOMMatrix(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_d_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `d` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/d)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn set_d(&self, d: f64) {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_d_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                d: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_d_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            d: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let d = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(d);
                __widl_f_set_d_DOMMatrix(self_, d)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_e_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `e` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/e)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn e(&self) -> f64 {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_e_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_e_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_e_DOMMatrix(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_e_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `e` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/e)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn set_e(&self, e: f64) {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_e_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                e: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_e_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            e: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let e = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(e);
                __widl_f_set_e_DOMMatrix(self_, e)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_f_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `f` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/f)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn f(&self) -> f64 {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_f_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_f_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_f_DOMMatrix(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_f_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `f` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/f)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn set_f(&self, f: f64) {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_f_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                f: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_f_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            f: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let f = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(f);
                __widl_f_set_f_DOMMatrix(self_, f)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_m11_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `m11` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m11)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn m11(&self) -> f64 {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_m11_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_m11_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_m11_DOMMatrix(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_m11_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `m11` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m11)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn set_m11(&self, m11: f64) {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_m11_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                m11: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_m11_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            m11: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(m11);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let m11 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(m11);
                __widl_f_set_m11_DOMMatrix(self_, m11)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_m12_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `m12` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m12)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn m12(&self) -> f64 {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_m12_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_m12_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_m12_DOMMatrix(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_m12_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `m12` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m12)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn set_m12(&self, m12: f64) {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_m12_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                m12: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_m12_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            m12: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(m12);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let m12 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(m12);
                __widl_f_set_m12_DOMMatrix(self_, m12)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_m13_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `m13` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m13)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn m13(&self) -> f64 {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_m13_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_m13_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_m13_DOMMatrix(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_m13_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `m13` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m13)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn set_m13(&self, m13: f64) {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_m13_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                m13: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_m13_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            m13: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(m13);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let m13 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(m13);
                __widl_f_set_m13_DOMMatrix(self_, m13)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_m14_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `m14` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m14)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn m14(&self) -> f64 {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_m14_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_m14_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_m14_DOMMatrix(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_m14_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `m14` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m14)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn set_m14(&self, m14: f64) {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_m14_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                m14: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_m14_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            m14: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(m14);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let m14 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(m14);
                __widl_f_set_m14_DOMMatrix(self_, m14)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_m21_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `m21` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m21)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn m21(&self) -> f64 {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_m21_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_m21_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_m21_DOMMatrix(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_m21_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `m21` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m21)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn set_m21(&self, m21: f64) {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_m21_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                m21: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_m21_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            m21: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(m21);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let m21 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(m21);
                __widl_f_set_m21_DOMMatrix(self_, m21)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_m22_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `m22` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m22)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn m22(&self) -> f64 {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_m22_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_m22_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_m22_DOMMatrix(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_m22_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `m22` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m22)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn set_m22(&self, m22: f64) {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_m22_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                m22: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_m22_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            m22: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(m22);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let m22 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(m22);
                __widl_f_set_m22_DOMMatrix(self_, m22)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_m23_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `m23` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m23)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn m23(&self) -> f64 {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_m23_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_m23_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_m23_DOMMatrix(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_m23_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `m23` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m23)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn set_m23(&self, m23: f64) {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_m23_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                m23: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_m23_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            m23: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(m23);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let m23 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(m23);
                __widl_f_set_m23_DOMMatrix(self_, m23)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_m24_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `m24` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m24)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn m24(&self) -> f64 {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_m24_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_m24_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_m24_DOMMatrix(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_m24_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `m24` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m24)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn set_m24(&self, m24: f64) {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_m24_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                m24: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_m24_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            m24: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(m24);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let m24 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(m24);
                __widl_f_set_m24_DOMMatrix(self_, m24)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_m31_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `m31` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m31)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn m31(&self) -> f64 {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_m31_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_m31_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_m31_DOMMatrix(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_m31_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `m31` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m31)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn set_m31(&self, m31: f64) {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_m31_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                m31: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_m31_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            m31: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(m31);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let m31 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(m31);
                __widl_f_set_m31_DOMMatrix(self_, m31)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_m32_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `m32` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m32)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn m32(&self) -> f64 {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_m32_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_m32_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_m32_DOMMatrix(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_m32_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `m32` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m32)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn set_m32(&self, m32: f64) {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_m32_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                m32: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_m32_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            m32: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(m32);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let m32 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(m32);
                __widl_f_set_m32_DOMMatrix(self_, m32)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_m33_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `m33` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m33)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn m33(&self) -> f64 {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_m33_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_m33_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_m33_DOMMatrix(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_m33_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `m33` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m33)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn set_m33(&self, m33: f64) {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_m33_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                m33: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_m33_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            m33: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(m33);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let m33 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(m33);
                __widl_f_set_m33_DOMMatrix(self_, m33)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_m34_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `m34` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m34)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn m34(&self) -> f64 {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_m34_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_m34_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_m34_DOMMatrix(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_m34_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `m34` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m34)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn set_m34(&self, m34: f64) {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_m34_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                m34: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_m34_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            m34: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(m34);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let m34 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(m34);
                __widl_f_set_m34_DOMMatrix(self_, m34)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_m41_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `m41` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m41)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn m41(&self) -> f64 {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_m41_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_m41_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_m41_DOMMatrix(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_m41_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `m41` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m41)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn set_m41(&self, m41: f64) {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_m41_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                m41: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_m41_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            m41: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(m41);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let m41 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(m41);
                __widl_f_set_m41_DOMMatrix(self_, m41)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_m42_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `m42` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m42)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn m42(&self) -> f64 {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_m42_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_m42_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_m42_DOMMatrix(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_m42_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `m42` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m42)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn set_m42(&self, m42: f64) {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_m42_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                m42: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_m42_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            m42: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(m42);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let m42 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(m42);
                __widl_f_set_m42_DOMMatrix(self_, m42)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_m43_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `m43` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m43)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn m43(&self) -> f64 {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_m43_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_m43_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_m43_DOMMatrix(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_m43_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `m43` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m43)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn set_m43(&self, m43: f64) {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_m43_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                m43: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_m43_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            m43: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(m43);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let m43 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(m43);
                __widl_f_set_m43_DOMMatrix(self_, m43)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_m44_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `m44` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m44)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn m44(&self) -> f64 {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_m44_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_m44_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_m44_DOMMatrix(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_m44_DOMMatrix() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrix as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomMatrix {
    #[cfg(all(feature = "DomMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `m44` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m44)\n\n*This API requires the following crate features to be activated: `DomMatrix`*"]
    #[allow(clippy::all)]
    pub fn set_m44(&self, m44: f64) {
        #[cfg(all(feature = "DomMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_m44_DOMMatrix(
                self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                m44: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_m44_DOMMatrix(
            self_: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            m44: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(m44);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let m44 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(m44);
                __widl_f_set_m44_DOMMatrix(self_, m44)
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
pub static __WASM_BINDGEN_GENERATED_db0fe6b6056f907d: [u8; 6034usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}P\x17\0\0\0\0M\0\0\x02\tDOMMatrix\x1B__widl_instanceof_DOMMatrix\0\0\0\0\x16__widl_f_new_DOMMatrix\x01\0\0\x01\tDOMMatrix\0\x01\0\x03new\0\0\0*__widl_f_new_with_transform_list_DOMMatrix\x01\0\0\x01\tDOMMatrix\0\x01\x01\x0Etransform_list\x03new\0\0\0!__widl_f_new_with_other_DOMMatrix\x01\0\0\x01\tDOMMatrix\0\x01\x01\x05other\x03new\0\0\0#__widl_f_new_with_array32_DOMMatrix\x01\0\0\x01\tDOMMatrix\0\x01\x01\x07array32\x03new\0\0\0#__widl_f_new_with_array64_DOMMatrix\x01\0\0\x01\tDOMMatrix\0\x01\x01\x07array64\x03new\0\0\0+__widl_f_new_with_number_sequence_DOMMatrix\x01\0\0\x01\tDOMMatrix\0\x01\x01\x0Fnumber_sequence\x03new\0\0\0\x1E__widl_f_invert_self_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\0\x01\x01\x05self_\ninvertSelf\0\0\0 __widl_f_multiply_self_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\0\x01\x02\x05self_\x05other\x0CmultiplySelf\0\0\0$__widl_f_pre_multiply_self_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\0\x01\x02\x05self_\x05other\x0FpreMultiplySelf\0\0\0)__widl_f_rotate_axis_angle_self_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\0\x01\x05\x05self_\x01x\x01y\x01z\x05angle\x13rotateAxisAngleSelf\0\0\0*__widl_f_rotate_from_vector_self_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\0\x01\x03\x05self_\x01x\x01y\x14rotateFromVectorSelf\0\0\0\x1E__widl_f_rotate_self_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\0\x01\x02\x05self_\x05angle\nrotateSelf\0\0\0,__widl_f_rotate_self_with_origin_x_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\0\x01\x03\x05self_\x05angle\x08origin_x\nrotateSelf\0\0\09__widl_f_rotate_self_with_origin_x_and_origin_y_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\0\x01\x04\x05self_\x05angle\x08origin_x\x08origin_y\nrotateSelf\0\0\0\x1F__widl_f_scale3d_self_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\0\x01\x02\x05self_\x05scale\x0Bscale3dSelf\0\0\0-__widl_f_scale3d_self_with_origin_x_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\0\x01\x03\x05self_\x05scale\x08origin_x\x0Bscale3dSelf\0\0\0:__widl_f_scale3d_self_with_origin_x_and_origin_y_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\0\x01\x04\x05self_\x05scale\x08origin_x\x08origin_y\x0Bscale3dSelf\0\0\0G__widl_f_scale3d_self_with_origin_x_and_origin_y_and_origin_z_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\0\x01\x05\x05self_\x05scale\x08origin_x\x08origin_y\x08origin_z\x0Bscale3dSelf\0\0\0)__widl_f_scale_non_uniform_self_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\0\x01\x02\x05self_\x07scale_x\x13scaleNonUniformSelf\0\0\06__widl_f_scale_non_uniform_self_with_scale_y_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\0\x01\x03\x05self_\x07scale_x\x07scale_y\x13scaleNonUniformSelf\0\0\0B__widl_f_scale_non_uniform_self_with_scale_y_and_scale_z_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\0\x01\x04\x05self_\x07scale_x\x07scale_y\x07scale_z\x13scaleNonUniformSelf\0\0\0O__widl_f_scale_non_uniform_self_with_scale_y_and_scale_z_and_origin_x_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\0\x01\x05\x05self_\x07scale_x\x07scale_y\x07scale_z\x08origin_x\x13scaleNonUniformSelf\0\0\0\\__widl_f_scale_non_uniform_self_with_scale_y_and_scale_z_and_origin_x_and_origin_y_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\0\x01\x06\x05self_\x07scale_x\x07scale_y\x07scale_z\x08origin_x\x08origin_y\x13scaleNonUniformSelf\0\0\0i__widl_f_scale_non_uniform_self_with_scale_y_and_scale_z_and_origin_x_and_origin_y_and_origin_z_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\0\x01\x07\x05self_\x07scale_x\x07scale_y\x07scale_z\x08origin_x\x08origin_y\x08origin_z\x13scaleNonUniformSelf\0\0\0\x1D__widl_f_scale_self_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\0\x01\x02\x05self_\x05scale\tscaleSelf\0\0\0+__widl_f_scale_self_with_origin_x_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\0\x01\x03\x05self_\x05scale\x08origin_x\tscaleSelf\0\0\08__widl_f_scale_self_with_origin_x_and_origin_y_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\0\x01\x04\x05self_\x05scale\x08origin_x\x08origin_y\tscaleSelf\0\0\0#__widl_f_set_matrix_value_DOMMatrix\x01\0\0\x01\tDOMMatrix\x01\0\0\x01\x02\x05self_\x0Etransform_list\x0EsetMatrixValue\0\0\0\x1E__widl_f_skew_x_self_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\0\x01\x02\x05self_\x02sx\tskewXSelf\0\0\0\x1E__widl_f_skew_y_self_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\0\x01\x02\x05self_\x02sy\tskewYSelf\0\0\0!__widl_f_translate_self_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\0\x01\x03\x05self_\x02tx\x02ty\rtranslateSelf\0\0\0)__widl_f_translate_self_with_tz_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\0\x01\x04\x05self_\x02tx\x02ty\x02tz\rtranslateSelf\0\0\0\x14__widl_f_a_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x01\x01a\x01\x01\x05self_\x01a\0\0\0\x18__widl_f_set_a_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x02\x01a\x01\x02\x05self_\x01a\x01a\0\0\0\x14__widl_f_b_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x01\x01b\x01\x01\x05self_\x01b\0\0\0\x18__widl_f_set_b_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x02\x01b\x01\x02\x05self_\x01b\x01b\0\0\0\x14__widl_f_c_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x01\x01c\x01\x01\x05self_\x01c\0\0\0\x18__widl_f_set_c_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x02\x01c\x01\x02\x05self_\x01c\x01c\0\0\0\x14__widl_f_d_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x01\x01d\x01\x01\x05self_\x01d\0\0\0\x18__widl_f_set_d_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x02\x01d\x01\x02\x05self_\x01d\x01d\0\0\0\x14__widl_f_e_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x01\x01e\x01\x01\x05self_\x01e\0\0\0\x18__widl_f_set_e_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x02\x01e\x01\x02\x05self_\x01e\x01e\0\0\0\x14__widl_f_f_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x01\x01f\x01\x01\x05self_\x01f\0\0\0\x18__widl_f_set_f_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x02\x01f\x01\x02\x05self_\x01f\x01f\0\0\0\x16__widl_f_m11_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x01\x03m11\x01\x01\x05self_\x03m11\0\0\0\x1A__widl_f_set_m11_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x02\x03m11\x01\x02\x05self_\x03m11\x03m11\0\0\0\x16__widl_f_m12_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x01\x03m12\x01\x01\x05self_\x03m12\0\0\0\x1A__widl_f_set_m12_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x02\x03m12\x01\x02\x05self_\x03m12\x03m12\0\0\0\x16__widl_f_m13_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x01\x03m13\x01\x01\x05self_\x03m13\0\0\0\x1A__widl_f_set_m13_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x02\x03m13\x01\x02\x05self_\x03m13\x03m13\0\0\0\x16__widl_f_m14_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x01\x03m14\x01\x01\x05self_\x03m14\0\0\0\x1A__widl_f_set_m14_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x02\x03m14\x01\x02\x05self_\x03m14\x03m14\0\0\0\x16__widl_f_m21_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x01\x03m21\x01\x01\x05self_\x03m21\0\0\0\x1A__widl_f_set_m21_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x02\x03m21\x01\x02\x05self_\x03m21\x03m21\0\0\0\x16__widl_f_m22_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x01\x03m22\x01\x01\x05self_\x03m22\0\0\0\x1A__widl_f_set_m22_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x02\x03m22\x01\x02\x05self_\x03m22\x03m22\0\0\0\x16__widl_f_m23_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x01\x03m23\x01\x01\x05self_\x03m23\0\0\0\x1A__widl_f_set_m23_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x02\x03m23\x01\x02\x05self_\x03m23\x03m23\0\0\0\x16__widl_f_m24_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x01\x03m24\x01\x01\x05self_\x03m24\0\0\0\x1A__widl_f_set_m24_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x02\x03m24\x01\x02\x05self_\x03m24\x03m24\0\0\0\x16__widl_f_m31_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x01\x03m31\x01\x01\x05self_\x03m31\0\0\0\x1A__widl_f_set_m31_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x02\x03m31\x01\x02\x05self_\x03m31\x03m31\0\0\0\x16__widl_f_m32_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x01\x03m32\x01\x01\x05self_\x03m32\0\0\0\x1A__widl_f_set_m32_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x02\x03m32\x01\x02\x05self_\x03m32\x03m32\0\0\0\x16__widl_f_m33_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x01\x03m33\x01\x01\x05self_\x03m33\0\0\0\x1A__widl_f_set_m33_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x02\x03m33\x01\x02\x05self_\x03m33\x03m33\0\0\0\x16__widl_f_m34_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x01\x03m34\x01\x01\x05self_\x03m34\0\0\0\x1A__widl_f_set_m34_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x02\x03m34\x01\x02\x05self_\x03m34\x03m34\0\0\0\x16__widl_f_m41_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x01\x03m41\x01\x01\x05self_\x03m41\0\0\0\x1A__widl_f_set_m41_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x02\x03m41\x01\x02\x05self_\x03m41\x03m41\0\0\0\x16__widl_f_m42_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x01\x03m42\x01\x01\x05self_\x03m42\0\0\0\x1A__widl_f_set_m42_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x02\x03m42\x01\x02\x05self_\x03m42\x03m42\0\0\0\x16__widl_f_m43_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x01\x03m43\x01\x01\x05self_\x03m43\0\0\0\x1A__widl_f_set_m43_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x02\x03m43\x01\x02\x05self_\x03m43\x03m43\0\0\0\x16__widl_f_m44_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x01\x03m44\x01\x01\x05self_\x03m44\0\0\0\x1A__widl_f_set_m44_DOMMatrix\0\0\0\x01\tDOMMatrix\x01\0\x02\x03m44\x01\x02\x05self_\x03m44\x03m44\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
