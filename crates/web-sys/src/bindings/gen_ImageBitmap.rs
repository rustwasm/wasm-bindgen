use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `ImageBitmap` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap)\n\n*This API requires the following crate features to be activated: `ImageBitmap`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ImageBitmap {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ImageBitmap: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ImageBitmap {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(11u32);
            inform(73u32);
            inform(109u32);
            inform(97u32);
            inform(103u32);
            inform(101u32);
            inform(66u32);
            inform(105u32);
            inform(116u32);
            inform(109u32);
            inform(97u32);
            inform(112u32);
        }
    }
    impl core::ops::Deref for ImageBitmap {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for ImageBitmap {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ImageBitmap {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ImageBitmap {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ImageBitmap {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ImageBitmap {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ImageBitmap {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ImageBitmap {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ImageBitmap {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ImageBitmap>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ImageBitmap {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ImageBitmap {
        #[inline]
        fn from(obj: JsValue) -> ImageBitmap {
            ImageBitmap { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ImageBitmap {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ImageBitmap> for ImageBitmap {
        #[inline]
        fn as_ref(&self) -> &ImageBitmap {
            self
        }
    }
    impl From<ImageBitmap> for JsValue {
        #[inline]
        fn from(obj: ImageBitmap) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ImageBitmap {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_ImageBitmap(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_ImageBitmap(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_ImageBitmap(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ImageBitmap { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ImageBitmap) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ImageBitmap> for ::js_sys::Object {
    #[inline]
    fn from(obj: ImageBitmap) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ImageBitmap {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "ImageBitmap",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_close_ImageBitmap() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ImageBitmap as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ImageBitmap {
    #[cfg(all(feature = "ImageBitmap",))]
    #[allow(bad_style)]
    #[doc = "The `close()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap/close)\n\n*This API requires the following crate features to be activated: `ImageBitmap`*"]
    #[allow(clippy::all)]
    pub fn close(&self) {
        #[cfg(all(feature = "ImageBitmap",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_close_ImageBitmap(
                self_: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_close_ImageBitmap(
            self_: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_close_ImageBitmap(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "ImageBitmap", feature = "ImageBitmapFormat",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_find_optimal_format_ImageBitmap() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ImageBitmap as WasmDescribe>::describe();
    <ImageBitmapFormat as WasmDescribe>::describe();
}
impl ImageBitmap {
    #[cfg(all(feature = "ImageBitmap", feature = "ImageBitmapFormat",))]
    #[allow(bad_style)]
    #[doc = "The `findOptimalFormat()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap/findOptimalFormat)\n\n*This API requires the following crate features to be activated: `ImageBitmap`, `ImageBitmapFormat`*"]
    #[allow(clippy::all)]
    pub fn find_optimal_format(&self) -> Result<ImageBitmapFormat, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ImageBitmap", feature = "ImageBitmapFormat",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_find_optimal_format_ImageBitmap(
                self_: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ImageBitmapFormat as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_find_optimal_format_ImageBitmap(
            self_: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ImageBitmapFormat as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_find_optimal_format_ImageBitmap(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ImageBitmapFormat as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ImageBitmap", feature = "ImageBitmapFormat",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_find_optimal_format_with_a_possible_formats_ImageBitmap(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ImageBitmap as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <ImageBitmapFormat as WasmDescribe>::describe();
}
impl ImageBitmap {
    #[cfg(all(feature = "ImageBitmap", feature = "ImageBitmapFormat",))]
    #[allow(bad_style)]
    #[doc = "The `findOptimalFormat()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap/findOptimalFormat)\n\n*This API requires the following crate features to be activated: `ImageBitmap`, `ImageBitmapFormat`*"]
    #[allow(clippy::all)]
    pub fn find_optimal_format_with_a_possible_formats(
        &self,
        a_possible_formats: &::wasm_bindgen::JsValue,
    ) -> Result<ImageBitmapFormat, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ImageBitmap", feature = "ImageBitmapFormat",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_find_optimal_format_with_a_possible_formats_ImageBitmap(
                self_: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_possible_formats : < & :: wasm_bindgen :: JsValue as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <ImageBitmapFormat as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_find_optimal_format_with_a_possible_formats_ImageBitmap(
            self_: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_possible_formats : < & :: wasm_bindgen :: JsValue as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> <ImageBitmapFormat as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(a_possible_formats);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_possible_formats =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        a_possible_formats,
                    );
                __widl_f_find_optimal_format_with_a_possible_formats_ImageBitmap(
                    self_,
                    a_possible_formats,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ImageBitmapFormat as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ImageBitmap", feature = "ImageBitmapFormat",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_map_data_into_with_buffer_source_ImageBitmap() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&ImageBitmap as WasmDescribe>::describe();
    <ImageBitmapFormat as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl ImageBitmap {
    #[cfg(all(feature = "ImageBitmap", feature = "ImageBitmapFormat",))]
    #[allow(bad_style)]
    #[doc = "The `mapDataInto()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap/mapDataInto)\n\n*This API requires the following crate features to be activated: `ImageBitmap`, `ImageBitmapFormat`*"]
    #[allow(clippy::all)]
    pub fn map_data_into_with_buffer_source(
        &self,
        a_format: ImageBitmapFormat,
        a_buffer: &::js_sys::Object,
        a_offset: i32,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ImageBitmap", feature = "ImageBitmapFormat",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_map_data_into_with_buffer_source_ImageBitmap(
                self_: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_format: <ImageBitmapFormat as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_buffer: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_offset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_map_data_into_with_buffer_source_ImageBitmap(
            self_: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_format: <ImageBitmapFormat as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_buffer: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_offset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(a_format);
            drop(a_buffer);
            drop(a_offset);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_format =
                    <ImageBitmapFormat as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_format);
                let a_buffer =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_buffer);
                let a_offset = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_offset);
                __widl_f_map_data_into_with_buffer_source_ImageBitmap(
                    self_, a_format, a_buffer, a_offset,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ImageBitmap", feature = "ImageBitmapFormat",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_map_data_into_with_u8_array_ImageBitmap() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&ImageBitmap as WasmDescribe>::describe();
    <ImageBitmapFormat as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl ImageBitmap {
    #[cfg(all(feature = "ImageBitmap", feature = "ImageBitmapFormat",))]
    #[allow(bad_style)]
    #[doc = "The `mapDataInto()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap/mapDataInto)\n\n*This API requires the following crate features to be activated: `ImageBitmap`, `ImageBitmapFormat`*"]
    #[allow(clippy::all)]
    pub fn map_data_into_with_u8_array(
        &self,
        a_format: ImageBitmapFormat,
        a_buffer: &mut [u8],
        a_offset: i32,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ImageBitmap", feature = "ImageBitmapFormat",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_map_data_into_with_u8_array_ImageBitmap(
                self_: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_format: <ImageBitmapFormat as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_buffer: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_offset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_map_data_into_with_u8_array_ImageBitmap(
            self_: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_format: <ImageBitmapFormat as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_buffer: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_offset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(a_format);
            drop(a_buffer);
            drop(a_offset);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_format =
                    <ImageBitmapFormat as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_format);
                let a_buffer =
                    <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_buffer);
                let a_offset = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_offset);
                __widl_f_map_data_into_with_u8_array_ImageBitmap(
                    self_, a_format, a_buffer, a_offset,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ImageBitmap", feature = "ImageBitmapFormat",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_mapped_data_length_ImageBitmap() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ImageBitmap as WasmDescribe>::describe();
    <ImageBitmapFormat as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl ImageBitmap {
    #[cfg(all(feature = "ImageBitmap", feature = "ImageBitmapFormat",))]
    #[allow(bad_style)]
    #[doc = "The `mappedDataLength()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap/mappedDataLength)\n\n*This API requires the following crate features to be activated: `ImageBitmap`, `ImageBitmapFormat`*"]
    #[allow(clippy::all)]
    pub fn mapped_data_length(
        &self,
        a_format: ImageBitmapFormat,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ImageBitmap", feature = "ImageBitmapFormat",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_mapped_data_length_ImageBitmap(
                self_: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_format: <ImageBitmapFormat as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_mapped_data_length_ImageBitmap(
            self_: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_format: <ImageBitmapFormat as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(a_format);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_format =
                    <ImageBitmapFormat as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_format);
                __widl_f_mapped_data_length_ImageBitmap(self_, a_format)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ImageBitmap",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_width_ImageBitmap() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ImageBitmap as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl ImageBitmap {
    #[cfg(all(feature = "ImageBitmap",))]
    #[allow(bad_style)]
    #[doc = "The `width` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap/width)\n\n*This API requires the following crate features to be activated: `ImageBitmap`*"]
    #[allow(clippy::all)]
    pub fn width(&self) -> u32 {
        #[cfg(all(feature = "ImageBitmap",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_width_ImageBitmap(
                self_: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_width_ImageBitmap(
            self_: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_width_ImageBitmap(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ImageBitmap",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_height_ImageBitmap() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ImageBitmap as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl ImageBitmap {
    #[cfg(all(feature = "ImageBitmap",))]
    #[allow(bad_style)]
    #[doc = "The `height` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap/height)\n\n*This API requires the following crate features to be activated: `ImageBitmap`*"]
    #[allow(clippy::all)]
    pub fn height(&self) -> u32 {
        #[cfg(all(feature = "ImageBitmap",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_height_ImageBitmap(
                self_: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_height_ImageBitmap(
            self_: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_height_ImageBitmap(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_1e1059be63516a0d: [u8; 913usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}O\x03\0\0\0\0\t\0\0\x02\x0BImageBitmap\x1D__widl_instanceof_ImageBitmap\0\0\0\0\x1A__widl_f_close_ImageBitmap\0\0\0\x01\x0BImageBitmap\x01\0\0\x01\x01\x05self_\x05close\0\0\0(__widl_f_find_optimal_format_ImageBitmap\x01\0\0\x01\x0BImageBitmap\x01\0\0\x01\x01\x05self_\x11findOptimalFormat\0\0\0@__widl_f_find_optimal_format_with_a_possible_formats_ImageBitmap\x01\0\0\x01\x0BImageBitmap\x01\0\0\x01\x02\x05self_\x12a_possible_formats\x11findOptimalFormat\0\0\05__widl_f_map_data_into_with_buffer_source_ImageBitmap\x01\0\0\x01\x0BImageBitmap\x01\0\0\x01\x04\x05self_\x08a_format\x08a_buffer\x08a_offset\x0BmapDataInto\0\0\00__widl_f_map_data_into_with_u8_array_ImageBitmap\x01\0\0\x01\x0BImageBitmap\x01\0\0\x01\x04\x05self_\x08a_format\x08a_buffer\x08a_offset\x0BmapDataInto\0\0\0'__widl_f_mapped_data_length_ImageBitmap\x01\0\0\x01\x0BImageBitmap\x01\0\0\x01\x02\x05self_\x08a_format\x10mappedDataLength\0\0\0\x1A__widl_f_width_ImageBitmap\0\0\0\x01\x0BImageBitmap\x01\0\x01\x05width\x01\x01\x05self_\x05width\0\0\0\x1B__widl_f_height_ImageBitmap\0\0\0\x01\x0BImageBitmap\x01\0\x01\x06height\x01\x01\x05self_\x06height\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
