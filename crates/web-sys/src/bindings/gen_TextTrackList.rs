use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `TextTrackList` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList)\n\n*This API requires the following crate features to be activated: `TextTrackList`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct TextTrackList {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_TextTrackList: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for TextTrackList {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
            inform(84u32);
            inform(101u32);
            inform(120u32);
            inform(116u32);
            inform(84u32);
            inform(114u32);
            inform(97u32);
            inform(99u32);
            inform(107u32);
            inform(76u32);
            inform(105u32);
            inform(115u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for TextTrackList {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for TextTrackList {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for TextTrackList {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a TextTrackList {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for TextTrackList {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            TextTrackList {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for TextTrackList {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a TextTrackList {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for TextTrackList {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<TextTrackList>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(TextTrackList {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for TextTrackList {
        #[inline]
        fn from(obj: JsValue) -> TextTrackList {
            TextTrackList { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for TextTrackList {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<TextTrackList> for TextTrackList {
        #[inline]
        fn as_ref(&self) -> &TextTrackList {
            self
        }
    }
    impl From<TextTrackList> for JsValue {
        #[inline]
        fn from(obj: TextTrackList) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for TextTrackList {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_TextTrackList(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_TextTrackList(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_TextTrackList(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            TextTrackList { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const TextTrackList) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<TextTrackList> for EventTarget {
    #[inline]
    fn from(obj: TextTrackList) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for TextTrackList {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<TextTrackList> for ::js_sys::Object {
    #[inline]
    fn from(obj: TextTrackList) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for TextTrackList {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "TextTrack", feature = "TextTrackList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_track_by_id_TextTrackList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TextTrackList as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<TextTrack> as WasmDescribe>::describe();
}
impl TextTrackList {
    #[cfg(all(feature = "TextTrack", feature = "TextTrackList",))]
    #[allow(bad_style)]
    #[doc = "The `getTrackById()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/getTrackById)\n\n*This API requires the following crate features to be activated: `TextTrack`, `TextTrackList`*"]
    #[allow(clippy::all)]
    pub fn get_track_by_id(&self, id: &str) -> Option<TextTrack> {
        #[cfg(all(feature = "TextTrack", feature = "TextTrackList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_track_by_id_TextTrackList(
                self_: <&TextTrackList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<TextTrack> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_track_by_id_TextTrackList(
            self_: <&TextTrackList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<TextTrack> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(id);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrackList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(id);
                __widl_f_get_track_by_id_TextTrackList(self_, id)
            };
            <Option<TextTrack> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TextTrack", feature = "TextTrackList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_TextTrackList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TextTrackList as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<TextTrack> as WasmDescribe>::describe();
}
impl TextTrackList {
    #[cfg(all(feature = "TextTrack", feature = "TextTrackList",))]
    #[allow(bad_style)]
    #[doc = "The indexing getter\n\n\n\n*This API requires the following crate features to be activated: `TextTrack`, `TextTrackList`*"]
    #[allow(clippy::all)]
    pub fn get(&self, index: u32) -> Option<TextTrack> {
        #[cfg(all(feature = "TextTrack", feature = "TextTrackList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_TextTrackList(
                self_: <&TextTrackList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<TextTrack> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_TextTrackList(
            self_: <&TextTrackList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<TextTrack> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrackList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_get_TextTrackList(self_, index)
            };
            <Option<TextTrack> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TextTrackList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_length_TextTrackList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TextTrackList as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl TextTrackList {
    #[cfg(all(feature = "TextTrackList",))]
    #[allow(bad_style)]
    #[doc = "The `length` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/length)\n\n*This API requires the following crate features to be activated: `TextTrackList`*"]
    #[allow(clippy::all)]
    pub fn length(&self) -> u32 {
        #[cfg(all(feature = "TextTrackList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_length_TextTrackList(
                self_: <&TextTrackList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_length_TextTrackList(
            self_: <&TextTrackList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrackList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_length_TextTrackList(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TextTrackList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onchange_TextTrackList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TextTrackList as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl TextTrackList {
    #[cfg(all(feature = "TextTrackList",))]
    #[allow(bad_style)]
    #[doc = "The `onchange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/onchange)\n\n*This API requires the following crate features to be activated: `TextTrackList`*"]
    #[allow(clippy::all)]
    pub fn onchange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "TextTrackList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onchange_TextTrackList(
                self_: <&TextTrackList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onchange_TextTrackList(
            self_: <&TextTrackList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrackList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onchange_TextTrackList(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TextTrackList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onchange_TextTrackList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TextTrackList as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TextTrackList {
    #[cfg(all(feature = "TextTrackList",))]
    #[allow(bad_style)]
    #[doc = "The `onchange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/onchange)\n\n*This API requires the following crate features to be activated: `TextTrackList`*"]
    #[allow(clippy::all)]
    pub fn set_onchange(&self, onchange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "TextTrackList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onchange_TextTrackList(
                self_: <&TextTrackList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onchange: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onchange_TextTrackList(
            self_: <&TextTrackList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onchange: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onchange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrackList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onchange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onchange,
                    );
                __widl_f_set_onchange_TextTrackList(self_, onchange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TextTrackList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onaddtrack_TextTrackList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TextTrackList as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl TextTrackList {
    #[cfg(all(feature = "TextTrackList",))]
    #[allow(bad_style)]
    #[doc = "The `onaddtrack` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/onaddtrack)\n\n*This API requires the following crate features to be activated: `TextTrackList`*"]
    #[allow(clippy::all)]
    pub fn onaddtrack(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "TextTrackList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onaddtrack_TextTrackList(
                self_: <&TextTrackList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onaddtrack_TextTrackList(
            self_: <&TextTrackList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrackList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onaddtrack_TextTrackList(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TextTrackList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onaddtrack_TextTrackList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TextTrackList as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TextTrackList {
    #[cfg(all(feature = "TextTrackList",))]
    #[allow(bad_style)]
    #[doc = "The `onaddtrack` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/onaddtrack)\n\n*This API requires the following crate features to be activated: `TextTrackList`*"]
    #[allow(clippy::all)]
    pub fn set_onaddtrack(&self, onaddtrack: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "TextTrackList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onaddtrack_TextTrackList(
                self_: <&TextTrackList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onaddtrack : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onaddtrack_TextTrackList(
            self_: <&TextTrackList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onaddtrack: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onaddtrack);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrackList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onaddtrack =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onaddtrack,
                    );
                __widl_f_set_onaddtrack_TextTrackList(self_, onaddtrack)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TextTrackList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onremovetrack_TextTrackList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TextTrackList as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl TextTrackList {
    #[cfg(all(feature = "TextTrackList",))]
    #[allow(bad_style)]
    #[doc = "The `onremovetrack` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/onremovetrack)\n\n*This API requires the following crate features to be activated: `TextTrackList`*"]
    #[allow(clippy::all)]
    pub fn onremovetrack(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "TextTrackList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onremovetrack_TextTrackList(
                self_: <&TextTrackList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onremovetrack_TextTrackList(
            self_: <&TextTrackList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrackList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onremovetrack_TextTrackList(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TextTrackList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onremovetrack_TextTrackList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TextTrackList as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TextTrackList {
    #[cfg(all(feature = "TextTrackList",))]
    #[allow(bad_style)]
    #[doc = "The `onremovetrack` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/onremovetrack)\n\n*This API requires the following crate features to be activated: `TextTrackList`*"]
    #[allow(clippy::all)]
    pub fn set_onremovetrack(&self, onremovetrack: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "TextTrackList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onremovetrack_TextTrackList(
                self_: <&TextTrackList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onremovetrack : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onremovetrack_TextTrackList(
            self_: <&TextTrackList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onremovetrack: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onremovetrack);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrackList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onremovetrack =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onremovetrack,
                    );
                __widl_f_set_onremovetrack_TextTrackList(self_, onremovetrack)
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
pub static __WASM_BINDGEN_GENERATED_804b1c34fd8888cb: [u8; 967usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x85\x03\0\0\0\0\n\0\0\x02\rTextTrackList\x1F__widl_instanceof_TextTrackList\0\0\0\0&__widl_f_get_track_by_id_TextTrackList\0\0\0\x01\rTextTrackList\x01\0\0\x01\x02\x05self_\x02id\x0CgetTrackById\0\0\0\x1A__widl_f_get_TextTrackList\0\0\0\x01\rTextTrackList\x01\0\x03\x01\x02\x05self_\x05index\x03get\0\0\0\x1D__widl_f_length_TextTrackList\0\0\0\x01\rTextTrackList\x01\0\x01\x06length\x01\x01\x05self_\x06length\0\0\0\x1F__widl_f_onchange_TextTrackList\0\0\0\x01\rTextTrackList\x01\0\x01\x08onchange\x01\x01\x05self_\x08onchange\0\0\0#__widl_f_set_onchange_TextTrackList\0\0\0\x01\rTextTrackList\x01\0\x02\x08onchange\x01\x02\x05self_\x08onchange\x08onchange\0\0\0!__widl_f_onaddtrack_TextTrackList\0\0\0\x01\rTextTrackList\x01\0\x01\nonaddtrack\x01\x01\x05self_\nonaddtrack\0\0\0%__widl_f_set_onaddtrack_TextTrackList\0\0\0\x01\rTextTrackList\x01\0\x02\nonaddtrack\x01\x02\x05self_\nonaddtrack\nonaddtrack\0\0\0$__widl_f_onremovetrack_TextTrackList\0\0\0\x01\rTextTrackList\x01\0\x01\ronremovetrack\x01\x01\x05self_\ronremovetrack\0\0\0(__widl_f_set_onremovetrack_TextTrackList\0\0\0\x01\rTextTrackList\x01\0\x02\ronremovetrack\x01\x02\x05self_\ronremovetrack\ronremovetrack\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
