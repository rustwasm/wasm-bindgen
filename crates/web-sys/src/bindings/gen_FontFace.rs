use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `FontFace` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct FontFace {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_FontFace: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for FontFace {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(8u32);
            inform(70u32);
            inform(111u32);
            inform(110u32);
            inform(116u32);
            inform(70u32);
            inform(97u32);
            inform(99u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for FontFace {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for FontFace {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for FontFace {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a FontFace {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for FontFace {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            FontFace {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for FontFace {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a FontFace {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for FontFace {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<FontFace>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(FontFace {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for FontFace {
        #[inline]
        fn from(obj: JsValue) -> FontFace {
            FontFace { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for FontFace {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<FontFace> for FontFace {
        #[inline]
        fn as_ref(&self) -> &FontFace {
            self
        }
    }
    impl From<FontFace> for JsValue {
        #[inline]
        fn from(obj: FontFace) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for FontFace {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_FontFace(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_FontFace(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_FontFace(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            FontFace { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const FontFace) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<FontFace> for ::js_sys::Object {
    #[inline]
    fn from(obj: FontFace) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for FontFace {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "FontFace",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_str_FontFace() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <FontFace as WasmDescribe>::describe();
}
impl FontFace {
    #[cfg(all(feature = "FontFace",))]
    #[allow(bad_style)]
    #[doc = "The `new FontFace(..)` constructor, creating a new instance of `FontFace`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    #[allow(clippy::all)]
    pub fn new_with_str(family: &str, source: &str) -> Result<FontFace, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "FontFace",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_str_FontFace(
                family: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                source: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <FontFace as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_str_FontFace(
            family: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            source: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <FontFace as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(family);
            drop(source);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let family = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(family);
                let source = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(source);
                __widl_f_new_with_str_FontFace(family, source)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<FontFace as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "FontFace",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_array_buffer_FontFace() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&::js_sys::ArrayBuffer as WasmDescribe>::describe();
    <FontFace as WasmDescribe>::describe();
}
impl FontFace {
    #[cfg(all(feature = "FontFace",))]
    #[allow(bad_style)]
    #[doc = "The `new FontFace(..)` constructor, creating a new instance of `FontFace`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    #[allow(clippy::all)]
    pub fn new_with_array_buffer(
        family: &str,
        source: &::js_sys::ArrayBuffer,
    ) -> Result<FontFace, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "FontFace",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_array_buffer_FontFace(
                family: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                source: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <FontFace as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_array_buffer_FontFace(
            family: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            source: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <FontFace as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(family);
            drop(source);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let family = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(family);
                let source =
                    <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        source,
                    );
                __widl_f_new_with_array_buffer_FontFace(family, source)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<FontFace as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "FontFace",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_array_buffer_view_FontFace() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <FontFace as WasmDescribe>::describe();
}
impl FontFace {
    #[cfg(all(feature = "FontFace",))]
    #[allow(bad_style)]
    #[doc = "The `new FontFace(..)` constructor, creating a new instance of `FontFace`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    #[allow(clippy::all)]
    pub fn new_with_array_buffer_view(
        family: &str,
        source: &::js_sys::Object,
    ) -> Result<FontFace, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "FontFace",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_array_buffer_view_FontFace(
                family: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                source: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <FontFace as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_array_buffer_view_FontFace(
            family: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            source: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <FontFace as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(family);
            drop(source);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let family = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(family);
                let source =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(source);
                __widl_f_new_with_array_buffer_view_FontFace(family, source)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<FontFace as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "FontFace",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_u8_array_FontFace() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <FontFace as WasmDescribe>::describe();
}
impl FontFace {
    #[cfg(all(feature = "FontFace",))]
    #[allow(bad_style)]
    #[doc = "The `new FontFace(..)` constructor, creating a new instance of `FontFace`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    #[allow(clippy::all)]
    pub fn new_with_u8_array(
        family: &str,
        source: &mut [u8],
    ) -> Result<FontFace, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "FontFace",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_u8_array_FontFace(
                family: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                source: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <FontFace as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_u8_array_FontFace(
            family: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            source: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <FontFace as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(family);
            drop(source);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let family = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(family);
                let source = <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(source);
                __widl_f_new_with_u8_array_FontFace(family, source)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<FontFace as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "FontFace", feature = "FontFaceDescriptors",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_str_and_descriptors_FontFace() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&FontFaceDescriptors as WasmDescribe>::describe();
    <FontFace as WasmDescribe>::describe();
}
impl FontFace {
    #[cfg(all(feature = "FontFace", feature = "FontFaceDescriptors",))]
    #[allow(bad_style)]
    #[doc = "The `new FontFace(..)` constructor, creating a new instance of `FontFace`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)\n\n*This API requires the following crate features to be activated: `FontFace`, `FontFaceDescriptors`*"]
    #[allow(clippy::all)]
    pub fn new_with_str_and_descriptors(
        family: &str,
        source: &str,
        descriptors: &FontFaceDescriptors,
    ) -> Result<FontFace, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "FontFace", feature = "FontFaceDescriptors",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_str_and_descriptors_FontFace(
                family: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                source: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                descriptors: <&FontFaceDescriptors as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <FontFace as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_str_and_descriptors_FontFace(
            family: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            source: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            descriptors: <&FontFaceDescriptors as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <FontFace as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(family);
            drop(source);
            drop(descriptors);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let family = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(family);
                let source = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(source);
                let descriptors =
                    <&FontFaceDescriptors as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        descriptors,
                    );
                __widl_f_new_with_str_and_descriptors_FontFace(family, source, descriptors)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<FontFace as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "FontFace", feature = "FontFaceDescriptors",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_array_buffer_and_descriptors_FontFace() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&str as WasmDescribe>::describe();
    <&::js_sys::ArrayBuffer as WasmDescribe>::describe();
    <&FontFaceDescriptors as WasmDescribe>::describe();
    <FontFace as WasmDescribe>::describe();
}
impl FontFace {
    #[cfg(all(feature = "FontFace", feature = "FontFaceDescriptors",))]
    #[allow(bad_style)]
    #[doc = "The `new FontFace(..)` constructor, creating a new instance of `FontFace`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)\n\n*This API requires the following crate features to be activated: `FontFace`, `FontFaceDescriptors`*"]
    #[allow(clippy::all)]
    pub fn new_with_array_buffer_and_descriptors(
        family: &str,
        source: &::js_sys::ArrayBuffer,
        descriptors: &FontFaceDescriptors,
    ) -> Result<FontFace, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "FontFace", feature = "FontFaceDescriptors",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_array_buffer_and_descriptors_FontFace(
                family: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                source: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                descriptors: <&FontFaceDescriptors as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <FontFace as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_array_buffer_and_descriptors_FontFace(
            family: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            source: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            descriptors: <&FontFaceDescriptors as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <FontFace as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(family);
            drop(source);
            drop(descriptors);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let family = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(family);
                let source =
                    <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        source,
                    );
                let descriptors =
                    <&FontFaceDescriptors as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        descriptors,
                    );
                __widl_f_new_with_array_buffer_and_descriptors_FontFace(family, source, descriptors)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<FontFace as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "FontFace", feature = "FontFaceDescriptors",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_array_buffer_view_and_descriptors_FontFace()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&str as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <&FontFaceDescriptors as WasmDescribe>::describe();
    <FontFace as WasmDescribe>::describe();
}
impl FontFace {
    #[cfg(all(feature = "FontFace", feature = "FontFaceDescriptors",))]
    #[allow(bad_style)]
    #[doc = "The `new FontFace(..)` constructor, creating a new instance of `FontFace`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)\n\n*This API requires the following crate features to be activated: `FontFace`, `FontFaceDescriptors`*"]
    #[allow(clippy::all)]
    pub fn new_with_array_buffer_view_and_descriptors(
        family: &str,
        source: &::js_sys::Object,
        descriptors: &FontFaceDescriptors,
    ) -> Result<FontFace, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "FontFace", feature = "FontFaceDescriptors",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_array_buffer_view_and_descriptors_FontFace(
                family: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                source: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                descriptors: <&FontFaceDescriptors as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <FontFace as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_array_buffer_view_and_descriptors_FontFace(
            family: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            source: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            descriptors: <&FontFaceDescriptors as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <FontFace as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(family);
            drop(source);
            drop(descriptors);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let family = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(family);
                let source =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(source);
                let descriptors =
                    <&FontFaceDescriptors as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        descriptors,
                    );
                __widl_f_new_with_array_buffer_view_and_descriptors_FontFace(
                    family,
                    source,
                    descriptors,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<FontFace as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "FontFace", feature = "FontFaceDescriptors",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_u8_array_and_descriptors_FontFace() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&str as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <&FontFaceDescriptors as WasmDescribe>::describe();
    <FontFace as WasmDescribe>::describe();
}
impl FontFace {
    #[cfg(all(feature = "FontFace", feature = "FontFaceDescriptors",))]
    #[allow(bad_style)]
    #[doc = "The `new FontFace(..)` constructor, creating a new instance of `FontFace`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)\n\n*This API requires the following crate features to be activated: `FontFace`, `FontFaceDescriptors`*"]
    #[allow(clippy::all)]
    pub fn new_with_u8_array_and_descriptors(
        family: &str,
        source: &mut [u8],
        descriptors: &FontFaceDescriptors,
    ) -> Result<FontFace, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "FontFace", feature = "FontFaceDescriptors",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_u8_array_and_descriptors_FontFace(
                family: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                source: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                descriptors: <&FontFaceDescriptors as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <FontFace as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_u8_array_and_descriptors_FontFace(
            family: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            source: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            descriptors: <&FontFaceDescriptors as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <FontFace as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(family);
            drop(source);
            drop(descriptors);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let family = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(family);
                let source = <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(source);
                let descriptors =
                    <&FontFaceDescriptors as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        descriptors,
                    );
                __widl_f_new_with_u8_array_and_descriptors_FontFace(family, source, descriptors)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<FontFace as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "FontFace",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_load_FontFace() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FontFace as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl FontFace {
    #[cfg(all(feature = "FontFace",))]
    #[allow(bad_style)]
    #[doc = "The `load()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/load)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    #[allow(clippy::all)]
    pub fn load(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "FontFace",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_load_FontFace(
                self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_load_FontFace(
            self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_load_FontFace(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "FontFace",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_family_FontFace() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FontFace as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl FontFace {
    #[cfg(all(feature = "FontFace",))]
    #[allow(bad_style)]
    #[doc = "The `family` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/family)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    #[allow(clippy::all)]
    pub fn family(&self) -> String {
        #[cfg(all(feature = "FontFace",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_family_FontFace(
                self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_family_FontFace(
            self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_family_FontFace(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FontFace",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_family_FontFace() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FontFace as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FontFace {
    #[cfg(all(feature = "FontFace",))]
    #[allow(bad_style)]
    #[doc = "The `family` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/family)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    #[allow(clippy::all)]
    pub fn set_family(&self, family: &str) {
        #[cfg(all(feature = "FontFace",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_family_FontFace(
                self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                family: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_family_FontFace(
            self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            family: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(family);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let family = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(family);
                __widl_f_set_family_FontFace(self_, family)
            };
            ()
        }
    }
}
#[cfg(all(feature = "FontFace",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_style_FontFace() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FontFace as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl FontFace {
    #[cfg(all(feature = "FontFace",))]
    #[allow(bad_style)]
    #[doc = "The `style` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/style)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    #[allow(clippy::all)]
    pub fn style(&self) -> String {
        #[cfg(all(feature = "FontFace",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_style_FontFace(
                self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_style_FontFace(
            self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_style_FontFace(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FontFace",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_style_FontFace() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FontFace as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FontFace {
    #[cfg(all(feature = "FontFace",))]
    #[allow(bad_style)]
    #[doc = "The `style` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/style)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    #[allow(clippy::all)]
    pub fn set_style(&self, style: &str) {
        #[cfg(all(feature = "FontFace",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_style_FontFace(
                self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                style: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_style_FontFace(
            self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            style: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(style);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let style = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(style);
                __widl_f_set_style_FontFace(self_, style)
            };
            ()
        }
    }
}
#[cfg(all(feature = "FontFace",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_weight_FontFace() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FontFace as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl FontFace {
    #[cfg(all(feature = "FontFace",))]
    #[allow(bad_style)]
    #[doc = "The `weight` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/weight)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    #[allow(clippy::all)]
    pub fn weight(&self) -> String {
        #[cfg(all(feature = "FontFace",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_weight_FontFace(
                self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_weight_FontFace(
            self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_weight_FontFace(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FontFace",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_weight_FontFace() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FontFace as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FontFace {
    #[cfg(all(feature = "FontFace",))]
    #[allow(bad_style)]
    #[doc = "The `weight` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/weight)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    #[allow(clippy::all)]
    pub fn set_weight(&self, weight: &str) {
        #[cfg(all(feature = "FontFace",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_weight_FontFace(
                self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                weight: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_weight_FontFace(
            self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            weight: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(weight);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let weight = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(weight);
                __widl_f_set_weight_FontFace(self_, weight)
            };
            ()
        }
    }
}
#[cfg(all(feature = "FontFace",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stretch_FontFace() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FontFace as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl FontFace {
    #[cfg(all(feature = "FontFace",))]
    #[allow(bad_style)]
    #[doc = "The `stretch` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/stretch)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    #[allow(clippy::all)]
    pub fn stretch(&self) -> String {
        #[cfg(all(feature = "FontFace",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stretch_FontFace(
                self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stretch_FontFace(
            self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_stretch_FontFace(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FontFace",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_stretch_FontFace() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FontFace as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FontFace {
    #[cfg(all(feature = "FontFace",))]
    #[allow(bad_style)]
    #[doc = "The `stretch` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/stretch)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    #[allow(clippy::all)]
    pub fn set_stretch(&self, stretch: &str) {
        #[cfg(all(feature = "FontFace",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_stretch_FontFace(
                self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                stretch: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_stretch_FontFace(
            self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            stretch: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(stretch);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let stretch = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(stretch);
                __widl_f_set_stretch_FontFace(self_, stretch)
            };
            ()
        }
    }
}
#[cfg(all(feature = "FontFace",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_unicode_range_FontFace() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FontFace as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl FontFace {
    #[cfg(all(feature = "FontFace",))]
    #[allow(bad_style)]
    #[doc = "The `unicodeRange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/unicodeRange)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    #[allow(clippy::all)]
    pub fn unicode_range(&self) -> String {
        #[cfg(all(feature = "FontFace",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_unicode_range_FontFace(
                self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_unicode_range_FontFace(
            self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_unicode_range_FontFace(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FontFace",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_unicode_range_FontFace() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FontFace as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FontFace {
    #[cfg(all(feature = "FontFace",))]
    #[allow(bad_style)]
    #[doc = "The `unicodeRange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/unicodeRange)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    #[allow(clippy::all)]
    pub fn set_unicode_range(&self, unicode_range: &str) {
        #[cfg(all(feature = "FontFace",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_unicode_range_FontFace(
                self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unicode_range: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_unicode_range_FontFace(
            self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unicode_range: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(unicode_range);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let unicode_range =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(unicode_range);
                __widl_f_set_unicode_range_FontFace(self_, unicode_range)
            };
            ()
        }
    }
}
#[cfg(all(feature = "FontFace",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_variant_FontFace() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FontFace as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl FontFace {
    #[cfg(all(feature = "FontFace",))]
    #[allow(bad_style)]
    #[doc = "The `variant` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/variant)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    #[allow(clippy::all)]
    pub fn variant(&self) -> String {
        #[cfg(all(feature = "FontFace",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_variant_FontFace(
                self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_variant_FontFace(
            self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_variant_FontFace(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FontFace",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_variant_FontFace() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FontFace as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FontFace {
    #[cfg(all(feature = "FontFace",))]
    #[allow(bad_style)]
    #[doc = "The `variant` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/variant)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    #[allow(clippy::all)]
    pub fn set_variant(&self, variant: &str) {
        #[cfg(all(feature = "FontFace",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_variant_FontFace(
                self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                variant: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_variant_FontFace(
            self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            variant: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(variant);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let variant = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(variant);
                __widl_f_set_variant_FontFace(self_, variant)
            };
            ()
        }
    }
}
#[cfg(all(feature = "FontFace",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_feature_settings_FontFace() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FontFace as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl FontFace {
    #[cfg(all(feature = "FontFace",))]
    #[allow(bad_style)]
    #[doc = "The `featureSettings` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/featureSettings)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    #[allow(clippy::all)]
    pub fn feature_settings(&self) -> String {
        #[cfg(all(feature = "FontFace",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_feature_settings_FontFace(
                self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_feature_settings_FontFace(
            self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_feature_settings_FontFace(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FontFace",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_feature_settings_FontFace() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FontFace as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FontFace {
    #[cfg(all(feature = "FontFace",))]
    #[allow(bad_style)]
    #[doc = "The `featureSettings` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/featureSettings)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    #[allow(clippy::all)]
    pub fn set_feature_settings(&self, feature_settings: &str) {
        #[cfg(all(feature = "FontFace",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_feature_settings_FontFace(
                self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                feature_settings: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_feature_settings_FontFace(
            self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            feature_settings: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(feature_settings);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let feature_settings =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(feature_settings);
                __widl_f_set_feature_settings_FontFace(self_, feature_settings)
            };
            ()
        }
    }
}
#[cfg(all(feature = "FontFace",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_variation_settings_FontFace() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FontFace as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl FontFace {
    #[cfg(all(feature = "FontFace",))]
    #[allow(bad_style)]
    #[doc = "The `variationSettings` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/variationSettings)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    #[allow(clippy::all)]
    pub fn variation_settings(&self) -> String {
        #[cfg(all(feature = "FontFace",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_variation_settings_FontFace(
                self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_variation_settings_FontFace(
            self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_variation_settings_FontFace(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FontFace",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_variation_settings_FontFace() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FontFace as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FontFace {
    #[cfg(all(feature = "FontFace",))]
    #[allow(bad_style)]
    #[doc = "The `variationSettings` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/variationSettings)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    #[allow(clippy::all)]
    pub fn set_variation_settings(&self, variation_settings: &str) {
        #[cfg(all(feature = "FontFace",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_variation_settings_FontFace(
                self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                variation_settings: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_variation_settings_FontFace(
            self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            variation_settings: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(variation_settings);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let variation_settings =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(variation_settings);
                __widl_f_set_variation_settings_FontFace(self_, variation_settings)
            };
            ()
        }
    }
}
#[cfg(all(feature = "FontFace",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_display_FontFace() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FontFace as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl FontFace {
    #[cfg(all(feature = "FontFace",))]
    #[allow(bad_style)]
    #[doc = "The `display` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/display)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    #[allow(clippy::all)]
    pub fn display(&self) -> String {
        #[cfg(all(feature = "FontFace",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_display_FontFace(
                self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_display_FontFace(
            self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_display_FontFace(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FontFace",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_display_FontFace() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FontFace as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FontFace {
    #[cfg(all(feature = "FontFace",))]
    #[allow(bad_style)]
    #[doc = "The `display` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/display)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    #[allow(clippy::all)]
    pub fn set_display(&self, display: &str) {
        #[cfg(all(feature = "FontFace",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_display_FontFace(
                self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                display: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_display_FontFace(
            self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            display: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(display);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let display = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(display);
                __widl_f_set_display_FontFace(self_, display)
            };
            ()
        }
    }
}
#[cfg(all(feature = "FontFace", feature = "FontFaceLoadStatus",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_status_FontFace() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FontFace as WasmDescribe>::describe();
    <FontFaceLoadStatus as WasmDescribe>::describe();
}
impl FontFace {
    #[cfg(all(feature = "FontFace", feature = "FontFaceLoadStatus",))]
    #[allow(bad_style)]
    #[doc = "The `status` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/status)\n\n*This API requires the following crate features to be activated: `FontFace`, `FontFaceLoadStatus`*"]
    #[allow(clippy::all)]
    pub fn status(&self) -> FontFaceLoadStatus {
        #[cfg(all(feature = "FontFace", feature = "FontFaceLoadStatus",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_status_FontFace(
                self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <FontFaceLoadStatus as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_status_FontFace(
            self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <FontFaceLoadStatus as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_status_FontFace(self_)
            };
            <FontFaceLoadStatus as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FontFace",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_loaded_FontFace() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FontFace as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl FontFace {
    #[cfg(all(feature = "FontFace",))]
    #[allow(bad_style)]
    #[doc = "The `loaded` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/loaded)\n\n*This API requires the following crate features to be activated: `FontFace`*"]
    #[allow(clippy::all)]
    pub fn loaded(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "FontFace",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_loaded_FontFace(
                self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_loaded_FontFace(
            self_: <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FontFace as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_loaded_FontFace(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_4edf61735ccb9282: [u8; 2536usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xA6\t\0\0\0\0\x1E\0\0\x02\x08FontFace\x1A__widl_instanceof_FontFace\0\0\0\0\x1E__widl_f_new_with_str_FontFace\x01\0\0\x01\x08FontFace\0\x01\x02\x06family\x06source\x03new\0\0\0'__widl_f_new_with_array_buffer_FontFace\x01\0\0\x01\x08FontFace\0\x01\x02\x06family\x06source\x03new\0\0\0,__widl_f_new_with_array_buffer_view_FontFace\x01\0\0\x01\x08FontFace\0\x01\x02\x06family\x06source\x03new\0\0\0#__widl_f_new_with_u8_array_FontFace\x01\0\0\x01\x08FontFace\0\x01\x02\x06family\x06source\x03new\0\0\0.__widl_f_new_with_str_and_descriptors_FontFace\x01\0\0\x01\x08FontFace\0\x01\x03\x06family\x06source\x0Bdescriptors\x03new\0\0\07__widl_f_new_with_array_buffer_and_descriptors_FontFace\x01\0\0\x01\x08FontFace\0\x01\x03\x06family\x06source\x0Bdescriptors\x03new\0\0\0<__widl_f_new_with_array_buffer_view_and_descriptors_FontFace\x01\0\0\x01\x08FontFace\0\x01\x03\x06family\x06source\x0Bdescriptors\x03new\0\0\03__widl_f_new_with_u8_array_and_descriptors_FontFace\x01\0\0\x01\x08FontFace\0\x01\x03\x06family\x06source\x0Bdescriptors\x03new\0\0\0\x16__widl_f_load_FontFace\x01\0\0\x01\x08FontFace\x01\0\0\x01\x01\x05self_\x04load\0\0\0\x18__widl_f_family_FontFace\0\0\0\x01\x08FontFace\x01\0\x01\x06family\x01\x01\x05self_\x06family\0\0\0\x1C__widl_f_set_family_FontFace\0\0\0\x01\x08FontFace\x01\0\x02\x06family\x01\x02\x05self_\x06family\x06family\0\0\0\x17__widl_f_style_FontFace\0\0\0\x01\x08FontFace\x01\0\x01\x05style\x01\x01\x05self_\x05style\0\0\0\x1B__widl_f_set_style_FontFace\0\0\0\x01\x08FontFace\x01\0\x02\x05style\x01\x02\x05self_\x05style\x05style\0\0\0\x18__widl_f_weight_FontFace\0\0\0\x01\x08FontFace\x01\0\x01\x06weight\x01\x01\x05self_\x06weight\0\0\0\x1C__widl_f_set_weight_FontFace\0\0\0\x01\x08FontFace\x01\0\x02\x06weight\x01\x02\x05self_\x06weight\x06weight\0\0\0\x19__widl_f_stretch_FontFace\0\0\0\x01\x08FontFace\x01\0\x01\x07stretch\x01\x01\x05self_\x07stretch\0\0\0\x1D__widl_f_set_stretch_FontFace\0\0\0\x01\x08FontFace\x01\0\x02\x07stretch\x01\x02\x05self_\x07stretch\x07stretch\0\0\0\x1F__widl_f_unicode_range_FontFace\0\0\0\x01\x08FontFace\x01\0\x01\x0CunicodeRange\x01\x01\x05self_\x0CunicodeRange\0\0\0#__widl_f_set_unicode_range_FontFace\0\0\0\x01\x08FontFace\x01\0\x02\x0CunicodeRange\x01\x02\x05self_\runicode_range\x0CunicodeRange\0\0\0\x19__widl_f_variant_FontFace\0\0\0\x01\x08FontFace\x01\0\x01\x07variant\x01\x01\x05self_\x07variant\0\0\0\x1D__widl_f_set_variant_FontFace\0\0\0\x01\x08FontFace\x01\0\x02\x07variant\x01\x02\x05self_\x07variant\x07variant\0\0\0\"__widl_f_feature_settings_FontFace\0\0\0\x01\x08FontFace\x01\0\x01\x0FfeatureSettings\x01\x01\x05self_\x0FfeatureSettings\0\0\0&__widl_f_set_feature_settings_FontFace\0\0\0\x01\x08FontFace\x01\0\x02\x0FfeatureSettings\x01\x02\x05self_\x10feature_settings\x0FfeatureSettings\0\0\0$__widl_f_variation_settings_FontFace\0\0\0\x01\x08FontFace\x01\0\x01\x11variationSettings\x01\x01\x05self_\x11variationSettings\0\0\0(__widl_f_set_variation_settings_FontFace\0\0\0\x01\x08FontFace\x01\0\x02\x11variationSettings\x01\x02\x05self_\x12variation_settings\x11variationSettings\0\0\0\x19__widl_f_display_FontFace\0\0\0\x01\x08FontFace\x01\0\x01\x07display\x01\x01\x05self_\x07display\0\0\0\x1D__widl_f_set_display_FontFace\0\0\0\x01\x08FontFace\x01\0\x02\x07display\x01\x02\x05self_\x07display\x07display\0\0\0\x18__widl_f_status_FontFace\0\0\0\x01\x08FontFace\x01\0\x01\x06status\x01\x01\x05self_\x06status\0\0\0\x18__widl_f_loaded_FontFace\x01\0\0\x01\x08FontFace\x01\0\x01\x06loaded\x01\x01\x05self_\x06loaded\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
