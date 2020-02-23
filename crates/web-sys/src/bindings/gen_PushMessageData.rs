use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `PushMessageData` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushMessageData)\n\n*This API requires the following crate features to be activated: `PushMessageData`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct PushMessageData {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_PushMessageData: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for PushMessageData {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(80u32);
            inform(117u32);
            inform(115u32);
            inform(104u32);
            inform(77u32);
            inform(101u32);
            inform(115u32);
            inform(115u32);
            inform(97u32);
            inform(103u32);
            inform(101u32);
            inform(68u32);
            inform(97u32);
            inform(116u32);
            inform(97u32);
        }
    }
    impl core::ops::Deref for PushMessageData {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for PushMessageData {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for PushMessageData {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PushMessageData {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for PushMessageData {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            PushMessageData {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for PushMessageData {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a PushMessageData {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for PushMessageData {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<PushMessageData>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(PushMessageData {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for PushMessageData {
        #[inline]
        fn from(obj: JsValue) -> PushMessageData {
            PushMessageData { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for PushMessageData {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<PushMessageData> for PushMessageData {
        #[inline]
        fn as_ref(&self) -> &PushMessageData {
            self
        }
    }
    impl From<PushMessageData> for JsValue {
        #[inline]
        fn from(obj: PushMessageData) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for PushMessageData {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_PushMessageData(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_PushMessageData(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_PushMessageData(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PushMessageData { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PushMessageData) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<PushMessageData> for ::js_sys::Object {
    #[inline]
    fn from(obj: PushMessageData) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for PushMessageData {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "PushMessageData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_array_buffer_PushMessageData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PushMessageData as WasmDescribe>::describe();
    <::js_sys::ArrayBuffer as WasmDescribe>::describe();
}
impl PushMessageData {
    #[cfg(all(feature = "PushMessageData",))]
    #[allow(bad_style)]
    #[doc = "The `arrayBuffer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushMessageData/arrayBuffer)\n\n*This API requires the following crate features to be activated: `PushMessageData`*"]
    #[allow(clippy::all)]
    pub fn array_buffer(&self) -> Result<::js_sys::ArrayBuffer, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PushMessageData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_array_buffer_PushMessageData(
                self_: <&PushMessageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::ArrayBuffer as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_array_buffer_PushMessageData(
            self_: <&PushMessageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::ArrayBuffer as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PushMessageData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_array_buffer_PushMessageData(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::ArrayBuffer as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Blob", feature = "PushMessageData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_blob_PushMessageData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PushMessageData as WasmDescribe>::describe();
    <Blob as WasmDescribe>::describe();
}
impl PushMessageData {
    #[cfg(all(feature = "Blob", feature = "PushMessageData",))]
    #[allow(bad_style)]
    #[doc = "The `blob()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushMessageData/blob)\n\n*This API requires the following crate features to be activated: `Blob`, `PushMessageData`*"]
    #[allow(clippy::all)]
    pub fn blob(&self) -> Result<Blob, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob", feature = "PushMessageData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_blob_PushMessageData(
                self_: <&PushMessageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_blob_PushMessageData(
            self_: <&PushMessageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Blob as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PushMessageData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_blob_PushMessageData(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Blob as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "PushMessageData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_json_PushMessageData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PushMessageData as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl PushMessageData {
    #[cfg(all(feature = "PushMessageData",))]
    #[allow(bad_style)]
    #[doc = "The `json()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushMessageData/json)\n\n*This API requires the following crate features to be activated: `PushMessageData`*"]
    #[allow(clippy::all)]
    pub fn json(&self) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PushMessageData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_json_PushMessageData(
                self_: <&PushMessageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_json_PushMessageData(
            self_: <&PushMessageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PushMessageData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_json_PushMessageData(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "PushMessageData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_text_PushMessageData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PushMessageData as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl PushMessageData {
    #[cfg(all(feature = "PushMessageData",))]
    #[allow(bad_style)]
    #[doc = "The `text()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushMessageData/text)\n\n*This API requires the following crate features to be activated: `PushMessageData`*"]
    #[allow(clippy::all)]
    pub fn text(&self) -> String {
        #[cfg(all(feature = "PushMessageData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_text_PushMessageData(
                self_: <&PushMessageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_text_PushMessageData(
            self_: <&PushMessageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PushMessageData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_text_PushMessageData(self_)
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
pub static __WASM_BINDGEN_GENERATED_0b89381dc1023067: [u8; 450usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x80\x01\0\0\0\0\x05\0\0\x02\x0FPushMessageData!__widl_instanceof_PushMessageData\0\0\0\0%__widl_f_array_buffer_PushMessageData\x01\0\0\x01\x0FPushMessageData\x01\0\0\x01\x01\x05self_\x0BarrayBuffer\0\0\0\x1D__widl_f_blob_PushMessageData\x01\0\0\x01\x0FPushMessageData\x01\0\0\x01\x01\x05self_\x04blob\0\0\0\x1D__widl_f_json_PushMessageData\x01\0\0\x01\x0FPushMessageData\x01\0\0\x01\x01\x05self_\x04json\0\0\0\x1D__widl_f_text_PushMessageData\0\0\0\x01\x0FPushMessageData\x01\0\0\x01\x01\x05self_\x04text\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
