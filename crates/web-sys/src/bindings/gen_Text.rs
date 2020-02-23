use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Text` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text)\n\n*This API requires the following crate features to be activated: `Text`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Text {
    obj: CharacterData,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Text: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Text {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(4u32);
            inform(84u32);
            inform(101u32);
            inform(120u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for Text {
        type Target = CharacterData;
        #[inline]
        fn deref(&self) -> &CharacterData {
            &self.obj
        }
    }
    impl IntoWasmAbi for Text {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Text {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Text {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Text {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Text {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Text {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Text {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Text {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Text>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Text {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Text {
        #[inline]
        fn from(obj: JsValue) -> Text {
            Text { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Text {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Text> for Text {
        #[inline]
        fn as_ref(&self) -> &Text {
            self
        }
    }
    impl From<Text> for JsValue {
        #[inline]
        fn from(obj: Text) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Text {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Text(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Text(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Text(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Text { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Text) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Text> for CharacterData {
    #[inline]
    fn from(obj: Text) -> CharacterData {
        use wasm_bindgen::JsCast;
        CharacterData::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<CharacterData> for Text {
    #[inline]
    fn as_ref(&self) -> &CharacterData {
        use wasm_bindgen::JsCast;
        CharacterData::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<Text> for Node {
    #[inline]
    fn from(obj: Text) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for Text {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<Text> for EventTarget {
    #[inline]
    fn from(obj: Text) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for Text {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<Text> for ::js_sys::Object {
    #[inline]
    fn from(obj: Text) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Text {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Text",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_Text() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <Text as WasmDescribe>::describe();
}
impl Text {
    #[cfg(all(feature = "Text",))]
    #[allow(bad_style)]
    #[doc = "The `new Text(..)` constructor, creating a new instance of `Text`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/Text)\n\n*This API requires the following crate features to be activated: `Text`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<Text, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Text",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_Text() -> <Text as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_Text() -> <Text as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_Text() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Text as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Text",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_data_Text() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <Text as WasmDescribe>::describe();
}
impl Text {
    #[cfg(all(feature = "Text",))]
    #[allow(bad_style)]
    #[doc = "The `new Text(..)` constructor, creating a new instance of `Text`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/Text)\n\n*This API requires the following crate features to be activated: `Text`*"]
    #[allow(clippy::all)]
    pub fn new_with_data(data: &str) -> Result<Text, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Text",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_data_Text(
                data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Text as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_data_Text(
            data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Text as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_new_with_data_Text(data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Text as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Text",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_split_text_Text() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Text as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Text as WasmDescribe>::describe();
}
impl Text {
    #[cfg(all(feature = "Text",))]
    #[allow(bad_style)]
    #[doc = "The `splitText()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/splitText)\n\n*This API requires the following crate features to be activated: `Text`*"]
    #[allow(clippy::all)]
    pub fn split_text(&self, offset: u32) -> Result<Text, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Text",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_split_text_Text(
                self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                offset: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Text as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_split_text_Text(
            self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            offset: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Text as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(offset);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let offset = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(offset);
                __widl_f_split_text_Text(self_, offset)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Text as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Text",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_whole_text_Text() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Text as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Text {
    #[cfg(all(feature = "Text",))]
    #[allow(bad_style)]
    #[doc = "The `wholeText` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/wholeText)\n\n*This API requires the following crate features to be activated: `Text`*"]
    #[allow(clippy::all)]
    pub fn whole_text(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Text",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_whole_text_Text(
                self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_whole_text_Text(
            self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_whole_text_Text(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "HtmlSlotElement", feature = "Text",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_assigned_slot_Text() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Text as WasmDescribe>::describe();
    <Option<HtmlSlotElement> as WasmDescribe>::describe();
}
impl Text {
    #[cfg(all(feature = "HtmlSlotElement", feature = "Text",))]
    #[allow(bad_style)]
    #[doc = "The `assignedSlot` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/assignedSlot)\n\n*This API requires the following crate features to be activated: `HtmlSlotElement`, `Text`*"]
    #[allow(clippy::all)]
    pub fn assigned_slot(&self) -> Option<HtmlSlotElement> {
        #[cfg(all(feature = "HtmlSlotElement", feature = "Text",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_assigned_slot_Text(
                self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<HtmlSlotElement> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_assigned_slot_Text(
            self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<HtmlSlotElement> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_assigned_slot_Text(self_)
            };
            <Option<HtmlSlotElement> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomPoint", feature = "DomPointInit", feature = "Text",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_point_from_node_with_text_Text() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Text as WasmDescribe>::describe();
    <&DomPointInit as WasmDescribe>::describe();
    <&Text as WasmDescribe>::describe();
    <DomPoint as WasmDescribe>::describe();
}
impl Text {
    #[cfg(all(feature = "DomPoint", feature = "DomPointInit", feature = "Text",))]
    #[allow(bad_style)]
    #[doc = "The `convertPointFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertPointFromNode)\n\n*This API requires the following crate features to be activated: `DomPoint`, `DomPointInit`, `Text`*"]
    #[allow(clippy::all)]
    pub fn convert_point_from_node_with_text(
        &self,
        point: &DomPointInit,
        from: &Text,
    ) -> Result<DomPoint, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomPoint", feature = "DomPointInit", feature = "Text",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_point_from_node_with_text_Text(
                self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                point: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_point_from_node_with_text_Text(
            self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            point: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(point);
            drop(from);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let point = <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(point);
                let from = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                __widl_f_convert_point_from_node_with_text_Text(self_, point, from)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomPoint as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "DomPoint",
    feature = "DomPointInit",
    feature = "Element",
    feature = "Text",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_point_from_node_with_element_Text() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Text as WasmDescribe>::describe();
    <&DomPointInit as WasmDescribe>::describe();
    <&Element as WasmDescribe>::describe();
    <DomPoint as WasmDescribe>::describe();
}
impl Text {
    #[cfg(all(
        feature = "DomPoint",
        feature = "DomPointInit",
        feature = "Element",
        feature = "Text",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertPointFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertPointFromNode)\n\n*This API requires the following crate features to be activated: `DomPoint`, `DomPointInit`, `Element`, `Text`*"]
    #[allow(clippy::all)]
    pub fn convert_point_from_node_with_element(
        &self,
        point: &DomPointInit,
        from: &Element,
    ) -> Result<DomPoint, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "DomPoint",
            feature = "DomPointInit",
            feature = "Element",
            feature = "Text",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_point_from_node_with_element_Text(
                self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                point: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_point_from_node_with_element_Text(
            self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            point: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(point);
            drop(from);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let point = <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(point);
                let from = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                __widl_f_convert_point_from_node_with_element_Text(self_, point, from)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomPoint as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "Document",
    feature = "DomPoint",
    feature = "DomPointInit",
    feature = "Text",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_point_from_node_with_document_Text() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Text as WasmDescribe>::describe();
    <&DomPointInit as WasmDescribe>::describe();
    <&Document as WasmDescribe>::describe();
    <DomPoint as WasmDescribe>::describe();
}
impl Text {
    #[cfg(all(
        feature = "Document",
        feature = "DomPoint",
        feature = "DomPointInit",
        feature = "Text",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertPointFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertPointFromNode)\n\n*This API requires the following crate features to be activated: `Document`, `DomPoint`, `DomPointInit`, `Text`*"]
    #[allow(clippy::all)]
    pub fn convert_point_from_node_with_document(
        &self,
        point: &DomPointInit,
        from: &Document,
    ) -> Result<DomPoint, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "Document",
            feature = "DomPoint",
            feature = "DomPointInit",
            feature = "Text",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_point_from_node_with_document_Text(
                self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                point: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_point_from_node_with_document_Text(
            self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            point: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(point);
            drop(from);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let point = <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(point);
                let from = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                __widl_f_convert_point_from_node_with_document_Text(self_, point, from)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomPoint as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "ConvertCoordinateOptions",
    feature = "DomPoint",
    feature = "DomPointInit",
    feature = "Text",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_point_from_node_with_text_and_options_Text()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Text as WasmDescribe>::describe();
    <&DomPointInit as WasmDescribe>::describe();
    <&Text as WasmDescribe>::describe();
    <&ConvertCoordinateOptions as WasmDescribe>::describe();
    <DomPoint as WasmDescribe>::describe();
}
impl Text {
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomPoint",
        feature = "DomPointInit",
        feature = "Text",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertPointFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertPointFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomPoint`, `DomPointInit`, `Text`*"]
    #[allow(clippy::all)]
    pub fn convert_point_from_node_with_text_and_options(
        &self,
        point: &DomPointInit,
        from: &Text,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomPoint, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "ConvertCoordinateOptions",
            feature = "DomPoint",
            feature = "DomPointInit",
            feature = "Text",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_point_from_node_with_text_and_options_Text(
                self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                point: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_point_from_node_with_text_and_options_Text(
            self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            point: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(point);
            drop(from);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let point = <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(point);
                let from = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                let options =
                    <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_convert_point_from_node_with_text_and_options_Text(
                    self_, point, from, options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomPoint as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "ConvertCoordinateOptions",
    feature = "DomPoint",
    feature = "DomPointInit",
    feature = "Element",
    feature = "Text",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_point_from_node_with_element_and_options_Text(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Text as WasmDescribe>::describe();
    <&DomPointInit as WasmDescribe>::describe();
    <&Element as WasmDescribe>::describe();
    <&ConvertCoordinateOptions as WasmDescribe>::describe();
    <DomPoint as WasmDescribe>::describe();
}
impl Text {
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomPoint",
        feature = "DomPointInit",
        feature = "Element",
        feature = "Text",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertPointFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertPointFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomPoint`, `DomPointInit`, `Element`, `Text`*"]
    #[allow(clippy::all)]
    pub fn convert_point_from_node_with_element_and_options(
        &self,
        point: &DomPointInit,
        from: &Element,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomPoint, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "ConvertCoordinateOptions",
            feature = "DomPoint",
            feature = "DomPointInit",
            feature = "Element",
            feature = "Text",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_point_from_node_with_element_and_options_Text(
                self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                point: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_point_from_node_with_element_and_options_Text(
            self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            point: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(point);
            drop(from);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let point = <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(point);
                let from = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                let options =
                    <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_convert_point_from_node_with_element_and_options_Text(
                    self_, point, from, options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomPoint as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "ConvertCoordinateOptions",
    feature = "Document",
    feature = "DomPoint",
    feature = "DomPointInit",
    feature = "Text",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_point_from_node_with_document_and_options_Text(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Text as WasmDescribe>::describe();
    <&DomPointInit as WasmDescribe>::describe();
    <&Document as WasmDescribe>::describe();
    <&ConvertCoordinateOptions as WasmDescribe>::describe();
    <DomPoint as WasmDescribe>::describe();
}
impl Text {
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "Document",
        feature = "DomPoint",
        feature = "DomPointInit",
        feature = "Text",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertPointFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertPointFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomPoint`, `DomPointInit`, `Text`*"]
    #[allow(clippy::all)]
    pub fn convert_point_from_node_with_document_and_options(
        &self,
        point: &DomPointInit,
        from: &Document,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomPoint, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "ConvertCoordinateOptions",
            feature = "Document",
            feature = "DomPoint",
            feature = "DomPointInit",
            feature = "Text",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_point_from_node_with_document_and_options_Text(
                self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                point: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_point_from_node_with_document_and_options_Text(
            self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            point: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(point);
            drop(from);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let point = <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(point);
                let from = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                let options =
                    <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_convert_point_from_node_with_document_and_options_Text(
                    self_, point, from, options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomPoint as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomQuad", feature = "Text",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_quad_from_node_with_text_Text() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Text as WasmDescribe>::describe();
    <&DomQuad as WasmDescribe>::describe();
    <&Text as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Text {
    #[cfg(all(feature = "DomQuad", feature = "Text",))]
    #[allow(bad_style)]
    #[doc = "The `convertQuadFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertQuadFromNode)\n\n*This API requires the following crate features to be activated: `DomQuad`, `Text`*"]
    #[allow(clippy::all)]
    pub fn convert_quad_from_node_with_text(
        &self,
        quad: &DomQuad,
        from: &Text,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomQuad", feature = "Text",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_quad_from_node_with_text_Text(
                self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                quad: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_quad_from_node_with_text_Text(
            self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            quad: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(quad);
            drop(from);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let quad = <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(quad);
                let from = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                __widl_f_convert_quad_from_node_with_text_Text(self_, quad, from)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomQuad as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomQuad", feature = "Element", feature = "Text",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_quad_from_node_with_element_Text() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Text as WasmDescribe>::describe();
    <&DomQuad as WasmDescribe>::describe();
    <&Element as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Text {
    #[cfg(all(feature = "DomQuad", feature = "Element", feature = "Text",))]
    #[allow(bad_style)]
    #[doc = "The `convertQuadFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertQuadFromNode)\n\n*This API requires the following crate features to be activated: `DomQuad`, `Element`, `Text`*"]
    #[allow(clippy::all)]
    pub fn convert_quad_from_node_with_element(
        &self,
        quad: &DomQuad,
        from: &Element,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomQuad", feature = "Element", feature = "Text",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_quad_from_node_with_element_Text(
                self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                quad: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_quad_from_node_with_element_Text(
            self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            quad: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(quad);
            drop(from);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let quad = <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(quad);
                let from = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                __widl_f_convert_quad_from_node_with_element_Text(self_, quad, from)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomQuad as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Document", feature = "DomQuad", feature = "Text",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_quad_from_node_with_document_Text() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Text as WasmDescribe>::describe();
    <&DomQuad as WasmDescribe>::describe();
    <&Document as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Text {
    #[cfg(all(feature = "Document", feature = "DomQuad", feature = "Text",))]
    #[allow(bad_style)]
    #[doc = "The `convertQuadFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertQuadFromNode)\n\n*This API requires the following crate features to be activated: `Document`, `DomQuad`, `Text`*"]
    #[allow(clippy::all)]
    pub fn convert_quad_from_node_with_document(
        &self,
        quad: &DomQuad,
        from: &Document,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "DomQuad", feature = "Text",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_quad_from_node_with_document_Text(
                self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                quad: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_quad_from_node_with_document_Text(
            self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            quad: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(quad);
            drop(from);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let quad = <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(quad);
                let from = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                __widl_f_convert_quad_from_node_with_document_Text(self_, quad, from)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomQuad as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "ConvertCoordinateOptions",
    feature = "DomQuad",
    feature = "Text",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_quad_from_node_with_text_and_options_Text() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Text as WasmDescribe>::describe();
    <&DomQuad as WasmDescribe>::describe();
    <&Text as WasmDescribe>::describe();
    <&ConvertCoordinateOptions as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Text {
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "Text",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertQuadFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertQuadFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomQuad`, `Text`*"]
    #[allow(clippy::all)]
    pub fn convert_quad_from_node_with_text_and_options(
        &self,
        quad: &DomQuad,
        from: &Text,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "ConvertCoordinateOptions",
            feature = "DomQuad",
            feature = "Text",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_quad_from_node_with_text_and_options_Text(
                self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                quad: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_quad_from_node_with_text_and_options_Text(
            self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            quad: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(quad);
            drop(from);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let quad = <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(quad);
                let from = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                let options =
                    <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_convert_quad_from_node_with_text_and_options_Text(
                    self_, quad, from, options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomQuad as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "ConvertCoordinateOptions",
    feature = "DomQuad",
    feature = "Element",
    feature = "Text",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_quad_from_node_with_element_and_options_Text(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Text as WasmDescribe>::describe();
    <&DomQuad as WasmDescribe>::describe();
    <&Element as WasmDescribe>::describe();
    <&ConvertCoordinateOptions as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Text {
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "Element",
        feature = "Text",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertQuadFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertQuadFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomQuad`, `Element`, `Text`*"]
    #[allow(clippy::all)]
    pub fn convert_quad_from_node_with_element_and_options(
        &self,
        quad: &DomQuad,
        from: &Element,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "ConvertCoordinateOptions",
            feature = "DomQuad",
            feature = "Element",
            feature = "Text",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_quad_from_node_with_element_and_options_Text(
                self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                quad: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_quad_from_node_with_element_and_options_Text(
            self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            quad: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(quad);
            drop(from);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let quad = <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(quad);
                let from = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                let options =
                    <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_convert_quad_from_node_with_element_and_options_Text(
                    self_, quad, from, options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomQuad as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "ConvertCoordinateOptions",
    feature = "Document",
    feature = "DomQuad",
    feature = "Text",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_quad_from_node_with_document_and_options_Text(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Text as WasmDescribe>::describe();
    <&DomQuad as WasmDescribe>::describe();
    <&Document as WasmDescribe>::describe();
    <&ConvertCoordinateOptions as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Text {
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "Document",
        feature = "DomQuad",
        feature = "Text",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertQuadFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertQuadFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `Text`*"]
    #[allow(clippy::all)]
    pub fn convert_quad_from_node_with_document_and_options(
        &self,
        quad: &DomQuad,
        from: &Document,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "ConvertCoordinateOptions",
            feature = "Document",
            feature = "DomQuad",
            feature = "Text",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_quad_from_node_with_document_and_options_Text(
                self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                quad: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_quad_from_node_with_document_and_options_Text(
            self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            quad: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(quad);
            drop(from);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let quad = <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(quad);
                let from = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                let options =
                    <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_convert_quad_from_node_with_document_and_options_Text(
                    self_, quad, from, options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomQuad as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly", feature = "Text",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_rect_from_node_with_text_Text() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Text as WasmDescribe>::describe();
    <&DomRectReadOnly as WasmDescribe>::describe();
    <&Text as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Text {
    #[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly", feature = "Text",))]
    #[allow(bad_style)]
    #[doc = "The `convertRectFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertRectFromNode)\n\n*This API requires the following crate features to be activated: `DomQuad`, `DomRectReadOnly`, `Text`*"]
    #[allow(clippy::all)]
    pub fn convert_rect_from_node_with_text(
        &self,
        rect: &DomRectReadOnly,
        from: &Text,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly", feature = "Text",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_rect_from_node_with_text_Text(
                self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rect: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_rect_from_node_with_text_Text(
            self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rect: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(rect);
            drop(from);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rect = <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rect);
                let from = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                __widl_f_convert_rect_from_node_with_text_Text(self_, rect, from)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomQuad as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "DomQuad",
    feature = "DomRectReadOnly",
    feature = "Element",
    feature = "Text",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_rect_from_node_with_element_Text() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Text as WasmDescribe>::describe();
    <&DomRectReadOnly as WasmDescribe>::describe();
    <&Element as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Text {
    #[cfg(all(
        feature = "DomQuad",
        feature = "DomRectReadOnly",
        feature = "Element",
        feature = "Text",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertRectFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertRectFromNode)\n\n*This API requires the following crate features to be activated: `DomQuad`, `DomRectReadOnly`, `Element`, `Text`*"]
    #[allow(clippy::all)]
    pub fn convert_rect_from_node_with_element(
        &self,
        rect: &DomRectReadOnly,
        from: &Element,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "DomQuad",
            feature = "DomRectReadOnly",
            feature = "Element",
            feature = "Text",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_rect_from_node_with_element_Text(
                self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rect: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_rect_from_node_with_element_Text(
            self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rect: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(rect);
            drop(from);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rect = <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rect);
                let from = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                __widl_f_convert_rect_from_node_with_element_Text(self_, rect, from)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomQuad as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "Document",
    feature = "DomQuad",
    feature = "DomRectReadOnly",
    feature = "Text",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_rect_from_node_with_document_Text() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Text as WasmDescribe>::describe();
    <&DomRectReadOnly as WasmDescribe>::describe();
    <&Document as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Text {
    #[cfg(all(
        feature = "Document",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
        feature = "Text",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertRectFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertRectFromNode)\n\n*This API requires the following crate features to be activated: `Document`, `DomQuad`, `DomRectReadOnly`, `Text`*"]
    #[allow(clippy::all)]
    pub fn convert_rect_from_node_with_document(
        &self,
        rect: &DomRectReadOnly,
        from: &Document,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "Document",
            feature = "DomQuad",
            feature = "DomRectReadOnly",
            feature = "Text",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_rect_from_node_with_document_Text(
                self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rect: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_rect_from_node_with_document_Text(
            self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rect: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(rect);
            drop(from);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rect = <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rect);
                let from = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                __widl_f_convert_rect_from_node_with_document_Text(self_, rect, from)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomQuad as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "ConvertCoordinateOptions",
    feature = "DomQuad",
    feature = "DomRectReadOnly",
    feature = "Text",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_rect_from_node_with_text_and_options_Text() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Text as WasmDescribe>::describe();
    <&DomRectReadOnly as WasmDescribe>::describe();
    <&Text as WasmDescribe>::describe();
    <&ConvertCoordinateOptions as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Text {
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
        feature = "Text",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertRectFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertRectFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomQuad`, `DomRectReadOnly`, `Text`*"]
    #[allow(clippy::all)]
    pub fn convert_rect_from_node_with_text_and_options(
        &self,
        rect: &DomRectReadOnly,
        from: &Text,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "ConvertCoordinateOptions",
            feature = "DomQuad",
            feature = "DomRectReadOnly",
            feature = "Text",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_rect_from_node_with_text_and_options_Text(
                self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rect: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_rect_from_node_with_text_and_options_Text(
            self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rect: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(rect);
            drop(from);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rect = <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rect);
                let from = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                let options =
                    <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_convert_rect_from_node_with_text_and_options_Text(
                    self_, rect, from, options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomQuad as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "ConvertCoordinateOptions",
    feature = "DomQuad",
    feature = "DomRectReadOnly",
    feature = "Element",
    feature = "Text",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_rect_from_node_with_element_and_options_Text(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Text as WasmDescribe>::describe();
    <&DomRectReadOnly as WasmDescribe>::describe();
    <&Element as WasmDescribe>::describe();
    <&ConvertCoordinateOptions as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Text {
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
        feature = "Element",
        feature = "Text",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertRectFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertRectFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomQuad`, `DomRectReadOnly`, `Element`, `Text`*"]
    #[allow(clippy::all)]
    pub fn convert_rect_from_node_with_element_and_options(
        &self,
        rect: &DomRectReadOnly,
        from: &Element,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "ConvertCoordinateOptions",
            feature = "DomQuad",
            feature = "DomRectReadOnly",
            feature = "Element",
            feature = "Text",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_rect_from_node_with_element_and_options_Text(
                self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rect: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_rect_from_node_with_element_and_options_Text(
            self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rect: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(rect);
            drop(from);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rect = <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rect);
                let from = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                let options =
                    <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_convert_rect_from_node_with_element_and_options_Text(
                    self_, rect, from, options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomQuad as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "ConvertCoordinateOptions",
    feature = "Document",
    feature = "DomQuad",
    feature = "DomRectReadOnly",
    feature = "Text",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_rect_from_node_with_document_and_options_Text(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Text as WasmDescribe>::describe();
    <&DomRectReadOnly as WasmDescribe>::describe();
    <&Document as WasmDescribe>::describe();
    <&ConvertCoordinateOptions as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Text {
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "Document",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
        feature = "Text",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertRectFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertRectFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `DomRectReadOnly`, `Text`*"]
    #[allow(clippy::all)]
    pub fn convert_rect_from_node_with_document_and_options(
        &self,
        rect: &DomRectReadOnly,
        from: &Document,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "ConvertCoordinateOptions",
            feature = "Document",
            feature = "DomQuad",
            feature = "DomRectReadOnly",
            feature = "Text",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_rect_from_node_with_document_and_options_Text(
                self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rect: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_rect_from_node_with_document_and_options_Text(
            self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rect: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(rect);
            drop(from);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rect = <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rect);
                let from = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                let options =
                    <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_convert_rect_from_node_with_document_and_options_Text(
                    self_, rect, from, options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomQuad as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Text",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_box_quads_Text() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Text as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl Text {
    #[cfg(all(feature = "Text",))]
    #[allow(bad_style)]
    #[doc = "The `getBoxQuads()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/getBoxQuads)\n\n*This API requires the following crate features to be activated: `Text`*"]
    #[allow(clippy::all)]
    pub fn get_box_quads(&self) -> Result<::js_sys::Array, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Text",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_box_quads_Text(
                self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_box_quads_Text(
            self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_box_quads_Text(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "BoxQuadOptions", feature = "Text",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_box_quads_with_options_Text() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Text as WasmDescribe>::describe();
    <&BoxQuadOptions as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl Text {
    #[cfg(all(feature = "BoxQuadOptions", feature = "Text",))]
    #[allow(bad_style)]
    #[doc = "The `getBoxQuads()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/getBoxQuads)\n\n*This API requires the following crate features to be activated: `BoxQuadOptions`, `Text`*"]
    #[allow(clippy::all)]
    pub fn get_box_quads_with_options(
        &self,
        options: &BoxQuadOptions,
    ) -> Result<::js_sys::Array, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "BoxQuadOptions", feature = "Text",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_box_quads_with_options_Text(
                self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&BoxQuadOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_box_quads_with_options_Text(
            self_: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&BoxQuadOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let options =
                    <&BoxQuadOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_get_box_quads_with_options_Text(self_, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_8f9addcfc9e8ef42: [u8; 2623usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xFD\t\0\0\0\0\x1A\0\0\x02\x04Text\x16__widl_instanceof_Text\0\0\0\0\x11__widl_f_new_Text\x01\0\0\x01\x04Text\0\x01\0\x03new\0\0\0\x1B__widl_f_new_with_data_Text\x01\0\0\x01\x04Text\0\x01\x01\x04data\x03new\0\0\0\x18__widl_f_split_text_Text\x01\0\0\x01\x04Text\x01\0\0\x01\x02\x05self_\x06offset\tsplitText\0\0\0\x18__widl_f_whole_text_Text\x01\0\0\x01\x04Text\x01\0\x01\twholeText\x01\x01\x05self_\twholeText\0\0\0\x1B__widl_f_assigned_slot_Text\0\0\0\x01\x04Text\x01\0\x01\x0CassignedSlot\x01\x01\x05self_\x0CassignedSlot\0\0\0/__widl_f_convert_point_from_node_with_text_Text\x01\0\0\x01\x04Text\x01\0\0\x01\x03\x05self_\x05point\x04from\x14convertPointFromNode\0\0\02__widl_f_convert_point_from_node_with_element_Text\x01\0\0\x01\x04Text\x01\0\0\x01\x03\x05self_\x05point\x04from\x14convertPointFromNode\0\0\03__widl_f_convert_point_from_node_with_document_Text\x01\0\0\x01\x04Text\x01\0\0\x01\x03\x05self_\x05point\x04from\x14convertPointFromNode\0\0\0;__widl_f_convert_point_from_node_with_text_and_options_Text\x01\0\0\x01\x04Text\x01\0\0\x01\x04\x05self_\x05point\x04from\x07options\x14convertPointFromNode\0\0\0>__widl_f_convert_point_from_node_with_element_and_options_Text\x01\0\0\x01\x04Text\x01\0\0\x01\x04\x05self_\x05point\x04from\x07options\x14convertPointFromNode\0\0\0?__widl_f_convert_point_from_node_with_document_and_options_Text\x01\0\0\x01\x04Text\x01\0\0\x01\x04\x05self_\x05point\x04from\x07options\x14convertPointFromNode\0\0\0.__widl_f_convert_quad_from_node_with_text_Text\x01\0\0\x01\x04Text\x01\0\0\x01\x03\x05self_\x04quad\x04from\x13convertQuadFromNode\0\0\01__widl_f_convert_quad_from_node_with_element_Text\x01\0\0\x01\x04Text\x01\0\0\x01\x03\x05self_\x04quad\x04from\x13convertQuadFromNode\0\0\02__widl_f_convert_quad_from_node_with_document_Text\x01\0\0\x01\x04Text\x01\0\0\x01\x03\x05self_\x04quad\x04from\x13convertQuadFromNode\0\0\0:__widl_f_convert_quad_from_node_with_text_and_options_Text\x01\0\0\x01\x04Text\x01\0\0\x01\x04\x05self_\x04quad\x04from\x07options\x13convertQuadFromNode\0\0\0=__widl_f_convert_quad_from_node_with_element_and_options_Text\x01\0\0\x01\x04Text\x01\0\0\x01\x04\x05self_\x04quad\x04from\x07options\x13convertQuadFromNode\0\0\0>__widl_f_convert_quad_from_node_with_document_and_options_Text\x01\0\0\x01\x04Text\x01\0\0\x01\x04\x05self_\x04quad\x04from\x07options\x13convertQuadFromNode\0\0\0.__widl_f_convert_rect_from_node_with_text_Text\x01\0\0\x01\x04Text\x01\0\0\x01\x03\x05self_\x04rect\x04from\x13convertRectFromNode\0\0\01__widl_f_convert_rect_from_node_with_element_Text\x01\0\0\x01\x04Text\x01\0\0\x01\x03\x05self_\x04rect\x04from\x13convertRectFromNode\0\0\02__widl_f_convert_rect_from_node_with_document_Text\x01\0\0\x01\x04Text\x01\0\0\x01\x03\x05self_\x04rect\x04from\x13convertRectFromNode\0\0\0:__widl_f_convert_rect_from_node_with_text_and_options_Text\x01\0\0\x01\x04Text\x01\0\0\x01\x04\x05self_\x04rect\x04from\x07options\x13convertRectFromNode\0\0\0=__widl_f_convert_rect_from_node_with_element_and_options_Text\x01\0\0\x01\x04Text\x01\0\0\x01\x04\x05self_\x04rect\x04from\x07options\x13convertRectFromNode\0\0\0>__widl_f_convert_rect_from_node_with_document_and_options_Text\x01\0\0\x01\x04Text\x01\0\0\x01\x04\x05self_\x04rect\x04from\x07options\x13convertRectFromNode\0\0\0\x1B__widl_f_get_box_quads_Text\x01\0\0\x01\x04Text\x01\0\0\x01\x01\x05self_\x0BgetBoxQuads\0\0\0(__widl_f_get_box_quads_with_options_Text\x01\0\0\x01\x04Text\x01\0\0\x01\x02\x05self_\x07options\x0BgetBoxQuads\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
