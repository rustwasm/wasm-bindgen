#[allow(bad_style)]
pub struct Event {
    obj: ::wasm_bindgen::JsValue,
}
impl ::wasm_bindgen::describe::WasmDescribe for Event {
    fn describe() {
        ::wasm_bindgen::JsValue::describe();
    }
}
impl ::wasm_bindgen::convert::IntoWasmAbi for Event {
    type Abi = <::wasm_bindgen::JsValue as ::wasm_bindgen::convert::IntoWasmAbi>::Abi;
    fn into_abi(self, extra: &mut ::wasm_bindgen::convert::Stack) -> Self::Abi {
        self.obj.into_abi(extra)
    }
}
impl ::wasm_bindgen::convert::FromWasmAbi for Event {
    type Abi = <::wasm_bindgen::JsValue as ::wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi, extra: &mut ::wasm_bindgen::convert::Stack) -> Self {
        Event {
            obj: ::wasm_bindgen::JsValue::from_abi(js, extra),
        }
    }
}
impl<'a> ::wasm_bindgen::convert::IntoWasmAbi for &'a Event {
    type Abi = <&'a ::wasm_bindgen::JsValue as ::wasm_bindgen::convert::IntoWasmAbi>::Abi;
    fn into_abi(self, extra: &mut ::wasm_bindgen::convert::Stack) -> Self::Abi {
        (&self.obj).into_abi(extra)
    }
}
impl ::wasm_bindgen::convert::RefFromWasmAbi for Event {
    type Abi = <::wasm_bindgen::JsValue as ::wasm_bindgen::convert::RefFromWasmAbi>::Abi;
    type Anchor = ::wasm_bindgen::__rt::core::mem::ManuallyDrop<Event>;
    unsafe fn ref_from_abi(
        js: Self::Abi,
        extra: &mut ::wasm_bindgen::convert::Stack,
    ) -> Self::Anchor {
        let tmp =
            <::wasm_bindgen::JsValue as ::wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(
                js, extra,
            );
        ::wasm_bindgen::__rt::core::mem::ManuallyDrop::new(Event {
            obj: ::wasm_bindgen::__rt::core::mem::ManuallyDrop::into_inner(tmp),
        })
    }
}
impl From<::wasm_bindgen::JsValue> for Event {
    fn from(obj: ::wasm_bindgen::JsValue) -> Event {
        Event { obj }
    }
}
impl From<Event> for ::wasm_bindgen::JsValue {
    fn from(obj: Event) -> ::wasm_bindgen::JsValue {
        obj.obj
    }
}
#[allow(non_upper_case_globals)]
#[wasm_custom_section = "__wasm_bindgen_unstable"]
const __WASM_BINDGEN_GENERATED_wasm_bindgen_webidl_0_1_0_0 : [ u8 ; 180usize ] = * b"\xB0\0\0\0{\"exports\":[],\"enums\":[],\"imports\":[{\"module\":null,\"version\":null,\"js_namespace\":null,\"kind\":{\"kind\":\"type\"}}],\"structs\":[],\"version\":\"0.2.11 (3879f6f42)\",\"schema_version\":\"4\"}" ;
