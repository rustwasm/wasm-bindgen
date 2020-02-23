use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `CharacterData` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct CharacterData {
    obj: Node,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CharacterData: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for CharacterData {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
            inform(67u32);
            inform(104u32);
            inform(97u32);
            inform(114u32);
            inform(97u32);
            inform(99u32);
            inform(116u32);
            inform(101u32);
            inform(114u32);
            inform(68u32);
            inform(97u32);
            inform(116u32);
            inform(97u32);
        }
    }
    impl core::ops::Deref for CharacterData {
        type Target = Node;
        #[inline]
        fn deref(&self) -> &Node {
            &self.obj
        }
    }
    impl IntoWasmAbi for CharacterData {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for CharacterData {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CharacterData {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for CharacterData {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            CharacterData {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for CharacterData {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a CharacterData {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for CharacterData {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<CharacterData>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(CharacterData {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for CharacterData {
        #[inline]
        fn from(obj: JsValue) -> CharacterData {
            CharacterData { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for CharacterData {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<CharacterData> for CharacterData {
        #[inline]
        fn as_ref(&self) -> &CharacterData {
            self
        }
    }
    impl From<CharacterData> for JsValue {
        #[inline]
        fn from(obj: CharacterData) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for CharacterData {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_CharacterData(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_CharacterData(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_CharacterData(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CharacterData { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CharacterData) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<CharacterData> for Node {
    #[inline]
    fn from(obj: CharacterData) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for CharacterData {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<CharacterData> for EventTarget {
    #[inline]
    fn from(obj: CharacterData) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for CharacterData {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<CharacterData> for ::js_sys::Object {
    #[inline]
    fn from(obj: CharacterData) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for CharacterData {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_data_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CharacterData as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `appendData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/appendData)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn append_data(&self, data: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_data_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_data_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_append_data_CharacterData(self_, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_data_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CharacterData as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `deleteData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/deleteData)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn delete_data(&self, offset: u32, count: u32) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_data_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                offset: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                count: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_data_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            offset: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            count: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(offset);
            drop(count);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let offset = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(offset);
                let count = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(count);
                __widl_f_delete_data_CharacterData(self_, offset, count)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_insert_data_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CharacterData as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `insertData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/insertData)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn insert_data(&self, offset: u32, data: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_insert_data_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                offset: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_insert_data_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            offset: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(offset);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let offset = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(offset);
                let data = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_insert_data_CharacterData(self_, offset, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_data_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&CharacterData as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `replaceData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceData)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn replace_data(
        &self,
        offset: u32,
        count: u32,
        data: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_data_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                offset: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                count: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_data_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            offset: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            count: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(offset);
            drop(count);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let offset = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(offset);
                let count = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(count);
                let data = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_replace_data_CharacterData(self_, offset, count, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_substring_data_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CharacterData as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `substringData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/substringData)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn substring_data(
        &self,
        offset: u32,
        count: u32,
    ) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_substring_data_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                offset: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                count: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_substring_data_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            offset: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            count: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(offset);
            drop(count);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let offset = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(offset);
                let count = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(count);
                __widl_f_substring_data_CharacterData(self_, offset, count)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_data_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CharacterData as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `data` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/data)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn data(&self) -> String {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_data_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_data_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_data_CharacterData(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_data_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CharacterData as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `data` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/data)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn set_data(&self, data: &str) {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_data_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_data_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_set_data_CharacterData(self_, data)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_length_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CharacterData as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `length` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/length)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn length(&self) -> u32 {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_length_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_length_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_length_CharacterData(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_node_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CharacterData as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn after_with_node(&self, nodes: &::js_sys::Array) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_node_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_node_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes);
                __widl_f_after_with_node_CharacterData(self_, nodes)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_node_0_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CharacterData as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn after_with_node_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_node_0_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_node_0_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_after_with_node_0_CharacterData(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_node_1_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CharacterData as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    #[allow(clippy::all)]
    pub fn after_with_node_1(&self, nodes_1: &Node) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_node_1_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_node_1_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                __widl_f_after_with_node_1_CharacterData(self_, nodes_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_node_2_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CharacterData as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    #[allow(clippy::all)]
    pub fn after_with_node_2(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_node_2_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_node_2_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                __widl_f_after_with_node_2_CharacterData(self_, nodes_1, nodes_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_node_3_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&CharacterData as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    #[allow(clippy::all)]
    pub fn after_with_node_3(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_node_3_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_node_3_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                __widl_f_after_with_node_3_CharacterData(self_, nodes_1, nodes_2, nodes_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_node_4_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&CharacterData as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    #[allow(clippy::all)]
    pub fn after_with_node_4(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_node_4_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_node_4_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                __widl_f_after_with_node_4_CharacterData(self_, nodes_1, nodes_2, nodes_3, nodes_4)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_node_5_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&CharacterData as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    #[allow(clippy::all)]
    pub fn after_with_node_5(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_node_5_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_node_5_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            drop(nodes_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                __widl_f_after_with_node_5_CharacterData(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_node_6_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&CharacterData as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    #[allow(clippy::all)]
    pub fn after_with_node_6(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_node_6_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_node_6_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_6: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            drop(nodes_5);
            drop(nodes_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                __widl_f_after_with_node_6_CharacterData(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_node_7_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&CharacterData as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    #[allow(clippy::all)]
    pub fn after_with_node_7(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_node_7_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_7: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_node_7_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_6: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_7: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            drop(nodes_5);
            drop(nodes_6);
            drop(nodes_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                let nodes_7 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_7);
                __widl_f_after_with_node_7_CharacterData(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6, nodes_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_str_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CharacterData as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn after_with_str(&self, nodes: &::js_sys::Array) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_str_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_str_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes);
                __widl_f_after_with_str_CharacterData(self_, nodes)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_str_0_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CharacterData as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn after_with_str_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_str_0_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_str_0_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_after_with_str_0_CharacterData(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_str_1_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CharacterData as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn after_with_str_1(&self, nodes_1: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_str_1_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_str_1_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                __widl_f_after_with_str_1_CharacterData(self_, nodes_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_str_2_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CharacterData as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn after_with_str_2(
        &self,
        nodes_1: &str,
        nodes_2: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_str_2_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_str_2_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                __widl_f_after_with_str_2_CharacterData(self_, nodes_1, nodes_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_str_3_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&CharacterData as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn after_with_str_3(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_str_3_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_str_3_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                __widl_f_after_with_str_3_CharacterData(self_, nodes_1, nodes_2, nodes_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_str_4_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&CharacterData as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn after_with_str_4(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_str_4_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_str_4_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                __widl_f_after_with_str_4_CharacterData(self_, nodes_1, nodes_2, nodes_3, nodes_4)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_str_5_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&CharacterData as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn after_with_str_5(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_str_5_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_str_5_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            drop(nodes_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                __widl_f_after_with_str_5_CharacterData(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_str_6_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&CharacterData as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn after_with_str_6(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_str_6_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_str_6_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            drop(nodes_5);
            drop(nodes_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                __widl_f_after_with_str_6_CharacterData(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_str_7_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&CharacterData as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn after_with_str_7(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_str_7_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_7: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_str_7_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_7: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            drop(nodes_5);
            drop(nodes_6);
            drop(nodes_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                let nodes_7 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_7);
                __widl_f_after_with_str_7_CharacterData(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6, nodes_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_node_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CharacterData as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn before_with_node(&self, nodes: &::js_sys::Array) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_node_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_node_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes);
                __widl_f_before_with_node_CharacterData(self_, nodes)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_node_0_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CharacterData as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn before_with_node_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_node_0_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_node_0_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_before_with_node_0_CharacterData(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_node_1_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CharacterData as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    #[allow(clippy::all)]
    pub fn before_with_node_1(&self, nodes_1: &Node) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_node_1_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_node_1_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                __widl_f_before_with_node_1_CharacterData(self_, nodes_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_node_2_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CharacterData as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    #[allow(clippy::all)]
    pub fn before_with_node_2(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_node_2_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_node_2_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                __widl_f_before_with_node_2_CharacterData(self_, nodes_1, nodes_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_node_3_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&CharacterData as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    #[allow(clippy::all)]
    pub fn before_with_node_3(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_node_3_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_node_3_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                __widl_f_before_with_node_3_CharacterData(self_, nodes_1, nodes_2, nodes_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_node_4_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&CharacterData as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    #[allow(clippy::all)]
    pub fn before_with_node_4(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_node_4_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_node_4_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                __widl_f_before_with_node_4_CharacterData(self_, nodes_1, nodes_2, nodes_3, nodes_4)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_node_5_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&CharacterData as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    #[allow(clippy::all)]
    pub fn before_with_node_5(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_node_5_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_node_5_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            drop(nodes_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                __widl_f_before_with_node_5_CharacterData(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_node_6_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&CharacterData as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    #[allow(clippy::all)]
    pub fn before_with_node_6(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_node_6_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_node_6_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_6: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            drop(nodes_5);
            drop(nodes_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                __widl_f_before_with_node_6_CharacterData(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_node_7_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&CharacterData as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    #[allow(clippy::all)]
    pub fn before_with_node_7(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_node_7_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_7: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_node_7_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_6: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_7: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            drop(nodes_5);
            drop(nodes_6);
            drop(nodes_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                let nodes_7 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_7);
                __widl_f_before_with_node_7_CharacterData(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6, nodes_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_str_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CharacterData as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn before_with_str(&self, nodes: &::js_sys::Array) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_str_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_str_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes);
                __widl_f_before_with_str_CharacterData(self_, nodes)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_str_0_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CharacterData as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn before_with_str_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_str_0_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_str_0_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_before_with_str_0_CharacterData(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_str_1_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CharacterData as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn before_with_str_1(&self, nodes_1: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_str_1_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_str_1_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                __widl_f_before_with_str_1_CharacterData(self_, nodes_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_str_2_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CharacterData as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn before_with_str_2(
        &self,
        nodes_1: &str,
        nodes_2: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_str_2_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_str_2_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                __widl_f_before_with_str_2_CharacterData(self_, nodes_1, nodes_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_str_3_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&CharacterData as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn before_with_str_3(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_str_3_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_str_3_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                __widl_f_before_with_str_3_CharacterData(self_, nodes_1, nodes_2, nodes_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_str_4_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&CharacterData as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn before_with_str_4(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_str_4_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_str_4_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                __widl_f_before_with_str_4_CharacterData(self_, nodes_1, nodes_2, nodes_3, nodes_4)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_str_5_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&CharacterData as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn before_with_str_5(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_str_5_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_str_5_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            drop(nodes_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                __widl_f_before_with_str_5_CharacterData(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_str_6_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&CharacterData as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn before_with_str_6(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_str_6_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_str_6_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            drop(nodes_5);
            drop(nodes_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                __widl_f_before_with_str_6_CharacterData(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_str_7_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&CharacterData as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn before_with_str_7(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_str_7_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_7: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_str_7_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_7: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            drop(nodes_5);
            drop(nodes_6);
            drop(nodes_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                let nodes_7 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_7);
                __widl_f_before_with_str_7_CharacterData(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6, nodes_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CharacterData as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `remove()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/remove)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn remove(&self) {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_remove_CharacterData(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_node_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CharacterData as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_node(
        &self,
        nodes: &::js_sys::Array,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_node_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_node_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes);
                __widl_f_replace_with_with_node_CharacterData(self_, nodes)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_node_0_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CharacterData as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_node_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_node_0_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_node_0_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_replace_with_with_node_0_CharacterData(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_node_1_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CharacterData as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_node_1(&self, nodes_1: &Node) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_node_1_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_node_1_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                __widl_f_replace_with_with_node_1_CharacterData(self_, nodes_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_node_2_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CharacterData as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_node_2(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_node_2_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_node_2_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                __widl_f_replace_with_with_node_2_CharacterData(self_, nodes_1, nodes_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_node_3_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&CharacterData as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_node_3(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_node_3_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_node_3_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                __widl_f_replace_with_with_node_3_CharacterData(self_, nodes_1, nodes_2, nodes_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_node_4_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&CharacterData as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_node_4(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_node_4_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_node_4_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                __widl_f_replace_with_with_node_4_CharacterData(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_node_5_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&CharacterData as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_node_5(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_node_5_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_node_5_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            drop(nodes_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                __widl_f_replace_with_with_node_5_CharacterData(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_node_6_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&CharacterData as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_node_6(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_node_6_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_node_6_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_6: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            drop(nodes_5);
            drop(nodes_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                __widl_f_replace_with_with_node_6_CharacterData(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_node_7_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&CharacterData as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_node_7(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_node_7_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_7: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_node_7_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_6: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_7: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            drop(nodes_5);
            drop(nodes_6);
            drop(nodes_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                let nodes_7 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_7);
                __widl_f_replace_with_with_node_7_CharacterData(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6, nodes_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_str_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CharacterData as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_str(
        &self,
        nodes: &::js_sys::Array,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_str_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_str_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes);
                __widl_f_replace_with_with_str_CharacterData(self_, nodes)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_str_0_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CharacterData as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_str_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_str_0_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_str_0_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_replace_with_with_str_0_CharacterData(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_str_1_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CharacterData as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_str_1(&self, nodes_1: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_str_1_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_str_1_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                __widl_f_replace_with_with_str_1_CharacterData(self_, nodes_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_str_2_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CharacterData as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_str_2(
        &self,
        nodes_1: &str,
        nodes_2: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_str_2_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_str_2_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                __widl_f_replace_with_with_str_2_CharacterData(self_, nodes_1, nodes_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_str_3_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&CharacterData as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_str_3(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_str_3_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_str_3_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                __widl_f_replace_with_with_str_3_CharacterData(self_, nodes_1, nodes_2, nodes_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_str_4_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&CharacterData as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_str_4(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_str_4_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_str_4_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                __widl_f_replace_with_with_str_4_CharacterData(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_str_5_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&CharacterData as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_str_5(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_str_5_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_str_5_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            drop(nodes_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                __widl_f_replace_with_with_str_5_CharacterData(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_str_6_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&CharacterData as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_str_6(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_str_6_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_str_6_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            drop(nodes_5);
            drop(nodes_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                __widl_f_replace_with_with_str_6_CharacterData(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_str_7_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&CharacterData as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_str_7(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CharacterData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_str_7_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_7: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_str_7_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_7: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            drop(nodes_5);
            drop(nodes_6);
            drop(nodes_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                let nodes_7 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_7);
                __widl_f_replace_with_with_str_7_CharacterData(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6, nodes_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CharacterData", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_previous_element_sibling_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CharacterData as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `previousElementSibling` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/previousElementSibling)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Element`*"]
    #[allow(clippy::all)]
    pub fn previous_element_sibling(&self) -> Option<Element> {
        #[cfg(all(feature = "CharacterData", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_previous_element_sibling_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_previous_element_sibling_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_previous_element_sibling_CharacterData(self_)
            };
            <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CharacterData", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_next_element_sibling_CharacterData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CharacterData as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl CharacterData {
    #[cfg(all(feature = "CharacterData", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `nextElementSibling` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/nextElementSibling)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Element`*"]
    #[allow(clippy::all)]
    pub fn next_element_sibling(&self) -> Option<Element> {
        #[cfg(all(feature = "CharacterData", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_next_element_sibling_CharacterData(
                self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_next_element_sibling_CharacterData(
            self_: <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CharacterData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_next_element_sibling_CharacterData(self_)
            };
            <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_c86d549ae8b32f83: [u8; 7025usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}/\x1B\0\0\0\0B\0\0\x02\rCharacterData\x1F__widl_instanceof_CharacterData\0\0\0\0\"__widl_f_append_data_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x02\x05self_\x04data\nappendData\0\0\0\"__widl_f_delete_data_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x03\x05self_\x06offset\x05count\ndeleteData\0\0\0\"__widl_f_insert_data_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x03\x05self_\x06offset\x04data\ninsertData\0\0\0#__widl_f_replace_data_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x04\x05self_\x06offset\x05count\x04data\x0BreplaceData\0\0\0%__widl_f_substring_data_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x03\x05self_\x06offset\x05count\rsubstringData\0\0\0\x1B__widl_f_data_CharacterData\0\0\0\x01\rCharacterData\x01\0\x01\x04data\x01\x01\x05self_\x04data\0\0\0\x1F__widl_f_set_data_CharacterData\0\0\0\x01\rCharacterData\x01\0\x02\x04data\x01\x02\x05self_\x04data\x04data\0\0\0\x1D__widl_f_length_CharacterData\0\0\0\x01\rCharacterData\x01\0\x01\x06length\x01\x01\x05self_\x06length\0\0\0&__widl_f_after_with_node_CharacterData\x01\x01\0\x01\rCharacterData\x01\0\0\x01\x02\x05self_\x05nodes\x05after\0\0\0(__widl_f_after_with_node_0_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x01\x05self_\x05after\0\0\0(__widl_f_after_with_node_1_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x02\x05self_\x07nodes_1\x05after\0\0\0(__widl_f_after_with_node_2_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x03\x05self_\x07nodes_1\x07nodes_2\x05after\0\0\0(__widl_f_after_with_node_3_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x04\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x05after\0\0\0(__widl_f_after_with_node_4_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x05\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x05after\0\0\0(__widl_f_after_with_node_5_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x06\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x05after\0\0\0(__widl_f_after_with_node_6_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x07\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x05after\0\0\0(__widl_f_after_with_node_7_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x08\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07nodes_7\x05after\0\0\0%__widl_f_after_with_str_CharacterData\x01\x01\0\x01\rCharacterData\x01\0\0\x01\x02\x05self_\x05nodes\x05after\0\0\0'__widl_f_after_with_str_0_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x01\x05self_\x05after\0\0\0'__widl_f_after_with_str_1_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x02\x05self_\x07nodes_1\x05after\0\0\0'__widl_f_after_with_str_2_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x03\x05self_\x07nodes_1\x07nodes_2\x05after\0\0\0'__widl_f_after_with_str_3_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x04\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x05after\0\0\0'__widl_f_after_with_str_4_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x05\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x05after\0\0\0'__widl_f_after_with_str_5_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x06\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x05after\0\0\0'__widl_f_after_with_str_6_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x07\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x05after\0\0\0'__widl_f_after_with_str_7_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x08\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07nodes_7\x05after\0\0\0'__widl_f_before_with_node_CharacterData\x01\x01\0\x01\rCharacterData\x01\0\0\x01\x02\x05self_\x05nodes\x06before\0\0\0)__widl_f_before_with_node_0_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x01\x05self_\x06before\0\0\0)__widl_f_before_with_node_1_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x02\x05self_\x07nodes_1\x06before\0\0\0)__widl_f_before_with_node_2_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x03\x05self_\x07nodes_1\x07nodes_2\x06before\0\0\0)__widl_f_before_with_node_3_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x04\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x06before\0\0\0)__widl_f_before_with_node_4_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x05\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x06before\0\0\0)__widl_f_before_with_node_5_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x06\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x06before\0\0\0)__widl_f_before_with_node_6_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x07\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x06before\0\0\0)__widl_f_before_with_node_7_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x08\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07nodes_7\x06before\0\0\0&__widl_f_before_with_str_CharacterData\x01\x01\0\x01\rCharacterData\x01\0\0\x01\x02\x05self_\x05nodes\x06before\0\0\0(__widl_f_before_with_str_0_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x01\x05self_\x06before\0\0\0(__widl_f_before_with_str_1_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x02\x05self_\x07nodes_1\x06before\0\0\0(__widl_f_before_with_str_2_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x03\x05self_\x07nodes_1\x07nodes_2\x06before\0\0\0(__widl_f_before_with_str_3_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x04\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x06before\0\0\0(__widl_f_before_with_str_4_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x05\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x06before\0\0\0(__widl_f_before_with_str_5_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x06\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x06before\0\0\0(__widl_f_before_with_str_6_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x07\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x06before\0\0\0(__widl_f_before_with_str_7_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x08\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07nodes_7\x06before\0\0\0\x1D__widl_f_remove_CharacterData\0\0\0\x01\rCharacterData\x01\0\0\x01\x01\x05self_\x06remove\0\0\0-__widl_f_replace_with_with_node_CharacterData\x01\x01\0\x01\rCharacterData\x01\0\0\x01\x02\x05self_\x05nodes\x0BreplaceWith\0\0\0/__widl_f_replace_with_with_node_0_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x01\x05self_\x0BreplaceWith\0\0\0/__widl_f_replace_with_with_node_1_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x02\x05self_\x07nodes_1\x0BreplaceWith\0\0\0/__widl_f_replace_with_with_node_2_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x03\x05self_\x07nodes_1\x07nodes_2\x0BreplaceWith\0\0\0/__widl_f_replace_with_with_node_3_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x04\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x0BreplaceWith\0\0\0/__widl_f_replace_with_with_node_4_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x05\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x0BreplaceWith\0\0\0/__widl_f_replace_with_with_node_5_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x06\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x0BreplaceWith\0\0\0/__widl_f_replace_with_with_node_6_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x07\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x0BreplaceWith\0\0\0/__widl_f_replace_with_with_node_7_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x08\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07nodes_7\x0BreplaceWith\0\0\0,__widl_f_replace_with_with_str_CharacterData\x01\x01\0\x01\rCharacterData\x01\0\0\x01\x02\x05self_\x05nodes\x0BreplaceWith\0\0\0.__widl_f_replace_with_with_str_0_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x01\x05self_\x0BreplaceWith\0\0\0.__widl_f_replace_with_with_str_1_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x02\x05self_\x07nodes_1\x0BreplaceWith\0\0\0.__widl_f_replace_with_with_str_2_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x03\x05self_\x07nodes_1\x07nodes_2\x0BreplaceWith\0\0\0.__widl_f_replace_with_with_str_3_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x04\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x0BreplaceWith\0\0\0.__widl_f_replace_with_with_str_4_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x05\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x0BreplaceWith\0\0\0.__widl_f_replace_with_with_str_5_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x06\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x0BreplaceWith\0\0\0.__widl_f_replace_with_with_str_6_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x07\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x0BreplaceWith\0\0\0.__widl_f_replace_with_with_str_7_CharacterData\x01\0\0\x01\rCharacterData\x01\0\0\x01\x08\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07nodes_7\x0BreplaceWith\0\0\0/__widl_f_previous_element_sibling_CharacterData\0\0\0\x01\rCharacterData\x01\0\x01\x16previousElementSibling\x01\x01\x05self_\x16previousElementSibling\0\0\0+__widl_f_next_element_sibling_CharacterData\0\0\0\x01\rCharacterData\x01\0\x01\x12nextElementSibling\x01\x01\x05self_\x12nextElementSibling\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
