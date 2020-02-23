use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLMediaElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlMediaElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlMediaElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlMediaElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(77u32);
            inform(101u32);
            inform(100u32);
            inform(105u32);
            inform(97u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for HtmlMediaElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlMediaElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlMediaElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlMediaElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlMediaElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlMediaElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlMediaElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlMediaElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlMediaElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlMediaElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlMediaElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlMediaElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlMediaElement {
            HtmlMediaElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlMediaElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlMediaElement> for HtmlMediaElement {
        #[inline]
        fn as_ref(&self) -> &HtmlMediaElement {
            self
        }
    }
    impl From<HtmlMediaElement> for JsValue {
        #[inline]
        fn from(obj: HtmlMediaElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlMediaElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLMediaElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLMediaElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLMediaElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlMediaElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlMediaElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlMediaElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlMediaElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlMediaElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlMediaElement> for Element {
    #[inline]
    fn from(obj: HtmlMediaElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlMediaElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlMediaElement> for Node {
    #[inline]
    fn from(obj: HtmlMediaElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlMediaElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlMediaElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlMediaElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlMediaElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlMediaElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlMediaElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlMediaElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(
    feature = "HtmlMediaElement",
    feature = "TextTrack",
    feature = "TextTrackKind",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_text_track_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <TextTrackKind as WasmDescribe>::describe();
    <TextTrack as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(
        feature = "HtmlMediaElement",
        feature = "TextTrack",
        feature = "TextTrackKind",
    ))]
    #[allow(bad_style)]
    #[doc = "The `addTextTrack()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/addTextTrack)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`, `TextTrack`, `TextTrackKind`*"]
    #[allow(clippy::all)]
    pub fn add_text_track(&self, kind: TextTrackKind) -> TextTrack {
        #[cfg(all(
            feature = "HtmlMediaElement",
            feature = "TextTrack",
            feature = "TextTrackKind",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_text_track_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                kind: <TextTrackKind as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TextTrack as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_text_track_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            kind: <TextTrackKind as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TextTrack as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(kind);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let kind = <TextTrackKind as wasm_bindgen::convert::IntoWasmAbi>::into_abi(kind);
                __widl_f_add_text_track_HTMLMediaElement(self_, kind)
            };
            <TextTrack as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "HtmlMediaElement",
    feature = "TextTrack",
    feature = "TextTrackKind",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_text_track_with_label_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <TextTrackKind as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <TextTrack as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(
        feature = "HtmlMediaElement",
        feature = "TextTrack",
        feature = "TextTrackKind",
    ))]
    #[allow(bad_style)]
    #[doc = "The `addTextTrack()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/addTextTrack)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`, `TextTrack`, `TextTrackKind`*"]
    #[allow(clippy::all)]
    pub fn add_text_track_with_label(&self, kind: TextTrackKind, label: &str) -> TextTrack {
        #[cfg(all(
            feature = "HtmlMediaElement",
            feature = "TextTrack",
            feature = "TextTrackKind",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_text_track_with_label_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                kind: <TextTrackKind as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TextTrack as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_text_track_with_label_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            kind: <TextTrackKind as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TextTrack as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(kind);
            drop(label);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let kind = <TextTrackKind as wasm_bindgen::convert::IntoWasmAbi>::into_abi(kind);
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                __widl_f_add_text_track_with_label_HTMLMediaElement(self_, kind, label)
            };
            <TextTrack as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "HtmlMediaElement",
    feature = "TextTrack",
    feature = "TextTrackKind",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_text_track_with_label_and_language_HTMLMediaElement(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <TextTrackKind as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <TextTrack as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(
        feature = "HtmlMediaElement",
        feature = "TextTrack",
        feature = "TextTrackKind",
    ))]
    #[allow(bad_style)]
    #[doc = "The `addTextTrack()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/addTextTrack)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`, `TextTrack`, `TextTrackKind`*"]
    #[allow(clippy::all)]
    pub fn add_text_track_with_label_and_language(
        &self,
        kind: TextTrackKind,
        label: &str,
        language: &str,
    ) -> TextTrack {
        #[cfg(all(
            feature = "HtmlMediaElement",
            feature = "TextTrack",
            feature = "TextTrackKind",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_text_track_with_label_and_language_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                kind: <TextTrackKind as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                language: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TextTrack as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_text_track_with_label_and_language_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            kind: <TextTrackKind as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            language: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TextTrack as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(kind);
            drop(label);
            drop(language);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let kind = <TextTrackKind as wasm_bindgen::convert::IntoWasmAbi>::into_abi(kind);
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                let language = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(language);
                __widl_f_add_text_track_with_label_and_language_HTMLMediaElement(
                    self_, kind, label, language,
                )
            };
            <TextTrack as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_can_play_type_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `canPlayType()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/canPlayType)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn can_play_type(&self, type_: &str) -> String {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_can_play_type_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_can_play_type_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_can_play_type_HTMLMediaElement(self_, type_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_fast_seek_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `fastSeek()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/fastSeek)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn fast_seek(&self, time: f64) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_fast_seek_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                time: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_fast_seek_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            time: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(time);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let time = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(time);
                __widl_f_fast_seek_HTMLMediaElement(self_, time)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_has_suspend_taint_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `hasSuspendTaint()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/hasSuspendTaint)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn has_suspend_taint(&self) -> bool {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_has_suspend_taint_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_has_suspend_taint_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_has_suspend_taint_HTMLMediaElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_load_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `load()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/load)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn load(&self) {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_load_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_load_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_load_HTMLMediaElement(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pause_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `pause()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/pause)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn pause(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pause_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pause_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_pause_HTMLMediaElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_play_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `play()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/play)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn play(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_play_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_play_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_play_HTMLMediaElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_seek_to_next_frame_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `seekToNextFrame()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/seekToNextFrame)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn seek_to_next_frame(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_seek_to_next_frame_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_seek_to_next_frame_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_seek_to_next_frame_HTMLMediaElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement", feature = "MediaKeys",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_media_keys_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <Option<&MediaKeys> as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement", feature = "MediaKeys",))]
    #[allow(bad_style)]
    #[doc = "The `setMediaKeys()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/setMediaKeys)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaKeys`*"]
    #[allow(clippy::all)]
    pub fn set_media_keys(&self, media_keys: Option<&MediaKeys>) -> ::js_sys::Promise {
        #[cfg(all(feature = "HtmlMediaElement", feature = "MediaKeys",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_media_keys_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                media_keys: <Option<&MediaKeys> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_media_keys_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            media_keys: <Option<&MediaKeys> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(media_keys);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let media_keys =
                    <Option<&MediaKeys> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        media_keys,
                    );
                __widl_f_set_media_keys_HTMLMediaElement(self_, media_keys)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_visible_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `setVisible()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/setVisible)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn set_visible(&self, a_visible: bool) {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_visible_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_visible: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_visible_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_visible: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(a_visible);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_visible = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_visible);
                __widl_f_set_visible_HTMLMediaElement(self_, a_visible)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement", feature = "MediaError",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_error_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <Option<MediaError> as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement", feature = "MediaError",))]
    #[allow(bad_style)]
    #[doc = "The `error` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/error)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaError`*"]
    #[allow(clippy::all)]
    pub fn error(&self) -> Option<MediaError> {
        #[cfg(all(feature = "HtmlMediaElement", feature = "MediaError",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<MediaError> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<MediaError> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_error_HTMLMediaElement(self_)
            };
            <Option<MediaError> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_src_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `src` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/src)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn src(&self) -> String {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_src_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_src_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_src_HTMLMediaElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_src_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `src` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/src)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn set_src(&self, src: &str) {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_src_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                src: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_src_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            src: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(src);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let src = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(src);
                __widl_f_set_src_HTMLMediaElement(self_, src)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_current_src_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `currentSrc` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/currentSrc)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn current_src(&self) -> String {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_current_src_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_current_src_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_current_src_HTMLMediaElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement", feature = "MediaStream",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_src_object_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <Option<MediaStream> as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement", feature = "MediaStream",))]
    #[allow(bad_style)]
    #[doc = "The `srcObject` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/srcObject)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaStream`*"]
    #[allow(clippy::all)]
    pub fn src_object(&self) -> Option<MediaStream> {
        #[cfg(all(feature = "HtmlMediaElement", feature = "MediaStream",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_src_object_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<MediaStream> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_src_object_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<MediaStream> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_src_object_HTMLMediaElement(self_)
            };
            <Option<MediaStream> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement", feature = "MediaStream",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_src_object_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <Option<&MediaStream> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement", feature = "MediaStream",))]
    #[allow(bad_style)]
    #[doc = "The `srcObject` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/srcObject)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaStream`*"]
    #[allow(clippy::all)]
    pub fn set_src_object(&self, src_object: Option<&MediaStream>) {
        #[cfg(all(feature = "HtmlMediaElement", feature = "MediaStream",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_src_object_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                src_object: <Option<&MediaStream> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_src_object_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            src_object: <Option<&MediaStream> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(src_object);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let src_object =
                    <Option<&MediaStream> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        src_object,
                    );
                __widl_f_set_src_object_HTMLMediaElement(self_, src_object)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_cross_origin_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `crossOrigin` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/crossOrigin)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn cross_origin(&self) -> Option<String> {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_cross_origin_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_cross_origin_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_cross_origin_HTMLMediaElement(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_cross_origin_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `crossOrigin` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/crossOrigin)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn set_cross_origin(&self, cross_origin: Option<&str>) {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_cross_origin_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cross_origin: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_cross_origin_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cross_origin: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(cross_origin);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let cross_origin =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cross_origin);
                __widl_f_set_cross_origin_HTMLMediaElement(self_, cross_origin)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_network_state_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `networkState` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/networkState)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn network_state(&self) -> u16 {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_network_state_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_network_state_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_network_state_HTMLMediaElement(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_preload_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `preload` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/preload)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn preload(&self) -> String {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_preload_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_preload_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_preload_HTMLMediaElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_preload_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `preload` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/preload)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn set_preload(&self, preload: &str) {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_preload_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                preload: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_preload_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            preload: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(preload);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let preload = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(preload);
                __widl_f_set_preload_HTMLMediaElement(self_, preload)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement", feature = "TimeRanges",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_buffered_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <TimeRanges as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement", feature = "TimeRanges",))]
    #[allow(bad_style)]
    #[doc = "The `buffered` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/buffered)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`, `TimeRanges`*"]
    #[allow(clippy::all)]
    pub fn buffered(&self) -> TimeRanges {
        #[cfg(all(feature = "HtmlMediaElement", feature = "TimeRanges",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_buffered_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TimeRanges as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_buffered_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TimeRanges as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_buffered_HTMLMediaElement(self_)
            };
            <TimeRanges as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ready_state_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `readyState` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/readyState)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn ready_state(&self) -> u16 {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ready_state_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ready_state_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ready_state_HTMLMediaElement(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_seeking_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `seeking` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/seeking)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn seeking(&self) -> bool {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_seeking_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_seeking_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_seeking_HTMLMediaElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_current_time_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `currentTime` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/currentTime)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn current_time(&self) -> f64 {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_current_time_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_current_time_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_current_time_HTMLMediaElement(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_current_time_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `currentTime` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/currentTime)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn set_current_time(&self, current_time: f64) {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_current_time_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                current_time: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_current_time_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            current_time: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(current_time);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let current_time =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(current_time);
                __widl_f_set_current_time_HTMLMediaElement(self_, current_time)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_duration_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `duration` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/duration)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn duration(&self) -> f64 {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_duration_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_duration_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_duration_HTMLMediaElement(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_paused_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `paused` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/paused)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn paused(&self) -> bool {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_paused_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_paused_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_paused_HTMLMediaElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_default_playback_rate_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `defaultPlaybackRate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/defaultPlaybackRate)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn default_playback_rate(&self) -> f64 {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_default_playback_rate_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_default_playback_rate_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_default_playback_rate_HTMLMediaElement(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_default_playback_rate_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `defaultPlaybackRate` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/defaultPlaybackRate)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn set_default_playback_rate(&self, default_playback_rate: f64) {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_default_playback_rate_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                default_playback_rate: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_default_playback_rate_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            default_playback_rate: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(default_playback_rate);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let default_playback_rate =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(default_playback_rate);
                __widl_f_set_default_playback_rate_HTMLMediaElement(self_, default_playback_rate)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_playback_rate_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `playbackRate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/playbackRate)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn playback_rate(&self) -> f64 {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_playback_rate_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_playback_rate_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_playback_rate_HTMLMediaElement(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_playback_rate_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `playbackRate` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/playbackRate)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn set_playback_rate(&self, playback_rate: f64) {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_playback_rate_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                playback_rate: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_playback_rate_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            playback_rate: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(playback_rate);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let playback_rate =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(playback_rate);
                __widl_f_set_playback_rate_HTMLMediaElement(self_, playback_rate)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement", feature = "TimeRanges",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_played_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <TimeRanges as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement", feature = "TimeRanges",))]
    #[allow(bad_style)]
    #[doc = "The `played` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/played)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`, `TimeRanges`*"]
    #[allow(clippy::all)]
    pub fn played(&self) -> TimeRanges {
        #[cfg(all(feature = "HtmlMediaElement", feature = "TimeRanges",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_played_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TimeRanges as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_played_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TimeRanges as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_played_HTMLMediaElement(self_)
            };
            <TimeRanges as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement", feature = "TimeRanges",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_seekable_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <TimeRanges as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement", feature = "TimeRanges",))]
    #[allow(bad_style)]
    #[doc = "The `seekable` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/seekable)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`, `TimeRanges`*"]
    #[allow(clippy::all)]
    pub fn seekable(&self) -> TimeRanges {
        #[cfg(all(feature = "HtmlMediaElement", feature = "TimeRanges",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_seekable_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TimeRanges as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_seekable_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TimeRanges as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_seekable_HTMLMediaElement(self_)
            };
            <TimeRanges as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ended_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `ended` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/ended)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn ended(&self) -> bool {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ended_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ended_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ended_HTMLMediaElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_autoplay_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `autoplay` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/autoplay)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn autoplay(&self) -> bool {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_autoplay_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_autoplay_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_autoplay_HTMLMediaElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_autoplay_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `autoplay` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/autoplay)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn set_autoplay(&self, autoplay: bool) {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_autoplay_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                autoplay: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_autoplay_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            autoplay: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(autoplay);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let autoplay = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(autoplay);
                __widl_f_set_autoplay_HTMLMediaElement(self_, autoplay)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_loop_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `loop` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loop)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn loop_(&self) -> bool {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_loop_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_loop_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_loop_HTMLMediaElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_loop_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `loop` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loop)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn set_loop(&self, loop_: bool) {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_loop_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                loop_: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_loop_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            loop_: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(loop_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let loop_ = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(loop_);
                __widl_f_set_loop_HTMLMediaElement(self_, loop_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_controls_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `controls` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/controls)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn controls(&self) -> bool {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_controls_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_controls_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_controls_HTMLMediaElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_controls_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `controls` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/controls)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn set_controls(&self, controls: bool) {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_controls_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                controls: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_controls_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            controls: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(controls);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let controls = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(controls);
                __widl_f_set_controls_HTMLMediaElement(self_, controls)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_volume_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `volume` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/volume)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn volume(&self) -> f64 {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_volume_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_volume_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_volume_HTMLMediaElement(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_volume_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `volume` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/volume)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn set_volume(&self, volume: f64) {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_volume_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                volume: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_volume_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            volume: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(volume);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let volume = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(volume);
                __widl_f_set_volume_HTMLMediaElement(self_, volume)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_muted_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `muted` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/muted)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn muted(&self) -> bool {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_muted_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_muted_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_muted_HTMLMediaElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_muted_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `muted` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/muted)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn set_muted(&self, muted: bool) {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_muted_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                muted: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_muted_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            muted: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(muted);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let muted = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(muted);
                __widl_f_set_muted_HTMLMediaElement(self_, muted)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_default_muted_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `defaultMuted` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/defaultMuted)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn default_muted(&self) -> bool {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_default_muted_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_default_muted_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_default_muted_HTMLMediaElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_default_muted_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `defaultMuted` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/defaultMuted)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn set_default_muted(&self, default_muted: bool) {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_default_muted_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                default_muted: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_default_muted_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            default_muted: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(default_muted);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let default_muted =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(default_muted);
                __widl_f_set_default_muted_HTMLMediaElement(self_, default_muted)
            };
            ()
        }
    }
}
#[cfg(all(feature = "AudioTrackList", feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_audio_tracks_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <AudioTrackList as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "AudioTrackList", feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `audioTracks` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/audioTracks)\n\n*This API requires the following crate features to be activated: `AudioTrackList`, `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn audio_tracks(&self) -> AudioTrackList {
        #[cfg(all(feature = "AudioTrackList", feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_audio_tracks_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioTrackList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_audio_tracks_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioTrackList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_audio_tracks_HTMLMediaElement(self_)
            };
            <AudioTrackList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement", feature = "VideoTrackList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_video_tracks_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <VideoTrackList as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement", feature = "VideoTrackList",))]
    #[allow(bad_style)]
    #[doc = "The `videoTracks` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/videoTracks)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`, `VideoTrackList`*"]
    #[allow(clippy::all)]
    pub fn video_tracks(&self) -> VideoTrackList {
        #[cfg(all(feature = "HtmlMediaElement", feature = "VideoTrackList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_video_tracks_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <VideoTrackList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_video_tracks_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <VideoTrackList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_video_tracks_HTMLMediaElement(self_)
            };
            <VideoTrackList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement", feature = "TextTrackList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_text_tracks_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <Option<TextTrackList> as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement", feature = "TextTrackList",))]
    #[allow(bad_style)]
    #[doc = "The `textTracks` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/textTracks)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`, `TextTrackList`*"]
    #[allow(clippy::all)]
    pub fn text_tracks(&self) -> Option<TextTrackList> {
        #[cfg(all(feature = "HtmlMediaElement", feature = "TextTrackList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_text_tracks_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<TextTrackList> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_text_tracks_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<TextTrackList> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_text_tracks_HTMLMediaElement(self_)
            };
            <Option<TextTrackList> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement", feature = "MediaKeys",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_media_keys_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <Option<MediaKeys> as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement", feature = "MediaKeys",))]
    #[allow(bad_style)]
    #[doc = "The `mediaKeys` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/mediaKeys)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaKeys`*"]
    #[allow(clippy::all)]
    pub fn media_keys(&self) -> Option<MediaKeys> {
        #[cfg(all(feature = "HtmlMediaElement", feature = "MediaKeys",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_media_keys_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<MediaKeys> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_media_keys_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<MediaKeys> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_media_keys_HTMLMediaElement(self_)
            };
            <Option<MediaKeys> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onencrypted_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `onencrypted` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/onencrypted)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn onencrypted(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onencrypted_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onencrypted_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onencrypted_HTMLMediaElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onencrypted_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `onencrypted` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/onencrypted)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn set_onencrypted(&self, onencrypted: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onencrypted_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onencrypted : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onencrypted_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onencrypted: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onencrypted);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onencrypted =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onencrypted,
                    );
                __widl_f_set_onencrypted_HTMLMediaElement(self_, onencrypted)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onwaitingforkey_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `onwaitingforkey` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/onwaitingforkey)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn onwaitingforkey(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onwaitingforkey_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onwaitingforkey_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onwaitingforkey_HTMLMediaElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMediaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onwaitingforkey_HTMLMediaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMediaElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMediaElement {
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(bad_style)]
    #[doc = "The `onwaitingforkey` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/onwaitingforkey)\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    #[allow(clippy::all)]
    pub fn set_onwaitingforkey(&self, onwaitingforkey: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlMediaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onwaitingforkey_HTMLMediaElement(
                self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onwaitingforkey : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onwaitingforkey_HTMLMediaElement(
            self_: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onwaitingforkey : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onwaitingforkey);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onwaitingforkey =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onwaitingforkey,
                    );
                __widl_f_set_onwaitingforkey_HTMLMediaElement(self_, onwaitingforkey)
            };
            ()
        }
    }
}
impl HtmlMediaElement {
    pub const NETWORK_EMPTY: u16 = 0i64 as u16;
}
impl HtmlMediaElement {
    pub const NETWORK_IDLE: u16 = 1u64 as u16;
}
impl HtmlMediaElement {
    pub const NETWORK_LOADING: u16 = 2u64 as u16;
}
impl HtmlMediaElement {
    pub const NETWORK_NO_SOURCE: u16 = 3u64 as u16;
}
impl HtmlMediaElement {
    pub const HAVE_NOTHING: u16 = 0i64 as u16;
}
impl HtmlMediaElement {
    pub const HAVE_METADATA: u16 = 1u64 as u16;
}
impl HtmlMediaElement {
    pub const HAVE_CURRENT_DATA: u16 = 2u64 as u16;
}
impl HtmlMediaElement {
    pub const HAVE_FUTURE_DATA: u16 = 3u64 as u16;
}
impl HtmlMediaElement {
    pub const HAVE_ENOUGH_DATA: u16 = 4u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_687772414e364b86: [u8; 5626usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xB8\x15\0\0\0\0:\0\0\x02\x10HTMLMediaElement\"__widl_instanceof_HTMLMediaElement\0\0\0\0(__widl_f_add_text_track_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\0\x01\x02\x05self_\x04kind\x0CaddTextTrack\0\0\03__widl_f_add_text_track_with_label_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\0\x01\x03\x05self_\x04kind\x05label\x0CaddTextTrack\0\0\0@__widl_f_add_text_track_with_label_and_language_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\0\x01\x04\x05self_\x04kind\x05label\x08language\x0CaddTextTrack\0\0\0'__widl_f_can_play_type_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\0\x01\x02\x05self_\x05type_\x0BcanPlayType\0\0\0#__widl_f_fast_seek_HTMLMediaElement\x01\0\0\x01\x10HTMLMediaElement\x01\0\0\x01\x02\x05self_\x04time\x08fastSeek\0\0\0+__widl_f_has_suspend_taint_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\0\x01\x01\x05self_\x0FhasSuspendTaint\0\0\0\x1E__widl_f_load_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\0\x01\x01\x05self_\x04load\0\0\0\x1F__widl_f_pause_HTMLMediaElement\x01\0\0\x01\x10HTMLMediaElement\x01\0\0\x01\x01\x05self_\x05pause\0\0\0\x1E__widl_f_play_HTMLMediaElement\x01\0\0\x01\x10HTMLMediaElement\x01\0\0\x01\x01\x05self_\x04play\0\0\0,__widl_f_seek_to_next_frame_HTMLMediaElement\x01\0\0\x01\x10HTMLMediaElement\x01\0\0\x01\x01\x05self_\x0FseekToNextFrame\0\0\0(__widl_f_set_media_keys_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\0\x01\x02\x05self_\nmedia_keys\x0CsetMediaKeys\0\0\0%__widl_f_set_visible_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\0\x01\x02\x05self_\ta_visible\nsetVisible\0\0\0\x1F__widl_f_error_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x01\x05error\x01\x01\x05self_\x05error\0\0\0\x1D__widl_f_src_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x01\x03src\x01\x01\x05self_\x03src\0\0\0!__widl_f_set_src_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x02\x03src\x01\x02\x05self_\x03src\x03src\0\0\0%__widl_f_current_src_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x01\ncurrentSrc\x01\x01\x05self_\ncurrentSrc\0\0\0$__widl_f_src_object_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x01\tsrcObject\x01\x01\x05self_\tsrcObject\0\0\0(__widl_f_set_src_object_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x02\tsrcObject\x01\x02\x05self_\nsrc_object\tsrcObject\0\0\0&__widl_f_cross_origin_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x01\x0BcrossOrigin\x01\x01\x05self_\x0BcrossOrigin\0\0\0*__widl_f_set_cross_origin_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x02\x0BcrossOrigin\x01\x02\x05self_\x0Ccross_origin\x0BcrossOrigin\0\0\0'__widl_f_network_state_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x01\x0CnetworkState\x01\x01\x05self_\x0CnetworkState\0\0\0!__widl_f_preload_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x01\x07preload\x01\x01\x05self_\x07preload\0\0\0%__widl_f_set_preload_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x02\x07preload\x01\x02\x05self_\x07preload\x07preload\0\0\0\"__widl_f_buffered_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x01\x08buffered\x01\x01\x05self_\x08buffered\0\0\0%__widl_f_ready_state_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x01\nreadyState\x01\x01\x05self_\nreadyState\0\0\0!__widl_f_seeking_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x01\x07seeking\x01\x01\x05self_\x07seeking\0\0\0&__widl_f_current_time_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x01\x0BcurrentTime\x01\x01\x05self_\x0BcurrentTime\0\0\0*__widl_f_set_current_time_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x02\x0BcurrentTime\x01\x02\x05self_\x0Ccurrent_time\x0BcurrentTime\0\0\0\"__widl_f_duration_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x01\x08duration\x01\x01\x05self_\x08duration\0\0\0 __widl_f_paused_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x01\x06paused\x01\x01\x05self_\x06paused\0\0\0/__widl_f_default_playback_rate_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x01\x13defaultPlaybackRate\x01\x01\x05self_\x13defaultPlaybackRate\0\0\03__widl_f_set_default_playback_rate_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x02\x13defaultPlaybackRate\x01\x02\x05self_\x15default_playback_rate\x13defaultPlaybackRate\0\0\0'__widl_f_playback_rate_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x01\x0CplaybackRate\x01\x01\x05self_\x0CplaybackRate\0\0\0+__widl_f_set_playback_rate_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x02\x0CplaybackRate\x01\x02\x05self_\rplayback_rate\x0CplaybackRate\0\0\0 __widl_f_played_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x01\x06played\x01\x01\x05self_\x06played\0\0\0\"__widl_f_seekable_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x01\x08seekable\x01\x01\x05self_\x08seekable\0\0\0\x1F__widl_f_ended_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x01\x05ended\x01\x01\x05self_\x05ended\0\0\0\"__widl_f_autoplay_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x01\x08autoplay\x01\x01\x05self_\x08autoplay\0\0\0&__widl_f_set_autoplay_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x02\x08autoplay\x01\x02\x05self_\x08autoplay\x08autoplay\0\0\0\x1E__widl_f_loop_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x01\x04loop\x01\x01\x05self_\x04loop\0\0\0\"__widl_f_set_loop_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x02\x04loop\x01\x02\x05self_\x05loop_\x04loop\0\0\0\"__widl_f_controls_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x01\x08controls\x01\x01\x05self_\x08controls\0\0\0&__widl_f_set_controls_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x02\x08controls\x01\x02\x05self_\x08controls\x08controls\0\0\0 __widl_f_volume_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x01\x06volume\x01\x01\x05self_\x06volume\0\0\0$__widl_f_set_volume_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x02\x06volume\x01\x02\x05self_\x06volume\x06volume\0\0\0\x1F__widl_f_muted_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x01\x05muted\x01\x01\x05self_\x05muted\0\0\0#__widl_f_set_muted_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x02\x05muted\x01\x02\x05self_\x05muted\x05muted\0\0\0'__widl_f_default_muted_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x01\x0CdefaultMuted\x01\x01\x05self_\x0CdefaultMuted\0\0\0+__widl_f_set_default_muted_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x02\x0CdefaultMuted\x01\x02\x05self_\rdefault_muted\x0CdefaultMuted\0\0\0&__widl_f_audio_tracks_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x01\x0BaudioTracks\x01\x01\x05self_\x0BaudioTracks\0\0\0&__widl_f_video_tracks_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x01\x0BvideoTracks\x01\x01\x05self_\x0BvideoTracks\0\0\0%__widl_f_text_tracks_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x01\ntextTracks\x01\x01\x05self_\ntextTracks\0\0\0$__widl_f_media_keys_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x01\tmediaKeys\x01\x01\x05self_\tmediaKeys\0\0\0%__widl_f_onencrypted_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x01\x0Bonencrypted\x01\x01\x05self_\x0Bonencrypted\0\0\0)__widl_f_set_onencrypted_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x02\x0Bonencrypted\x01\x02\x05self_\x0Bonencrypted\x0Bonencrypted\0\0\0)__widl_f_onwaitingforkey_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x01\x0Fonwaitingforkey\x01\x01\x05self_\x0Fonwaitingforkey\0\0\0-__widl_f_set_onwaitingforkey_HTMLMediaElement\0\0\0\x01\x10HTMLMediaElement\x01\0\x02\x0Fonwaitingforkey\x01\x02\x05self_\x0Fonwaitingforkey\x0Fonwaitingforkey\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
