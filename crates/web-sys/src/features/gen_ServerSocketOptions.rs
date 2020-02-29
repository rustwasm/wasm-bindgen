use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = ServerSocketOptions ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ServerSocketOptions` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `ServerSocketOptions`*
    pub type ServerSocketOptions;

}

impl ServerSocketOptions {
    ///Construct a new `ServerSocketOptions`.
    ///
    ///*This API requires the following crate features to be activated: `ServerSocketOptions`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    #[cfg(feature = "TcpSocketBinaryType")]
    ///Change the `binaryType` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `ServerSocketOptions`, `TcpSocketBinaryType`*

    pub fn binary_type(&mut self, val: TcpSocketBinaryType) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("binaryType"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
