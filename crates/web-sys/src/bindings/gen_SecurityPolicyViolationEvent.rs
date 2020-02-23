use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SecurityPolicyViolationEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent)\n\n*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SecurityPolicyViolationEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SecurityPolicyViolationEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SecurityPolicyViolationEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(28u32);
            inform(83u32);
            inform(101u32);
            inform(99u32);
            inform(117u32);
            inform(114u32);
            inform(105u32);
            inform(116u32);
            inform(121u32);
            inform(80u32);
            inform(111u32);
            inform(108u32);
            inform(105u32);
            inform(99u32);
            inform(121u32);
            inform(86u32);
            inform(105u32);
            inform(111u32);
            inform(108u32);
            inform(97u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for SecurityPolicyViolationEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for SecurityPolicyViolationEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SecurityPolicyViolationEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SecurityPolicyViolationEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SecurityPolicyViolationEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SecurityPolicyViolationEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SecurityPolicyViolationEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SecurityPolicyViolationEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SecurityPolicyViolationEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SecurityPolicyViolationEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SecurityPolicyViolationEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SecurityPolicyViolationEvent {
        #[inline]
        fn from(obj: JsValue) -> SecurityPolicyViolationEvent {
            SecurityPolicyViolationEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SecurityPolicyViolationEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SecurityPolicyViolationEvent> for SecurityPolicyViolationEvent {
        #[inline]
        fn as_ref(&self) -> &SecurityPolicyViolationEvent {
            self
        }
    }
    impl From<SecurityPolicyViolationEvent> for JsValue {
        #[inline]
        fn from(obj: SecurityPolicyViolationEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SecurityPolicyViolationEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SecurityPolicyViolationEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SecurityPolicyViolationEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SecurityPolicyViolationEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SecurityPolicyViolationEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SecurityPolicyViolationEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SecurityPolicyViolationEvent> for Event {
    #[inline]
    fn from(obj: SecurityPolicyViolationEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for SecurityPolicyViolationEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SecurityPolicyViolationEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: SecurityPolicyViolationEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SecurityPolicyViolationEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SecurityPolicyViolationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_SecurityPolicyViolationEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <SecurityPolicyViolationEvent as WasmDescribe>::describe();
}
impl SecurityPolicyViolationEvent {
    #[cfg(all(feature = "SecurityPolicyViolationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new SecurityPolicyViolationEvent(..)` constructor, creating a new instance of `SecurityPolicyViolationEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/SecurityPolicyViolationEvent)\n\n*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<SecurityPolicyViolationEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SecurityPolicyViolationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_SecurityPolicyViolationEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SecurityPolicyViolationEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_SecurityPolicyViolationEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SecurityPolicyViolationEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_SecurityPolicyViolationEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(
                <SecurityPolicyViolationEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                    _ret,
                ),
            )
        }
    }
}
#[cfg(all(
    feature = "SecurityPolicyViolationEvent",
    feature = "SecurityPolicyViolationEventInit",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_SecurityPolicyViolationEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&SecurityPolicyViolationEventInit as WasmDescribe>::describe();
    <SecurityPolicyViolationEvent as WasmDescribe>::describe();
}
impl SecurityPolicyViolationEvent {
    #[cfg(all(
        feature = "SecurityPolicyViolationEvent",
        feature = "SecurityPolicyViolationEventInit",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new SecurityPolicyViolationEvent(..)` constructor, creating a new instance of `SecurityPolicyViolationEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/SecurityPolicyViolationEvent)\n\n*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`, `SecurityPolicyViolationEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &SecurityPolicyViolationEventInit,
    ) -> Result<SecurityPolicyViolationEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "SecurityPolicyViolationEvent",
            feature = "SecurityPolicyViolationEventInit",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_SecurityPolicyViolationEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict : < & SecurityPolicyViolationEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <SecurityPolicyViolationEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_SecurityPolicyViolationEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict : < & SecurityPolicyViolationEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> <SecurityPolicyViolationEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            drop(event_init_dict);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let event_init_dict = < & SecurityPolicyViolationEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( event_init_dict ) ;
                __widl_f_new_with_event_init_dict_SecurityPolicyViolationEvent(
                    type_,
                    event_init_dict,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(
                <SecurityPolicyViolationEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                    _ret,
                ),
            )
        }
    }
}
#[cfg(all(feature = "SecurityPolicyViolationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_document_uri_SecurityPolicyViolationEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SecurityPolicyViolationEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl SecurityPolicyViolationEvent {
    #[cfg(all(feature = "SecurityPolicyViolationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `documentURI` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/documentURI)\n\n*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    #[allow(clippy::all)]
    pub fn document_uri(&self) -> String {
        #[cfg(all(feature = "SecurityPolicyViolationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_document_uri_SecurityPolicyViolationEvent(
                self_: <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_document_uri_SecurityPolicyViolationEvent(
            self_: <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_document_uri_SecurityPolicyViolationEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SecurityPolicyViolationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_referrer_SecurityPolicyViolationEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SecurityPolicyViolationEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl SecurityPolicyViolationEvent {
    #[cfg(all(feature = "SecurityPolicyViolationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `referrer` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/referrer)\n\n*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    #[allow(clippy::all)]
    pub fn referrer(&self) -> String {
        #[cfg(all(feature = "SecurityPolicyViolationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_referrer_SecurityPolicyViolationEvent(
                self_: <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_referrer_SecurityPolicyViolationEvent(
            self_: <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_referrer_SecurityPolicyViolationEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SecurityPolicyViolationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_blocked_uri_SecurityPolicyViolationEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SecurityPolicyViolationEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl SecurityPolicyViolationEvent {
    #[cfg(all(feature = "SecurityPolicyViolationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `blockedURI` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/blockedURI)\n\n*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    #[allow(clippy::all)]
    pub fn blocked_uri(&self) -> String {
        #[cfg(all(feature = "SecurityPolicyViolationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_blocked_uri_SecurityPolicyViolationEvent(
                self_: <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_blocked_uri_SecurityPolicyViolationEvent(
            self_: <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_blocked_uri_SecurityPolicyViolationEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SecurityPolicyViolationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_violated_directive_SecurityPolicyViolationEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SecurityPolicyViolationEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl SecurityPolicyViolationEvent {
    #[cfg(all(feature = "SecurityPolicyViolationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `violatedDirective` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/violatedDirective)\n\n*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    #[allow(clippy::all)]
    pub fn violated_directive(&self) -> String {
        #[cfg(all(feature = "SecurityPolicyViolationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_violated_directive_SecurityPolicyViolationEvent(
                self_: <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_violated_directive_SecurityPolicyViolationEvent(
            self_: <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_violated_directive_SecurityPolicyViolationEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SecurityPolicyViolationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_effective_directive_SecurityPolicyViolationEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SecurityPolicyViolationEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl SecurityPolicyViolationEvent {
    #[cfg(all(feature = "SecurityPolicyViolationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `effectiveDirective` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/effectiveDirective)\n\n*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    #[allow(clippy::all)]
    pub fn effective_directive(&self) -> String {
        #[cfg(all(feature = "SecurityPolicyViolationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_effective_directive_SecurityPolicyViolationEvent(
                self_: <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_effective_directive_SecurityPolicyViolationEvent(
            self_: <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_effective_directive_SecurityPolicyViolationEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SecurityPolicyViolationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_original_policy_SecurityPolicyViolationEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SecurityPolicyViolationEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl SecurityPolicyViolationEvent {
    #[cfg(all(feature = "SecurityPolicyViolationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `originalPolicy` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/originalPolicy)\n\n*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    #[allow(clippy::all)]
    pub fn original_policy(&self) -> String {
        #[cfg(all(feature = "SecurityPolicyViolationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_original_policy_SecurityPolicyViolationEvent(
                self_: <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_original_policy_SecurityPolicyViolationEvent(
            self_: <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_original_policy_SecurityPolicyViolationEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SecurityPolicyViolationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_source_file_SecurityPolicyViolationEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SecurityPolicyViolationEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl SecurityPolicyViolationEvent {
    #[cfg(all(feature = "SecurityPolicyViolationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `sourceFile` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/sourceFile)\n\n*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    #[allow(clippy::all)]
    pub fn source_file(&self) -> String {
        #[cfg(all(feature = "SecurityPolicyViolationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_source_file_SecurityPolicyViolationEvent(
                self_: <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_source_file_SecurityPolicyViolationEvent(
            self_: <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_source_file_SecurityPolicyViolationEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SecurityPolicyViolationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_sample_SecurityPolicyViolationEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SecurityPolicyViolationEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl SecurityPolicyViolationEvent {
    #[cfg(all(feature = "SecurityPolicyViolationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `sample` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/sample)\n\n*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    #[allow(clippy::all)]
    pub fn sample(&self) -> String {
        #[cfg(all(feature = "SecurityPolicyViolationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_sample_SecurityPolicyViolationEvent(
                self_: <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_sample_SecurityPolicyViolationEvent(
            self_: <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_sample_SecurityPolicyViolationEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "SecurityPolicyViolationEvent",
    feature = "SecurityPolicyViolationEventDisposition",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_disposition_SecurityPolicyViolationEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SecurityPolicyViolationEvent as WasmDescribe>::describe();
    <SecurityPolicyViolationEventDisposition as WasmDescribe>::describe();
}
impl SecurityPolicyViolationEvent {
    #[cfg(all(
        feature = "SecurityPolicyViolationEvent",
        feature = "SecurityPolicyViolationEventDisposition",
    ))]
    #[allow(bad_style)]
    #[doc = "The `disposition` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/disposition)\n\n*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`, `SecurityPolicyViolationEventDisposition`*"]
    #[allow(clippy::all)]
    pub fn disposition(&self) -> SecurityPolicyViolationEventDisposition {
        #[cfg(all(
            feature = "SecurityPolicyViolationEvent",
            feature = "SecurityPolicyViolationEventDisposition",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_disposition_SecurityPolicyViolationEvent(
                self_: <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SecurityPolicyViolationEventDisposition as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_disposition_SecurityPolicyViolationEvent(
            self_: <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SecurityPolicyViolationEventDisposition as wasm_bindgen::convert::FromWasmAbi>::Abi
        {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_disposition_SecurityPolicyViolationEvent(self_)
            };
            < SecurityPolicyViolationEventDisposition as wasm_bindgen :: convert :: FromWasmAbi > :: from_abi ( _ret )
        }
    }
}
#[cfg(all(feature = "SecurityPolicyViolationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_status_code_SecurityPolicyViolationEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SecurityPolicyViolationEvent as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl SecurityPolicyViolationEvent {
    #[cfg(all(feature = "SecurityPolicyViolationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `statusCode` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/statusCode)\n\n*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    #[allow(clippy::all)]
    pub fn status_code(&self) -> u16 {
        #[cfg(all(feature = "SecurityPolicyViolationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_status_code_SecurityPolicyViolationEvent(
                self_: <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_status_code_SecurityPolicyViolationEvent(
            self_: <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_status_code_SecurityPolicyViolationEvent(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SecurityPolicyViolationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_line_number_SecurityPolicyViolationEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SecurityPolicyViolationEvent as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl SecurityPolicyViolationEvent {
    #[cfg(all(feature = "SecurityPolicyViolationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `lineNumber` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/lineNumber)\n\n*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    #[allow(clippy::all)]
    pub fn line_number(&self) -> i32 {
        #[cfg(all(feature = "SecurityPolicyViolationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_line_number_SecurityPolicyViolationEvent(
                self_: <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_line_number_SecurityPolicyViolationEvent(
            self_: <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_line_number_SecurityPolicyViolationEvent(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SecurityPolicyViolationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_column_number_SecurityPolicyViolationEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SecurityPolicyViolationEvent as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl SecurityPolicyViolationEvent {
    #[cfg(all(feature = "SecurityPolicyViolationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `columnNumber` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/columnNumber)\n\n*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    #[allow(clippy::all)]
    pub fn column_number(&self) -> i32 {
        #[cfg(all(feature = "SecurityPolicyViolationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_column_number_SecurityPolicyViolationEvent(
                self_: <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_column_number_SecurityPolicyViolationEvent(
            self_: <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SecurityPolicyViolationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_column_number_SecurityPolicyViolationEvent(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_29479d70eed9901e: [u8; 1880usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x16\x07\0\0\0\0\x0F\0\0\x02\x1CSecurityPolicyViolationEvent.__widl_instanceof_SecurityPolicyViolationEvent\0\0\0\0)__widl_f_new_SecurityPolicyViolationEvent\x01\0\0\x01\x1CSecurityPolicyViolationEvent\0\x01\x01\x05type_\x03new\0\0\0>__widl_f_new_with_event_init_dict_SecurityPolicyViolationEvent\x01\0\0\x01\x1CSecurityPolicyViolationEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\02__widl_f_document_uri_SecurityPolicyViolationEvent\0\0\0\x01\x1CSecurityPolicyViolationEvent\x01\0\x01\x0BdocumentURI\x01\x01\x05self_\x0BdocumentURI\0\0\0.__widl_f_referrer_SecurityPolicyViolationEvent\0\0\0\x01\x1CSecurityPolicyViolationEvent\x01\0\x01\x08referrer\x01\x01\x05self_\x08referrer\0\0\01__widl_f_blocked_uri_SecurityPolicyViolationEvent\0\0\0\x01\x1CSecurityPolicyViolationEvent\x01\0\x01\nblockedURI\x01\x01\x05self_\nblockedURI\0\0\08__widl_f_violated_directive_SecurityPolicyViolationEvent\0\0\0\x01\x1CSecurityPolicyViolationEvent\x01\0\x01\x11violatedDirective\x01\x01\x05self_\x11violatedDirective\0\0\09__widl_f_effective_directive_SecurityPolicyViolationEvent\0\0\0\x01\x1CSecurityPolicyViolationEvent\x01\0\x01\x12effectiveDirective\x01\x01\x05self_\x12effectiveDirective\0\0\05__widl_f_original_policy_SecurityPolicyViolationEvent\0\0\0\x01\x1CSecurityPolicyViolationEvent\x01\0\x01\x0EoriginalPolicy\x01\x01\x05self_\x0EoriginalPolicy\0\0\01__widl_f_source_file_SecurityPolicyViolationEvent\0\0\0\x01\x1CSecurityPolicyViolationEvent\x01\0\x01\nsourceFile\x01\x01\x05self_\nsourceFile\0\0\0,__widl_f_sample_SecurityPolicyViolationEvent\0\0\0\x01\x1CSecurityPolicyViolationEvent\x01\0\x01\x06sample\x01\x01\x05self_\x06sample\0\0\01__widl_f_disposition_SecurityPolicyViolationEvent\0\0\0\x01\x1CSecurityPolicyViolationEvent\x01\0\x01\x0Bdisposition\x01\x01\x05self_\x0Bdisposition\0\0\01__widl_f_status_code_SecurityPolicyViolationEvent\0\0\0\x01\x1CSecurityPolicyViolationEvent\x01\0\x01\nstatusCode\x01\x01\x05self_\nstatusCode\0\0\01__widl_f_line_number_SecurityPolicyViolationEvent\0\0\0\x01\x1CSecurityPolicyViolationEvent\x01\0\x01\nlineNumber\x01\x01\x05self_\nlineNumber\0\0\03__widl_f_column_number_SecurityPolicyViolationEvent\0\0\0\x01\x1CSecurityPolicyViolationEvent\x01\0\x01\x0CcolumnNumber\x01\x01\x05self_\x0CcolumnNumber\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
