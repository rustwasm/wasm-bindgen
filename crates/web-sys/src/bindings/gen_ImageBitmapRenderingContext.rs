use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `ImageBitmapRenderingContext` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmapRenderingContext)\n\n*This API requires the following crate features to be activated: `ImageBitmapRenderingContext`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ImageBitmapRenderingContext {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ImageBitmapRenderingContext: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ImageBitmapRenderingContext {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(27u32);
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
            inform(82u32);
            inform(101u32);
            inform(110u32);
            inform(100u32);
            inform(101u32);
            inform(114u32);
            inform(105u32);
            inform(110u32);
            inform(103u32);
            inform(67u32);
            inform(111u32);
            inform(110u32);
            inform(116u32);
            inform(101u32);
            inform(120u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for ImageBitmapRenderingContext {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for ImageBitmapRenderingContext {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ImageBitmapRenderingContext {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ImageBitmapRenderingContext {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ImageBitmapRenderingContext {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ImageBitmapRenderingContext {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ImageBitmapRenderingContext {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ImageBitmapRenderingContext {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ImageBitmapRenderingContext {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ImageBitmapRenderingContext>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ImageBitmapRenderingContext {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ImageBitmapRenderingContext {
        #[inline]
        fn from(obj: JsValue) -> ImageBitmapRenderingContext {
            ImageBitmapRenderingContext { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ImageBitmapRenderingContext {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ImageBitmapRenderingContext> for ImageBitmapRenderingContext {
        #[inline]
        fn as_ref(&self) -> &ImageBitmapRenderingContext {
            self
        }
    }
    impl From<ImageBitmapRenderingContext> for JsValue {
        #[inline]
        fn from(obj: ImageBitmapRenderingContext) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ImageBitmapRenderingContext {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_ImageBitmapRenderingContext(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_ImageBitmapRenderingContext(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_ImageBitmapRenderingContext(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ImageBitmapRenderingContext { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ImageBitmapRenderingContext) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ImageBitmapRenderingContext> for ::js_sys::Object {
    #[inline]
    fn from(obj: ImageBitmapRenderingContext) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ImageBitmapRenderingContext {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "ImageBitmap", feature = "ImageBitmapRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_transfer_from_image_bitmap_ImageBitmapRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ImageBitmapRenderingContext as WasmDescribe>::describe();
    <&ImageBitmap as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ImageBitmapRenderingContext {
    #[cfg(all(feature = "ImageBitmap", feature = "ImageBitmapRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `transferFromImageBitmap()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmapRenderingContext/transferFromImageBitmap)\n\n*This API requires the following crate features to be activated: `ImageBitmap`, `ImageBitmapRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn transfer_from_image_bitmap(&self, bitmap: &ImageBitmap) {
        #[cfg(all(feature = "ImageBitmap", feature = "ImageBitmapRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_transfer_from_image_bitmap_ImageBitmapRenderingContext(
                self_: <&ImageBitmapRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bitmap: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_transfer_from_image_bitmap_ImageBitmapRenderingContext(
            self_: <&ImageBitmapRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bitmap: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(bitmap);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ImageBitmapRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let bitmap = <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bitmap);
                __widl_f_transfer_from_image_bitmap_ImageBitmapRenderingContext(self_, bitmap)
            };
            ()
        }
    }
}
#[cfg(all(feature = "ImageBitmap", feature = "ImageBitmapRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_transfer_image_bitmap_ImageBitmapRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ImageBitmapRenderingContext as WasmDescribe>::describe();
    <&ImageBitmap as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ImageBitmapRenderingContext {
    #[cfg(all(feature = "ImageBitmap", feature = "ImageBitmapRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `transferImageBitmap()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmapRenderingContext/transferImageBitmap)\n\n*This API requires the following crate features to be activated: `ImageBitmap`, `ImageBitmapRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn transfer_image_bitmap(&self, bitmap: &ImageBitmap) {
        #[cfg(all(feature = "ImageBitmap", feature = "ImageBitmapRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_transfer_image_bitmap_ImageBitmapRenderingContext(
                self_: <&ImageBitmapRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bitmap: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_transfer_image_bitmap_ImageBitmapRenderingContext(
            self_: <&ImageBitmapRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bitmap: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(bitmap);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ImageBitmapRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let bitmap = <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bitmap);
                __widl_f_transfer_image_bitmap_ImageBitmapRenderingContext(self_, bitmap)
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
pub static __WASM_BINDGEN_GENERATED_1b84c4bbe81a1666: [u8; 456usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x86\x01\0\0\0\0\x03\0\0\x02\x1BImageBitmapRenderingContext-__widl_instanceof_ImageBitmapRenderingContext\0\0\0\0?__widl_f_transfer_from_image_bitmap_ImageBitmapRenderingContext\0\0\0\x01\x1BImageBitmapRenderingContext\x01\0\0\x01\x02\x05self_\x06bitmap\x17transferFromImageBitmap\0\0\0:__widl_f_transfer_image_bitmap_ImageBitmapRenderingContext\0\0\0\x01\x1BImageBitmapRenderingContext\x01\0\0\x01\x02\x05self_\x06bitmap\x13transferImageBitmap\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
