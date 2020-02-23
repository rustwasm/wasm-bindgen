use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Selection` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection)\n\n*This API requires the following crate features to be activated: `Selection`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Selection {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Selection: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Selection {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(9u32);
            inform(83u32);
            inform(101u32);
            inform(108u32);
            inform(101u32);
            inform(99u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for Selection {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for Selection {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Selection {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Selection {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Selection {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Selection {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Selection {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Selection {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Selection {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Selection>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Selection {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Selection {
        #[inline]
        fn from(obj: JsValue) -> Selection {
            Selection { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Selection {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Selection> for Selection {
        #[inline]
        fn as_ref(&self) -> &Selection {
            self
        }
    }
    impl From<Selection> for JsValue {
        #[inline]
        fn from(obj: Selection) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Selection {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Selection(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Selection(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Selection(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Selection { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Selection) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Selection> for ::js_sys::Object {
    #[inline]
    fn from(obj: Selection) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Selection {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Range", feature = "Selection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_range_Selection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Selection as WasmDescribe>::describe();
    <&Range as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Selection {
    #[cfg(all(feature = "Range", feature = "Selection",))]
    #[allow(bad_style)]
    #[doc = "The `addRange()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/addRange)\n\n*This API requires the following crate features to be activated: `Range`, `Selection`*"]
    #[allow(clippy::all)]
    pub fn add_range(&self, range: &Range) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Range", feature = "Selection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_range_Selection(
                self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                range: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_range_Selection(
            self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            range: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(range);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Selection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let range = <&Range as wasm_bindgen::convert::IntoWasmAbi>::into_abi(range);
                __widl_f_add_range_Selection(self_, range)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Node", feature = "Selection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_collapse_Selection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Selection as WasmDescribe>::describe();
    <Option<&Node> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Selection {
    #[cfg(all(feature = "Node", feature = "Selection",))]
    #[allow(bad_style)]
    #[doc = "The `collapse()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/collapse)\n\n*This API requires the following crate features to be activated: `Node`, `Selection`*"]
    #[allow(clippy::all)]
    pub fn collapse(&self, node: Option<&Node>) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "Selection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_collapse_Selection(
                self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                node: <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_collapse_Selection(
            self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            node: <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(node);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Selection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let node = <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(node);
                __widl_f_collapse_Selection(self_, node)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Node", feature = "Selection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_collapse_with_offset_Selection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Selection as WasmDescribe>::describe();
    <Option<&Node> as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Selection {
    #[cfg(all(feature = "Node", feature = "Selection",))]
    #[allow(bad_style)]
    #[doc = "The `collapse()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/collapse)\n\n*This API requires the following crate features to be activated: `Node`, `Selection`*"]
    #[allow(clippy::all)]
    pub fn collapse_with_offset(
        &self,
        node: Option<&Node>,
        offset: u32,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "Selection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_collapse_with_offset_Selection(
                self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                node: <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                offset: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_collapse_with_offset_Selection(
            self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            node: <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            offset: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(node);
            drop(offset);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Selection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let node = <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(node);
                let offset = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(offset);
                __widl_f_collapse_with_offset_Selection(self_, node, offset)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Selection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_collapse_to_end_Selection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Selection as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Selection {
    #[cfg(all(feature = "Selection",))]
    #[allow(bad_style)]
    #[doc = "The `collapseToEnd()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/collapseToEnd)\n\n*This API requires the following crate features to be activated: `Selection`*"]
    #[allow(clippy::all)]
    pub fn collapse_to_end(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Selection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_collapse_to_end_Selection(
                self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_collapse_to_end_Selection(
            self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Selection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_collapse_to_end_Selection(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Selection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_collapse_to_start_Selection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Selection as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Selection {
    #[cfg(all(feature = "Selection",))]
    #[allow(bad_style)]
    #[doc = "The `collapseToStart()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/collapseToStart)\n\n*This API requires the following crate features to be activated: `Selection`*"]
    #[allow(clippy::all)]
    pub fn collapse_to_start(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Selection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_collapse_to_start_Selection(
                self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_collapse_to_start_Selection(
            self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Selection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_collapse_to_start_Selection(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Node", feature = "Selection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_contains_node_Selection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Selection as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Selection {
    #[cfg(all(feature = "Node", feature = "Selection",))]
    #[allow(bad_style)]
    #[doc = "The `containsNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/containsNode)\n\n*This API requires the following crate features to be activated: `Node`, `Selection`*"]
    #[allow(clippy::all)]
    pub fn contains_node(&self, node: &Node) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "Selection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_contains_node_Selection(
                self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_contains_node_Selection(
            self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(node);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Selection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let node = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(node);
                __widl_f_contains_node_Selection(self_, node)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Node", feature = "Selection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_contains_node_with_allow_partial_containment_Selection(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Selection as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Selection {
    #[cfg(all(feature = "Node", feature = "Selection",))]
    #[allow(bad_style)]
    #[doc = "The `containsNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/containsNode)\n\n*This API requires the following crate features to be activated: `Node`, `Selection`*"]
    #[allow(clippy::all)]
    pub fn contains_node_with_allow_partial_containment(
        &self,
        node: &Node,
        allow_partial_containment: bool,
    ) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "Selection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_contains_node_with_allow_partial_containment_Selection(
                self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                allow_partial_containment: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_contains_node_with_allow_partial_containment_Selection(
            self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            allow_partial_containment: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(node);
            drop(allow_partial_containment);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Selection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let node = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(node);
                let allow_partial_containment =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        allow_partial_containment,
                    );
                __widl_f_contains_node_with_allow_partial_containment_Selection(
                    self_,
                    node,
                    allow_partial_containment,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Selection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_from_document_Selection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Selection as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Selection {
    #[cfg(all(feature = "Selection",))]
    #[allow(bad_style)]
    #[doc = "The `deleteFromDocument()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/deleteFromDocument)\n\n*This API requires the following crate features to be activated: `Selection`*"]
    #[allow(clippy::all)]
    pub fn delete_from_document(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Selection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_from_document_Selection(
                self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_from_document_Selection(
            self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Selection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_delete_from_document_Selection(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Selection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_empty_Selection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Selection as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Selection {
    #[cfg(all(feature = "Selection",))]
    #[allow(bad_style)]
    #[doc = "The `empty()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/empty)\n\n*This API requires the following crate features to be activated: `Selection`*"]
    #[allow(clippy::all)]
    pub fn empty(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Selection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_empty_Selection(
                self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_empty_Selection(
            self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Selection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_empty_Selection(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Node", feature = "Selection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_extend_Selection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Selection as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Selection {
    #[cfg(all(feature = "Node", feature = "Selection",))]
    #[allow(bad_style)]
    #[doc = "The `extend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/extend)\n\n*This API requires the following crate features to be activated: `Node`, `Selection`*"]
    #[allow(clippy::all)]
    pub fn extend(&self, node: &Node) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "Selection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_extend_Selection(
                self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_extend_Selection(
            self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(node);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Selection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let node = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(node);
                __widl_f_extend_Selection(self_, node)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Node", feature = "Selection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_extend_with_offset_Selection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Selection as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Selection {
    #[cfg(all(feature = "Node", feature = "Selection",))]
    #[allow(bad_style)]
    #[doc = "The `extend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/extend)\n\n*This API requires the following crate features to be activated: `Node`, `Selection`*"]
    #[allow(clippy::all)]
    pub fn extend_with_offset(
        &self,
        node: &Node,
        offset: u32,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "Selection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_extend_with_offset_Selection(
                self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                offset: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_extend_with_offset_Selection(
            self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            offset: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(node);
            drop(offset);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Selection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let node = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(node);
                let offset = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(offset);
                __widl_f_extend_with_offset_Selection(self_, node, offset)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Range", feature = "Selection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_range_at_Selection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Selection as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Range as WasmDescribe>::describe();
}
impl Selection {
    #[cfg(all(feature = "Range", feature = "Selection",))]
    #[allow(bad_style)]
    #[doc = "The `getRangeAt()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/getRangeAt)\n\n*This API requires the following crate features to be activated: `Range`, `Selection`*"]
    #[allow(clippy::all)]
    pub fn get_range_at(&self, index: u32) -> Result<Range, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Range", feature = "Selection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_range_at_Selection(
                self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Range as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_range_at_Selection(
            self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Range as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Selection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_get_range_at_Selection(self_, index)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Range as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Selection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_modify_Selection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Selection as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Selection {
    #[cfg(all(feature = "Selection",))]
    #[allow(bad_style)]
    #[doc = "The `modify()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/modify)\n\n*This API requires the following crate features to be activated: `Selection`*"]
    #[allow(clippy::all)]
    pub fn modify(
        &self,
        alter: &str,
        direction: &str,
        granularity: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Selection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_modify_Selection(
                self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alter: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                direction: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                granularity: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_modify_Selection(
            self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alter: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            direction: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            granularity: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(alter);
            drop(direction);
            drop(granularity);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Selection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let alter = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alter);
                let direction = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(direction);
                let granularity =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(granularity);
                __widl_f_modify_Selection(self_, alter, direction, granularity)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Selection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_all_ranges_Selection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Selection as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Selection {
    #[cfg(all(feature = "Selection",))]
    #[allow(bad_style)]
    #[doc = "The `removeAllRanges()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/removeAllRanges)\n\n*This API requires the following crate features to be activated: `Selection`*"]
    #[allow(clippy::all)]
    pub fn remove_all_ranges(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Selection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_all_ranges_Selection(
                self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_all_ranges_Selection(
            self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Selection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_remove_all_ranges_Selection(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Range", feature = "Selection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_range_Selection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Selection as WasmDescribe>::describe();
    <&Range as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Selection {
    #[cfg(all(feature = "Range", feature = "Selection",))]
    #[allow(bad_style)]
    #[doc = "The `removeRange()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/removeRange)\n\n*This API requires the following crate features to be activated: `Range`, `Selection`*"]
    #[allow(clippy::all)]
    pub fn remove_range(&self, range: &Range) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Range", feature = "Selection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_range_Selection(
                self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                range: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_range_Selection(
            self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            range: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(range);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Selection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let range = <&Range as wasm_bindgen::convert::IntoWasmAbi>::into_abi(range);
                __widl_f_remove_range_Selection(self_, range)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Node", feature = "Selection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_select_all_children_Selection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Selection as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Selection {
    #[cfg(all(feature = "Node", feature = "Selection",))]
    #[allow(bad_style)]
    #[doc = "The `selectAllChildren()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/selectAllChildren)\n\n*This API requires the following crate features to be activated: `Node`, `Selection`*"]
    #[allow(clippy::all)]
    pub fn select_all_children(&self, node: &Node) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "Selection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_select_all_children_Selection(
                self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_select_all_children_Selection(
            self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(node);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Selection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let node = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(node);
                __widl_f_select_all_children_Selection(self_, node)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Node", feature = "Selection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_base_and_extent_Selection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&Selection as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Selection {
    #[cfg(all(feature = "Node", feature = "Selection",))]
    #[allow(bad_style)]
    #[doc = "The `setBaseAndExtent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/setBaseAndExtent)\n\n*This API requires the following crate features to be activated: `Node`, `Selection`*"]
    #[allow(clippy::all)]
    pub fn set_base_and_extent(
        &self,
        anchor_node: &Node,
        anchor_offset: u32,
        focus_node: &Node,
        focus_offset: u32,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "Selection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_base_and_extent_Selection(
                self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                anchor_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                anchor_offset: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                focus_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                focus_offset: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_base_and_extent_Selection(
            self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            anchor_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            anchor_offset: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            focus_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            focus_offset: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(anchor_node);
            drop(anchor_offset);
            drop(focus_node);
            drop(focus_offset);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Selection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let anchor_node =
                    <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(anchor_node);
                let anchor_offset =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(anchor_offset);
                let focus_node =
                    <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(focus_node);
                let focus_offset =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(focus_offset);
                __widl_f_set_base_and_extent_Selection(
                    self_,
                    anchor_node,
                    anchor_offset,
                    focus_node,
                    focus_offset,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Node", feature = "Selection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_position_Selection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Selection as WasmDescribe>::describe();
    <Option<&Node> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Selection {
    #[cfg(all(feature = "Node", feature = "Selection",))]
    #[allow(bad_style)]
    #[doc = "The `setPosition()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/setPosition)\n\n*This API requires the following crate features to be activated: `Node`, `Selection`*"]
    #[allow(clippy::all)]
    pub fn set_position(&self, node: Option<&Node>) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "Selection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_position_Selection(
                self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                node: <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_position_Selection(
            self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            node: <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(node);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Selection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let node = <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(node);
                __widl_f_set_position_Selection(self_, node)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Node", feature = "Selection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_position_with_offset_Selection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Selection as WasmDescribe>::describe();
    <Option<&Node> as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Selection {
    #[cfg(all(feature = "Node", feature = "Selection",))]
    #[allow(bad_style)]
    #[doc = "The `setPosition()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/setPosition)\n\n*This API requires the following crate features to be activated: `Node`, `Selection`*"]
    #[allow(clippy::all)]
    pub fn set_position_with_offset(
        &self,
        node: Option<&Node>,
        offset: u32,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "Selection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_position_with_offset_Selection(
                self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                node: <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                offset: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_position_with_offset_Selection(
            self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            node: <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            offset: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(node);
            drop(offset);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Selection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let node = <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(node);
                let offset = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(offset);
                __widl_f_set_position_with_offset_Selection(self_, node, offset)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Node", feature = "Selection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_anchor_node_Selection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Selection as WasmDescribe>::describe();
    <Option<Node> as WasmDescribe>::describe();
}
impl Selection {
    #[cfg(all(feature = "Node", feature = "Selection",))]
    #[allow(bad_style)]
    #[doc = "The `anchorNode` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/anchorNode)\n\n*This API requires the following crate features to be activated: `Node`, `Selection`*"]
    #[allow(clippy::all)]
    pub fn anchor_node(&self) -> Option<Node> {
        #[cfg(all(feature = "Node", feature = "Selection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_anchor_node_Selection(
                self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_anchor_node_Selection(
            self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Selection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_anchor_node_Selection(self_)
            };
            <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Selection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_anchor_offset_Selection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Selection as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl Selection {
    #[cfg(all(feature = "Selection",))]
    #[allow(bad_style)]
    #[doc = "The `anchorOffset` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/anchorOffset)\n\n*This API requires the following crate features to be activated: `Selection`*"]
    #[allow(clippy::all)]
    pub fn anchor_offset(&self) -> u32 {
        #[cfg(all(feature = "Selection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_anchor_offset_Selection(
                self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_anchor_offset_Selection(
            self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Selection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_anchor_offset_Selection(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Node", feature = "Selection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_focus_node_Selection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Selection as WasmDescribe>::describe();
    <Option<Node> as WasmDescribe>::describe();
}
impl Selection {
    #[cfg(all(feature = "Node", feature = "Selection",))]
    #[allow(bad_style)]
    #[doc = "The `focusNode` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/focusNode)\n\n*This API requires the following crate features to be activated: `Node`, `Selection`*"]
    #[allow(clippy::all)]
    pub fn focus_node(&self) -> Option<Node> {
        #[cfg(all(feature = "Node", feature = "Selection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_focus_node_Selection(
                self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_focus_node_Selection(
            self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Selection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_focus_node_Selection(self_)
            };
            <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Selection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_focus_offset_Selection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Selection as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl Selection {
    #[cfg(all(feature = "Selection",))]
    #[allow(bad_style)]
    #[doc = "The `focusOffset` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/focusOffset)\n\n*This API requires the following crate features to be activated: `Selection`*"]
    #[allow(clippy::all)]
    pub fn focus_offset(&self) -> u32 {
        #[cfg(all(feature = "Selection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_focus_offset_Selection(
                self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_focus_offset_Selection(
            self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Selection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_focus_offset_Selection(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Selection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_collapsed_Selection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Selection as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Selection {
    #[cfg(all(feature = "Selection",))]
    #[allow(bad_style)]
    #[doc = "The `isCollapsed` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/isCollapsed)\n\n*This API requires the following crate features to be activated: `Selection`*"]
    #[allow(clippy::all)]
    pub fn is_collapsed(&self) -> bool {
        #[cfg(all(feature = "Selection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_collapsed_Selection(
                self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_collapsed_Selection(
            self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Selection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_is_collapsed_Selection(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Selection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_range_count_Selection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Selection as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl Selection {
    #[cfg(all(feature = "Selection",))]
    #[allow(bad_style)]
    #[doc = "The `rangeCount` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/rangeCount)\n\n*This API requires the following crate features to be activated: `Selection`*"]
    #[allow(clippy::all)]
    pub fn range_count(&self) -> u32 {
        #[cfg(all(feature = "Selection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_range_count_Selection(
                self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_range_count_Selection(
            self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Selection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_range_count_Selection(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Selection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_Selection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Selection as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Selection {
    #[cfg(all(feature = "Selection",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/type)\n\n*This API requires the following crate features to be activated: `Selection`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> String {
        #[cfg(all(feature = "Selection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_Selection(
                self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_Selection(
            self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Selection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_Selection(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Selection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_caret_bidi_level_Selection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Selection as WasmDescribe>::describe();
    <Option<i16> as WasmDescribe>::describe();
}
impl Selection {
    #[cfg(all(feature = "Selection",))]
    #[allow(bad_style)]
    #[doc = "The `caretBidiLevel` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/caretBidiLevel)\n\n*This API requires the following crate features to be activated: `Selection`*"]
    #[allow(clippy::all)]
    pub fn caret_bidi_level(&self) -> Result<Option<i16>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Selection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_caret_bidi_level_Selection(
                self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<i16> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_caret_bidi_level_Selection(
            self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<i16> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Selection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_caret_bidi_level_Selection(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<i16> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Selection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_caret_bidi_level_Selection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Selection as WasmDescribe>::describe();
    <Option<i16> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Selection {
    #[cfg(all(feature = "Selection",))]
    #[allow(bad_style)]
    #[doc = "The `caretBidiLevel` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/caretBidiLevel)\n\n*This API requires the following crate features to be activated: `Selection`*"]
    #[allow(clippy::all)]
    pub fn set_caret_bidi_level(
        &self,
        caret_bidi_level: Option<i16>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Selection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_caret_bidi_level_Selection(
                self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                caret_bidi_level: <Option<i16> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_caret_bidi_level_Selection(
            self_: <&Selection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            caret_bidi_level: <Option<i16> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(caret_bidi_level);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Selection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let caret_bidi_level =
                    <Option<i16> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(caret_bidi_level);
                __widl_f_set_caret_bidi_level_Selection(self_, caret_bidi_level)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_d49306c2cd052231: [u8; 2536usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xA6\t\0\0\0\0\x1D\0\0\x02\tSelection\x1B__widl_instanceof_Selection\0\0\0\0\x1C__widl_f_add_range_Selection\x01\0\0\x01\tSelection\x01\0\0\x01\x02\x05self_\x05range\x08addRange\0\0\0\x1B__widl_f_collapse_Selection\x01\0\0\x01\tSelection\x01\0\0\x01\x02\x05self_\x04node\x08collapse\0\0\0'__widl_f_collapse_with_offset_Selection\x01\0\0\x01\tSelection\x01\0\0\x01\x03\x05self_\x04node\x06offset\x08collapse\0\0\0\"__widl_f_collapse_to_end_Selection\x01\0\0\x01\tSelection\x01\0\0\x01\x01\x05self_\rcollapseToEnd\0\0\0$__widl_f_collapse_to_start_Selection\x01\0\0\x01\tSelection\x01\0\0\x01\x01\x05self_\x0FcollapseToStart\0\0\0 __widl_f_contains_node_Selection\x01\0\0\x01\tSelection\x01\0\0\x01\x02\x05self_\x04node\x0CcontainsNode\0\0\0?__widl_f_contains_node_with_allow_partial_containment_Selection\x01\0\0\x01\tSelection\x01\0\0\x01\x03\x05self_\x04node\x19allow_partial_containment\x0CcontainsNode\0\0\0'__widl_f_delete_from_document_Selection\x01\0\0\x01\tSelection\x01\0\0\x01\x01\x05self_\x12deleteFromDocument\0\0\0\x18__widl_f_empty_Selection\x01\0\0\x01\tSelection\x01\0\0\x01\x01\x05self_\x05empty\0\0\0\x19__widl_f_extend_Selection\x01\0\0\x01\tSelection\x01\0\0\x01\x02\x05self_\x04node\x06extend\0\0\0%__widl_f_extend_with_offset_Selection\x01\0\0\x01\tSelection\x01\0\0\x01\x03\x05self_\x04node\x06offset\x06extend\0\0\0\x1F__widl_f_get_range_at_Selection\x01\0\0\x01\tSelection\x01\0\0\x01\x02\x05self_\x05index\ngetRangeAt\0\0\0\x19__widl_f_modify_Selection\x01\0\0\x01\tSelection\x01\0\0\x01\x04\x05self_\x05alter\tdirection\x0Bgranularity\x06modify\0\0\0$__widl_f_remove_all_ranges_Selection\x01\0\0\x01\tSelection\x01\0\0\x01\x01\x05self_\x0FremoveAllRanges\0\0\0\x1F__widl_f_remove_range_Selection\x01\0\0\x01\tSelection\x01\0\0\x01\x02\x05self_\x05range\x0BremoveRange\0\0\0&__widl_f_select_all_children_Selection\x01\0\0\x01\tSelection\x01\0\0\x01\x02\x05self_\x04node\x11selectAllChildren\0\0\0&__widl_f_set_base_and_extent_Selection\x01\0\0\x01\tSelection\x01\0\0\x01\x05\x05self_\x0Banchor_node\ranchor_offset\nfocus_node\x0Cfocus_offset\x10setBaseAndExtent\0\0\0\x1F__widl_f_set_position_Selection\x01\0\0\x01\tSelection\x01\0\0\x01\x02\x05self_\x04node\x0BsetPosition\0\0\0+__widl_f_set_position_with_offset_Selection\x01\0\0\x01\tSelection\x01\0\0\x01\x03\x05self_\x04node\x06offset\x0BsetPosition\0\0\0\x1E__widl_f_anchor_node_Selection\0\0\0\x01\tSelection\x01\0\x01\nanchorNode\x01\x01\x05self_\nanchorNode\0\0\0 __widl_f_anchor_offset_Selection\0\0\0\x01\tSelection\x01\0\x01\x0CanchorOffset\x01\x01\x05self_\x0CanchorOffset\0\0\0\x1D__widl_f_focus_node_Selection\0\0\0\x01\tSelection\x01\0\x01\tfocusNode\x01\x01\x05self_\tfocusNode\0\0\0\x1F__widl_f_focus_offset_Selection\0\0\0\x01\tSelection\x01\0\x01\x0BfocusOffset\x01\x01\x05self_\x0BfocusOffset\0\0\0\x1F__widl_f_is_collapsed_Selection\0\0\0\x01\tSelection\x01\0\x01\x0BisCollapsed\x01\x01\x05self_\x0BisCollapsed\0\0\0\x1E__widl_f_range_count_Selection\0\0\0\x01\tSelection\x01\0\x01\nrangeCount\x01\x01\x05self_\nrangeCount\0\0\0\x17__widl_f_type_Selection\0\0\0\x01\tSelection\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0#__widl_f_caret_bidi_level_Selection\x01\0\0\x01\tSelection\x01\0\x01\x0EcaretBidiLevel\x01\x01\x05self_\x0EcaretBidiLevel\0\0\0'__widl_f_set_caret_bidi_level_Selection\x01\0\0\x01\tSelection\x01\0\x02\x0EcaretBidiLevel\x01\x02\x05self_\x10caret_bidi_level\x0EcaretBidiLevel\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
