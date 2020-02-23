use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLVideoElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement)\n\n*This API requires the following crate features to be activated: `HtmlVideoElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlVideoElement {
    obj: HtmlMediaElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlVideoElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlVideoElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(86u32);
            inform(105u32);
            inform(100u32);
            inform(101u32);
            inform(111u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for HtmlVideoElement {
        type Target = HtmlMediaElement;
        #[inline]
        fn deref(&self) -> &HtmlMediaElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlVideoElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlVideoElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlVideoElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlVideoElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlVideoElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlVideoElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlVideoElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlVideoElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlVideoElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlVideoElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlVideoElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlVideoElement {
            HtmlVideoElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlVideoElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlVideoElement> for HtmlVideoElement {
        #[inline]
        fn as_ref(&self) -> &HtmlVideoElement {
            self
        }
    }
    impl From<HtmlVideoElement> for JsValue {
        #[inline]
        fn from(obj: HtmlVideoElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlVideoElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLVideoElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLVideoElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLVideoElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlVideoElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlVideoElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlVideoElement> for HtmlMediaElement {
    #[inline]
    fn from(obj: HtmlVideoElement) -> HtmlMediaElement {
        use wasm_bindgen::JsCast;
        HtmlMediaElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlMediaElement> for HtmlVideoElement {
    #[inline]
    fn as_ref(&self) -> &HtmlMediaElement {
        use wasm_bindgen::JsCast;
        HtmlMediaElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlVideoElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlVideoElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlVideoElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlVideoElement> for Element {
    #[inline]
    fn from(obj: HtmlVideoElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlVideoElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlVideoElement> for Node {
    #[inline]
    fn from(obj: HtmlVideoElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlVideoElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlVideoElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlVideoElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlVideoElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlVideoElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlVideoElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlVideoElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlVideoElement", feature = "VideoPlaybackQuality",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_video_playback_quality_HTMLVideoElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlVideoElement as WasmDescribe>::describe();
    <VideoPlaybackQuality as WasmDescribe>::describe();
}
impl HtmlVideoElement {
    #[cfg(all(feature = "HtmlVideoElement", feature = "VideoPlaybackQuality",))]
    #[allow(bad_style)]
    #[doc = "The `getVideoPlaybackQuality()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/getVideoPlaybackQuality)\n\n*This API requires the following crate features to be activated: `HtmlVideoElement`, `VideoPlaybackQuality`*"]
    #[allow(clippy::all)]
    pub fn get_video_playback_quality(&self) -> VideoPlaybackQuality {
        #[cfg(all(feature = "HtmlVideoElement", feature = "VideoPlaybackQuality",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_video_playback_quality_HTMLVideoElement(
                self_: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <VideoPlaybackQuality as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_video_playback_quality_HTMLVideoElement(
            self_: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <VideoPlaybackQuality as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_video_playback_quality_HTMLVideoElement(self_)
            };
            <VideoPlaybackQuality as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlVideoElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_width_HTMLVideoElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlVideoElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl HtmlVideoElement {
    #[cfg(all(feature = "HtmlVideoElement",))]
    #[allow(bad_style)]
    #[doc = "The `width` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/width)\n\n*This API requires the following crate features to be activated: `HtmlVideoElement`*"]
    #[allow(clippy::all)]
    pub fn width(&self) -> u32 {
        #[cfg(all(feature = "HtmlVideoElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_width_HTMLVideoElement(
                self_: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_width_HTMLVideoElement(
            self_: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_width_HTMLVideoElement(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlVideoElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_width_HTMLVideoElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlVideoElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlVideoElement {
    #[cfg(all(feature = "HtmlVideoElement",))]
    #[allow(bad_style)]
    #[doc = "The `width` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/width)\n\n*This API requires the following crate features to be activated: `HtmlVideoElement`*"]
    #[allow(clippy::all)]
    pub fn set_width(&self, width: u32) {
        #[cfg(all(feature = "HtmlVideoElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_width_HTMLVideoElement(
                self_: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_width_HTMLVideoElement(
            self_: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            width: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(width);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let width = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                __widl_f_set_width_HTMLVideoElement(self_, width)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlVideoElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_height_HTMLVideoElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlVideoElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl HtmlVideoElement {
    #[cfg(all(feature = "HtmlVideoElement",))]
    #[allow(bad_style)]
    #[doc = "The `height` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/height)\n\n*This API requires the following crate features to be activated: `HtmlVideoElement`*"]
    #[allow(clippy::all)]
    pub fn height(&self) -> u32 {
        #[cfg(all(feature = "HtmlVideoElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_height_HTMLVideoElement(
                self_: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_height_HTMLVideoElement(
            self_: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_height_HTMLVideoElement(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlVideoElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_height_HTMLVideoElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlVideoElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlVideoElement {
    #[cfg(all(feature = "HtmlVideoElement",))]
    #[allow(bad_style)]
    #[doc = "The `height` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/height)\n\n*This API requires the following crate features to be activated: `HtmlVideoElement`*"]
    #[allow(clippy::all)]
    pub fn set_height(&self, height: u32) {
        #[cfg(all(feature = "HtmlVideoElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_height_HTMLVideoElement(
                self_: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                height: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_height_HTMLVideoElement(
            self_: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            height: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(height);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let height = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(height);
                __widl_f_set_height_HTMLVideoElement(self_, height)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlVideoElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_video_width_HTMLVideoElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlVideoElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl HtmlVideoElement {
    #[cfg(all(feature = "HtmlVideoElement",))]
    #[allow(bad_style)]
    #[doc = "The `videoWidth` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/videoWidth)\n\n*This API requires the following crate features to be activated: `HtmlVideoElement`*"]
    #[allow(clippy::all)]
    pub fn video_width(&self) -> u32 {
        #[cfg(all(feature = "HtmlVideoElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_video_width_HTMLVideoElement(
                self_: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_video_width_HTMLVideoElement(
            self_: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_video_width_HTMLVideoElement(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlVideoElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_video_height_HTMLVideoElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlVideoElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl HtmlVideoElement {
    #[cfg(all(feature = "HtmlVideoElement",))]
    #[allow(bad_style)]
    #[doc = "The `videoHeight` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/videoHeight)\n\n*This API requires the following crate features to be activated: `HtmlVideoElement`*"]
    #[allow(clippy::all)]
    pub fn video_height(&self) -> u32 {
        #[cfg(all(feature = "HtmlVideoElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_video_height_HTMLVideoElement(
                self_: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_video_height_HTMLVideoElement(
            self_: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_video_height_HTMLVideoElement(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlVideoElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_poster_HTMLVideoElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlVideoElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlVideoElement {
    #[cfg(all(feature = "HtmlVideoElement",))]
    #[allow(bad_style)]
    #[doc = "The `poster` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/poster)\n\n*This API requires the following crate features to be activated: `HtmlVideoElement`*"]
    #[allow(clippy::all)]
    pub fn poster(&self) -> String {
        #[cfg(all(feature = "HtmlVideoElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_poster_HTMLVideoElement(
                self_: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_poster_HTMLVideoElement(
            self_: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_poster_HTMLVideoElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlVideoElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_poster_HTMLVideoElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlVideoElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlVideoElement {
    #[cfg(all(feature = "HtmlVideoElement",))]
    #[allow(bad_style)]
    #[doc = "The `poster` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/poster)\n\n*This API requires the following crate features to be activated: `HtmlVideoElement`*"]
    #[allow(clippy::all)]
    pub fn set_poster(&self, poster: &str) {
        #[cfg(all(feature = "HtmlVideoElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_poster_HTMLVideoElement(
                self_: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                poster: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_poster_HTMLVideoElement(
            self_: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            poster: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(poster);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let poster = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(poster);
                __widl_f_set_poster_HTMLVideoElement(self_, poster)
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
pub static __WASM_BINDGEN_GENERATED_60101d6d35e401e4: [u8; 984usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x96\x03\0\0\0\0\n\0\0\x02\x10HTMLVideoElement\"__widl_instanceof_HTMLVideoElement\0\0\0\04__widl_f_get_video_playback_quality_HTMLVideoElement\0\0\0\x01\x10HTMLVideoElement\x01\0\0\x01\x01\x05self_\x17getVideoPlaybackQuality\0\0\0\x1F__widl_f_width_HTMLVideoElement\0\0\0\x01\x10HTMLVideoElement\x01\0\x01\x05width\x01\x01\x05self_\x05width\0\0\0#__widl_f_set_width_HTMLVideoElement\0\0\0\x01\x10HTMLVideoElement\x01\0\x02\x05width\x01\x02\x05self_\x05width\x05width\0\0\0 __widl_f_height_HTMLVideoElement\0\0\0\x01\x10HTMLVideoElement\x01\0\x01\x06height\x01\x01\x05self_\x06height\0\0\0$__widl_f_set_height_HTMLVideoElement\0\0\0\x01\x10HTMLVideoElement\x01\0\x02\x06height\x01\x02\x05self_\x06height\x06height\0\0\0%__widl_f_video_width_HTMLVideoElement\0\0\0\x01\x10HTMLVideoElement\x01\0\x01\nvideoWidth\x01\x01\x05self_\nvideoWidth\0\0\0&__widl_f_video_height_HTMLVideoElement\0\0\0\x01\x10HTMLVideoElement\x01\0\x01\x0BvideoHeight\x01\x01\x05self_\x0BvideoHeight\0\0\0 __widl_f_poster_HTMLVideoElement\0\0\0\x01\x10HTMLVideoElement\x01\0\x01\x06poster\x01\x01\x05self_\x06poster\0\0\0$__widl_f_set_poster_HTMLVideoElement\0\0\0\x01\x10HTMLVideoElement\x01\0\x02\x06poster\x01\x02\x05self_\x06poster\x06poster\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
