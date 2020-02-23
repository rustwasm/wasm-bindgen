use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `PaymentRequestUpdateEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestUpdateEvent)\n\n*This API requires the following crate features to be activated: `PaymentRequestUpdateEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct PaymentRequestUpdateEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_PaymentRequestUpdateEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for PaymentRequestUpdateEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(25u32);
            inform(80u32);
            inform(97u32);
            inform(121u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
            inform(82u32);
            inform(101u32);
            inform(113u32);
            inform(117u32);
            inform(101u32);
            inform(115u32);
            inform(116u32);
            inform(85u32);
            inform(112u32);
            inform(100u32);
            inform(97u32);
            inform(116u32);
            inform(101u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for PaymentRequestUpdateEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for PaymentRequestUpdateEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for PaymentRequestUpdateEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PaymentRequestUpdateEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for PaymentRequestUpdateEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            PaymentRequestUpdateEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for PaymentRequestUpdateEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a PaymentRequestUpdateEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for PaymentRequestUpdateEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<PaymentRequestUpdateEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(PaymentRequestUpdateEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for PaymentRequestUpdateEvent {
        #[inline]
        fn from(obj: JsValue) -> PaymentRequestUpdateEvent {
            PaymentRequestUpdateEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for PaymentRequestUpdateEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<PaymentRequestUpdateEvent> for PaymentRequestUpdateEvent {
        #[inline]
        fn as_ref(&self) -> &PaymentRequestUpdateEvent {
            self
        }
    }
    impl From<PaymentRequestUpdateEvent> for JsValue {
        #[inline]
        fn from(obj: PaymentRequestUpdateEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for PaymentRequestUpdateEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_PaymentRequestUpdateEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_PaymentRequestUpdateEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_PaymentRequestUpdateEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PaymentRequestUpdateEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PaymentRequestUpdateEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<PaymentRequestUpdateEvent> for Event {
    #[inline]
    fn from(obj: PaymentRequestUpdateEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for PaymentRequestUpdateEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<PaymentRequestUpdateEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: PaymentRequestUpdateEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for PaymentRequestUpdateEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "PaymentRequestUpdateEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_PaymentRequestUpdateEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <PaymentRequestUpdateEvent as WasmDescribe>::describe();
}
impl PaymentRequestUpdateEvent {
    #[cfg(all(feature = "PaymentRequestUpdateEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new PaymentRequestUpdateEvent(..)` constructor, creating a new instance of `PaymentRequestUpdateEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestUpdateEvent/PaymentRequestUpdateEvent)\n\n*This API requires the following crate features to be activated: `PaymentRequestUpdateEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<PaymentRequestUpdateEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PaymentRequestUpdateEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_PaymentRequestUpdateEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <PaymentRequestUpdateEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_PaymentRequestUpdateEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PaymentRequestUpdateEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_PaymentRequestUpdateEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<PaymentRequestUpdateEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "PaymentRequestUpdateEvent",
    feature = "PaymentRequestUpdateEventInit",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_PaymentRequestUpdateEvent()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&PaymentRequestUpdateEventInit as WasmDescribe>::describe();
    <PaymentRequestUpdateEvent as WasmDescribe>::describe();
}
impl PaymentRequestUpdateEvent {
    #[cfg(all(
        feature = "PaymentRequestUpdateEvent",
        feature = "PaymentRequestUpdateEventInit",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new PaymentRequestUpdateEvent(..)` constructor, creating a new instance of `PaymentRequestUpdateEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestUpdateEvent/PaymentRequestUpdateEvent)\n\n*This API requires the following crate features to be activated: `PaymentRequestUpdateEvent`, `PaymentRequestUpdateEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &PaymentRequestUpdateEventInit,
    ) -> Result<PaymentRequestUpdateEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "PaymentRequestUpdateEvent",
            feature = "PaymentRequestUpdateEventInit",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_PaymentRequestUpdateEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict : < & PaymentRequestUpdateEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <PaymentRequestUpdateEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_PaymentRequestUpdateEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict : < & PaymentRequestUpdateEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> <PaymentRequestUpdateEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                let event_init_dict = < & PaymentRequestUpdateEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( event_init_dict ) ;
                __widl_f_new_with_event_init_dict_PaymentRequestUpdateEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<PaymentRequestUpdateEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "PaymentRequestUpdateEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_update_with_PaymentRequestUpdateEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PaymentRequestUpdateEvent as WasmDescribe>::describe();
    <&::js_sys::Promise as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl PaymentRequestUpdateEvent {
    #[cfg(all(feature = "PaymentRequestUpdateEvent",))]
    #[allow(bad_style)]
    #[doc = "The `updateWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestUpdateEvent/updateWith)\n\n*This API requires the following crate features to be activated: `PaymentRequestUpdateEvent`*"]
    #[allow(clippy::all)]
    pub fn update_with(
        &self,
        details_promise: &::js_sys::Promise,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PaymentRequestUpdateEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_update_with_PaymentRequestUpdateEvent(
                self_: <&PaymentRequestUpdateEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                details_promise: <&::js_sys::Promise as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_update_with_PaymentRequestUpdateEvent(
            self_: <&PaymentRequestUpdateEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            details_promise: <&::js_sys::Promise as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(details_promise);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PaymentRequestUpdateEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let details_promise =
                    <&::js_sys::Promise as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        details_promise,
                    );
                __widl_f_update_with_PaymentRequestUpdateEvent(self_, details_promise)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_6d896c25211dca7b: [u8; 504usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xB6\x01\0\0\0\0\x04\0\0\x02\x19PaymentRequestUpdateEvent+__widl_instanceof_PaymentRequestUpdateEvent\0\0\0\0&__widl_f_new_PaymentRequestUpdateEvent\x01\0\0\x01\x19PaymentRequestUpdateEvent\0\x01\x01\x05type_\x03new\0\0\0;__widl_f_new_with_event_init_dict_PaymentRequestUpdateEvent\x01\0\0\x01\x19PaymentRequestUpdateEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0.__widl_f_update_with_PaymentRequestUpdateEvent\x01\0\0\x01\x19PaymentRequestUpdateEvent\x01\0\0\x01\x02\x05self_\x0Fdetails_promise\nupdateWith\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
