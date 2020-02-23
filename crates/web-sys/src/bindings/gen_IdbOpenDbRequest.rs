use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `IDBOpenDBRequest` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest)\n\n*This API requires the following crate features to be activated: `IdbOpenDbRequest`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct IdbOpenDbRequest {
    obj: IdbRequest,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_IdbOpenDbRequest: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for IdbOpenDbRequest {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(73u32);
            inform(68u32);
            inform(66u32);
            inform(79u32);
            inform(112u32);
            inform(101u32);
            inform(110u32);
            inform(68u32);
            inform(66u32);
            inform(82u32);
            inform(101u32);
            inform(113u32);
            inform(117u32);
            inform(101u32);
            inform(115u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for IdbOpenDbRequest {
        type Target = IdbRequest;
        #[inline]
        fn deref(&self) -> &IdbRequest {
            &self.obj
        }
    }
    impl IntoWasmAbi for IdbOpenDbRequest {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for IdbOpenDbRequest {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a IdbOpenDbRequest {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for IdbOpenDbRequest {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            IdbOpenDbRequest {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for IdbOpenDbRequest {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a IdbOpenDbRequest {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for IdbOpenDbRequest {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<IdbOpenDbRequest>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(IdbOpenDbRequest {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for IdbOpenDbRequest {
        #[inline]
        fn from(obj: JsValue) -> IdbOpenDbRequest {
            IdbOpenDbRequest { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for IdbOpenDbRequest {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<IdbOpenDbRequest> for IdbOpenDbRequest {
        #[inline]
        fn as_ref(&self) -> &IdbOpenDbRequest {
            self
        }
    }
    impl From<IdbOpenDbRequest> for JsValue {
        #[inline]
        fn from(obj: IdbOpenDbRequest) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for IdbOpenDbRequest {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_IDBOpenDBRequest(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_IDBOpenDBRequest(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_IDBOpenDBRequest(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            IdbOpenDbRequest { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const IdbOpenDbRequest) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<IdbOpenDbRequest> for IdbRequest {
    #[inline]
    fn from(obj: IdbOpenDbRequest) -> IdbRequest {
        use wasm_bindgen::JsCast;
        IdbRequest::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<IdbRequest> for IdbOpenDbRequest {
    #[inline]
    fn as_ref(&self) -> &IdbRequest {
        use wasm_bindgen::JsCast;
        IdbRequest::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<IdbOpenDbRequest> for EventTarget {
    #[inline]
    fn from(obj: IdbOpenDbRequest) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for IdbOpenDbRequest {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<IdbOpenDbRequest> for ::js_sys::Object {
    #[inline]
    fn from(obj: IdbOpenDbRequest) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for IdbOpenDbRequest {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "IdbOpenDbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onblocked_IDBOpenDBRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbOpenDbRequest as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl IdbOpenDbRequest {
    #[cfg(all(feature = "IdbOpenDbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `onblocked` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest/onblocked)\n\n*This API requires the following crate features to be activated: `IdbOpenDbRequest`*"]
    #[allow(clippy::all)]
    pub fn onblocked(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "IdbOpenDbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onblocked_IDBOpenDBRequest(
                self_: <&IdbOpenDbRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onblocked_IDBOpenDBRequest(
            self_: <&IdbOpenDbRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&IdbOpenDbRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onblocked_IDBOpenDBRequest(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbOpenDbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onblocked_IDBOpenDBRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbOpenDbRequest as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl IdbOpenDbRequest {
    #[cfg(all(feature = "IdbOpenDbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `onblocked` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest/onblocked)\n\n*This API requires the following crate features to be activated: `IdbOpenDbRequest`*"]
    #[allow(clippy::all)]
    pub fn set_onblocked(&self, onblocked: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "IdbOpenDbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onblocked_IDBOpenDBRequest(
                self_: <&IdbOpenDbRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onblocked: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onblocked_IDBOpenDBRequest(
            self_: <&IdbOpenDbRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onblocked: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onblocked);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&IdbOpenDbRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onblocked =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onblocked,
                    );
                __widl_f_set_onblocked_IDBOpenDBRequest(self_, onblocked)
            };
            ()
        }
    }
}
#[cfg(all(feature = "IdbOpenDbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onupgradeneeded_IDBOpenDBRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbOpenDbRequest as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl IdbOpenDbRequest {
    #[cfg(all(feature = "IdbOpenDbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `onupgradeneeded` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest/onupgradeneeded)\n\n*This API requires the following crate features to be activated: `IdbOpenDbRequest`*"]
    #[allow(clippy::all)]
    pub fn onupgradeneeded(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "IdbOpenDbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onupgradeneeded_IDBOpenDBRequest(
                self_: <&IdbOpenDbRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onupgradeneeded_IDBOpenDBRequest(
            self_: <&IdbOpenDbRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&IdbOpenDbRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onupgradeneeded_IDBOpenDBRequest(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbOpenDbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onupgradeneeded_IDBOpenDBRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbOpenDbRequest as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl IdbOpenDbRequest {
    #[cfg(all(feature = "IdbOpenDbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `onupgradeneeded` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest/onupgradeneeded)\n\n*This API requires the following crate features to be activated: `IdbOpenDbRequest`*"]
    #[allow(clippy::all)]
    pub fn set_onupgradeneeded(&self, onupgradeneeded: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "IdbOpenDbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onupgradeneeded_IDBOpenDBRequest(
                self_: <&IdbOpenDbRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onupgradeneeded : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onupgradeneeded_IDBOpenDBRequest(
            self_: <&IdbOpenDbRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onupgradeneeded : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onupgradeneeded);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&IdbOpenDbRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onupgradeneeded =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onupgradeneeded,
                    );
                __widl_f_set_onupgradeneeded_IDBOpenDBRequest(self_, onupgradeneeded)
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
pub static __WASM_BINDGEN_GENERATED_e0b0d70ac6bce58b: [u8; 595usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x11\x02\0\0\0\0\x05\0\0\x02\x10IDBOpenDBRequest\"__widl_instanceof_IDBOpenDBRequest\0\0\0\0#__widl_f_onblocked_IDBOpenDBRequest\0\0\0\x01\x10IDBOpenDBRequest\x01\0\x01\tonblocked\x01\x01\x05self_\tonblocked\0\0\0'__widl_f_set_onblocked_IDBOpenDBRequest\0\0\0\x01\x10IDBOpenDBRequest\x01\0\x02\tonblocked\x01\x02\x05self_\tonblocked\tonblocked\0\0\0)__widl_f_onupgradeneeded_IDBOpenDBRequest\0\0\0\x01\x10IDBOpenDBRequest\x01\0\x01\x0Fonupgradeneeded\x01\x01\x05self_\x0Fonupgradeneeded\0\0\0-__widl_f_set_onupgradeneeded_IDBOpenDBRequest\0\0\0\x01\x10IDBOpenDBRequest\x01\0\x02\x0Fonupgradeneeded\x01\x02\x05self_\x0Fonupgradeneeded\x0Fonupgradeneeded\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
