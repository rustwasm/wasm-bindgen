use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MutationObserver` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver)\n\n*This API requires the following crate features to be activated: `MutationObserver`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MutationObserver {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MutationObserver: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MutationObserver {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(77u32);
            inform(117u32);
            inform(116u32);
            inform(97u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
            inform(79u32);
            inform(98u32);
            inform(115u32);
            inform(101u32);
            inform(114u32);
            inform(118u32);
            inform(101u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for MutationObserver {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for MutationObserver {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MutationObserver {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MutationObserver {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MutationObserver {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MutationObserver {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MutationObserver {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MutationObserver {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MutationObserver {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MutationObserver>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MutationObserver {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MutationObserver {
        #[inline]
        fn from(obj: JsValue) -> MutationObserver {
            MutationObserver { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MutationObserver {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MutationObserver> for MutationObserver {
        #[inline]
        fn as_ref(&self) -> &MutationObserver {
            self
        }
    }
    impl From<MutationObserver> for JsValue {
        #[inline]
        fn from(obj: MutationObserver) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MutationObserver {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MutationObserver(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MutationObserver(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MutationObserver(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MutationObserver { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MutationObserver) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MutationObserver> for ::js_sys::Object {
    #[inline]
    fn from(obj: MutationObserver) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MutationObserver {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MutationObserver",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_MutationObserver() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&::js_sys::Function as WasmDescribe>::describe();
    <MutationObserver as WasmDescribe>::describe();
}
impl MutationObserver {
    #[cfg(all(feature = "MutationObserver",))]
    #[allow(bad_style)]
    #[doc = "The `new MutationObserver(..)` constructor, creating a new instance of `MutationObserver`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/MutationObserver)\n\n*This API requires the following crate features to be activated: `MutationObserver`*"]
    #[allow(clippy::all)]
    pub fn new(
        mutation_callback: &::js_sys::Function,
    ) -> Result<MutationObserver, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MutationObserver",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_MutationObserver(
                mutation_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MutationObserver as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_MutationObserver(
            mutation_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MutationObserver as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(mutation_callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let mutation_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        mutation_callback,
                    );
                __widl_f_new_MutationObserver(mutation_callback)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MutationObserver as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MutationObserver",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_disconnect_MutationObserver() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MutationObserver as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MutationObserver {
    #[cfg(all(feature = "MutationObserver",))]
    #[allow(bad_style)]
    #[doc = "The `disconnect()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/disconnect)\n\n*This API requires the following crate features to be activated: `MutationObserver`*"]
    #[allow(clippy::all)]
    pub fn disconnect(&self) {
        #[cfg(all(feature = "MutationObserver",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_disconnect_MutationObserver(
                self_: <&MutationObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_disconnect_MutationObserver(
            self_: <&MutationObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MutationObserver as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_disconnect_MutationObserver(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "MutationObserver", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_observe_MutationObserver() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MutationObserver as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MutationObserver {
    #[cfg(all(feature = "MutationObserver", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `observe()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/observe)\n\n*This API requires the following crate features to be activated: `MutationObserver`, `Node`*"]
    #[allow(clippy::all)]
    pub fn observe(&self, target: &Node) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MutationObserver", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_observe_MutationObserver(
                self_: <&MutationObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_observe_MutationObserver(
            self_: <&MutationObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MutationObserver as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                __widl_f_observe_MutationObserver(self_, target)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(
    feature = "MutationObserver",
    feature = "MutationObserverInit",
    feature = "Node",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_observe_with_options_MutationObserver() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&MutationObserver as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&MutationObserverInit as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MutationObserver {
    #[cfg(all(
        feature = "MutationObserver",
        feature = "MutationObserverInit",
        feature = "Node",
    ))]
    #[allow(bad_style)]
    #[doc = "The `observe()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/observe)\n\n*This API requires the following crate features to be activated: `MutationObserver`, `MutationObserverInit`, `Node`*"]
    #[allow(clippy::all)]
    pub fn observe_with_options(
        &self,
        target: &Node,
        options: &MutationObserverInit,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "MutationObserver",
            feature = "MutationObserverInit",
            feature = "Node",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_observe_with_options_MutationObserver(
                self_: <&MutationObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&MutationObserverInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_observe_with_options_MutationObserver(
            self_: <&MutationObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&MutationObserverInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MutationObserver as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let options =
                    <&MutationObserverInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_observe_with_options_MutationObserver(self_, target, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "MutationObserver",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_take_records_MutationObserver() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MutationObserver as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl MutationObserver {
    #[cfg(all(feature = "MutationObserver",))]
    #[allow(bad_style)]
    #[doc = "The `takeRecords()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/takeRecords)\n\n*This API requires the following crate features to be activated: `MutationObserver`*"]
    #[allow(clippy::all)]
    pub fn take_records(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "MutationObserver",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_take_records_MutationObserver(
                self_: <&MutationObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_take_records_MutationObserver(
            self_: <&MutationObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MutationObserver as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_take_records_MutationObserver(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_065e34db2d4ef2b4: [u8; 598usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x14\x02\0\0\0\0\x06\0\0\x02\x10MutationObserver\"__widl_instanceof_MutationObserver\0\0\0\0\x1D__widl_f_new_MutationObserver\x01\0\0\x01\x10MutationObserver\0\x01\x01\x11mutation_callback\x03new\0\0\0$__widl_f_disconnect_MutationObserver\0\0\0\x01\x10MutationObserver\x01\0\0\x01\x01\x05self_\ndisconnect\0\0\0!__widl_f_observe_MutationObserver\x01\0\0\x01\x10MutationObserver\x01\0\0\x01\x02\x05self_\x06target\x07observe\0\0\0.__widl_f_observe_with_options_MutationObserver\x01\0\0\x01\x10MutationObserver\x01\0\0\x01\x03\x05self_\x06target\x07options\x07observe\0\0\0&__widl_f_take_records_MutationObserver\0\0\0\x01\x10MutationObserver\x01\0\0\x01\x01\x05self_\x0BtakeRecords\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
