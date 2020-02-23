use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `TextDecoder` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder)\n\n*This API requires the following crate features to be activated: `TextDecoder`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct TextDecoder {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_TextDecoder: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for TextDecoder {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(11u32);
            inform(84u32);
            inform(101u32);
            inform(120u32);
            inform(116u32);
            inform(68u32);
            inform(101u32);
            inform(99u32);
            inform(111u32);
            inform(100u32);
            inform(101u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for TextDecoder {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for TextDecoder {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for TextDecoder {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a TextDecoder {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for TextDecoder {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            TextDecoder {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for TextDecoder {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a TextDecoder {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for TextDecoder {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<TextDecoder>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(TextDecoder {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for TextDecoder {
        #[inline]
        fn from(obj: JsValue) -> TextDecoder {
            TextDecoder { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for TextDecoder {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<TextDecoder> for TextDecoder {
        #[inline]
        fn as_ref(&self) -> &TextDecoder {
            self
        }
    }
    impl From<TextDecoder> for JsValue {
        #[inline]
        fn from(obj: TextDecoder) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for TextDecoder {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_TextDecoder(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_TextDecoder(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_TextDecoder(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            TextDecoder { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const TextDecoder) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<TextDecoder> for ::js_sys::Object {
    #[inline]
    fn from(obj: TextDecoder) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for TextDecoder {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "TextDecoder",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_TextDecoder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <TextDecoder as WasmDescribe>::describe();
}
impl TextDecoder {
    #[cfg(all(feature = "TextDecoder",))]
    #[allow(bad_style)]
    #[doc = "The `new TextDecoder(..)` constructor, creating a new instance of `TextDecoder`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/TextDecoder)\n\n*This API requires the following crate features to be activated: `TextDecoder`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<TextDecoder, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TextDecoder",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_TextDecoder() -> <TextDecoder as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_TextDecoder(
        ) -> <TextDecoder as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_TextDecoder() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<TextDecoder as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "TextDecoder",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_label_TextDecoder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <TextDecoder as WasmDescribe>::describe();
}
impl TextDecoder {
    #[cfg(all(feature = "TextDecoder",))]
    #[allow(bad_style)]
    #[doc = "The `new TextDecoder(..)` constructor, creating a new instance of `TextDecoder`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/TextDecoder)\n\n*This API requires the following crate features to be activated: `TextDecoder`*"]
    #[allow(clippy::all)]
    pub fn new_with_label(label: &str) -> Result<TextDecoder, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TextDecoder",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_label_TextDecoder(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TextDecoder as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_label_TextDecoder(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TextDecoder as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(label);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                __widl_f_new_with_label_TextDecoder(label)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<TextDecoder as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "TextDecoder", feature = "TextDecoderOptions",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_label_and_options_TextDecoder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&TextDecoderOptions as WasmDescribe>::describe();
    <TextDecoder as WasmDescribe>::describe();
}
impl TextDecoder {
    #[cfg(all(feature = "TextDecoder", feature = "TextDecoderOptions",))]
    #[allow(bad_style)]
    #[doc = "The `new TextDecoder(..)` constructor, creating a new instance of `TextDecoder`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/TextDecoder)\n\n*This API requires the following crate features to be activated: `TextDecoder`, `TextDecoderOptions`*"]
    #[allow(clippy::all)]
    pub fn new_with_label_and_options(
        label: &str,
        options: &TextDecoderOptions,
    ) -> Result<TextDecoder, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TextDecoder", feature = "TextDecoderOptions",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_label_and_options_TextDecoder(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&TextDecoderOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TextDecoder as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_label_and_options_TextDecoder(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&TextDecoderOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TextDecoder as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(label);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                let options =
                    <&TextDecoderOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_new_with_label_and_options_TextDecoder(label, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<TextDecoder as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "TextDecoder",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_decode_TextDecoder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TextDecoder as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl TextDecoder {
    #[cfg(all(feature = "TextDecoder",))]
    #[allow(bad_style)]
    #[doc = "The `decode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/decode)\n\n*This API requires the following crate features to be activated: `TextDecoder`*"]
    #[allow(clippy::all)]
    pub fn decode(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TextDecoder",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_decode_TextDecoder(
                self_: <&TextDecoder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_decode_TextDecoder(
            self_: <&TextDecoder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextDecoder as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_decode_TextDecoder(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "TextDecoder",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_decode_with_buffer_source_TextDecoder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TextDecoder as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl TextDecoder {
    #[cfg(all(feature = "TextDecoder",))]
    #[allow(bad_style)]
    #[doc = "The `decode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/decode)\n\n*This API requires the following crate features to be activated: `TextDecoder`*"]
    #[allow(clippy::all)]
    pub fn decode_with_buffer_source(
        &self,
        input: &::js_sys::Object,
    ) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TextDecoder",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_decode_with_buffer_source_TextDecoder(
                self_: <&TextDecoder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                input: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_decode_with_buffer_source_TextDecoder(
            self_: <&TextDecoder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            input: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(input);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextDecoder as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let input =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(input);
                __widl_f_decode_with_buffer_source_TextDecoder(self_, input)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "TextDecoder",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_decode_with_u8_array_TextDecoder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TextDecoder as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl TextDecoder {
    #[cfg(all(feature = "TextDecoder",))]
    #[allow(bad_style)]
    #[doc = "The `decode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/decode)\n\n*This API requires the following crate features to be activated: `TextDecoder`*"]
    #[allow(clippy::all)]
    pub fn decode_with_u8_array(
        &self,
        input: &mut [u8],
    ) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TextDecoder",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_decode_with_u8_array_TextDecoder(
                self_: <&TextDecoder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                input: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_decode_with_u8_array_TextDecoder(
            self_: <&TextDecoder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            input: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(input);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextDecoder as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let input = <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(input);
                __widl_f_decode_with_u8_array_TextDecoder(self_, input)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "TextDecodeOptions", feature = "TextDecoder",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_decode_with_buffer_source_and_options_TextDecoder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&TextDecoder as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <&TextDecodeOptions as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl TextDecoder {
    #[cfg(all(feature = "TextDecodeOptions", feature = "TextDecoder",))]
    #[allow(bad_style)]
    #[doc = "The `decode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/decode)\n\n*This API requires the following crate features to be activated: `TextDecodeOptions`, `TextDecoder`*"]
    #[allow(clippy::all)]
    pub fn decode_with_buffer_source_and_options(
        &self,
        input: &::js_sys::Object,
        options: &TextDecodeOptions,
    ) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TextDecodeOptions", feature = "TextDecoder",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_decode_with_buffer_source_and_options_TextDecoder(
                self_: <&TextDecoder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                input: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&TextDecodeOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_decode_with_buffer_source_and_options_TextDecoder(
            self_: <&TextDecoder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            input: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&TextDecodeOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(input);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextDecoder as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let input =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(input);
                let options =
                    <&TextDecodeOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_decode_with_buffer_source_and_options_TextDecoder(self_, input, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "TextDecodeOptions", feature = "TextDecoder",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_decode_with_u8_array_and_options_TextDecoder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&TextDecoder as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <&TextDecodeOptions as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl TextDecoder {
    #[cfg(all(feature = "TextDecodeOptions", feature = "TextDecoder",))]
    #[allow(bad_style)]
    #[doc = "The `decode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/decode)\n\n*This API requires the following crate features to be activated: `TextDecodeOptions`, `TextDecoder`*"]
    #[allow(clippy::all)]
    pub fn decode_with_u8_array_and_options(
        &self,
        input: &mut [u8],
        options: &TextDecodeOptions,
    ) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TextDecodeOptions", feature = "TextDecoder",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_decode_with_u8_array_and_options_TextDecoder(
                self_: <&TextDecoder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                input: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&TextDecodeOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_decode_with_u8_array_and_options_TextDecoder(
            self_: <&TextDecoder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            input: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&TextDecodeOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(input);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextDecoder as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let input = <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(input);
                let options =
                    <&TextDecodeOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_decode_with_u8_array_and_options_TextDecoder(self_, input, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "TextDecoder",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_encoding_TextDecoder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TextDecoder as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl TextDecoder {
    #[cfg(all(feature = "TextDecoder",))]
    #[allow(bad_style)]
    #[doc = "The `encoding` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/encoding)\n\n*This API requires the following crate features to be activated: `TextDecoder`*"]
    #[allow(clippy::all)]
    pub fn encoding(&self) -> String {
        #[cfg(all(feature = "TextDecoder",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_encoding_TextDecoder(
                self_: <&TextDecoder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_encoding_TextDecoder(
            self_: <&TextDecoder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextDecoder as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_encoding_TextDecoder(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TextDecoder",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_fatal_TextDecoder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TextDecoder as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl TextDecoder {
    #[cfg(all(feature = "TextDecoder",))]
    #[allow(bad_style)]
    #[doc = "The `fatal` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/fatal)\n\n*This API requires the following crate features to be activated: `TextDecoder`*"]
    #[allow(clippy::all)]
    pub fn fatal(&self) -> bool {
        #[cfg(all(feature = "TextDecoder",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_fatal_TextDecoder(
                self_: <&TextDecoder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_fatal_TextDecoder(
            self_: <&TextDecoder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextDecoder as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_fatal_TextDecoder(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_a06b476a10190ec5: [u8; 960usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}~\x03\0\0\0\0\x0B\0\0\x02\x0BTextDecoder\x1D__widl_instanceof_TextDecoder\0\0\0\0\x18__widl_f_new_TextDecoder\x01\0\0\x01\x0BTextDecoder\0\x01\0\x03new\0\0\0#__widl_f_new_with_label_TextDecoder\x01\0\0\x01\x0BTextDecoder\0\x01\x01\x05label\x03new\0\0\0/__widl_f_new_with_label_and_options_TextDecoder\x01\0\0\x01\x0BTextDecoder\0\x01\x02\x05label\x07options\x03new\0\0\0\x1B__widl_f_decode_TextDecoder\x01\0\0\x01\x0BTextDecoder\x01\0\0\x01\x01\x05self_\x06decode\0\0\0.__widl_f_decode_with_buffer_source_TextDecoder\x01\0\0\x01\x0BTextDecoder\x01\0\0\x01\x02\x05self_\x05input\x06decode\0\0\0)__widl_f_decode_with_u8_array_TextDecoder\x01\0\0\x01\x0BTextDecoder\x01\0\0\x01\x02\x05self_\x05input\x06decode\0\0\0:__widl_f_decode_with_buffer_source_and_options_TextDecoder\x01\0\0\x01\x0BTextDecoder\x01\0\0\x01\x03\x05self_\x05input\x07options\x06decode\0\0\05__widl_f_decode_with_u8_array_and_options_TextDecoder\x01\0\0\x01\x0BTextDecoder\x01\0\0\x01\x03\x05self_\x05input\x07options\x06decode\0\0\0\x1D__widl_f_encoding_TextDecoder\0\0\0\x01\x0BTextDecoder\x01\0\x01\x08encoding\x01\x01\x05self_\x08encoding\0\0\0\x1A__widl_f_fatal_TextDecoder\0\0\0\x01\x0BTextDecoder\x01\0\x01\x05fatal\x01\x01\x05self_\x05fatal\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
