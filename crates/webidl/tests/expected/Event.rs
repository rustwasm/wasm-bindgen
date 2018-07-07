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
#[allow(non_upper_case_globals)]
#[wasm_custom_section = "__wasm_bindgen_unstable"]