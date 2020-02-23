use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `TreeWalker` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker)\n\n*This API requires the following crate features to be activated: `TreeWalker`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct TreeWalker {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_TreeWalker: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for TreeWalker {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(10u32);
            inform(84u32);
            inform(114u32);
            inform(101u32);
            inform(101u32);
            inform(87u32);
            inform(97u32);
            inform(108u32);
            inform(107u32);
            inform(101u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for TreeWalker {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for TreeWalker {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for TreeWalker {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a TreeWalker {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for TreeWalker {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            TreeWalker {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for TreeWalker {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a TreeWalker {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for TreeWalker {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<TreeWalker>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(TreeWalker {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for TreeWalker {
        #[inline]
        fn from(obj: JsValue) -> TreeWalker {
            TreeWalker { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for TreeWalker {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<TreeWalker> for TreeWalker {
        #[inline]
        fn as_ref(&self) -> &TreeWalker {
            self
        }
    }
    impl From<TreeWalker> for JsValue {
        #[inline]
        fn from(obj: TreeWalker) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for TreeWalker {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_TreeWalker(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_TreeWalker(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_TreeWalker(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            TreeWalker { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const TreeWalker) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<TreeWalker> for ::js_sys::Object {
    #[inline]
    fn from(obj: TreeWalker) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for TreeWalker {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Node", feature = "TreeWalker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_first_child_TreeWalker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TreeWalker as WasmDescribe>::describe();
    <Option<Node> as WasmDescribe>::describe();
}
impl TreeWalker {
    #[cfg(all(feature = "Node", feature = "TreeWalker",))]
    #[allow(bad_style)]
    #[doc = "The `firstChild()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/firstChild)\n\n*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
    #[allow(clippy::all)]
    pub fn first_child(&self) -> Result<Option<Node>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "TreeWalker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_first_child_TreeWalker(
                self_: <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_first_child_TreeWalker(
            self_: <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_first_child_TreeWalker(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Node> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Node", feature = "TreeWalker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_last_child_TreeWalker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TreeWalker as WasmDescribe>::describe();
    <Option<Node> as WasmDescribe>::describe();
}
impl TreeWalker {
    #[cfg(all(feature = "Node", feature = "TreeWalker",))]
    #[allow(bad_style)]
    #[doc = "The `lastChild()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/lastChild)\n\n*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
    #[allow(clippy::all)]
    pub fn last_child(&self) -> Result<Option<Node>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "TreeWalker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_last_child_TreeWalker(
                self_: <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_last_child_TreeWalker(
            self_: <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_last_child_TreeWalker(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Node> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Node", feature = "TreeWalker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_next_node_TreeWalker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TreeWalker as WasmDescribe>::describe();
    <Option<Node> as WasmDescribe>::describe();
}
impl TreeWalker {
    #[cfg(all(feature = "Node", feature = "TreeWalker",))]
    #[allow(bad_style)]
    #[doc = "The `nextNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/nextNode)\n\n*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
    #[allow(clippy::all)]
    pub fn next_node(&self) -> Result<Option<Node>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "TreeWalker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_next_node_TreeWalker(
                self_: <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_next_node_TreeWalker(
            self_: <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_next_node_TreeWalker(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Node> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Node", feature = "TreeWalker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_next_sibling_TreeWalker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TreeWalker as WasmDescribe>::describe();
    <Option<Node> as WasmDescribe>::describe();
}
impl TreeWalker {
    #[cfg(all(feature = "Node", feature = "TreeWalker",))]
    #[allow(bad_style)]
    #[doc = "The `nextSibling()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/nextSibling)\n\n*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
    #[allow(clippy::all)]
    pub fn next_sibling(&self) -> Result<Option<Node>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "TreeWalker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_next_sibling_TreeWalker(
                self_: <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_next_sibling_TreeWalker(
            self_: <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_next_sibling_TreeWalker(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Node> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Node", feature = "TreeWalker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_parent_node_TreeWalker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TreeWalker as WasmDescribe>::describe();
    <Option<Node> as WasmDescribe>::describe();
}
impl TreeWalker {
    #[cfg(all(feature = "Node", feature = "TreeWalker",))]
    #[allow(bad_style)]
    #[doc = "The `parentNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/parentNode)\n\n*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
    #[allow(clippy::all)]
    pub fn parent_node(&self) -> Result<Option<Node>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "TreeWalker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_parent_node_TreeWalker(
                self_: <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_parent_node_TreeWalker(
            self_: <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_parent_node_TreeWalker(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Node> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Node", feature = "TreeWalker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_previous_node_TreeWalker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TreeWalker as WasmDescribe>::describe();
    <Option<Node> as WasmDescribe>::describe();
}
impl TreeWalker {
    #[cfg(all(feature = "Node", feature = "TreeWalker",))]
    #[allow(bad_style)]
    #[doc = "The `previousNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/previousNode)\n\n*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
    #[allow(clippy::all)]
    pub fn previous_node(&self) -> Result<Option<Node>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "TreeWalker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_previous_node_TreeWalker(
                self_: <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_previous_node_TreeWalker(
            self_: <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_previous_node_TreeWalker(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Node> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Node", feature = "TreeWalker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_previous_sibling_TreeWalker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TreeWalker as WasmDescribe>::describe();
    <Option<Node> as WasmDescribe>::describe();
}
impl TreeWalker {
    #[cfg(all(feature = "Node", feature = "TreeWalker",))]
    #[allow(bad_style)]
    #[doc = "The `previousSibling()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/previousSibling)\n\n*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
    #[allow(clippy::all)]
    pub fn previous_sibling(&self) -> Result<Option<Node>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "TreeWalker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_previous_sibling_TreeWalker(
                self_: <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_previous_sibling_TreeWalker(
            self_: <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_previous_sibling_TreeWalker(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Node> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Node", feature = "TreeWalker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_root_TreeWalker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TreeWalker as WasmDescribe>::describe();
    <Node as WasmDescribe>::describe();
}
impl TreeWalker {
    #[cfg(all(feature = "Node", feature = "TreeWalker",))]
    #[allow(bad_style)]
    #[doc = "The `root` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/root)\n\n*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
    #[allow(clippy::all)]
    pub fn root(&self) -> Node {
        #[cfg(all(feature = "Node", feature = "TreeWalker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_root_TreeWalker(
                self_: <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_root_TreeWalker(
            self_: <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_root_TreeWalker(self_)
            };
            <Node as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TreeWalker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_what_to_show_TreeWalker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TreeWalker as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl TreeWalker {
    #[cfg(all(feature = "TreeWalker",))]
    #[allow(bad_style)]
    #[doc = "The `whatToShow` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/whatToShow)\n\n*This API requires the following crate features to be activated: `TreeWalker`*"]
    #[allow(clippy::all)]
    pub fn what_to_show(&self) -> u32 {
        #[cfg(all(feature = "TreeWalker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_what_to_show_TreeWalker(
                self_: <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_what_to_show_TreeWalker(
            self_: <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_what_to_show_TreeWalker(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "NodeFilter", feature = "TreeWalker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_filter_TreeWalker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TreeWalker as WasmDescribe>::describe();
    <Option<NodeFilter> as WasmDescribe>::describe();
}
impl TreeWalker {
    #[cfg(all(feature = "NodeFilter", feature = "TreeWalker",))]
    #[allow(bad_style)]
    #[doc = "The `filter` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/filter)\n\n*This API requires the following crate features to be activated: `NodeFilter`, `TreeWalker`*"]
    #[allow(clippy::all)]
    pub fn filter(&self) -> Option<NodeFilter> {
        #[cfg(all(feature = "NodeFilter", feature = "TreeWalker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_filter_TreeWalker(
                self_: <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<NodeFilter> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_filter_TreeWalker(
            self_: <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<NodeFilter> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_filter_TreeWalker(self_)
            };
            <Option<NodeFilter> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Node", feature = "TreeWalker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_current_node_TreeWalker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TreeWalker as WasmDescribe>::describe();
    <Node as WasmDescribe>::describe();
}
impl TreeWalker {
    #[cfg(all(feature = "Node", feature = "TreeWalker",))]
    #[allow(bad_style)]
    #[doc = "The `currentNode` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/currentNode)\n\n*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
    #[allow(clippy::all)]
    pub fn current_node(&self) -> Node {
        #[cfg(all(feature = "Node", feature = "TreeWalker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_current_node_TreeWalker(
                self_: <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_current_node_TreeWalker(
            self_: <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_current_node_TreeWalker(self_)
            };
            <Node as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Node", feature = "TreeWalker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_current_node_TreeWalker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TreeWalker as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TreeWalker {
    #[cfg(all(feature = "Node", feature = "TreeWalker",))]
    #[allow(bad_style)]
    #[doc = "The `currentNode` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/currentNode)\n\n*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
    #[allow(clippy::all)]
    pub fn set_current_node(&self, current_node: &Node) {
        #[cfg(all(feature = "Node", feature = "TreeWalker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_current_node_TreeWalker(
                self_: <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                current_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_current_node_TreeWalker(
            self_: <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            current_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(current_node);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeWalker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let current_node =
                    <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(current_node);
                __widl_f_set_current_node_TreeWalker(self_, current_node)
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
pub static __WASM_BINDGEN_GENERATED_f4bcd1b3f74e1bda: [u8; 1070usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xEC\x03\0\0\0\0\r\0\0\x02\nTreeWalker\x1C__widl_instanceof_TreeWalker\0\0\0\0\x1F__widl_f_first_child_TreeWalker\x01\0\0\x01\nTreeWalker\x01\0\0\x01\x01\x05self_\nfirstChild\0\0\0\x1E__widl_f_last_child_TreeWalker\x01\0\0\x01\nTreeWalker\x01\0\0\x01\x01\x05self_\tlastChild\0\0\0\x1D__widl_f_next_node_TreeWalker\x01\0\0\x01\nTreeWalker\x01\0\0\x01\x01\x05self_\x08nextNode\0\0\0 __widl_f_next_sibling_TreeWalker\x01\0\0\x01\nTreeWalker\x01\0\0\x01\x01\x05self_\x0BnextSibling\0\0\0\x1F__widl_f_parent_node_TreeWalker\x01\0\0\x01\nTreeWalker\x01\0\0\x01\x01\x05self_\nparentNode\0\0\0!__widl_f_previous_node_TreeWalker\x01\0\0\x01\nTreeWalker\x01\0\0\x01\x01\x05self_\x0CpreviousNode\0\0\0$__widl_f_previous_sibling_TreeWalker\x01\0\0\x01\nTreeWalker\x01\0\0\x01\x01\x05self_\x0FpreviousSibling\0\0\0\x18__widl_f_root_TreeWalker\0\0\0\x01\nTreeWalker\x01\0\x01\x04root\x01\x01\x05self_\x04root\0\0\0 __widl_f_what_to_show_TreeWalker\0\0\0\x01\nTreeWalker\x01\0\x01\nwhatToShow\x01\x01\x05self_\nwhatToShow\0\0\0\x1A__widl_f_filter_TreeWalker\0\0\0\x01\nTreeWalker\x01\0\x01\x06filter\x01\x01\x05self_\x06filter\0\0\0 __widl_f_current_node_TreeWalker\0\0\0\x01\nTreeWalker\x01\0\x01\x0BcurrentNode\x01\x01\x05self_\x0BcurrentNode\0\0\0$__widl_f_set_current_node_TreeWalker\0\0\0\x01\nTreeWalker\x01\0\x02\x0BcurrentNode\x01\x02\x05self_\x0Ccurrent_node\x0BcurrentNode\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
