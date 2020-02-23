use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MediaList` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaList)\n\n*This API requires the following crate features to be activated: `MediaList`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MediaList {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MediaList: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MediaList {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(9u32);
            inform(77u32);
            inform(101u32);
            inform(100u32);
            inform(105u32);
            inform(97u32);
            inform(76u32);
            inform(105u32);
            inform(115u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for MediaList {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for MediaList {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MediaList {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MediaList {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MediaList {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MediaList {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MediaList {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MediaList {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MediaList {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MediaList>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MediaList {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MediaList {
        #[inline]
        fn from(obj: JsValue) -> MediaList {
            MediaList { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MediaList {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MediaList> for MediaList {
        #[inline]
        fn as_ref(&self) -> &MediaList {
            self
        }
    }
    impl From<MediaList> for JsValue {
        #[inline]
        fn from(obj: MediaList) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MediaList {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MediaList(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MediaList(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MediaList(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MediaList { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MediaList) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MediaList> for ::js_sys::Object {
    #[inline]
    fn from(obj: MediaList) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MediaList {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MediaList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_medium_MediaList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaList as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaList {
    #[cfg(all(feature = "MediaList",))]
    #[allow(bad_style)]
    #[doc = "The `appendMedium()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaList/appendMedium)\n\n*This API requires the following crate features to be activated: `MediaList`*"]
    #[allow(clippy::all)]
    pub fn append_medium(&self, new_medium: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_medium_MediaList(
                self_: <&MediaList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                new_medium: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_medium_MediaList(
            self_: <&MediaList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            new_medium: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(new_medium);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let new_medium = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(new_medium);
                __widl_f_append_medium_MediaList(self_, new_medium)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "MediaList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_medium_MediaList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaList as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaList {
    #[cfg(all(feature = "MediaList",))]
    #[allow(bad_style)]
    #[doc = "The `deleteMedium()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaList/deleteMedium)\n\n*This API requires the following crate features to be activated: `MediaList`*"]
    #[allow(clippy::all)]
    pub fn delete_medium(&self, old_medium: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_medium_MediaList(
                self_: <&MediaList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                old_medium: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_medium_MediaList(
            self_: <&MediaList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            old_medium: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(old_medium);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let old_medium = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(old_medium);
                __widl_f_delete_medium_MediaList(self_, old_medium)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "MediaList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_item_MediaList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaList as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl MediaList {
    #[cfg(all(feature = "MediaList",))]
    #[allow(bad_style)]
    #[doc = "The `item()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaList/item)\n\n*This API requires the following crate features to be activated: `MediaList`*"]
    #[allow(clippy::all)]
    pub fn item(&self, index: u32) -> Option<String> {
        #[cfg(all(feature = "MediaList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_item_MediaList(
                self_: <&MediaList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_item_MediaList(
            self_: <&MediaList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_item_MediaList(self_, index)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_MediaList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaList as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl MediaList {
    #[cfg(all(feature = "MediaList",))]
    #[allow(bad_style)]
    #[doc = "The indexing getter\n\n\n\n*This API requires the following crate features to be activated: `MediaList`*"]
    #[allow(clippy::all)]
    pub fn get(&self, index: u32) -> Option<String> {
        #[cfg(all(feature = "MediaList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_MediaList(
                self_: <&MediaList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_MediaList(
            self_: <&MediaList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_get_MediaList(self_, index)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_media_text_MediaList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaList as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl MediaList {
    #[cfg(all(feature = "MediaList",))]
    #[allow(bad_style)]
    #[doc = "The `mediaText` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaList/mediaText)\n\n*This API requires the following crate features to be activated: `MediaList`*"]
    #[allow(clippy::all)]
    pub fn media_text(&self) -> String {
        #[cfg(all(feature = "MediaList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_media_text_MediaList(
                self_: <&MediaList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_media_text_MediaList(
            self_: <&MediaList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_media_text_MediaList(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_media_text_MediaList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaList as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaList {
    #[cfg(all(feature = "MediaList",))]
    #[allow(bad_style)]
    #[doc = "The `mediaText` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaList/mediaText)\n\n*This API requires the following crate features to be activated: `MediaList`*"]
    #[allow(clippy::all)]
    pub fn set_media_text(&self, media_text: &str) {
        #[cfg(all(feature = "MediaList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_media_text_MediaList(
                self_: <&MediaList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                media_text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_media_text_MediaList(
            self_: <&MediaList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            media_text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(media_text);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let media_text = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(media_text);
                __widl_f_set_media_text_MediaList(self_, media_text)
            };
            ()
        }
    }
}
#[cfg(all(feature = "MediaList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_length_MediaList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaList as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl MediaList {
    #[cfg(all(feature = "MediaList",))]
    #[allow(bad_style)]
    #[doc = "The `length` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaList/length)\n\n*This API requires the following crate features to be activated: `MediaList`*"]
    #[allow(clippy::all)]
    pub fn length(&self) -> u32 {
        #[cfg(all(feature = "MediaList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_length_MediaList(
                self_: <&MediaList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_length_MediaList(
            self_: <&MediaList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_length_MediaList(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_11717968febff099: [u8; 680usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}f\x02\0\0\0\0\x08\0\0\x02\tMediaList\x1B__widl_instanceof_MediaList\0\0\0\0 __widl_f_append_medium_MediaList\x01\0\0\x01\tMediaList\x01\0\0\x01\x02\x05self_\nnew_medium\x0CappendMedium\0\0\0 __widl_f_delete_medium_MediaList\x01\0\0\x01\tMediaList\x01\0\0\x01\x02\x05self_\nold_medium\x0CdeleteMedium\0\0\0\x17__widl_f_item_MediaList\0\0\0\x01\tMediaList\x01\0\0\x01\x02\x05self_\x05index\x04item\0\0\0\x16__widl_f_get_MediaList\0\0\0\x01\tMediaList\x01\0\x03\x01\x02\x05self_\x05index\x03get\0\0\0\x1D__widl_f_media_text_MediaList\0\0\0\x01\tMediaList\x01\0\x01\tmediaText\x01\x01\x05self_\tmediaText\0\0\0!__widl_f_set_media_text_MediaList\0\0\0\x01\tMediaList\x01\0\x02\tmediaText\x01\x02\x05self_\nmedia_text\tmediaText\0\0\0\x19__widl_f_length_MediaList\0\0\0\x01\tMediaList\x01\0\x01\x06length\x01\x01\x05self_\x06length\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
