use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `TextEncoder` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder)\n\n*This API requires the following crate features to be activated: `TextEncoder`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct TextEncoder {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_TextEncoder: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for TextEncoder {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(11u32);
            inform(84u32);
            inform(101u32);
            inform(120u32);
            inform(116u32);
            inform(69u32);
            inform(110u32);
            inform(99u32);
            inform(111u32);
            inform(100u32);
            inform(101u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for TextEncoder {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for TextEncoder {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for TextEncoder {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a TextEncoder {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for TextEncoder {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            TextEncoder {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for TextEncoder {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a TextEncoder {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for TextEncoder {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<TextEncoder>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(TextEncoder {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for TextEncoder {
        #[inline]
        fn from(obj: JsValue) -> TextEncoder {
            TextEncoder { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for TextEncoder {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<TextEncoder> for TextEncoder {
        #[inline]
        fn as_ref(&self) -> &TextEncoder {
            self
        }
    }
    impl From<TextEncoder> for JsValue {
        #[inline]
        fn from(obj: TextEncoder) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for TextEncoder {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_TextEncoder(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_TextEncoder(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_TextEncoder(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            TextEncoder { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const TextEncoder) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<TextEncoder> for ::js_sys::Object {
    #[inline]
    fn from(obj: TextEncoder) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for TextEncoder {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "TextEncoder",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_TextEncoder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <TextEncoder as WasmDescribe>::describe();
}
impl TextEncoder {
    #[cfg(all(feature = "TextEncoder",))]
    #[allow(bad_style)]
    #[doc = "The `new TextEncoder(..)` constructor, creating a new instance of `TextEncoder`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder/TextEncoder)\n\n*This API requires the following crate features to be activated: `TextEncoder`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<TextEncoder, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TextEncoder",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_TextEncoder() -> <TextEncoder as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_TextEncoder(
        ) -> <TextEncoder as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_TextEncoder() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<TextEncoder as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "TextEncoder",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_encode_TextEncoder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TextEncoder as WasmDescribe>::describe();
    <Vec<u8> as WasmDescribe>::describe();
}
impl TextEncoder {
    #[cfg(all(feature = "TextEncoder",))]
    #[allow(bad_style)]
    #[doc = "The `encode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder/encode)\n\n*This API requires the following crate features to be activated: `TextEncoder`*"]
    #[allow(clippy::all)]
    pub fn encode(&self) -> Vec<u8> {
        #[cfg(all(feature = "TextEncoder",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_encode_TextEncoder(
                self_: <&TextEncoder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Vec<u8> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_encode_TextEncoder(
            self_: <&TextEncoder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Vec<u8> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextEncoder as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_encode_TextEncoder(self_)
            };
            <Vec<u8> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TextEncoder",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_encode_with_input_TextEncoder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TextEncoder as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Vec<u8> as WasmDescribe>::describe();
}
impl TextEncoder {
    #[cfg(all(feature = "TextEncoder",))]
    #[allow(bad_style)]
    #[doc = "The `encode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder/encode)\n\n*This API requires the following crate features to be activated: `TextEncoder`*"]
    #[allow(clippy::all)]
    pub fn encode_with_input(&self, input: &str) -> Vec<u8> {
        #[cfg(all(feature = "TextEncoder",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_encode_with_input_TextEncoder(
                self_: <&TextEncoder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                input: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Vec<u8> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_encode_with_input_TextEncoder(
            self_: <&TextEncoder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            input: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Vec<u8> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(input);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextEncoder as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let input = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(input);
                __widl_f_encode_with_input_TextEncoder(self_, input)
            };
            <Vec<u8> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TextEncoder",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_encoding_TextEncoder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TextEncoder as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl TextEncoder {
    #[cfg(all(feature = "TextEncoder",))]
    #[allow(bad_style)]
    #[doc = "The `encoding` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder/encoding)\n\n*This API requires the following crate features to be activated: `TextEncoder`*"]
    #[allow(clippy::all)]
    pub fn encoding(&self) -> String {
        #[cfg(all(feature = "TextEncoder",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_encoding_TextEncoder(
                self_: <&TextEncoder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_encoding_TextEncoder(
            self_: <&TextEncoder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextEncoder as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_encoding_TextEncoder(self_)
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
pub static __WASM_BINDGEN_GENERATED_817323173c54fec2: [u8; 427usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}i\x01\0\0\0\0\x05\0\0\x02\x0BTextEncoder\x1D__widl_instanceof_TextEncoder\0\0\0\0\x18__widl_f_new_TextEncoder\x01\0\0\x01\x0BTextEncoder\0\x01\0\x03new\0\0\0\x1B__widl_f_encode_TextEncoder\0\0\0\x01\x0BTextEncoder\x01\0\0\x01\x01\x05self_\x06encode\0\0\0&__widl_f_encode_with_input_TextEncoder\0\0\0\x01\x0BTextEncoder\x01\0\0\x01\x02\x05self_\x05input\x06encode\0\0\0\x1D__widl_f_encoding_TextEncoder\0\0\0\x01\x0BTextEncoder\x01\0\x01\x08encoding\x01\x01\x05self_\x08encoding\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
