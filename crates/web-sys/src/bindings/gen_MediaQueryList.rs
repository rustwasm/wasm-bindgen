use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MediaQueryList` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList)\n\n*This API requires the following crate features to be activated: `MediaQueryList`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MediaQueryList {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MediaQueryList: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MediaQueryList {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(14u32);
            inform(77u32);
            inform(101u32);
            inform(100u32);
            inform(105u32);
            inform(97u32);
            inform(81u32);
            inform(117u32);
            inform(101u32);
            inform(114u32);
            inform(121u32);
            inform(76u32);
            inform(105u32);
            inform(115u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for MediaQueryList {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for MediaQueryList {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MediaQueryList {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MediaQueryList {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MediaQueryList {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MediaQueryList {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MediaQueryList {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MediaQueryList {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MediaQueryList {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MediaQueryList>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MediaQueryList {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MediaQueryList {
        #[inline]
        fn from(obj: JsValue) -> MediaQueryList {
            MediaQueryList { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MediaQueryList {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MediaQueryList> for MediaQueryList {
        #[inline]
        fn as_ref(&self) -> &MediaQueryList {
            self
        }
    }
    impl From<MediaQueryList> for JsValue {
        #[inline]
        fn from(obj: MediaQueryList) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MediaQueryList {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MediaQueryList(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MediaQueryList(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MediaQueryList(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MediaQueryList { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MediaQueryList) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MediaQueryList> for EventTarget {
    #[inline]
    fn from(obj: MediaQueryList) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for MediaQueryList {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<MediaQueryList> for ::js_sys::Object {
    #[inline]
    fn from(obj: MediaQueryList) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MediaQueryList {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MediaQueryList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_listener_with_opt_callback_MediaQueryList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaQueryList as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaQueryList {
    #[cfg(all(feature = "MediaQueryList",))]
    #[allow(bad_style)]
    #[doc = "The `addListener()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/addListener)\n\n*This API requires the following crate features to be activated: `MediaQueryList`*"]
    #[allow(clippy::all)]
    pub fn add_listener_with_opt_callback(
        &self,
        listener: Option<&::js_sys::Function>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaQueryList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_listener_with_opt_callback_MediaQueryList(
                self_: <&MediaQueryList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                listener: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_listener_with_opt_callback_MediaQueryList(
            self_: <&MediaQueryList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            listener: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(listener);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaQueryList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let listener =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        listener,
                    );
                __widl_f_add_listener_with_opt_callback_MediaQueryList(self_, listener)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "EventListener", feature = "MediaQueryList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_listener_with_opt_event_listener_MediaQueryList()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaQueryList as WasmDescribe>::describe();
    <Option<&EventListener> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaQueryList {
    #[cfg(all(feature = "EventListener", feature = "MediaQueryList",))]
    #[allow(bad_style)]
    #[doc = "The `addListener()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/addListener)\n\n*This API requires the following crate features to be activated: `EventListener`, `MediaQueryList`*"]
    #[allow(clippy::all)]
    pub fn add_listener_with_opt_event_listener(
        &self,
        listener: Option<&EventListener>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "EventListener", feature = "MediaQueryList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_listener_with_opt_event_listener_MediaQueryList(
                self_: <&MediaQueryList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                listener: <Option<&EventListener> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_listener_with_opt_event_listener_MediaQueryList(
            self_: <&MediaQueryList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            listener: <Option<&EventListener> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(listener);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaQueryList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let listener =
                    <Option<&EventListener> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        listener,
                    );
                __widl_f_add_listener_with_opt_event_listener_MediaQueryList(self_, listener)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "MediaQueryList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_listener_with_opt_callback_MediaQueryList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaQueryList as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaQueryList {
    #[cfg(all(feature = "MediaQueryList",))]
    #[allow(bad_style)]
    #[doc = "The `removeListener()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/removeListener)\n\n*This API requires the following crate features to be activated: `MediaQueryList`*"]
    #[allow(clippy::all)]
    pub fn remove_listener_with_opt_callback(
        &self,
        listener: Option<&::js_sys::Function>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaQueryList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_listener_with_opt_callback_MediaQueryList(
                self_: <&MediaQueryList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                listener: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_listener_with_opt_callback_MediaQueryList(
            self_: <&MediaQueryList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            listener: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(listener);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaQueryList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let listener =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        listener,
                    );
                __widl_f_remove_listener_with_opt_callback_MediaQueryList(self_, listener)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "EventListener", feature = "MediaQueryList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_listener_with_opt_event_listener_MediaQueryList(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaQueryList as WasmDescribe>::describe();
    <Option<&EventListener> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaQueryList {
    #[cfg(all(feature = "EventListener", feature = "MediaQueryList",))]
    #[allow(bad_style)]
    #[doc = "The `removeListener()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/removeListener)\n\n*This API requires the following crate features to be activated: `EventListener`, `MediaQueryList`*"]
    #[allow(clippy::all)]
    pub fn remove_listener_with_opt_event_listener(
        &self,
        listener: Option<&EventListener>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "EventListener", feature = "MediaQueryList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_listener_with_opt_event_listener_MediaQueryList(
                self_: <&MediaQueryList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                listener: <Option<&EventListener> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_listener_with_opt_event_listener_MediaQueryList(
            self_: <&MediaQueryList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            listener: <Option<&EventListener> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(listener);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaQueryList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let listener =
                    <Option<&EventListener> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        listener,
                    );
                __widl_f_remove_listener_with_opt_event_listener_MediaQueryList(self_, listener)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "MediaQueryList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_media_MediaQueryList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaQueryList as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl MediaQueryList {
    #[cfg(all(feature = "MediaQueryList",))]
    #[allow(bad_style)]
    #[doc = "The `media` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/media)\n\n*This API requires the following crate features to be activated: `MediaQueryList`*"]
    #[allow(clippy::all)]
    pub fn media(&self) -> String {
        #[cfg(all(feature = "MediaQueryList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_media_MediaQueryList(
                self_: <&MediaQueryList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_media_MediaQueryList(
            self_: <&MediaQueryList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaQueryList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_media_MediaQueryList(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaQueryList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_matches_MediaQueryList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaQueryList as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl MediaQueryList {
    #[cfg(all(feature = "MediaQueryList",))]
    #[allow(bad_style)]
    #[doc = "The `matches` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/matches)\n\n*This API requires the following crate features to be activated: `MediaQueryList`*"]
    #[allow(clippy::all)]
    pub fn matches(&self) -> bool {
        #[cfg(all(feature = "MediaQueryList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_matches_MediaQueryList(
                self_: <&MediaQueryList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_matches_MediaQueryList(
            self_: <&MediaQueryList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaQueryList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_matches_MediaQueryList(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaQueryList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onchange_MediaQueryList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaQueryList as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl MediaQueryList {
    #[cfg(all(feature = "MediaQueryList",))]
    #[allow(bad_style)]
    #[doc = "The `onchange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/onchange)\n\n*This API requires the following crate features to be activated: `MediaQueryList`*"]
    #[allow(clippy::all)]
    pub fn onchange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "MediaQueryList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onchange_MediaQueryList(
                self_: <&MediaQueryList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onchange_MediaQueryList(
            self_: <&MediaQueryList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaQueryList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onchange_MediaQueryList(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaQueryList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onchange_MediaQueryList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaQueryList as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaQueryList {
    #[cfg(all(feature = "MediaQueryList",))]
    #[allow(bad_style)]
    #[doc = "The `onchange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/onchange)\n\n*This API requires the following crate features to be activated: `MediaQueryList`*"]
    #[allow(clippy::all)]
    pub fn set_onchange(&self, onchange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "MediaQueryList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onchange_MediaQueryList(
                self_: <&MediaQueryList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onchange: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onchange_MediaQueryList(
            self_: <&MediaQueryList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&MediaQueryList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onchange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onchange,
                    );
                __widl_f_set_onchange_MediaQueryList(self_, onchange)
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
pub static __WASM_BINDGEN_GENERATED_0fcb0f364a69de5a: [u8; 954usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}x\x03\0\0\0\0\t\0\0\x02\x0EMediaQueryList __widl_instanceof_MediaQueryList\0\0\0\06__widl_f_add_listener_with_opt_callback_MediaQueryList\x01\0\0\x01\x0EMediaQueryList\x01\0\0\x01\x02\x05self_\x08listener\x0BaddListener\0\0\0<__widl_f_add_listener_with_opt_event_listener_MediaQueryList\x01\0\0\x01\x0EMediaQueryList\x01\0\0\x01\x02\x05self_\x08listener\x0BaddListener\0\0\09__widl_f_remove_listener_with_opt_callback_MediaQueryList\x01\0\0\x01\x0EMediaQueryList\x01\0\0\x01\x02\x05self_\x08listener\x0EremoveListener\0\0\0?__widl_f_remove_listener_with_opt_event_listener_MediaQueryList\x01\0\0\x01\x0EMediaQueryList\x01\0\0\x01\x02\x05self_\x08listener\x0EremoveListener\0\0\0\x1D__widl_f_media_MediaQueryList\0\0\0\x01\x0EMediaQueryList\x01\0\x01\x05media\x01\x01\x05self_\x05media\0\0\0\x1F__widl_f_matches_MediaQueryList\0\0\0\x01\x0EMediaQueryList\x01\0\x01\x07matches\x01\x01\x05self_\x07matches\0\0\0 __widl_f_onchange_MediaQueryList\0\0\0\x01\x0EMediaQueryList\x01\0\x01\x08onchange\x01\x01\x05self_\x08onchange\0\0\0$__widl_f_set_onchange_MediaQueryList\0\0\0\x01\x0EMediaQueryList\x01\0\x02\x08onchange\x01\x02\x05self_\x08onchange\x08onchange\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
