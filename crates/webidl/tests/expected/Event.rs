#[allow(bad_style)]
pub struct Event {
    obj: ::wasm_bindgen::JsValue,
}
impl ::wasm_bindgen::describe::WasmDescribe for Event {
    fn describe() {
        ::wasm_bindgen::JsValue::describe();
    }
}
impl ::wasm_bindgen::convert::IntoWasmAbi for Event {
    type Abi = <::wasm_bindgen::JsValue as ::wasm_bindgen::convert::IntoWasmAbi>::Abi;
    fn into_abi(self, extra: &mut ::wasm_bindgen::convert::Stack) -> Self::Abi {
        self.obj.into_abi(extra)
    }
}
impl ::wasm_bindgen::convert::FromWasmAbi for Event {
    type Abi = <::wasm_bindgen::JsValue as ::wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi, extra: &mut ::wasm_bindgen::convert::Stack) -> Self {
        Event {
            obj: ::wasm_bindgen::JsValue::from_abi(js, extra),
        }
    }
}
impl<'a> ::wasm_bindgen::convert::IntoWasmAbi for &'a Event {
    type Abi = <&'a ::wasm_bindgen::JsValue as ::wasm_bindgen::convert::IntoWasmAbi>::Abi;
    fn into_abi(self, extra: &mut ::wasm_bindgen::convert::Stack) -> Self::Abi {
        (&self.obj).into_abi(extra)
    }
}
impl ::wasm_bindgen::convert::RefFromWasmAbi for Event {
    type Abi = <::wasm_bindgen::JsValue as ::wasm_bindgen::convert::RefFromWasmAbi>::Abi;
    type Anchor = ::wasm_bindgen::__rt::core::mem::ManuallyDrop<Event>;
    unsafe fn ref_from_abi(
        js: Self::Abi,
        extra: &mut ::wasm_bindgen::convert::Stack,
    ) -> Self::Anchor {
        let tmp =
            <::wasm_bindgen::JsValue as ::wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(
                js, extra,
            );
        ::wasm_bindgen::__rt::core::mem::ManuallyDrop::new(Event {
            obj: ::wasm_bindgen::__rt::core::mem::ManuallyDrop::into_inner(tmp),
        })
    }
}
impl From<::wasm_bindgen::JsValue> for Event {
    fn from(obj: ::wasm_bindgen::JsValue) -> Event {
        Event { obj }
    }
}
impl From<Event> for ::wasm_bindgen::JsValue {
    fn from(obj: Event) -> ::wasm_bindgen::JsValue {
        obj.obj
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
pub extern "C" fn __wbindgen_describe___widl_f_new_Event() {
    use wasm_bindgen::describe::*;
    inform(FUNCTION);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <EventInit as WasmDescribe>::describe();
    inform(1);
    <Event as WasmDescribe>::describe();
}
impl Event {
    #[allow(bad_style)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    pub extern "C" fn new(type_: &str, event_init_dict: EventInit) -> Event {
        ::wasm_bindgen::__rt::link_this_library();
        #[wasm_import_module = "__wbindgen_placeholder__"]
        extern "C" {
            fn __widl_f_new_Event(
                type_: <&str as ::wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict: <EventInit as ::wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Event as ::wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        unsafe {
            let _ret = {
                let mut __stack = ::wasm_bindgen::convert::GlobalStack::new();
                let type_ =
                    <&str as ::wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_, &mut __stack);
                let event_init_dict = <EventInit as ::wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                    event_init_dict,
                    &mut __stack,
                );
                __widl_f_new_Event(type_, event_init_dict)
            };
            <Event as ::wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
                &mut ::wasm_bindgen::convert::GlobalStack::new(),
            )
        }
    }
    #[allow(bad_style, unused_variables)]
    #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
    pub extern "C" fn new(type_: &str, event_init_dict: EventInit) -> Event {
        panic!(
            "cannot call wasm-bindgen imported functions on \
            non-wasm targets"
        );
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
pub extern "C" fn __wbindgen_describe___widl_f_event_phase_Event() {
    use wasm_bindgen::describe::*;
    inform(FUNCTION);
    inform(1u32);
    <&Event as WasmDescribe>::describe();
    inform(1);
    <u16 as WasmDescribe>::describe();
}
impl Event {
    #[allow(bad_style)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    pub extern "C" fn event_phase(&self) -> u16 {
        ::wasm_bindgen::__rt::link_this_library();
        #[wasm_import_module = "__wbindgen_placeholder__"]
        extern "C" {
            fn __widl_f_event_phase_Event(
                self_: <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as ::wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        unsafe {
            let _ret = {
                let mut __stack = ::wasm_bindgen::convert::GlobalStack::new();
                let self_ =
                    <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::into_abi(self, &mut __stack);
                __widl_f_event_phase_Event(self_)
            };
            <u16 as ::wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
                &mut ::wasm_bindgen::convert::GlobalStack::new(),
            )
        }
    }
    #[allow(bad_style, unused_variables)]
    #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
    pub extern "C" fn event_phase(&self) -> u16 {
        panic!(
            "cannot call wasm-bindgen imported functions on \
            non-wasm targets"
        );
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
pub extern "C" fn __wbindgen_describe___widl_f_stop_propagation_Event() {
    use wasm_bindgen::describe::*;
    inform(FUNCTION);
    inform(1u32);
    <&Event as WasmDescribe>::describe();
    inform(0);
}
impl Event {
    #[allow(bad_style)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    pub extern "C" fn stop_propagation(&self) {
        ::wasm_bindgen::__rt::link_this_library();
        #[wasm_import_module = "__wbindgen_placeholder__"]
        extern "C" {
            fn __widl_f_stop_propagation_Event(
                self_: <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        unsafe {
            let _ret = {
                let mut __stack = ::wasm_bindgen::convert::GlobalStack::new();
                let self_ =
                    <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::into_abi(self, &mut __stack);
                __widl_f_stop_propagation_Event(self_)
            };
            ()
        }
    }
    #[allow(bad_style, unused_variables)]
    #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
    pub extern "C" fn stop_propagation(&self) {
        panic!(
            "cannot call wasm-bindgen imported functions on \
            non-wasm targets"
        );
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
pub extern "C" fn __wbindgen_describe___widl_f_stop_immediate_propagation_Event() {
    use wasm_bindgen::describe::*;
    inform(FUNCTION);
    inform(1u32);
    <&Event as WasmDescribe>::describe();
    inform(0);
}
impl Event {
    #[allow(bad_style)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    pub extern "C" fn stop_immediate_propagation(&self) {
        ::wasm_bindgen::__rt::link_this_library();
        #[wasm_import_module = "__wbindgen_placeholder__"]
        extern "C" {
            fn __widl_f_stop_immediate_propagation_Event(
                self_: <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        unsafe {
            let _ret = {
                let mut __stack = ::wasm_bindgen::convert::GlobalStack::new();
                let self_ =
                    <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::into_abi(self, &mut __stack);
                __widl_f_stop_immediate_propagation_Event(self_)
            };
            ()
        }
    }
    #[allow(bad_style, unused_variables)]
    #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
    pub extern "C" fn stop_immediate_propagation(&self) {
        panic!(
            "cannot call wasm-bindgen imported functions on \
            non-wasm targets"
        );
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
pub extern "C" fn __wbindgen_describe___widl_f_bubbles_Event() {
    use wasm_bindgen::describe::*;
    inform(FUNCTION);
    inform(1u32);
    <&Event as WasmDescribe>::describe();
    inform(1);
    <bool as WasmDescribe>::describe();
}
impl Event {
    #[allow(bad_style)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    pub extern "C" fn bubbles(&self) -> bool {
        ::wasm_bindgen::__rt::link_this_library();
        #[wasm_import_module = "__wbindgen_placeholder__"]
        extern "C" {
            fn __widl_f_bubbles_Event(
                self_: <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as ::wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        unsafe {
            let _ret = {
                let mut __stack = ::wasm_bindgen::convert::GlobalStack::new();
                let self_ =
                    <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::into_abi(self, &mut __stack);
                __widl_f_bubbles_Event(self_)
            };
            <bool as ::wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
                &mut ::wasm_bindgen::convert::GlobalStack::new(),
            )
        }
    }
    #[allow(bad_style, unused_variables)]
    #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
    pub extern "C" fn bubbles(&self) -> bool {
        panic!(
            "cannot call wasm-bindgen imported functions on \
            non-wasm targets"
        );
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
pub extern "C" fn __wbindgen_describe___widl_f_cancelable_Event() {
    use wasm_bindgen::describe::*;
    inform(FUNCTION);
    inform(1u32);
    <&Event as WasmDescribe>::describe();
    inform(1);
    <bool as WasmDescribe>::describe();
}
impl Event {
    #[allow(bad_style)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    pub extern "C" fn cancelable(&self) -> bool {
        ::wasm_bindgen::__rt::link_this_library();
        #[wasm_import_module = "__wbindgen_placeholder__"]
        extern "C" {
            fn __widl_f_cancelable_Event(
                self_: <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as ::wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        unsafe {
            let _ret = {
                let mut __stack = ::wasm_bindgen::convert::GlobalStack::new();
                let self_ =
                    <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::into_abi(self, &mut __stack);
                __widl_f_cancelable_Event(self_)
            };
            <bool as ::wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
                &mut ::wasm_bindgen::convert::GlobalStack::new(),
            )
        }
    }
    #[allow(bad_style, unused_variables)]
    #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
    pub extern "C" fn cancelable(&self) -> bool {
        panic!(
            "cannot call wasm-bindgen imported functions on \
            non-wasm targets"
        );
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
pub extern "C" fn __wbindgen_describe___widl_f_prevent_default_Event() {
    use wasm_bindgen::describe::*;
    inform(FUNCTION);
    inform(1u32);
    <&Event as WasmDescribe>::describe();
    inform(0);
}
impl Event {
    #[allow(bad_style)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    pub extern "C" fn prevent_default(&self) {
        ::wasm_bindgen::__rt::link_this_library();
        #[wasm_import_module = "__wbindgen_placeholder__"]
        extern "C" {
            fn __widl_f_prevent_default_Event(
                self_: <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        unsafe {
            let _ret = {
                let mut __stack = ::wasm_bindgen::convert::GlobalStack::new();
                let self_ =
                    <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::into_abi(self, &mut __stack);
                __widl_f_prevent_default_Event(self_)
            };
            ()
        }
    }
    #[allow(bad_style, unused_variables)]
    #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
    pub extern "C" fn prevent_default(&self) {
        panic!(
            "cannot call wasm-bindgen imported functions on \
            non-wasm targets"
        );
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
pub extern "C" fn __wbindgen_describe___widl_f_default_prevented_Event() {
    use wasm_bindgen::describe::*;
    inform(FUNCTION);
    inform(1u32);
    <&Event as WasmDescribe>::describe();
    inform(1);
    <bool as WasmDescribe>::describe();
}
impl Event {
    #[allow(bad_style)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    pub extern "C" fn default_prevented(&self) -> bool {
        ::wasm_bindgen::__rt::link_this_library();
        #[wasm_import_module = "__wbindgen_placeholder__"]
        extern "C" {
            fn __widl_f_default_prevented_Event(
                self_: <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as ::wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        unsafe {
            let _ret = {
                let mut __stack = ::wasm_bindgen::convert::GlobalStack::new();
                let self_ =
                    <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::into_abi(self, &mut __stack);
                __widl_f_default_prevented_Event(self_)
            };
            <bool as ::wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
                &mut ::wasm_bindgen::convert::GlobalStack::new(),
            )
        }
    }
    #[allow(bad_style, unused_variables)]
    #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
    pub extern "C" fn default_prevented(&self) -> bool {
        panic!(
            "cannot call wasm-bindgen imported functions on \
            non-wasm targets"
        );
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
pub extern "C" fn __wbindgen_describe___widl_f_composed_Event() {
    use wasm_bindgen::describe::*;
    inform(FUNCTION);
    inform(1u32);
    <&Event as WasmDescribe>::describe();
    inform(1);
    <bool as WasmDescribe>::describe();
}
impl Event {
    #[allow(bad_style)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    pub extern "C" fn composed(&self) -> bool {
        ::wasm_bindgen::__rt::link_this_library();
        #[wasm_import_module = "__wbindgen_placeholder__"]
        extern "C" {
            fn __widl_f_composed_Event(
                self_: <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as ::wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        unsafe {
            let _ret = {
                let mut __stack = ::wasm_bindgen::convert::GlobalStack::new();
                let self_ =
                    <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::into_abi(self, &mut __stack);
                __widl_f_composed_Event(self_)
            };
            <bool as ::wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
                &mut ::wasm_bindgen::convert::GlobalStack::new(),
            )
        }
    }
    #[allow(bad_style, unused_variables)]
    #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
    pub extern "C" fn composed(&self) -> bool {
        panic!(
            "cannot call wasm-bindgen imported functions on \
            non-wasm targets"
        );
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
pub extern "C" fn __wbindgen_describe___widl_f_is_trusted_Event() {
    use wasm_bindgen::describe::*;
    inform(FUNCTION);
    inform(1u32);
    <&Event as WasmDescribe>::describe();
    inform(1);
    <bool as WasmDescribe>::describe();
}
impl Event {
    #[allow(bad_style)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    pub extern "C" fn is_trusted(&self) -> bool {
        ::wasm_bindgen::__rt::link_this_library();
        #[wasm_import_module = "__wbindgen_placeholder__"]
        extern "C" {
            fn __widl_f_is_trusted_Event(
                self_: <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as ::wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        unsafe {
            let _ret = {
                let mut __stack = ::wasm_bindgen::convert::GlobalStack::new();
                let self_ =
                    <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::into_abi(self, &mut __stack);
                __widl_f_is_trusted_Event(self_)
            };
            <bool as ::wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
                &mut ::wasm_bindgen::convert::GlobalStack::new(),
            )
        }
    }
    #[allow(bad_style, unused_variables)]
    #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
    pub extern "C" fn is_trusted(&self) -> bool {
        panic!(
            "cannot call wasm-bindgen imported functions on \
            non-wasm targets"
        );
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
pub extern "C" fn __wbindgen_describe___widl_f_time_stamp_Event() {
    use wasm_bindgen::describe::*;
    inform(FUNCTION);
    inform(1u32);
    <&Event as WasmDescribe>::describe();
    inform(1);
    <DOMHighResTimeStamp as WasmDescribe>::describe();
}
impl Event {
    #[allow(bad_style)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    pub extern "C" fn time_stamp(&self) -> DOMHighResTimeStamp {
        ::wasm_bindgen::__rt::link_this_library();
        #[wasm_import_module = "__wbindgen_placeholder__"]
        extern "C" {
            fn __widl_f_time_stamp_Event(
                self_: <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DOMHighResTimeStamp as ::wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        unsafe {
            let _ret = {
                let mut __stack = ::wasm_bindgen::convert::GlobalStack::new();
                let self_ =
                    <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::into_abi(self, &mut __stack);
                __widl_f_time_stamp_Event(self_)
            };
            <DOMHighResTimeStamp as ::wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
                &mut ::wasm_bindgen::convert::GlobalStack::new(),
            )
        }
    }
    #[allow(bad_style, unused_variables)]
    #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
    pub extern "C" fn time_stamp(&self) -> DOMHighResTimeStamp {
        panic!(
            "cannot call wasm-bindgen imported functions on \
            non-wasm targets"
        );
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
pub extern "C" fn __wbindgen_describe___widl_f_init_event_Event() {
    use wasm_bindgen::describe::*;
    inform(FUNCTION);
    inform(4u32);
    <&Event as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    inform(0);
}
impl Event {
    #[allow(bad_style)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    pub extern "C" fn init_event(&self, type_: &str, bubbles: bool, cancelable: bool) {
        ::wasm_bindgen::__rt::link_this_library();
        #[wasm_import_module = "__wbindgen_placeholder__"]
        extern "C" {
            fn __widl_f_init_event_Event(
                self_: <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as ::wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bubbles: <bool as ::wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as ::wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        unsafe {
            let _ret = {
                let mut __stack = ::wasm_bindgen::convert::GlobalStack::new();
                let self_ =
                    <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::into_abi(self, &mut __stack);
                let type_ =
                    <&str as ::wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_, &mut __stack);
                let bubbles =
                    <bool as ::wasm_bindgen::convert::IntoWasmAbi>::into_abi(bubbles, &mut __stack);
                let cancelable = <bool as ::wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                    cancelable,
                    &mut __stack,
                );
                __widl_f_init_event_Event(self_, type_, bubbles, cancelable)
            };
            ()
        }
    }
    #[allow(bad_style, unused_variables)]
    #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
    pub extern "C" fn init_event(&self, type_: &str, bubbles: bool, cancelable: bool) {
        panic!(
            "cannot call wasm-bindgen imported functions on \
            non-wasm targets"
        );
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
pub extern "C" fn __wbindgen_describe___widl_f_cancel_bubble_Event() {
    use wasm_bindgen::describe::*;
    inform(FUNCTION);
    inform(1u32);
    <&Event as WasmDescribe>::describe();
    inform(1);
    <bool as WasmDescribe>::describe();
}
impl Event {
    #[allow(bad_style)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    pub extern "C" fn cancel_bubble(&self) -> bool {
        ::wasm_bindgen::__rt::link_this_library();
        #[wasm_import_module = "__wbindgen_placeholder__"]
        extern "C" {
            fn __widl_f_cancel_bubble_Event(
                self_: <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as ::wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        unsafe {
            let _ret = {
                let mut __stack = ::wasm_bindgen::convert::GlobalStack::new();
                let self_ =
                    <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::into_abi(self, &mut __stack);
                __widl_f_cancel_bubble_Event(self_)
            };
            <bool as ::wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
                &mut ::wasm_bindgen::convert::GlobalStack::new(),
            )
        }
    }
    #[allow(bad_style, unused_variables)]
    #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
    pub extern "C" fn cancel_bubble(&self) -> bool {
        panic!(
            "cannot call wasm-bindgen imported functions on \
            non-wasm targets"
        );
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
pub extern "C" fn __wbindgen_describe___widl_f_set_cancel_bubble_Event() {
    use wasm_bindgen::describe::*;
    inform(FUNCTION);
    inform(2u32);
    <&Event as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    inform(0);
}
impl Event {
    #[allow(bad_style)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    pub extern "C" fn set_cancel_bubble(&self, cancel_bubble: bool) {
        ::wasm_bindgen::__rt::link_this_library();
        #[wasm_import_module = "__wbindgen_placeholder__"]
        extern "C" {
            fn __widl_f_set_cancel_bubble_Event(
                self_: <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancel_bubble: <bool as ::wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        unsafe {
            let _ret = {
                let mut __stack = ::wasm_bindgen::convert::GlobalStack::new();
                let self_ =
                    <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::into_abi(self, &mut __stack);
                let cancel_bubble = <bool as ::wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                    cancel_bubble,
                    &mut __stack,
                );
                __widl_f_set_cancel_bubble_Event(self_, cancel_bubble)
            };
            ()
        }
    }
    #[allow(bad_style, unused_variables)]
    #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
    pub extern "C" fn set_cancel_bubble(&self, cancel_bubble: bool) {
        panic!(
            "cannot call wasm-bindgen imported functions on \
            non-wasm targets"
        );
    }
}
#[allow(non_camel_case_types)]
pub type DOMHighResTimeStamp = f64;
#[allow(non_camel_case_types)]
pub type EventInit = bool;
#[allow(non_upper_case_globals)]
#[wasm_custom_section = "__wasm_bindgen_unstable"]
const __WASM_BINDGEN_GENERATED_wasm_bindgen_webidl_0_2_11_0 : [ u8 ; 4451usize ] = * b"_\x11\0\0{\"exports\":[],\"enums\":[],\"imports\":[{\"module\":null,\"version\":null,\"js_namespace\":null,\"kind\":{\"kind\":\"type\"}},{\"module\":null,\"version\":null,\"js_namespace\":null,\"kind\":{\"kind\":\"function\",\"shim\":\"__widl_f_new_Event\",\"catch\":false,\"method\":false,\"js_new\":true,\"structural\":false,\"getter\":null,\"setter\":null,\"class\":\"Event\",\"function\":{\"name\":\"new\"}}},{\"module\":null,\"version\":null,\"js_namespace\":null,\"kind\":{\"kind\":\"function\",\"shim\":\"__widl_f_event_phase_Event\",\"catch\":false,\"method\":true,\"js_new\":false,\"structural\":false,\"getter\":\"eventPhase\",\"setter\":null,\"class\":\"Event\",\"function\":{\"name\":\"eventPhase\"}}},{\"module\":null,\"version\":null,\"js_namespace\":null,\"kind\":{\"kind\":\"function\",\"shim\":\"__widl_f_stop_propagation_Event\",\"catch\":false,\"method\":true,\"js_new\":false,\"structural\":false,\"getter\":null,\"setter\":null,\"class\":\"Event\",\"function\":{\"name\":\"stopPropagation\"}}},{\"module\":null,\"version\":null,\"js_namespace\":null,\"kind\":{\"kind\":\"function\",\"shim\":\"__widl_f_stop_immediate_propagation_Event\",\"catch\":false,\"method\":true,\"js_new\":false,\"structural\":false,\"getter\":null,\"setter\":null,\"class\":\"Event\",\"function\":{\"name\":\"stopImmediatePropagation\"}}},{\"module\":null,\"version\":null,\"js_namespace\":null,\"kind\":{\"kind\":\"function\",\"shim\":\"__widl_f_bubbles_Event\",\"catch\":false,\"method\":true,\"js_new\":false,\"structural\":false,\"getter\":\"bubbles\",\"setter\":null,\"class\":\"Event\",\"function\":{\"name\":\"bubbles\"}}},{\"module\":null,\"version\":null,\"js_namespace\":null,\"kind\":{\"kind\":\"function\",\"shim\":\"__widl_f_cancelable_Event\",\"catch\":false,\"method\":true,\"js_new\":false,\"structural\":false,\"getter\":\"cancelable\",\"setter\":null,\"class\":\"Event\",\"function\":{\"name\":\"cancelable\"}}},{\"module\":null,\"version\":null,\"js_namespace\":null,\"kind\":{\"kind\":\"function\",\"shim\":\"__widl_f_prevent_default_Event\",\"catch\":false,\"method\":true,\"js_new\":false,\"structural\":false,\"getter\":null,\"setter\":null,\"class\":\"Event\",\"function\":{\"name\":\"preventDefault\"}}},{\"module\":null,\"version\":null,\"js_namespace\":null,\"kind\":{\"kind\":\"function\",\"shim\":\"__widl_f_default_prevented_Event\",\"catch\":false,\"method\":true,\"js_new\":false,\"structural\":false,\"getter\":\"defaultPrevented\",\"setter\":null,\"class\":\"Event\",\"function\":{\"name\":\"defaultPrevented\"}}},{\"module\":null,\"version\":null,\"js_namespace\":null,\"kind\":{\"kind\":\"function\",\"shim\":\"__widl_f_default_prevented_by_chrome_Event\",\"catch\":false,\"method\":true,\"js_new\":false,\"structural\":false,\"getter\":\"defaultPreventedByChrome\",\"setter\":null,\"class\":\"Event\",\"function\":{\"name\":\"defaultPreventedByChrome\"}}},{\"module\":null,\"version\":null,\"js_namespace\":null,\"kind\":{\"kind\":\"function\",\"shim\":\"__widl_f_default_prevented_by_content_Event\",\"catch\":false,\"method\":true,\"js_new\":false,\"structural\":false,\"getter\":\"defaultPreventedByContent\",\"setter\":null,\"class\":\"Event\",\"function\":{\"name\":\"defaultPreventedByContent\"}}},{\"module\":null,\"version\":null,\"js_namespace\":null,\"kind\":{\"kind\":\"function\",\"shim\":\"__widl_f_composed_Event\",\"catch\":false,\"method\":true,\"js_new\":false,\"structural\":false,\"getter\":\"composed\",\"setter\":null,\"class\":\"Event\",\"function\":{\"name\":\"composed\"}}},{\"module\":null,\"version\":null,\"js_namespace\":null,\"kind\":{\"kind\":\"function\",\"shim\":\"__widl_f_is_trusted_Event\",\"catch\":false,\"method\":true,\"js_new\":false,\"structural\":false,\"getter\":\"isTrusted\",\"setter\":null,\"class\":\"Event\",\"function\":{\"name\":\"isTrusted\"}}},{\"module\":null,\"version\":null,\"js_namespace\":null,\"kind\":{\"kind\":\"function\",\"shim\":\"__widl_f_time_stamp_Event\",\"catch\":false,\"method\":true,\"js_new\":false,\"structural\":false,\"getter\":\"timeStamp\",\"setter\":null,\"class\":\"Event\",\"function\":{\"name\":\"timeStamp\"}}},{\"module\":null,\"version\":null,\"js_namespace\":null,\"kind\":{\"kind\":\"function\",\"shim\":\"__widl_f_init_event_Event\",\"catch\":false,\"method\":true,\"js_new\":false,\"structural\":false,\"getter\":null,\"setter\":null,\"class\":\"Event\",\"function\":{\"name\":\"initEvent\"}}},{\"module\":null,\"version\":null,\"js_namespace\":null,\"kind\":{\"kind\":\"function\",\"shim\":\"__widl_f_cancel_bubble_Event\",\"catch\":false,\"method\":true,\"js_new\":false,\"structural\":false,\"getter\":\"cancelBubble\",\"setter\":null,\"class\":\"Event\",\"function\":{\"name\":\"cancelBubble\"}}},{\"module\":null,\"version\":null,\"js_namespace\":null,\"kind\":{\"kind\":\"function\",\"shim\":\"__widl_f_set_cancel_bubble_Event\",\"catch\":false,\"method\":true,\"js_new\":false,\"structural\":false,\"getter\":null,\"setter\":\"cancelBubble\",\"class\":\"Event\",\"function\":{\"name\":\"set_CancelBubble\"}}}],\"structs\":[],\"version\":\"0.2.11 (161fce9d5)\",\"schema_version\":\"4\"}" ;
