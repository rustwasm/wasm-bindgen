use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLDocument` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlDocument {
    obj: Document,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlDocument: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlDocument {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(68u32);
            inform(111u32);
            inform(99u32);
            inform(117u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for HtmlDocument {
        type Target = Document;
        #[inline]
        fn deref(&self) -> &Document {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlDocument {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlDocument {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlDocument {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlDocument {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlDocument {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlDocument {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlDocument {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlDocument {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlDocument>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlDocument {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlDocument {
        #[inline]
        fn from(obj: JsValue) -> HtmlDocument {
            HtmlDocument { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlDocument {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlDocument> for HtmlDocument {
        #[inline]
        fn as_ref(&self) -> &HtmlDocument {
            self
        }
    }
    impl From<HtmlDocument> for JsValue {
        #[inline]
        fn from(obj: HtmlDocument) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlDocument {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLDocument(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLDocument(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLDocument(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlDocument { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlDocument) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlDocument> for Document {
    #[inline]
    fn from(obj: HtmlDocument) -> Document {
        use wasm_bindgen::JsCast;
        Document::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Document> for HtmlDocument {
    #[inline]
    fn as_ref(&self) -> &Document {
        use wasm_bindgen::JsCast;
        Document::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlDocument> for Node {
    #[inline]
    fn from(obj: HtmlDocument) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlDocument {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlDocument> for EventTarget {
    #[inline]
    fn from(obj: HtmlDocument) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlDocument {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlDocument> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlDocument) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlDocument {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_capture_events_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `captureEvents()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/captureEvents)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn capture_events(&self) {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_capture_events_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_capture_events_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_capture_events_HTMLDocument(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clear_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `clear()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/clear)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn clear(&self) {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_clear_HTMLDocument(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_close_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `close()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/close)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn close(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_close_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_close_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_close_HTMLDocument(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_exec_command_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `execCommand()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/execCommand)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn exec_command(&self, command_id: &str) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exec_command_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                command_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exec_command_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            command_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(command_id);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let command_id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(command_id);
                __widl_f_exec_command_HTMLDocument(self_, command_id)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_exec_command_with_show_ui_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `execCommand()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/execCommand)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn exec_command_with_show_ui(
        &self,
        command_id: &str,
        show_ui: bool,
    ) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exec_command_with_show_ui_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                command_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                show_ui: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exec_command_with_show_ui_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            command_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            show_ui: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(command_id);
            drop(show_ui);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let command_id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(command_id);
                let show_ui = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(show_ui);
                __widl_f_exec_command_with_show_ui_HTMLDocument(self_, command_id, show_ui)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_exec_command_with_show_ui_and_value_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `execCommand()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/execCommand)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn exec_command_with_show_ui_and_value(
        &self,
        command_id: &str,
        show_ui: bool,
        value: &str,
    ) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exec_command_with_show_ui_and_value_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                command_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                show_ui: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exec_command_with_show_ui_and_value_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            command_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            show_ui: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(command_id);
            drop(show_ui);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let command_id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(command_id);
                let show_ui = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(show_ui);
                let value = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_exec_command_with_show_ui_and_value_HTMLDocument(
                    self_, command_id, show_ui, value,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Document", feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_open_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <Document as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "Document", feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `open()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/open)\n\n*This API requires the following crate features to be activated: `Document`, `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn open(&self) -> Result<Document, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_open_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Document as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_open_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Document as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_open_HTMLDocument(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Document as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Document", feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_open_with_type_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Document as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "Document", feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `open()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/open)\n\n*This API requires the following crate features to be activated: `Document`, `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn open_with_type(&self, type_: &str) -> Result<Document, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_open_with_type_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Document as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_open_with_type_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Document as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_open_with_type_HTMLDocument(self_, type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Document as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Document", feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_open_with_type_and_replace_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Document as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "Document", feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `open()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/open)\n\n*This API requires the following crate features to be activated: `Document`, `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn open_with_type_and_replace(
        &self,
        type_: &str,
        replace: &str,
    ) -> Result<Document, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_open_with_type_and_replace_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                replace: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Document as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_open_with_type_and_replace_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            replace: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Document as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(type_);
            drop(replace);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let replace = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(replace);
                __widl_f_open_with_type_and_replace_HTMLDocument(self_, type_, replace)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Document as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "HtmlDocument", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_open_with_url_and_name_and_features_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<Window> as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `open()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/open)\n\n*This API requires the following crate features to be activated: `HtmlDocument`, `Window`*"]
    #[allow(clippy::all)]
    pub fn open_with_url_and_name_and_features(
        &self,
        url: &str,
        name: &str,
        features: &str,
    ) -> Result<Option<Window>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlDocument", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_open_with_url_and_name_and_features_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                features: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Window> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_open_with_url_and_name_and_features_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            features: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Window> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(url);
            drop(name);
            drop(features);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                let features = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(features);
                __widl_f_open_with_url_and_name_and_features_HTMLDocument(
                    self_, url, name, features,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Window> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlDocument", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_open_with_url_and_name_and_features_and_replace_HTMLDocument(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<Window> as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `open()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/open)\n\n*This API requires the following crate features to be activated: `HtmlDocument`, `Window`*"]
    #[allow(clippy::all)]
    pub fn open_with_url_and_name_and_features_and_replace(
        &self,
        url: &str,
        name: &str,
        features: &str,
        replace: bool,
    ) -> Result<Option<Window>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlDocument", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_open_with_url_and_name_and_features_and_replace_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                features: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                replace: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Window> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_open_with_url_and_name_and_features_and_replace_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            features: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            replace: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Window> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(url);
            drop(name);
            drop(features);
            drop(replace);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                let features = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(features);
                let replace = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(replace);
                __widl_f_open_with_url_and_name_and_features_and_replace_HTMLDocument(
                    self_, url, name, features, replace,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Window> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_query_command_enabled_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `queryCommandEnabled()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/queryCommandEnabled)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn query_command_enabled(&self, command_id: &str) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_query_command_enabled_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                command_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_query_command_enabled_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            command_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(command_id);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let command_id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(command_id);
                __widl_f_query_command_enabled_HTMLDocument(self_, command_id)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_query_command_indeterm_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `queryCommandIndeterm()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/queryCommandIndeterm)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn query_command_indeterm(
        &self,
        command_id: &str,
    ) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_query_command_indeterm_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                command_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_query_command_indeterm_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            command_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(command_id);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let command_id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(command_id);
                __widl_f_query_command_indeterm_HTMLDocument(self_, command_id)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_query_command_state_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `queryCommandState()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/queryCommandState)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn query_command_state(&self, command_id: &str) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_query_command_state_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                command_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_query_command_state_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            command_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(command_id);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let command_id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(command_id);
                __widl_f_query_command_state_HTMLDocument(self_, command_id)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_query_command_supported_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `queryCommandSupported()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/queryCommandSupported)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn query_command_supported(&self, command_id: &str) -> bool {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_query_command_supported_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                command_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_query_command_supported_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            command_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(command_id);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let command_id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(command_id);
                __widl_f_query_command_supported_HTMLDocument(self_, command_id)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_query_command_value_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `queryCommandValue()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/queryCommandValue)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn query_command_value(&self, command_id: &str) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_query_command_value_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                command_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_query_command_value_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            command_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(command_id);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let command_id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(command_id);
                __widl_f_query_command_value_HTMLDocument(self_, command_id)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_release_events_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `releaseEvents()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/releaseEvents)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn release_events(&self) {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_release_events_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_release_events_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_release_events_HTMLDocument(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_write_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `write()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn write(&self, text: &::js_sys::Array) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_write_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_write_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let text = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text);
                __widl_f_write_HTMLDocument(self_, text)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_write_0_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `write()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn write_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_write_0_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_write_0_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_write_0_HTMLDocument(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_write_1_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `write()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn write_1(&self, text_1: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_write_1_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_write_1_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let text_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_1);
                __widl_f_write_1_HTMLDocument(self_, text_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_write_2_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `write()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn write_2(&self, text_1: &str, text_2: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_write_2_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_write_2_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text_1);
            drop(text_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let text_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_1);
                let text_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_2);
                __widl_f_write_2_HTMLDocument(self_, text_1, text_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_write_3_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `write()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn write_3(
        &self,
        text_1: &str,
        text_2: &str,
        text_3: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_write_3_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_write_3_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text_1);
            drop(text_2);
            drop(text_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let text_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_1);
                let text_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_2);
                let text_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_3);
                __widl_f_write_3_HTMLDocument(self_, text_1, text_2, text_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_write_4_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `write()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn write_4(
        &self,
        text_1: &str,
        text_2: &str,
        text_3: &str,
        text_4: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_write_4_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_write_4_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text_1);
            drop(text_2);
            drop(text_3);
            drop(text_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let text_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_1);
                let text_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_2);
                let text_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_3);
                let text_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_4);
                __widl_f_write_4_HTMLDocument(self_, text_1, text_2, text_3, text_4)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_write_5_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `write()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn write_5(
        &self,
        text_1: &str,
        text_2: &str,
        text_3: &str,
        text_4: &str,
        text_5: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_write_5_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_write_5_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text_1);
            drop(text_2);
            drop(text_3);
            drop(text_4);
            drop(text_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let text_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_1);
                let text_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_2);
                let text_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_3);
                let text_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_4);
                let text_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_5);
                __widl_f_write_5_HTMLDocument(self_, text_1, text_2, text_3, text_4, text_5)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_write_6_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `write()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn write_6(
        &self,
        text_1: &str,
        text_2: &str,
        text_3: &str,
        text_4: &str,
        text_5: &str,
        text_6: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_write_6_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_write_6_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text_1);
            drop(text_2);
            drop(text_3);
            drop(text_4);
            drop(text_5);
            drop(text_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let text_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_1);
                let text_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_2);
                let text_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_3);
                let text_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_4);
                let text_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_5);
                let text_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_6);
                __widl_f_write_6_HTMLDocument(self_, text_1, text_2, text_3, text_4, text_5, text_6)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_write_7_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `write()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn write_7(
        &self,
        text_1: &str,
        text_2: &str,
        text_3: &str,
        text_4: &str,
        text_5: &str,
        text_6: &str,
        text_7: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_write_7_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_7: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_write_7_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_7: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text_1);
            drop(text_2);
            drop(text_3);
            drop(text_4);
            drop(text_5);
            drop(text_6);
            drop(text_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let text_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_1);
                let text_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_2);
                let text_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_3);
                let text_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_4);
                let text_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_5);
                let text_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_6);
                let text_7 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_7);
                __widl_f_write_7_HTMLDocument(
                    self_, text_1, text_2, text_3, text_4, text_5, text_6, text_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_writeln_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `writeln()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn writeln(&self, text: &::js_sys::Array) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_writeln_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_writeln_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let text = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text);
                __widl_f_writeln_HTMLDocument(self_, text)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_writeln_0_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `writeln()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn writeln_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_writeln_0_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_writeln_0_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_writeln_0_HTMLDocument(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_writeln_1_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `writeln()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn writeln_1(&self, text_1: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_writeln_1_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_writeln_1_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let text_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_1);
                __widl_f_writeln_1_HTMLDocument(self_, text_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_writeln_2_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `writeln()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn writeln_2(&self, text_1: &str, text_2: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_writeln_2_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_writeln_2_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text_1);
            drop(text_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let text_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_1);
                let text_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_2);
                __widl_f_writeln_2_HTMLDocument(self_, text_1, text_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_writeln_3_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `writeln()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn writeln_3(
        &self,
        text_1: &str,
        text_2: &str,
        text_3: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_writeln_3_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_writeln_3_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text_1);
            drop(text_2);
            drop(text_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let text_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_1);
                let text_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_2);
                let text_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_3);
                __widl_f_writeln_3_HTMLDocument(self_, text_1, text_2, text_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_writeln_4_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `writeln()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn writeln_4(
        &self,
        text_1: &str,
        text_2: &str,
        text_3: &str,
        text_4: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_writeln_4_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_writeln_4_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text_1);
            drop(text_2);
            drop(text_3);
            drop(text_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let text_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_1);
                let text_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_2);
                let text_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_3);
                let text_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_4);
                __widl_f_writeln_4_HTMLDocument(self_, text_1, text_2, text_3, text_4)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_writeln_5_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `writeln()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn writeln_5(
        &self,
        text_1: &str,
        text_2: &str,
        text_3: &str,
        text_4: &str,
        text_5: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_writeln_5_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_writeln_5_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text_1);
            drop(text_2);
            drop(text_3);
            drop(text_4);
            drop(text_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let text_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_1);
                let text_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_2);
                let text_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_3);
                let text_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_4);
                let text_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_5);
                __widl_f_writeln_5_HTMLDocument(self_, text_1, text_2, text_3, text_4, text_5)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_writeln_6_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `writeln()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn writeln_6(
        &self,
        text_1: &str,
        text_2: &str,
        text_3: &str,
        text_4: &str,
        text_5: &str,
        text_6: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_writeln_6_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_writeln_6_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text_1);
            drop(text_2);
            drop(text_3);
            drop(text_4);
            drop(text_5);
            drop(text_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let text_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_1);
                let text_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_2);
                let text_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_3);
                let text_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_4);
                let text_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_5);
                let text_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_6);
                __widl_f_writeln_6_HTMLDocument(
                    self_, text_1, text_2, text_3, text_4, text_5, text_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_writeln_7_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `writeln()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn writeln_7(
        &self,
        text_1: &str,
        text_2: &str,
        text_3: &str,
        text_4: &str,
        text_5: &str,
        text_6: &str,
        text_7: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_writeln_7_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_7: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_writeln_7_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_7: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text_1);
            drop(text_2);
            drop(text_3);
            drop(text_4);
            drop(text_5);
            drop(text_6);
            drop(text_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let text_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_1);
                let text_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_2);
                let text_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_3);
                let text_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_4);
                let text_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_5);
                let text_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_6);
                let text_7 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_7);
                __widl_f_writeln_7_HTMLDocument(
                    self_, text_1, text_2, text_3, text_4, text_5, text_6, text_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <::js_sys::Object as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The indexing getter\n\n\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn get(&self, name: &str) -> Result<::js_sys::Object, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_get_HTMLDocument(self_, name)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_domain_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `domain` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/domain)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn domain(&self) -> String {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_domain_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_domain_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_domain_HTMLDocument(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_domain_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `domain` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/domain)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn set_domain(&self, domain: &str) {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_domain_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                domain: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_domain_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            domain: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(domain);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let domain = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(domain);
                __widl_f_set_domain_HTMLDocument(self_, domain)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_cookie_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `cookie` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/cookie)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn cookie(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_cookie_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_cookie_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_cookie_HTMLDocument(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_cookie_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `cookie` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/cookie)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn set_cookie(&self, cookie: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_cookie_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cookie: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_cookie_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cookie: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(cookie);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let cookie = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cookie);
                __widl_f_set_cookie_HTMLDocument(self_, cookie)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_design_mode_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `designMode` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/designMode)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn design_mode(&self) -> String {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_design_mode_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_design_mode_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_design_mode_HTMLDocument(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_design_mode_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `designMode` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/designMode)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn set_design_mode(&self, design_mode: &str) {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_design_mode_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                design_mode: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_design_mode_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            design_mode: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(design_mode);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let design_mode =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(design_mode);
                __widl_f_set_design_mode_HTMLDocument(self_, design_mode)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_fg_color_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `fgColor` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/fgColor)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn fg_color(&self) -> String {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_fg_color_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_fg_color_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_fg_color_HTMLDocument(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_fg_color_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `fgColor` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/fgColor)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn set_fg_color(&self, fg_color: &str) {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_fg_color_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                fg_color: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_fg_color_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            fg_color: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(fg_color);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let fg_color = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(fg_color);
                __widl_f_set_fg_color_HTMLDocument(self_, fg_color)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_link_color_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `linkColor` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/linkColor)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn link_color(&self) -> String {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_link_color_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_link_color_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_link_color_HTMLDocument(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_link_color_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `linkColor` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/linkColor)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn set_link_color(&self, link_color: &str) {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_link_color_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                link_color: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_link_color_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            link_color: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(link_color);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let link_color = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(link_color);
                __widl_f_set_link_color_HTMLDocument(self_, link_color)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_vlink_color_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `vlinkColor` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/vlinkColor)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn vlink_color(&self) -> String {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_vlink_color_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_vlink_color_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_vlink_color_HTMLDocument(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_vlink_color_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `vlinkColor` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/vlinkColor)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn set_vlink_color(&self, vlink_color: &str) {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_vlink_color_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                vlink_color: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_vlink_color_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            vlink_color: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(vlink_color);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let vlink_color =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(vlink_color);
                __widl_f_set_vlink_color_HTMLDocument(self_, vlink_color)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_alink_color_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `alinkColor` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/alinkColor)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn alink_color(&self) -> String {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_alink_color_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_alink_color_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_alink_color_HTMLDocument(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_alink_color_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `alinkColor` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/alinkColor)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn set_alink_color(&self, alink_color: &str) {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_alink_color_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alink_color: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_alink_color_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alink_color: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(alink_color);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let alink_color =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alink_color);
                __widl_f_set_alink_color_HTMLDocument(self_, alink_color)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_bg_color_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `bgColor` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/bgColor)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn bg_color(&self) -> String {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_bg_color_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_bg_color_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_bg_color_HTMLDocument(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_bg_color_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `bgColor` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/bgColor)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn set_bg_color(&self, bg_color: &str) {
        #[cfg(all(feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_bg_color_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bg_color: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_bg_color_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bg_color: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(bg_color);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let bg_color = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bg_color);
                __widl_f_set_bg_color_HTMLDocument(self_, bg_color)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAllCollection", feature = "HtmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_all_HTMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlDocument as WasmDescribe>::describe();
    <HtmlAllCollection as WasmDescribe>::describe();
}
impl HtmlDocument {
    #[cfg(all(feature = "HtmlAllCollection", feature = "HtmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `all` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/all)\n\n*This API requires the following crate features to be activated: `HtmlAllCollection`, `HtmlDocument`*"]
    #[allow(clippy::all)]
    pub fn all(&self) -> HtmlAllCollection {
        #[cfg(all(feature = "HtmlAllCollection", feature = "HtmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_all_HTMLDocument(
                self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlAllCollection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_all_HTMLDocument(
            self_: <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HtmlAllCollection as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_all_HTMLDocument(self_)
            };
            <HtmlAllCollection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_00f55651b27710f3: [u8; 4965usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}#\x13\0\0\0\06\0\0\x02\x0CHTMLDocument\x1E__widl_instanceof_HTMLDocument\0\0\0\0$__widl_f_capture_events_HTMLDocument\0\0\0\x01\x0CHTMLDocument\x01\0\0\x01\x01\x05self_\rcaptureEvents\0\0\0\x1B__widl_f_clear_HTMLDocument\0\0\0\x01\x0CHTMLDocument\x01\0\0\x01\x01\x05self_\x05clear\0\0\0\x1B__widl_f_close_HTMLDocument\x01\0\0\x01\x0CHTMLDocument\x01\0\0\x01\x01\x05self_\x05close\0\0\0\"__widl_f_exec_command_HTMLDocument\x01\0\0\x01\x0CHTMLDocument\x01\0\0\x01\x02\x05self_\ncommand_id\x0BexecCommand\0\0\0/__widl_f_exec_command_with_show_ui_HTMLDocument\x01\0\0\x01\x0CHTMLDocument\x01\0\0\x01\x03\x05self_\ncommand_id\x07show_ui\x0BexecCommand\0\0\09__widl_f_exec_command_with_show_ui_and_value_HTMLDocument\x01\0\0\x01\x0CHTMLDocument\x01\0\0\x01\x04\x05self_\ncommand_id\x07show_ui\x05value\x0BexecCommand\0\0\0\x1A__widl_f_open_HTMLDocument\x01\0\0\x01\x0CHTMLDocument\x01\0\0\x01\x01\x05self_\x04open\0\0\0$__widl_f_open_with_type_HTMLDocument\x01\0\0\x01\x0CHTMLDocument\x01\0\0\x01\x02\x05self_\x05type_\x04open\0\0\00__widl_f_open_with_type_and_replace_HTMLDocument\x01\0\0\x01\x0CHTMLDocument\x01\0\0\x01\x03\x05self_\x05type_\x07replace\x04open\0\0\09__widl_f_open_with_url_and_name_and_features_HTMLDocument\x01\0\0\x01\x0CHTMLDocument\x01\0\0\x01\x04\x05self_\x03url\x04name\x08features\x04open\0\0\0E__widl_f_open_with_url_and_name_and_features_and_replace_HTMLDocument\x01\0\0\x01\x0CHTMLDocument\x01\0\0\x01\x05\x05self_\x03url\x04name\x08features\x07replace\x04open\0\0\0+__widl_f_query_command_enabled_HTMLDocument\x01\0\0\x01\x0CHTMLDocument\x01\0\0\x01\x02\x05self_\ncommand_id\x13queryCommandEnabled\0\0\0,__widl_f_query_command_indeterm_HTMLDocument\x01\0\0\x01\x0CHTMLDocument\x01\0\0\x01\x02\x05self_\ncommand_id\x14queryCommandIndeterm\0\0\0)__widl_f_query_command_state_HTMLDocument\x01\0\0\x01\x0CHTMLDocument\x01\0\0\x01\x02\x05self_\ncommand_id\x11queryCommandState\0\0\0-__widl_f_query_command_supported_HTMLDocument\0\0\0\x01\x0CHTMLDocument\x01\0\0\x01\x02\x05self_\ncommand_id\x15queryCommandSupported\0\0\0)__widl_f_query_command_value_HTMLDocument\x01\0\0\x01\x0CHTMLDocument\x01\0\0\x01\x02\x05self_\ncommand_id\x11queryCommandValue\0\0\0$__widl_f_release_events_HTMLDocument\0\0\0\x01\x0CHTMLDocument\x01\0\0\x01\x01\x05self_\rreleaseEvents\0\0\0\x1B__widl_f_write_HTMLDocument\x01\x01\0\x01\x0CHTMLDocument\x01\0\0\x01\x02\x05self_\x04text\x05write\0\0\0\x1D__widl_f_write_0_HTMLDocument\x01\0\0\x01\x0CHTMLDocument\x01\0\0\x01\x01\x05self_\x05write\0\0\0\x1D__widl_f_write_1_HTMLDocument\x01\0\0\x01\x0CHTMLDocument\x01\0\0\x01\x02\x05self_\x06text_1\x05write\0\0\0\x1D__widl_f_write_2_HTMLDocument\x01\0\0\x01\x0CHTMLDocument\x01\0\0\x01\x03\x05self_\x06text_1\x06text_2\x05write\0\0\0\x1D__widl_f_write_3_HTMLDocument\x01\0\0\x01\x0CHTMLDocument\x01\0\0\x01\x04\x05self_\x06text_1\x06text_2\x06text_3\x05write\0\0\0\x1D__widl_f_write_4_HTMLDocument\x01\0\0\x01\x0CHTMLDocument\x01\0\0\x01\x05\x05self_\x06text_1\x06text_2\x06text_3\x06text_4\x05write\0\0\0\x1D__widl_f_write_5_HTMLDocument\x01\0\0\x01\x0CHTMLDocument\x01\0\0\x01\x06\x05self_\x06text_1\x06text_2\x06text_3\x06text_4\x06text_5\x05write\0\0\0\x1D__widl_f_write_6_HTMLDocument\x01\0\0\x01\x0CHTMLDocument\x01\0\0\x01\x07\x05self_\x06text_1\x06text_2\x06text_3\x06text_4\x06text_5\x06text_6\x05write\0\0\0\x1D__widl_f_write_7_HTMLDocument\x01\0\0\x01\x0CHTMLDocument\x01\0\0\x01\x08\x05self_\x06text_1\x06text_2\x06text_3\x06text_4\x06text_5\x06text_6\x06text_7\x05write\0\0\0\x1D__widl_f_writeln_HTMLDocument\x01\x01\0\x01\x0CHTMLDocument\x01\0\0\x01\x02\x05self_\x04text\x07writeln\0\0\0\x1F__widl_f_writeln_0_HTMLDocument\x01\0\0\x01\x0CHTMLDocument\x01\0\0\x01\x01\x05self_\x07writeln\0\0\0\x1F__widl_f_writeln_1_HTMLDocument\x01\0\0\x01\x0CHTMLDocument\x01\0\0\x01\x02\x05self_\x06text_1\x07writeln\0\0\0\x1F__widl_f_writeln_2_HTMLDocument\x01\0\0\x01\x0CHTMLDocument\x01\0\0\x01\x03\x05self_\x06text_1\x06text_2\x07writeln\0\0\0\x1F__widl_f_writeln_3_HTMLDocument\x01\0\0\x01\x0CHTMLDocument\x01\0\0\x01\x04\x05self_\x06text_1\x06text_2\x06text_3\x07writeln\0\0\0\x1F__widl_f_writeln_4_HTMLDocument\x01\0\0\x01\x0CHTMLDocument\x01\0\0\x01\x05\x05self_\x06text_1\x06text_2\x06text_3\x06text_4\x07writeln\0\0\0\x1F__widl_f_writeln_5_HTMLDocument\x01\0\0\x01\x0CHTMLDocument\x01\0\0\x01\x06\x05self_\x06text_1\x06text_2\x06text_3\x06text_4\x06text_5\x07writeln\0\0\0\x1F__widl_f_writeln_6_HTMLDocument\x01\0\0\x01\x0CHTMLDocument\x01\0\0\x01\x07\x05self_\x06text_1\x06text_2\x06text_3\x06text_4\x06text_5\x06text_6\x07writeln\0\0\0\x1F__widl_f_writeln_7_HTMLDocument\x01\0\0\x01\x0CHTMLDocument\x01\0\0\x01\x08\x05self_\x06text_1\x06text_2\x06text_3\x06text_4\x06text_5\x06text_6\x06text_7\x07writeln\0\0\0\x19__widl_f_get_HTMLDocument\x01\0\0\x01\x0CHTMLDocument\x01\0\x03\x01\x02\x05self_\x04name\x03get\0\0\0\x1C__widl_f_domain_HTMLDocument\0\0\0\x01\x0CHTMLDocument\x01\0\x01\x06domain\x01\x01\x05self_\x06domain\0\0\0 __widl_f_set_domain_HTMLDocument\0\0\0\x01\x0CHTMLDocument\x01\0\x02\x06domain\x01\x02\x05self_\x06domain\x06domain\0\0\0\x1C__widl_f_cookie_HTMLDocument\x01\0\0\x01\x0CHTMLDocument\x01\0\x01\x06cookie\x01\x01\x05self_\x06cookie\0\0\0 __widl_f_set_cookie_HTMLDocument\x01\0\0\x01\x0CHTMLDocument\x01\0\x02\x06cookie\x01\x02\x05self_\x06cookie\x06cookie\0\0\0!__widl_f_design_mode_HTMLDocument\0\0\0\x01\x0CHTMLDocument\x01\0\x01\ndesignMode\x01\x01\x05self_\ndesignMode\0\0\0%__widl_f_set_design_mode_HTMLDocument\0\0\0\x01\x0CHTMLDocument\x01\0\x02\ndesignMode\x01\x02\x05self_\x0Bdesign_mode\ndesignMode\0\0\0\x1E__widl_f_fg_color_HTMLDocument\0\0\0\x01\x0CHTMLDocument\x01\0\x01\x07fgColor\x01\x01\x05self_\x07fgColor\0\0\0\"__widl_f_set_fg_color_HTMLDocument\0\0\0\x01\x0CHTMLDocument\x01\0\x02\x07fgColor\x01\x02\x05self_\x08fg_color\x07fgColor\0\0\0 __widl_f_link_color_HTMLDocument\0\0\0\x01\x0CHTMLDocument\x01\0\x01\tlinkColor\x01\x01\x05self_\tlinkColor\0\0\0$__widl_f_set_link_color_HTMLDocument\0\0\0\x01\x0CHTMLDocument\x01\0\x02\tlinkColor\x01\x02\x05self_\nlink_color\tlinkColor\0\0\0!__widl_f_vlink_color_HTMLDocument\0\0\0\x01\x0CHTMLDocument\x01\0\x01\nvlinkColor\x01\x01\x05self_\nvlinkColor\0\0\0%__widl_f_set_vlink_color_HTMLDocument\0\0\0\x01\x0CHTMLDocument\x01\0\x02\nvlinkColor\x01\x02\x05self_\x0Bvlink_color\nvlinkColor\0\0\0!__widl_f_alink_color_HTMLDocument\0\0\0\x01\x0CHTMLDocument\x01\0\x01\nalinkColor\x01\x01\x05self_\nalinkColor\0\0\0%__widl_f_set_alink_color_HTMLDocument\0\0\0\x01\x0CHTMLDocument\x01\0\x02\nalinkColor\x01\x02\x05self_\x0Balink_color\nalinkColor\0\0\0\x1E__widl_f_bg_color_HTMLDocument\0\0\0\x01\x0CHTMLDocument\x01\0\x01\x07bgColor\x01\x01\x05self_\x07bgColor\0\0\0\"__widl_f_set_bg_color_HTMLDocument\0\0\0\x01\x0CHTMLDocument\x01\0\x02\x07bgColor\x01\x02\x05self_\x08bg_color\x07bgColor\0\0\0\x19__widl_f_all_HTMLDocument\0\0\0\x01\x0CHTMLDocument\x01\0\x01\x03all\x01\x01\x05self_\x03all\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
