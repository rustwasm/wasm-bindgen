use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Comment` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Comment)\n\n*This API requires the following crate features to be activated: `Comment`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Comment {
    obj: CharacterData,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Comment: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Comment {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(7u32);
            inform(67u32);
            inform(111u32);
            inform(109u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for Comment {
        type Target = CharacterData;
        #[inline]
        fn deref(&self) -> &CharacterData {
            &self.obj
        }
    }
    impl IntoWasmAbi for Comment {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Comment {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Comment {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Comment {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Comment {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Comment {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Comment {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Comment {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Comment>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Comment {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Comment {
        #[inline]
        fn from(obj: JsValue) -> Comment {
            Comment { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Comment {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Comment> for Comment {
        #[inline]
        fn as_ref(&self) -> &Comment {
            self
        }
    }
    impl From<Comment> for JsValue {
        #[inline]
        fn from(obj: Comment) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Comment {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Comment(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Comment(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Comment(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Comment { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Comment) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Comment> for CharacterData {
    #[inline]
    fn from(obj: Comment) -> CharacterData {
        use wasm_bindgen::JsCast;
        CharacterData::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<CharacterData> for Comment {
    #[inline]
    fn as_ref(&self) -> &CharacterData {
        use wasm_bindgen::JsCast;
        CharacterData::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<Comment> for Node {
    #[inline]
    fn from(obj: Comment) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for Comment {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<Comment> for EventTarget {
    #[inline]
    fn from(obj: Comment) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for Comment {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<Comment> for ::js_sys::Object {
    #[inline]
    fn from(obj: Comment) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Comment {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Comment",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_Comment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <Comment as WasmDescribe>::describe();
}
impl Comment {
    #[cfg(all(feature = "Comment",))]
    #[allow(bad_style)]
    #[doc = "The `new Comment(..)` constructor, creating a new instance of `Comment`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Comment/Comment)\n\n*This API requires the following crate features to be activated: `Comment`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<Comment, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Comment",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_Comment() -> <Comment as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_Comment() -> <Comment as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_Comment() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Comment as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Comment",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_data_Comment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <Comment as WasmDescribe>::describe();
}
impl Comment {
    #[cfg(all(feature = "Comment",))]
    #[allow(bad_style)]
    #[doc = "The `new Comment(..)` constructor, creating a new instance of `Comment`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Comment/Comment)\n\n*This API requires the following crate features to be activated: `Comment`*"]
    #[allow(clippy::all)]
    pub fn new_with_data(data: &str) -> Result<Comment, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Comment",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_data_Comment(
                data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Comment as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_data_Comment(
            data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Comment as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_new_with_data_Comment(data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Comment as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_e19ecf56bd67d77e: [u8; 244usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xB2\0\0\0\0\0\x03\0\0\x02\x07Comment\x19__widl_instanceof_Comment\0\0\0\0\x14__widl_f_new_Comment\x01\0\0\x01\x07Comment\0\x01\0\x03new\0\0\0\x1E__widl_f_new_with_data_Comment\x01\0\0\x01\x07Comment\0\x01\x01\x04data\x03new\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
