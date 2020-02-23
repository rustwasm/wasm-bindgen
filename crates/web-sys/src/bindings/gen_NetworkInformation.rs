use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `NetworkInformation` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NetworkInformation)\n\n*This API requires the following crate features to be activated: `NetworkInformation`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct NetworkInformation {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_NetworkInformation: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for NetworkInformation {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(18u32);
            inform(78u32);
            inform(101u32);
            inform(116u32);
            inform(119u32);
            inform(111u32);
            inform(114u32);
            inform(107u32);
            inform(73u32);
            inform(110u32);
            inform(102u32);
            inform(111u32);
            inform(114u32);
            inform(109u32);
            inform(97u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for NetworkInformation {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for NetworkInformation {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for NetworkInformation {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a NetworkInformation {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for NetworkInformation {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            NetworkInformation {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for NetworkInformation {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a NetworkInformation {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for NetworkInformation {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<NetworkInformation>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(NetworkInformation {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for NetworkInformation {
        #[inline]
        fn from(obj: JsValue) -> NetworkInformation {
            NetworkInformation { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for NetworkInformation {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<NetworkInformation> for NetworkInformation {
        #[inline]
        fn as_ref(&self) -> &NetworkInformation {
            self
        }
    }
    impl From<NetworkInformation> for JsValue {
        #[inline]
        fn from(obj: NetworkInformation) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for NetworkInformation {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_NetworkInformation(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_NetworkInformation(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_NetworkInformation(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            NetworkInformation { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const NetworkInformation) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<NetworkInformation> for EventTarget {
    #[inline]
    fn from(obj: NetworkInformation) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for NetworkInformation {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<NetworkInformation> for ::js_sys::Object {
    #[inline]
    fn from(obj: NetworkInformation) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for NetworkInformation {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "ConnectionType", feature = "NetworkInformation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_NetworkInformation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&NetworkInformation as WasmDescribe>::describe();
    <ConnectionType as WasmDescribe>::describe();
}
impl NetworkInformation {
    #[cfg(all(feature = "ConnectionType", feature = "NetworkInformation",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NetworkInformation/type)\n\n*This API requires the following crate features to be activated: `ConnectionType`, `NetworkInformation`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> ConnectionType {
        #[cfg(all(feature = "ConnectionType", feature = "NetworkInformation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_NetworkInformation(
                self_: <&NetworkInformation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ConnectionType as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_NetworkInformation(
            self_: <&NetworkInformation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ConnectionType as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&NetworkInformation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_NetworkInformation(self_)
            };
            <ConnectionType as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "NetworkInformation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontypechange_NetworkInformation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&NetworkInformation as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl NetworkInformation {
    #[cfg(all(feature = "NetworkInformation",))]
    #[allow(bad_style)]
    #[doc = "The `ontypechange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NetworkInformation/ontypechange)\n\n*This API requires the following crate features to be activated: `NetworkInformation`*"]
    #[allow(clippy::all)]
    pub fn ontypechange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "NetworkInformation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontypechange_NetworkInformation(
                self_: <&NetworkInformation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontypechange_NetworkInformation(
            self_: <&NetworkInformation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&NetworkInformation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ontypechange_NetworkInformation(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "NetworkInformation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontypechange_NetworkInformation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&NetworkInformation as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl NetworkInformation {
    #[cfg(all(feature = "NetworkInformation",))]
    #[allow(bad_style)]
    #[doc = "The `ontypechange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NetworkInformation/ontypechange)\n\n*This API requires the following crate features to be activated: `NetworkInformation`*"]
    #[allow(clippy::all)]
    pub fn set_ontypechange(&self, ontypechange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "NetworkInformation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontypechange_NetworkInformation(
                self_: <&NetworkInformation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontypechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontypechange_NetworkInformation(
            self_: <&NetworkInformation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontypechange: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ontypechange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&NetworkInformation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ontypechange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontypechange,
                    );
                __widl_f_set_ontypechange_NetworkInformation(self_, ontypechange)
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
pub static __WASM_BINDGEN_GENERATED_cc109e5ffc336ece: [u8; 470usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x94\x01\0\0\0\0\x04\0\0\x02\x12NetworkInformation$__widl_instanceof_NetworkInformation\0\0\0\0 __widl_f_type_NetworkInformation\0\0\0\x01\x12NetworkInformation\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0(__widl_f_ontypechange_NetworkInformation\0\0\0\x01\x12NetworkInformation\x01\0\x01\x0Contypechange\x01\x01\x05self_\x0Contypechange\0\0\0,__widl_f_set_ontypechange_NetworkInformation\0\0\0\x01\x12NetworkInformation\x01\0\x02\x0Contypechange\x01\x02\x05self_\x0Contypechange\x0Contypechange\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
