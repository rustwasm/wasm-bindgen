use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `IntersectionObserver` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver)\n\n*This API requires the following crate features to be activated: `IntersectionObserver`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct IntersectionObserver {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_IntersectionObserver: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for IntersectionObserver {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(20u32);
            inform(73u32);
            inform(110u32);
            inform(116u32);
            inform(101u32);
            inform(114u32);
            inform(115u32);
            inform(101u32);
            inform(99u32);
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
    impl core::ops::Deref for IntersectionObserver {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for IntersectionObserver {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for IntersectionObserver {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a IntersectionObserver {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for IntersectionObserver {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            IntersectionObserver {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for IntersectionObserver {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a IntersectionObserver {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for IntersectionObserver {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<IntersectionObserver>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(IntersectionObserver {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for IntersectionObserver {
        #[inline]
        fn from(obj: JsValue) -> IntersectionObserver {
            IntersectionObserver { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for IntersectionObserver {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<IntersectionObserver> for IntersectionObserver {
        #[inline]
        fn as_ref(&self) -> &IntersectionObserver {
            self
        }
    }
    impl From<IntersectionObserver> for JsValue {
        #[inline]
        fn from(obj: IntersectionObserver) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for IntersectionObserver {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_IntersectionObserver(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_IntersectionObserver(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_IntersectionObserver(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            IntersectionObserver { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const IntersectionObserver) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<IntersectionObserver> for ::js_sys::Object {
    #[inline]
    fn from(obj: IntersectionObserver) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for IntersectionObserver {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "IntersectionObserver",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_IntersectionObserver() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&::js_sys::Function as WasmDescribe>::describe();
    <IntersectionObserver as WasmDescribe>::describe();
}
impl IntersectionObserver {
    #[cfg(all(feature = "IntersectionObserver",))]
    #[allow(bad_style)]
    #[doc = "The `new IntersectionObserver(..)` constructor, creating a new instance of `IntersectionObserver`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/IntersectionObserver)\n\n*This API requires the following crate features to be activated: `IntersectionObserver`*"]
    #[allow(clippy::all)]
    pub fn new(
        intersection_callback: &::js_sys::Function,
    ) -> Result<IntersectionObserver, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IntersectionObserver",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_IntersectionObserver(
                intersection_callback : < & :: js_sys :: Function as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <IntersectionObserver as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_IntersectionObserver(
            intersection_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IntersectionObserver as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(intersection_callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let intersection_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        intersection_callback,
                    );
                __widl_f_new_IntersectionObserver(intersection_callback)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IntersectionObserver as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IntersectionObserver", feature = "IntersectionObserverInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_options_IntersectionObserver() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&::js_sys::Function as WasmDescribe>::describe();
    <&IntersectionObserverInit as WasmDescribe>::describe();
    <IntersectionObserver as WasmDescribe>::describe();
}
impl IntersectionObserver {
    #[cfg(all(feature = "IntersectionObserver", feature = "IntersectionObserverInit",))]
    #[allow(bad_style)]
    #[doc = "The `new IntersectionObserver(..)` constructor, creating a new instance of `IntersectionObserver`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/IntersectionObserver)\n\n*This API requires the following crate features to be activated: `IntersectionObserver`, `IntersectionObserverInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_options(
        intersection_callback: &::js_sys::Function,
        options: &IntersectionObserverInit,
    ) -> Result<IntersectionObserver, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IntersectionObserver", feature = "IntersectionObserverInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_options_IntersectionObserver(
                intersection_callback : < & :: js_sys :: Function as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                options: <&IntersectionObserverInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IntersectionObserver as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_options_IntersectionObserver(
            intersection_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&IntersectionObserverInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IntersectionObserver as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(intersection_callback);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let intersection_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        intersection_callback,
                    );
                let options =
                    <&IntersectionObserverInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_new_with_options_IntersectionObserver(intersection_callback, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IntersectionObserver as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IntersectionObserver",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_disconnect_IntersectionObserver() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IntersectionObserver as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl IntersectionObserver {
    #[cfg(all(feature = "IntersectionObserver",))]
    #[allow(bad_style)]
    #[doc = "The `disconnect()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/disconnect)\n\n*This API requires the following crate features to be activated: `IntersectionObserver`*"]
    #[allow(clippy::all)]
    pub fn disconnect(&self) {
        #[cfg(all(feature = "IntersectionObserver",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_disconnect_IntersectionObserver(
                self_: <&IntersectionObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_disconnect_IntersectionObserver(
            self_: <&IntersectionObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&IntersectionObserver as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_disconnect_IntersectionObserver(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Element", feature = "IntersectionObserver",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_observe_IntersectionObserver() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IntersectionObserver as WasmDescribe>::describe();
    <&Element as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl IntersectionObserver {
    #[cfg(all(feature = "Element", feature = "IntersectionObserver",))]
    #[allow(bad_style)]
    #[doc = "The `observe()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/observe)\n\n*This API requires the following crate features to be activated: `Element`, `IntersectionObserver`*"]
    #[allow(clippy::all)]
    pub fn observe(&self, target: &Element) {
        #[cfg(all(feature = "Element", feature = "IntersectionObserver",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_observe_IntersectionObserver(
                self_: <&IntersectionObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_observe_IntersectionObserver(
            self_: <&IntersectionObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&IntersectionObserver as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                __widl_f_observe_IntersectionObserver(self_, target)
            };
            ()
        }
    }
}
#[cfg(all(feature = "IntersectionObserver",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_take_records_IntersectionObserver() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IntersectionObserver as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl IntersectionObserver {
    #[cfg(all(feature = "IntersectionObserver",))]
    #[allow(bad_style)]
    #[doc = "The `takeRecords()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/takeRecords)\n\n*This API requires the following crate features to be activated: `IntersectionObserver`*"]
    #[allow(clippy::all)]
    pub fn take_records(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "IntersectionObserver",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_take_records_IntersectionObserver(
                self_: <&IntersectionObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_take_records_IntersectionObserver(
            self_: <&IntersectionObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&IntersectionObserver as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_take_records_IntersectionObserver(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element", feature = "IntersectionObserver",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_unobserve_IntersectionObserver() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IntersectionObserver as WasmDescribe>::describe();
    <&Element as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl IntersectionObserver {
    #[cfg(all(feature = "Element", feature = "IntersectionObserver",))]
    #[allow(bad_style)]
    #[doc = "The `unobserve()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/unobserve)\n\n*This API requires the following crate features to be activated: `Element`, `IntersectionObserver`*"]
    #[allow(clippy::all)]
    pub fn unobserve(&self, target: &Element) {
        #[cfg(all(feature = "Element", feature = "IntersectionObserver",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_unobserve_IntersectionObserver(
                self_: <&IntersectionObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_unobserve_IntersectionObserver(
            self_: <&IntersectionObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&IntersectionObserver as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                __widl_f_unobserve_IntersectionObserver(self_, target)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Element", feature = "IntersectionObserver",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_root_IntersectionObserver() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IntersectionObserver as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl IntersectionObserver {
    #[cfg(all(feature = "Element", feature = "IntersectionObserver",))]
    #[allow(bad_style)]
    #[doc = "The `root` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/root)\n\n*This API requires the following crate features to be activated: `Element`, `IntersectionObserver`*"]
    #[allow(clippy::all)]
    pub fn root(&self) -> Option<Element> {
        #[cfg(all(feature = "Element", feature = "IntersectionObserver",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_root_IntersectionObserver(
                self_: <&IntersectionObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_root_IntersectionObserver(
            self_: <&IntersectionObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&IntersectionObserver as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_root_IntersectionObserver(self_)
            };
            <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IntersectionObserver",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_root_margin_IntersectionObserver() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IntersectionObserver as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl IntersectionObserver {
    #[cfg(all(feature = "IntersectionObserver",))]
    #[allow(bad_style)]
    #[doc = "The `rootMargin` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/rootMargin)\n\n*This API requires the following crate features to be activated: `IntersectionObserver`*"]
    #[allow(clippy::all)]
    pub fn root_margin(&self) -> String {
        #[cfg(all(feature = "IntersectionObserver",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_root_margin_IntersectionObserver(
                self_: <&IntersectionObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_root_margin_IntersectionObserver(
            self_: <&IntersectionObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&IntersectionObserver as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_root_margin_IntersectionObserver(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IntersectionObserver",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_thresholds_IntersectionObserver() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IntersectionObserver as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl IntersectionObserver {
    #[cfg(all(feature = "IntersectionObserver",))]
    #[allow(bad_style)]
    #[doc = "The `thresholds` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/thresholds)\n\n*This API requires the following crate features to be activated: `IntersectionObserver`*"]
    #[allow(clippy::all)]
    pub fn thresholds(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "IntersectionObserver",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_thresholds_IntersectionObserver(
                self_: <&IntersectionObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_thresholds_IntersectionObserver(
            self_: <&IntersectionObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&IntersectionObserver as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_thresholds_IntersectionObserver(self_)
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
pub static __WASM_BINDGEN_GENERATED_067f387e0f18d345: [u8; 1034usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xC8\x03\0\0\0\0\n\0\0\x02\x14IntersectionObserver&__widl_instanceof_IntersectionObserver\0\0\0\0!__widl_f_new_IntersectionObserver\x01\0\0\x01\x14IntersectionObserver\0\x01\x01\x15intersection_callback\x03new\0\0\0.__widl_f_new_with_options_IntersectionObserver\x01\0\0\x01\x14IntersectionObserver\0\x01\x02\x15intersection_callback\x07options\x03new\0\0\0(__widl_f_disconnect_IntersectionObserver\0\0\0\x01\x14IntersectionObserver\x01\0\0\x01\x01\x05self_\ndisconnect\0\0\0%__widl_f_observe_IntersectionObserver\0\0\0\x01\x14IntersectionObserver\x01\0\0\x01\x02\x05self_\x06target\x07observe\0\0\0*__widl_f_take_records_IntersectionObserver\0\0\0\x01\x14IntersectionObserver\x01\0\0\x01\x01\x05self_\x0BtakeRecords\0\0\0'__widl_f_unobserve_IntersectionObserver\0\0\0\x01\x14IntersectionObserver\x01\0\0\x01\x02\x05self_\x06target\tunobserve\0\0\0\"__widl_f_root_IntersectionObserver\0\0\0\x01\x14IntersectionObserver\x01\0\x01\x04root\x01\x01\x05self_\x04root\0\0\0)__widl_f_root_margin_IntersectionObserver\0\0\0\x01\x14IntersectionObserver\x01\0\x01\nrootMargin\x01\x01\x05self_\nrootMargin\0\0\0(__widl_f_thresholds_IntersectionObserver\0\0\0\x01\x14IntersectionObserver\x01\0\x01\nthresholds\x01\x01\x05self_\nthresholds\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
