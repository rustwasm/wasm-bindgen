use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `ImageData` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData)\n\n*This API requires the following crate features to be activated: `ImageData`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ImageData {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ImageData: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ImageData {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(9u32);
            inform(73u32);
            inform(109u32);
            inform(97u32);
            inform(103u32);
            inform(101u32);
            inform(68u32);
            inform(97u32);
            inform(116u32);
            inform(97u32);
        }
    }
    impl core::ops::Deref for ImageData {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for ImageData {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ImageData {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ImageData {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ImageData {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ImageData {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ImageData {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ImageData {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ImageData {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ImageData>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ImageData {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ImageData {
        #[inline]
        fn from(obj: JsValue) -> ImageData {
            ImageData { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ImageData {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ImageData> for ImageData {
        #[inline]
        fn as_ref(&self) -> &ImageData {
            self
        }
    }
    impl From<ImageData> for JsValue {
        #[inline]
        fn from(obj: ImageData) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ImageData {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_ImageData(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_ImageData(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_ImageData(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ImageData { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ImageData) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ImageData> for ::js_sys::Object {
    #[inline]
    fn from(obj: ImageData) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ImageData {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "ImageData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_sw_ImageData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <ImageData as WasmDescribe>::describe();
}
impl ImageData {
    #[cfg(all(feature = "ImageData",))]
    #[allow(bad_style)]
    #[doc = "The `new ImageData(..)` constructor, creating a new instance of `ImageData`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/ImageData)\n\n*This API requires the following crate features to be activated: `ImageData`*"]
    #[allow(clippy::all)]
    pub fn new_with_sw(sw: u32, sh: u32) -> Result<ImageData, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ImageData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_sw_ImageData(
                sw: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sh: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ImageData as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_sw_ImageData(
            sw: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sh: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ImageData as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(sw);
            drop(sh);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let sw = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sw);
                let sh = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sh);
                __widl_f_new_with_sw_ImageData(sw, sh)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ImageData as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "ImageData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_u8_clamped_array_ImageData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <::wasm_bindgen::Clamped<&mut [u8]> as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <ImageData as WasmDescribe>::describe();
}
impl ImageData {
    #[cfg(all(feature = "ImageData",))]
    #[allow(bad_style)]
    #[doc = "The `new ImageData(..)` constructor, creating a new instance of `ImageData`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/ImageData)\n\n*This API requires the following crate features to be activated: `ImageData`*"]
    #[allow(clippy::all)]
    pub fn new_with_u8_clamped_array(
        data: ::wasm_bindgen::Clamped<&mut [u8]>,
        sw: u32,
    ) -> Result<ImageData, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ImageData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_u8_clamped_array_ImageData(
                data : < :: wasm_bindgen :: Clamped < & mut [ u8 ] > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                sw: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ImageData as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_u8_clamped_array_ImageData(
            data: <::wasm_bindgen::Clamped<&mut [u8]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sw: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ImageData as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(data);
            drop(sw);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = < :: wasm_bindgen :: Clamped < & mut [ u8 ] > as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( data ) ;
                let sw = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sw);
                __widl_f_new_with_u8_clamped_array_ImageData(data, sw)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ImageData as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "ImageData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_u8_clamped_array_and_sh_ImageData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <::wasm_bindgen::Clamped<&mut [u8]> as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <ImageData as WasmDescribe>::describe();
}
impl ImageData {
    #[cfg(all(feature = "ImageData",))]
    #[allow(bad_style)]
    #[doc = "The `new ImageData(..)` constructor, creating a new instance of `ImageData`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/ImageData)\n\n*This API requires the following crate features to be activated: `ImageData`*"]
    #[allow(clippy::all)]
    pub fn new_with_u8_clamped_array_and_sh(
        data: ::wasm_bindgen::Clamped<&mut [u8]>,
        sw: u32,
        sh: u32,
    ) -> Result<ImageData, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ImageData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_u8_clamped_array_and_sh_ImageData(
                data : < :: wasm_bindgen :: Clamped < & mut [ u8 ] > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                sw: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sh: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ImageData as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_u8_clamped_array_and_sh_ImageData(
            data: <::wasm_bindgen::Clamped<&mut [u8]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sw: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sh: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ImageData as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(data);
            drop(sw);
            drop(sh);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = < :: wasm_bindgen :: Clamped < & mut [ u8 ] > as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( data ) ;
                let sw = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sw);
                let sh = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sh);
                __widl_f_new_with_u8_clamped_array_and_sh_ImageData(data, sw, sh)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ImageData as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "ImageData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_width_ImageData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ImageData as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl ImageData {
    #[cfg(all(feature = "ImageData",))]
    #[allow(bad_style)]
    #[doc = "The `width` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/width)\n\n*This API requires the following crate features to be activated: `ImageData`*"]
    #[allow(clippy::all)]
    pub fn width(&self) -> u32 {
        #[cfg(all(feature = "ImageData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_width_ImageData(
                self_: <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_width_ImageData(
            self_: <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_width_ImageData(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ImageData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_height_ImageData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ImageData as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl ImageData {
    #[cfg(all(feature = "ImageData",))]
    #[allow(bad_style)]
    #[doc = "The `height` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/height)\n\n*This API requires the following crate features to be activated: `ImageData`*"]
    #[allow(clippy::all)]
    pub fn height(&self) -> u32 {
        #[cfg(all(feature = "ImageData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_height_ImageData(
                self_: <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_height_ImageData(
            self_: <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_height_ImageData(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ImageData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_data_ImageData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ImageData as WasmDescribe>::describe();
    <::wasm_bindgen::Clamped<Vec<u8>> as WasmDescribe>::describe();
}
impl ImageData {
    #[cfg(all(feature = "ImageData",))]
    #[allow(bad_style)]
    #[doc = "The `data` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/data)\n\n*This API requires the following crate features to be activated: `ImageData`*"]
    #[allow(clippy::all)]
    pub fn data(&self) -> ::wasm_bindgen::Clamped<Vec<u8>> {
        #[cfg(all(feature = "ImageData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_data_ImageData(
                self_: <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::Clamped<Vec<u8>> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_data_ImageData(
            self_: <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::Clamped<Vec<u8>> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_data_ImageData(self_)
            };
            <::wasm_bindgen::Clamped<Vec<u8>> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_30e1ef52955e3335: [u8; 567usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xF5\x01\0\0\0\0\x07\0\0\x02\tImageData\x1B__widl_instanceof_ImageData\0\0\0\0\x1E__widl_f_new_with_sw_ImageData\x01\0\0\x01\tImageData\0\x01\x02\x02sw\x02sh\x03new\0\0\0,__widl_f_new_with_u8_clamped_array_ImageData\x01\0\0\x01\tImageData\0\x01\x02\x04data\x02sw\x03new\0\0\03__widl_f_new_with_u8_clamped_array_and_sh_ImageData\x01\0\0\x01\tImageData\0\x01\x03\x04data\x02sw\x02sh\x03new\0\0\0\x18__widl_f_width_ImageData\0\0\0\x01\tImageData\x01\0\x01\x05width\x01\x01\x05self_\x05width\0\0\0\x19__widl_f_height_ImageData\0\0\0\x01\tImageData\x01\0\x01\x06height\x01\x01\x05self_\x06height\0\0\0\x17__widl_f_data_ImageData\0\0\0\x01\tImageData\x01\0\x01\x04data\x01\x01\x05self_\x04data\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
