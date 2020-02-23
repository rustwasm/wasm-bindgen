use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `VRSubmitFrameResult` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRSubmitFrameResult)\n\n*This API requires the following crate features to be activated: `VrSubmitFrameResult`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct VrSubmitFrameResult {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_VrSubmitFrameResult: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for VrSubmitFrameResult {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(19u32);
            inform(86u32);
            inform(82u32);
            inform(83u32);
            inform(117u32);
            inform(98u32);
            inform(109u32);
            inform(105u32);
            inform(116u32);
            inform(70u32);
            inform(114u32);
            inform(97u32);
            inform(109u32);
            inform(101u32);
            inform(82u32);
            inform(101u32);
            inform(115u32);
            inform(117u32);
            inform(108u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for VrSubmitFrameResult {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for VrSubmitFrameResult {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for VrSubmitFrameResult {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a VrSubmitFrameResult {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for VrSubmitFrameResult {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            VrSubmitFrameResult {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for VrSubmitFrameResult {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a VrSubmitFrameResult {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for VrSubmitFrameResult {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<VrSubmitFrameResult>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(VrSubmitFrameResult {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for VrSubmitFrameResult {
        #[inline]
        fn from(obj: JsValue) -> VrSubmitFrameResult {
            VrSubmitFrameResult { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for VrSubmitFrameResult {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<VrSubmitFrameResult> for VrSubmitFrameResult {
        #[inline]
        fn as_ref(&self) -> &VrSubmitFrameResult {
            self
        }
    }
    impl From<VrSubmitFrameResult> for JsValue {
        #[inline]
        fn from(obj: VrSubmitFrameResult) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for VrSubmitFrameResult {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_VRSubmitFrameResult(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_VRSubmitFrameResult(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_VRSubmitFrameResult(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            VrSubmitFrameResult { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const VrSubmitFrameResult) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<VrSubmitFrameResult> for ::js_sys::Object {
    #[inline]
    fn from(obj: VrSubmitFrameResult) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for VrSubmitFrameResult {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "VrSubmitFrameResult",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_VRSubmitFrameResult() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <VrSubmitFrameResult as WasmDescribe>::describe();
}
impl VrSubmitFrameResult {
    #[cfg(all(feature = "VrSubmitFrameResult",))]
    #[allow(bad_style)]
    #[doc = "The `new VRSubmitFrameResult(..)` constructor, creating a new instance of `VRSubmitFrameResult`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRSubmitFrameResult/VRSubmitFrameResult)\n\n*This API requires the following crate features to be activated: `VrSubmitFrameResult`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<VrSubmitFrameResult, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "VrSubmitFrameResult",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_VRSubmitFrameResult(
            ) -> <VrSubmitFrameResult as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_VRSubmitFrameResult(
        ) -> <VrSubmitFrameResult as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_VRSubmitFrameResult() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<VrSubmitFrameResult as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "VrSubmitFrameResult",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_frame_num_VRSubmitFrameResult() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrSubmitFrameResult as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl VrSubmitFrameResult {
    #[cfg(all(feature = "VrSubmitFrameResult",))]
    #[allow(bad_style)]
    #[doc = "The `frameNum` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRSubmitFrameResult/frameNum)\n\n*This API requires the following crate features to be activated: `VrSubmitFrameResult`*"]
    #[allow(clippy::all)]
    pub fn frame_num(&self) -> u32 {
        #[cfg(all(feature = "VrSubmitFrameResult",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_frame_num_VRSubmitFrameResult(
                self_: <&VrSubmitFrameResult as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_frame_num_VRSubmitFrameResult(
            self_: <&VrSubmitFrameResult as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&VrSubmitFrameResult as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_frame_num_VRSubmitFrameResult(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VrSubmitFrameResult",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_base64_image_VRSubmitFrameResult() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrSubmitFrameResult as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl VrSubmitFrameResult {
    #[cfg(all(feature = "VrSubmitFrameResult",))]
    #[allow(bad_style)]
    #[doc = "The `base64Image` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRSubmitFrameResult/base64Image)\n\n*This API requires the following crate features to be activated: `VrSubmitFrameResult`*"]
    #[allow(clippy::all)]
    pub fn base64_image(&self) -> Option<String> {
        #[cfg(all(feature = "VrSubmitFrameResult",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_base64_image_VRSubmitFrameResult(
                self_: <&VrSubmitFrameResult as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_base64_image_VRSubmitFrameResult(
            self_: <&VrSubmitFrameResult as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&VrSubmitFrameResult as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_base64_image_VRSubmitFrameResult(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_231a4b0a8f5ae8d6: [u8; 433usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}o\x01\0\0\0\0\x04\0\0\x02\x13VRSubmitFrameResult%__widl_instanceof_VRSubmitFrameResult\0\0\0\0 __widl_f_new_VRSubmitFrameResult\x01\0\0\x01\x13VRSubmitFrameResult\0\x01\0\x03new\0\0\0&__widl_f_frame_num_VRSubmitFrameResult\0\0\0\x01\x13VRSubmitFrameResult\x01\0\x01\x08frameNum\x01\x01\x05self_\x08frameNum\0\0\0)__widl_f_base64_image_VRSubmitFrameResult\0\0\0\x01\x13VRSubmitFrameResult\x01\0\x01\x0Bbase64Image\x01\x01\x05self_\x0Bbase64Image\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
