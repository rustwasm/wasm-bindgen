use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `CustomElementRegistry` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry)\n\n*This API requires the following crate features to be activated: `CustomElementRegistry`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct CustomElementRegistry {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CustomElementRegistry: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for CustomElementRegistry {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(21u32);
            inform(67u32);
            inform(117u32);
            inform(115u32);
            inform(116u32);
            inform(111u32);
            inform(109u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
            inform(82u32);
            inform(101u32);
            inform(103u32);
            inform(105u32);
            inform(115u32);
            inform(116u32);
            inform(114u32);
            inform(121u32);
        }
    }
    impl core::ops::Deref for CustomElementRegistry {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for CustomElementRegistry {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for CustomElementRegistry {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CustomElementRegistry {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for CustomElementRegistry {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            CustomElementRegistry {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for CustomElementRegistry {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a CustomElementRegistry {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for CustomElementRegistry {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<CustomElementRegistry>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(CustomElementRegistry {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for CustomElementRegistry {
        #[inline]
        fn from(obj: JsValue) -> CustomElementRegistry {
            CustomElementRegistry { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for CustomElementRegistry {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<CustomElementRegistry> for CustomElementRegistry {
        #[inline]
        fn as_ref(&self) -> &CustomElementRegistry {
            self
        }
    }
    impl From<CustomElementRegistry> for JsValue {
        #[inline]
        fn from(obj: CustomElementRegistry) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for CustomElementRegistry {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_CustomElementRegistry(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_CustomElementRegistry(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_CustomElementRegistry(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CustomElementRegistry { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CustomElementRegistry) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<CustomElementRegistry> for ::js_sys::Object {
    #[inline]
    fn from(obj: CustomElementRegistry) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for CustomElementRegistry {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "CustomElementRegistry",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_define_CustomElementRegistry() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CustomElementRegistry as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CustomElementRegistry {
    #[cfg(all(feature = "CustomElementRegistry",))]
    #[allow(bad_style)]
    #[doc = "The `define()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/define)\n\n*This API requires the following crate features to be activated: `CustomElementRegistry`*"]
    #[allow(clippy::all)]
    pub fn define(
        &self,
        name: &str,
        function_constructor: &::js_sys::Function,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CustomElementRegistry",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_define_CustomElementRegistry(
                self_: <&CustomElementRegistry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                function_constructor : < & :: js_sys :: Function as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_define_CustomElementRegistry(
            self_: <&CustomElementRegistry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            function_constructor: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(name);
            drop(function_constructor);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CustomElementRegistry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                let function_constructor =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        function_constructor,
                    );
                __widl_f_define_CustomElementRegistry(self_, name, function_constructor)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(
    feature = "CustomElementRegistry",
    feature = "ElementDefinitionOptions",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_define_with_options_CustomElementRegistry() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&CustomElementRegistry as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <&ElementDefinitionOptions as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CustomElementRegistry {
    #[cfg(all(
        feature = "CustomElementRegistry",
        feature = "ElementDefinitionOptions",
    ))]
    #[allow(bad_style)]
    #[doc = "The `define()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/define)\n\n*This API requires the following crate features to be activated: `CustomElementRegistry`, `ElementDefinitionOptions`*"]
    #[allow(clippy::all)]
    pub fn define_with_options(
        &self,
        name: &str,
        function_constructor: &::js_sys::Function,
        options: &ElementDefinitionOptions,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "CustomElementRegistry",
            feature = "ElementDefinitionOptions",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_define_with_options_CustomElementRegistry(
                self_: <&CustomElementRegistry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                function_constructor : < & :: js_sys :: Function as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                options: <&ElementDefinitionOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_define_with_options_CustomElementRegistry(
            self_: <&CustomElementRegistry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            function_constructor: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&ElementDefinitionOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(name);
            drop(function_constructor);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CustomElementRegistry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                let function_constructor =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        function_constructor,
                    );
                let options =
                    <&ElementDefinitionOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_define_with_options_CustomElementRegistry(
                    self_,
                    name,
                    function_constructor,
                    options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CustomElementRegistry",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_CustomElementRegistry() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CustomElementRegistry as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl CustomElementRegistry {
    #[cfg(all(feature = "CustomElementRegistry",))]
    #[allow(bad_style)]
    #[doc = "The `get()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/get)\n\n*This API requires the following crate features to be activated: `CustomElementRegistry`*"]
    #[allow(clippy::all)]
    pub fn get(&self, name: &str) -> ::wasm_bindgen::JsValue {
        #[cfg(all(feature = "CustomElementRegistry",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_CustomElementRegistry(
                self_: <&CustomElementRegistry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_CustomElementRegistry(
            self_: <&CustomElementRegistry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CustomElementRegistry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_get_CustomElementRegistry(self_, name)
            };
            <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CustomElementRegistry", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_upgrade_CustomElementRegistry() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CustomElementRegistry as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CustomElementRegistry {
    #[cfg(all(feature = "CustomElementRegistry", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `upgrade()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/upgrade)\n\n*This API requires the following crate features to be activated: `CustomElementRegistry`, `Node`*"]
    #[allow(clippy::all)]
    pub fn upgrade(&self, root: &Node) {
        #[cfg(all(feature = "CustomElementRegistry", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_upgrade_CustomElementRegistry(
                self_: <&CustomElementRegistry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                root: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_upgrade_CustomElementRegistry(
            self_: <&CustomElementRegistry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            root: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(root);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CustomElementRegistry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let root = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(root);
                __widl_f_upgrade_CustomElementRegistry(self_, root)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CustomElementRegistry",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_when_defined_CustomElementRegistry() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CustomElementRegistry as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl CustomElementRegistry {
    #[cfg(all(feature = "CustomElementRegistry",))]
    #[allow(bad_style)]
    #[doc = "The `whenDefined()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/whenDefined)\n\n*This API requires the following crate features to be activated: `CustomElementRegistry`*"]
    #[allow(clippy::all)]
    pub fn when_defined(&self, name: &str) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CustomElementRegistry",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_when_defined_CustomElementRegistry(
                self_: <&CustomElementRegistry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_when_defined_CustomElementRegistry(
            self_: <&CustomElementRegistry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CustomElementRegistry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_when_defined_CustomElementRegistry(self_, name)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_15561141757cbb74: [u8; 691usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}q\x02\0\0\0\0\x06\0\0\x02\x15CustomElementRegistry'__widl_instanceof_CustomElementRegistry\0\0\0\0%__widl_f_define_CustomElementRegistry\x01\0\0\x01\x15CustomElementRegistry\x01\0\0\x01\x03\x05self_\x04name\x14function_constructor\x06define\0\0\02__widl_f_define_with_options_CustomElementRegistry\x01\0\0\x01\x15CustomElementRegistry\x01\0\0\x01\x04\x05self_\x04name\x14function_constructor\x07options\x06define\0\0\0\"__widl_f_get_CustomElementRegistry\0\0\0\x01\x15CustomElementRegistry\x01\0\0\x01\x02\x05self_\x04name\x03get\0\0\0&__widl_f_upgrade_CustomElementRegistry\0\0\0\x01\x15CustomElementRegistry\x01\0\0\x01\x02\x05self_\x04root\x07upgrade\0\0\0+__widl_f_when_defined_CustomElementRegistry\x01\0\0\x01\x15CustomElementRegistry\x01\0\0\x01\x02\x05self_\x04name\x0BwhenDefined\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
