use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `IDBFileRequest` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileRequest)\n\n*This API requires the following crate features to be activated: `IdbFileRequest`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct IdbFileRequest {
    obj: DomRequest,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_IdbFileRequest: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for IdbFileRequest {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(14u32);
            inform(73u32);
            inform(68u32);
            inform(66u32);
            inform(70u32);
            inform(105u32);
            inform(108u32);
            inform(101u32);
            inform(82u32);
            inform(101u32);
            inform(113u32);
            inform(117u32);
            inform(101u32);
            inform(115u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for IdbFileRequest {
        type Target = DomRequest;
        #[inline]
        fn deref(&self) -> &DomRequest {
            &self.obj
        }
    }
    impl IntoWasmAbi for IdbFileRequest {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for IdbFileRequest {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a IdbFileRequest {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for IdbFileRequest {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            IdbFileRequest {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for IdbFileRequest {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a IdbFileRequest {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for IdbFileRequest {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<IdbFileRequest>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(IdbFileRequest {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for IdbFileRequest {
        #[inline]
        fn from(obj: JsValue) -> IdbFileRequest {
            IdbFileRequest { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for IdbFileRequest {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<IdbFileRequest> for IdbFileRequest {
        #[inline]
        fn as_ref(&self) -> &IdbFileRequest {
            self
        }
    }
    impl From<IdbFileRequest> for JsValue {
        #[inline]
        fn from(obj: IdbFileRequest) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for IdbFileRequest {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_IDBFileRequest(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_IDBFileRequest(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_IDBFileRequest(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            IdbFileRequest { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const IdbFileRequest) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<IdbFileRequest> for DomRequest {
    #[inline]
    fn from(obj: IdbFileRequest) -> DomRequest {
        use wasm_bindgen::JsCast;
        DomRequest::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<DomRequest> for IdbFileRequest {
    #[inline]
    fn as_ref(&self) -> &DomRequest {
        use wasm_bindgen::JsCast;
        DomRequest::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<IdbFileRequest> for EventTarget {
    #[inline]
    fn from(obj: IdbFileRequest) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for IdbFileRequest {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<IdbFileRequest> for ::js_sys::Object {
    #[inline]
    fn from(obj: IdbFileRequest) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for IdbFileRequest {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_file_handle_IDBFileRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbFileRequest as WasmDescribe>::describe();
    <Option<IdbFileHandle> as WasmDescribe>::describe();
}
impl IdbFileRequest {
    #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
    #[allow(bad_style)]
    #[doc = "The `fileHandle` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileRequest/fileHandle)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[allow(clippy::all)]
    pub fn file_handle(&self) -> Option<IdbFileHandle> {
        #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_file_handle_IDBFileRequest(
                self_: <&IdbFileRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<IdbFileHandle> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_file_handle_IDBFileRequest(
            self_: <&IdbFileRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<IdbFileHandle> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_file_handle_IDBFileRequest(self_)
            };
            <Option<IdbFileHandle> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_locked_file_IDBFileRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbFileRequest as WasmDescribe>::describe();
    <Option<IdbFileHandle> as WasmDescribe>::describe();
}
impl IdbFileRequest {
    #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
    #[allow(bad_style)]
    #[doc = "The `lockedFile` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileRequest/lockedFile)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[allow(clippy::all)]
    pub fn locked_file(&self) -> Option<IdbFileHandle> {
        #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_locked_file_IDBFileRequest(
                self_: <&IdbFileRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<IdbFileHandle> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_locked_file_IDBFileRequest(
            self_: <&IdbFileRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<IdbFileHandle> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_locked_file_IDBFileRequest(self_)
            };
            <Option<IdbFileHandle> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbFileRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onprogress_IDBFileRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbFileRequest as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl IdbFileRequest {
    #[cfg(all(feature = "IdbFileRequest",))]
    #[allow(bad_style)]
    #[doc = "The `onprogress` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileRequest/onprogress)\n\n*This API requires the following crate features to be activated: `IdbFileRequest`*"]
    #[allow(clippy::all)]
    pub fn onprogress(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "IdbFileRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onprogress_IDBFileRequest(
                self_: <&IdbFileRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onprogress_IDBFileRequest(
            self_: <&IdbFileRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onprogress_IDBFileRequest(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbFileRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onprogress_IDBFileRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbFileRequest as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl IdbFileRequest {
    #[cfg(all(feature = "IdbFileRequest",))]
    #[allow(bad_style)]
    #[doc = "The `onprogress` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileRequest/onprogress)\n\n*This API requires the following crate features to be activated: `IdbFileRequest`*"]
    #[allow(clippy::all)]
    pub fn set_onprogress(&self, onprogress: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "IdbFileRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onprogress_IDBFileRequest(
                self_: <&IdbFileRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onprogress : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onprogress_IDBFileRequest(
            self_: <&IdbFileRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onprogress: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onprogress);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onprogress =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onprogress,
                    );
                __widl_f_set_onprogress_IDBFileRequest(self_, onprogress)
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
pub static __WASM_BINDGEN_GENERATED_7ff4833bf1e62874: [u8; 534usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xD4\x01\0\0\0\0\x05\0\0\x02\x0EIDBFileRequest __widl_instanceof_IDBFileRequest\0\0\0\0#__widl_f_file_handle_IDBFileRequest\0\0\0\x01\x0EIDBFileRequest\x01\0\x01\nfileHandle\x01\x01\x05self_\nfileHandle\0\0\0#__widl_f_locked_file_IDBFileRequest\0\0\0\x01\x0EIDBFileRequest\x01\0\x01\nlockedFile\x01\x01\x05self_\nlockedFile\0\0\0\"__widl_f_onprogress_IDBFileRequest\0\0\0\x01\x0EIDBFileRequest\x01\0\x01\nonprogress\x01\x01\x05self_\nonprogress\0\0\0&__widl_f_set_onprogress_IDBFileRequest\0\0\0\x01\x0EIDBFileRequest\x01\0\x02\nonprogress\x01\x02\x05self_\nonprogress\nonprogress\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
