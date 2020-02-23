use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Blob` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob)\n\n*This API requires the following crate features to be activated: `Blob`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Blob {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Blob: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Blob {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(4u32);
            inform(66u32);
            inform(108u32);
            inform(111u32);
            inform(98u32);
        }
    }
    impl core::ops::Deref for Blob {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for Blob {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Blob {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Blob {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Blob {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Blob {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Blob {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Blob {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Blob {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Blob>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Blob {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Blob {
        #[inline]
        fn from(obj: JsValue) -> Blob {
            Blob { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Blob {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Blob> for Blob {
        #[inline]
        fn as_ref(&self) -> &Blob {
            self
        }
    }
    impl From<Blob> for JsValue {
        #[inline]
        fn from(obj: Blob) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Blob {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Blob(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Blob(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Blob(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Blob { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Blob) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Blob> for ::js_sys::Object {
    #[inline]
    fn from(obj: Blob) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Blob {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Blob",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_Blob() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <Blob as WasmDescribe>::describe();
}
impl Blob {
    #[cfg(all(feature = "Blob",))]
    #[allow(bad_style)]
    #[doc = "The `new Blob(..)` constructor, creating a new instance of `Blob`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)\n\n*This API requires the following crate features to be activated: `Blob`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<Blob, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_Blob() -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_Blob() -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_Blob() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Blob as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Blob",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_buffer_source_sequence_Blob() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <Blob as WasmDescribe>::describe();
}
impl Blob {
    #[cfg(all(feature = "Blob",))]
    #[allow(bad_style)]
    #[doc = "The `new Blob(..)` constructor, creating a new instance of `Blob`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)\n\n*This API requires the following crate features to be activated: `Blob`*"]
    #[allow(clippy::all)]
    pub fn new_with_buffer_source_sequence(
        blob_parts: &::wasm_bindgen::JsValue,
    ) -> Result<Blob, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_buffer_source_sequence_Blob(
                blob_parts: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_buffer_source_sequence_Blob(
            blob_parts: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(blob_parts);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let blob_parts =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        blob_parts,
                    );
                __widl_f_new_with_buffer_source_sequence_Blob(blob_parts)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Blob as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Blob",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_u8_array_sequence_Blob() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <Blob as WasmDescribe>::describe();
}
impl Blob {
    #[cfg(all(feature = "Blob",))]
    #[allow(bad_style)]
    #[doc = "The `new Blob(..)` constructor, creating a new instance of `Blob`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)\n\n*This API requires the following crate features to be activated: `Blob`*"]
    #[allow(clippy::all)]
    pub fn new_with_u8_array_sequence(
        blob_parts: &::wasm_bindgen::JsValue,
    ) -> Result<Blob, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_u8_array_sequence_Blob(
                blob_parts: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_u8_array_sequence_Blob(
            blob_parts: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(blob_parts);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let blob_parts =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        blob_parts,
                    );
                __widl_f_new_with_u8_array_sequence_Blob(blob_parts)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Blob as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Blob",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_blob_sequence_Blob() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <Blob as WasmDescribe>::describe();
}
impl Blob {
    #[cfg(all(feature = "Blob",))]
    #[allow(bad_style)]
    #[doc = "The `new Blob(..)` constructor, creating a new instance of `Blob`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)\n\n*This API requires the following crate features to be activated: `Blob`*"]
    #[allow(clippy::all)]
    pub fn new_with_blob_sequence(
        blob_parts: &::wasm_bindgen::JsValue,
    ) -> Result<Blob, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_blob_sequence_Blob(
                blob_parts: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_blob_sequence_Blob(
            blob_parts: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(blob_parts);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let blob_parts =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        blob_parts,
                    );
                __widl_f_new_with_blob_sequence_Blob(blob_parts)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Blob as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Blob",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_str_sequence_Blob() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <Blob as WasmDescribe>::describe();
}
impl Blob {
    #[cfg(all(feature = "Blob",))]
    #[allow(bad_style)]
    #[doc = "The `new Blob(..)` constructor, creating a new instance of `Blob`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)\n\n*This API requires the following crate features to be activated: `Blob`*"]
    #[allow(clippy::all)]
    pub fn new_with_str_sequence(
        blob_parts: &::wasm_bindgen::JsValue,
    ) -> Result<Blob, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_str_sequence_Blob(
                blob_parts: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_str_sequence_Blob(
            blob_parts: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(blob_parts);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let blob_parts =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        blob_parts,
                    );
                __widl_f_new_with_str_sequence_Blob(blob_parts)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Blob as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Blob", feature = "BlobPropertyBag",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_buffer_source_sequence_and_options_Blob() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&BlobPropertyBag as WasmDescribe>::describe();
    <Blob as WasmDescribe>::describe();
}
impl Blob {
    #[cfg(all(feature = "Blob", feature = "BlobPropertyBag",))]
    #[allow(bad_style)]
    #[doc = "The `new Blob(..)` constructor, creating a new instance of `Blob`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)\n\n*This API requires the following crate features to be activated: `Blob`, `BlobPropertyBag`*"]
    #[allow(clippy::all)]
    pub fn new_with_buffer_source_sequence_and_options(
        blob_parts: &::wasm_bindgen::JsValue,
        options: &BlobPropertyBag,
    ) -> Result<Blob, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob", feature = "BlobPropertyBag",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_buffer_source_sequence_and_options_Blob(
                blob_parts: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&BlobPropertyBag as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_buffer_source_sequence_and_options_Blob(
            blob_parts: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&BlobPropertyBag as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(blob_parts);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let blob_parts =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        blob_parts,
                    );
                let options =
                    <&BlobPropertyBag as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_new_with_buffer_source_sequence_and_options_Blob(blob_parts, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Blob as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Blob", feature = "BlobPropertyBag",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_u8_array_sequence_and_options_Blob() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&BlobPropertyBag as WasmDescribe>::describe();
    <Blob as WasmDescribe>::describe();
}
impl Blob {
    #[cfg(all(feature = "Blob", feature = "BlobPropertyBag",))]
    #[allow(bad_style)]
    #[doc = "The `new Blob(..)` constructor, creating a new instance of `Blob`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)\n\n*This API requires the following crate features to be activated: `Blob`, `BlobPropertyBag`*"]
    #[allow(clippy::all)]
    pub fn new_with_u8_array_sequence_and_options(
        blob_parts: &::wasm_bindgen::JsValue,
        options: &BlobPropertyBag,
    ) -> Result<Blob, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob", feature = "BlobPropertyBag",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_u8_array_sequence_and_options_Blob(
                blob_parts: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&BlobPropertyBag as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_u8_array_sequence_and_options_Blob(
            blob_parts: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&BlobPropertyBag as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(blob_parts);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let blob_parts =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        blob_parts,
                    );
                let options =
                    <&BlobPropertyBag as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_new_with_u8_array_sequence_and_options_Blob(blob_parts, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Blob as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Blob", feature = "BlobPropertyBag",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_blob_sequence_and_options_Blob() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&BlobPropertyBag as WasmDescribe>::describe();
    <Blob as WasmDescribe>::describe();
}
impl Blob {
    #[cfg(all(feature = "Blob", feature = "BlobPropertyBag",))]
    #[allow(bad_style)]
    #[doc = "The `new Blob(..)` constructor, creating a new instance of `Blob`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)\n\n*This API requires the following crate features to be activated: `Blob`, `BlobPropertyBag`*"]
    #[allow(clippy::all)]
    pub fn new_with_blob_sequence_and_options(
        blob_parts: &::wasm_bindgen::JsValue,
        options: &BlobPropertyBag,
    ) -> Result<Blob, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob", feature = "BlobPropertyBag",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_blob_sequence_and_options_Blob(
                blob_parts: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&BlobPropertyBag as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_blob_sequence_and_options_Blob(
            blob_parts: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&BlobPropertyBag as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(blob_parts);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let blob_parts =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        blob_parts,
                    );
                let options =
                    <&BlobPropertyBag as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_new_with_blob_sequence_and_options_Blob(blob_parts, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Blob as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Blob", feature = "BlobPropertyBag",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_str_sequence_and_options_Blob() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&BlobPropertyBag as WasmDescribe>::describe();
    <Blob as WasmDescribe>::describe();
}
impl Blob {
    #[cfg(all(feature = "Blob", feature = "BlobPropertyBag",))]
    #[allow(bad_style)]
    #[doc = "The `new Blob(..)` constructor, creating a new instance of `Blob`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)\n\n*This API requires the following crate features to be activated: `Blob`, `BlobPropertyBag`*"]
    #[allow(clippy::all)]
    pub fn new_with_str_sequence_and_options(
        blob_parts: &::wasm_bindgen::JsValue,
        options: &BlobPropertyBag,
    ) -> Result<Blob, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob", feature = "BlobPropertyBag",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_str_sequence_and_options_Blob(
                blob_parts: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&BlobPropertyBag as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_str_sequence_and_options_Blob(
            blob_parts: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&BlobPropertyBag as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(blob_parts);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let blob_parts =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        blob_parts,
                    );
                let options =
                    <&BlobPropertyBag as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_new_with_str_sequence_and_options_Blob(blob_parts, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Blob as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Blob",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_array_buffer_Blob() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Blob as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Blob {
    #[cfg(all(feature = "Blob",))]
    #[allow(bad_style)]
    #[doc = "The `arrayBuffer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/arrayBuffer)\n\n*This API requires the following crate features to be activated: `Blob`*"]
    #[allow(clippy::all)]
    pub fn array_buffer(&self) -> ::js_sys::Promise {
        #[cfg(all(feature = "Blob",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_array_buffer_Blob(
                self_: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_array_buffer_Blob(
            self_: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Blob as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_array_buffer_Blob(self_)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Blob",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_slice_Blob() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Blob as WasmDescribe>::describe();
    <Blob as WasmDescribe>::describe();
}
impl Blob {
    #[cfg(all(feature = "Blob",))]
    #[allow(bad_style)]
    #[doc = "The `slice()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)\n\n*This API requires the following crate features to be activated: `Blob`*"]
    #[allow(clippy::all)]
    pub fn slice(&self) -> Result<Blob, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_slice_Blob(
                self_: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_slice_Blob(
            self_: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Blob as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_slice_Blob(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Blob as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Blob",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_slice_with_i32_Blob() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Blob as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <Blob as WasmDescribe>::describe();
}
impl Blob {
    #[cfg(all(feature = "Blob",))]
    #[allow(bad_style)]
    #[doc = "The `slice()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)\n\n*This API requires the following crate features to be activated: `Blob`*"]
    #[allow(clippy::all)]
    pub fn slice_with_i32(&self, start: i32) -> Result<Blob, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_slice_with_i32_Blob(
                self_: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_slice_with_i32_Blob(
            self_: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(start);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Blob as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let start = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start);
                __widl_f_slice_with_i32_Blob(self_, start)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Blob as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Blob",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_slice_with_f64_Blob() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Blob as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <Blob as WasmDescribe>::describe();
}
impl Blob {
    #[cfg(all(feature = "Blob",))]
    #[allow(bad_style)]
    #[doc = "The `slice()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)\n\n*This API requires the following crate features to be activated: `Blob`*"]
    #[allow(clippy::all)]
    pub fn slice_with_f64(&self, start: f64) -> Result<Blob, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_slice_with_f64_Blob(
                self_: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_slice_with_f64_Blob(
            self_: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(start);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Blob as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let start = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start);
                __widl_f_slice_with_f64_Blob(self_, start)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Blob as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Blob",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_slice_with_i32_and_i32_Blob() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Blob as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <Blob as WasmDescribe>::describe();
}
impl Blob {
    #[cfg(all(feature = "Blob",))]
    #[allow(bad_style)]
    #[doc = "The `slice()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)\n\n*This API requires the following crate features to be activated: `Blob`*"]
    #[allow(clippy::all)]
    pub fn slice_with_i32_and_i32(
        &self,
        start: i32,
        end: i32,
    ) -> Result<Blob, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_slice_with_i32_and_i32_Blob(
                self_: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_slice_with_i32_and_i32_Blob(
            self_: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            end: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(start);
            drop(end);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Blob as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let start = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start);
                let end = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end);
                __widl_f_slice_with_i32_and_i32_Blob(self_, start, end)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Blob as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Blob",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_slice_with_f64_and_i32_Blob() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Blob as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <Blob as WasmDescribe>::describe();
}
impl Blob {
    #[cfg(all(feature = "Blob",))]
    #[allow(bad_style)]
    #[doc = "The `slice()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)\n\n*This API requires the following crate features to be activated: `Blob`*"]
    #[allow(clippy::all)]
    pub fn slice_with_f64_and_i32(
        &self,
        start: f64,
        end: i32,
    ) -> Result<Blob, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_slice_with_f64_and_i32_Blob(
                self_: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_slice_with_f64_and_i32_Blob(
            self_: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            end: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(start);
            drop(end);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Blob as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let start = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start);
                let end = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end);
                __widl_f_slice_with_f64_and_i32_Blob(self_, start, end)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Blob as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Blob",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_slice_with_i32_and_f64_Blob() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Blob as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <Blob as WasmDescribe>::describe();
}
impl Blob {
    #[cfg(all(feature = "Blob",))]
    #[allow(bad_style)]
    #[doc = "The `slice()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)\n\n*This API requires the following crate features to be activated: `Blob`*"]
    #[allow(clippy::all)]
    pub fn slice_with_i32_and_f64(
        &self,
        start: i32,
        end: f64,
    ) -> Result<Blob, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_slice_with_i32_and_f64_Blob(
                self_: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_slice_with_i32_and_f64_Blob(
            self_: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            end: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(start);
            drop(end);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Blob as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let start = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start);
                let end = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end);
                __widl_f_slice_with_i32_and_f64_Blob(self_, start, end)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Blob as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Blob",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_slice_with_f64_and_f64_Blob() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Blob as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <Blob as WasmDescribe>::describe();
}
impl Blob {
    #[cfg(all(feature = "Blob",))]
    #[allow(bad_style)]
    #[doc = "The `slice()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)\n\n*This API requires the following crate features to be activated: `Blob`*"]
    #[allow(clippy::all)]
    pub fn slice_with_f64_and_f64(
        &self,
        start: f64,
        end: f64,
    ) -> Result<Blob, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_slice_with_f64_and_f64_Blob(
                self_: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_slice_with_f64_and_f64_Blob(
            self_: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            end: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(start);
            drop(end);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Blob as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let start = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start);
                let end = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end);
                __widl_f_slice_with_f64_and_f64_Blob(self_, start, end)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Blob as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Blob",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_slice_with_i32_and_i32_and_content_type_Blob() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Blob as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Blob as WasmDescribe>::describe();
}
impl Blob {
    #[cfg(all(feature = "Blob",))]
    #[allow(bad_style)]
    #[doc = "The `slice()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)\n\n*This API requires the following crate features to be activated: `Blob`*"]
    #[allow(clippy::all)]
    pub fn slice_with_i32_and_i32_and_content_type(
        &self,
        start: i32,
        end: i32,
        content_type: &str,
    ) -> Result<Blob, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_slice_with_i32_and_i32_and_content_type_Blob(
                self_: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                content_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_slice_with_i32_and_i32_and_content_type_Blob(
            self_: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            end: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            content_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(start);
            drop(end);
            drop(content_type);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Blob as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let start = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start);
                let end = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end);
                let content_type =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(content_type);
                __widl_f_slice_with_i32_and_i32_and_content_type_Blob(
                    self_,
                    start,
                    end,
                    content_type,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Blob as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Blob",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_slice_with_f64_and_i32_and_content_type_Blob() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Blob as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Blob as WasmDescribe>::describe();
}
impl Blob {
    #[cfg(all(feature = "Blob",))]
    #[allow(bad_style)]
    #[doc = "The `slice()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)\n\n*This API requires the following crate features to be activated: `Blob`*"]
    #[allow(clippy::all)]
    pub fn slice_with_f64_and_i32_and_content_type(
        &self,
        start: f64,
        end: i32,
        content_type: &str,
    ) -> Result<Blob, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_slice_with_f64_and_i32_and_content_type_Blob(
                self_: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                content_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_slice_with_f64_and_i32_and_content_type_Blob(
            self_: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            end: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            content_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(start);
            drop(end);
            drop(content_type);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Blob as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let start = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start);
                let end = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end);
                let content_type =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(content_type);
                __widl_f_slice_with_f64_and_i32_and_content_type_Blob(
                    self_,
                    start,
                    end,
                    content_type,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Blob as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Blob",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_slice_with_i32_and_f64_and_content_type_Blob() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Blob as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Blob as WasmDescribe>::describe();
}
impl Blob {
    #[cfg(all(feature = "Blob",))]
    #[allow(bad_style)]
    #[doc = "The `slice()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)\n\n*This API requires the following crate features to be activated: `Blob`*"]
    #[allow(clippy::all)]
    pub fn slice_with_i32_and_f64_and_content_type(
        &self,
        start: i32,
        end: f64,
        content_type: &str,
    ) -> Result<Blob, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_slice_with_i32_and_f64_and_content_type_Blob(
                self_: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                content_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_slice_with_i32_and_f64_and_content_type_Blob(
            self_: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            end: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            content_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(start);
            drop(end);
            drop(content_type);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Blob as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let start = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start);
                let end = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end);
                let content_type =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(content_type);
                __widl_f_slice_with_i32_and_f64_and_content_type_Blob(
                    self_,
                    start,
                    end,
                    content_type,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Blob as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Blob",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_slice_with_f64_and_f64_and_content_type_Blob() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Blob as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Blob as WasmDescribe>::describe();
}
impl Blob {
    #[cfg(all(feature = "Blob",))]
    #[allow(bad_style)]
    #[doc = "The `slice()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)\n\n*This API requires the following crate features to be activated: `Blob`*"]
    #[allow(clippy::all)]
    pub fn slice_with_f64_and_f64_and_content_type(
        &self,
        start: f64,
        end: f64,
        content_type: &str,
    ) -> Result<Blob, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_slice_with_f64_and_f64_and_content_type_Blob(
                self_: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                content_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_slice_with_f64_and_f64_and_content_type_Blob(
            self_: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            end: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            content_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(start);
            drop(end);
            drop(content_type);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Blob as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let start = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start);
                let end = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end);
                let content_type =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(content_type);
                __widl_f_slice_with_f64_and_f64_and_content_type_Blob(
                    self_,
                    start,
                    end,
                    content_type,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Blob as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Blob",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_text_Blob() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Blob as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Blob {
    #[cfg(all(feature = "Blob",))]
    #[allow(bad_style)]
    #[doc = "The `text()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/text)\n\n*This API requires the following crate features to be activated: `Blob`*"]
    #[allow(clippy::all)]
    pub fn text(&self) -> ::js_sys::Promise {
        #[cfg(all(feature = "Blob",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_text_Blob(
                self_: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_text_Blob(
            self_: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Blob as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_text_Blob(self_)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Blob",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_size_Blob() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Blob as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl Blob {
    #[cfg(all(feature = "Blob",))]
    #[allow(bad_style)]
    #[doc = "The `size` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/size)\n\n*This API requires the following crate features to be activated: `Blob`*"]
    #[allow(clippy::all)]
    pub fn size(&self) -> f64 {
        #[cfg(all(feature = "Blob",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_size_Blob(
                self_: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_size_Blob(
            self_: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Blob as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_size_Blob(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Blob",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_Blob() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Blob as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Blob {
    #[cfg(all(feature = "Blob",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/type)\n\n*This API requires the following crate features to be activated: `Blob`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> String {
        #[cfg(all(feature = "Blob",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_Blob(
                self_: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_Blob(
            self_: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Blob as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_Blob(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_13ce08ff2481306f: [u8; 1932usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}J\x07\0\0\0\0\x19\0\0\x02\x04Blob\x16__widl_instanceof_Blob\0\0\0\0\x11__widl_f_new_Blob\x01\0\0\x01\x04Blob\0\x01\0\x03new\0\0\0-__widl_f_new_with_buffer_source_sequence_Blob\x01\0\0\x01\x04Blob\0\x01\x01\nblob_parts\x03new\0\0\0(__widl_f_new_with_u8_array_sequence_Blob\x01\0\0\x01\x04Blob\0\x01\x01\nblob_parts\x03new\0\0\0$__widl_f_new_with_blob_sequence_Blob\x01\0\0\x01\x04Blob\0\x01\x01\nblob_parts\x03new\0\0\0#__widl_f_new_with_str_sequence_Blob\x01\0\0\x01\x04Blob\0\x01\x01\nblob_parts\x03new\0\0\09__widl_f_new_with_buffer_source_sequence_and_options_Blob\x01\0\0\x01\x04Blob\0\x01\x02\nblob_parts\x07options\x03new\0\0\04__widl_f_new_with_u8_array_sequence_and_options_Blob\x01\0\0\x01\x04Blob\0\x01\x02\nblob_parts\x07options\x03new\0\0\00__widl_f_new_with_blob_sequence_and_options_Blob\x01\0\0\x01\x04Blob\0\x01\x02\nblob_parts\x07options\x03new\0\0\0/__widl_f_new_with_str_sequence_and_options_Blob\x01\0\0\x01\x04Blob\0\x01\x02\nblob_parts\x07options\x03new\0\0\0\x1A__widl_f_array_buffer_Blob\0\0\0\x01\x04Blob\x01\0\0\x01\x01\x05self_\x0BarrayBuffer\0\0\0\x13__widl_f_slice_Blob\x01\0\0\x01\x04Blob\x01\0\0\x01\x01\x05self_\x05slice\0\0\0\x1C__widl_f_slice_with_i32_Blob\x01\0\0\x01\x04Blob\x01\0\0\x01\x02\x05self_\x05start\x05slice\0\0\0\x1C__widl_f_slice_with_f64_Blob\x01\0\0\x01\x04Blob\x01\0\0\x01\x02\x05self_\x05start\x05slice\0\0\0$__widl_f_slice_with_i32_and_i32_Blob\x01\0\0\x01\x04Blob\x01\0\0\x01\x03\x05self_\x05start\x03end\x05slice\0\0\0$__widl_f_slice_with_f64_and_i32_Blob\x01\0\0\x01\x04Blob\x01\0\0\x01\x03\x05self_\x05start\x03end\x05slice\0\0\0$__widl_f_slice_with_i32_and_f64_Blob\x01\0\0\x01\x04Blob\x01\0\0\x01\x03\x05self_\x05start\x03end\x05slice\0\0\0$__widl_f_slice_with_f64_and_f64_Blob\x01\0\0\x01\x04Blob\x01\0\0\x01\x03\x05self_\x05start\x03end\x05slice\0\0\05__widl_f_slice_with_i32_and_i32_and_content_type_Blob\x01\0\0\x01\x04Blob\x01\0\0\x01\x04\x05self_\x05start\x03end\x0Ccontent_type\x05slice\0\0\05__widl_f_slice_with_f64_and_i32_and_content_type_Blob\x01\0\0\x01\x04Blob\x01\0\0\x01\x04\x05self_\x05start\x03end\x0Ccontent_type\x05slice\0\0\05__widl_f_slice_with_i32_and_f64_and_content_type_Blob\x01\0\0\x01\x04Blob\x01\0\0\x01\x04\x05self_\x05start\x03end\x0Ccontent_type\x05slice\0\0\05__widl_f_slice_with_f64_and_f64_and_content_type_Blob\x01\0\0\x01\x04Blob\x01\0\0\x01\x04\x05self_\x05start\x03end\x0Ccontent_type\x05slice\0\0\0\x12__widl_f_text_Blob\0\0\0\x01\x04Blob\x01\0\0\x01\x01\x05self_\x04text\0\0\0\x12__widl_f_size_Blob\0\0\0\x01\x04Blob\x01\0\x01\x04size\x01\x01\x05self_\x04size\0\0\0\x12__widl_f_type_Blob\0\0\0\x01\x04Blob\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
