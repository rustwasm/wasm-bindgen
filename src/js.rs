//! Bindings to JavaScript's standard, built-in objects, including their methods
//! and properties.
//!
//! This does *not* include any Web, Node, or any other JS environment
//! APIs. Only the things that are guaranteed to exist in the global scope by
//! the ECMAScript standard.
//!
//! https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects
//!
//! ## A Note About `camelCase`, `snake_case`, and Naming Conventions
//!
//! JavaScript's global objects use `camelCase` naming conventions for functions
//! and methods, but Rust style is to use `snake_case`. These bindings expose
//! the Rust style `snake_case` name. Additionally, acronyms within a method
//! name are all lower case, where as in JavaScript they are all upper case. For
//! example, `decodeURI` in JavaScript is exposed as `decode_uri` in these
//! bindings.

use wasm_bindgen_macro::*;
use JsValue;
if_std! {
    use std::prelude::v1::*;
}

// When adding new imports:
//
// * Keep imports in alphabetical order.
//
// * Rename imports with `js_name = ...` according to the note about `camelCase`
// and `snake_case` in the module's documentation above.
//
// * Include the one sentence summary of the import from the MDN link in the
// module's documentation above, and the MDN link itself.
//
// * If a function or method can throw an exception, make it catchable by adding
// `#[wasm_bindgen(catch)]`.
//
// * Add a new `#[test]` to the `tests/all/js_globals.rs` file. If the imported
// function or method can throw an exception, make sure to also add test
// coverage for that case.

#[wasm_bindgen]
extern {
    /// The `decodeURI()` function decodes a Uniform Resource Identifier (URI)
    /// previously created by `encodeURI` or by a similar routine.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/decodeURI
    #[cfg(feature = "std")]
    #[wasm_bindgen(catch, js_name = decodeURI)]
    pub fn decode_uri(encoded: &str) -> Result<String, JsValue>;

    /// The `encodeURI()` function encodes a Uniform Resource Identifier (URI)
    /// by replacing each instance of certain characters by one, two, three, or
    /// four escape sequences representing the UTF-8 encoding of the character
    /// (will only be four escape sequences for characters composed of two
    /// "surrogate" characters).
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURI
    #[cfg(feature = "std")]
    #[wasm_bindgen(js_name = encodeURI)]
    pub fn encode_uri(decoded: &str) -> String;

    /// The `eval()` function evaluates JavaScript code represented as a string.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/eval
    #[wasm_bindgen(catch)]
    pub fn eval(js_source_text: &str) -> Result<JsValue, JsValue>;
}

// Object.
#[wasm_bindgen]
extern {
    pub type Object;

    /// The Object constructor creates an object wrapper.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object
    #[wasm_bindgen(constructor)]
    pub fn new() -> Object;

    /// The `hasOwnProperty()` method returns a boolean indicating whether the
    /// object has the specified property as its own property (as opposed to
    /// inheriting it).
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/hasOwnProperty
    #[wasm_bindgen(method, js_name = hasOwnProperty)]
    pub fn has_own_property(this: &Object, property: &str) -> bool;

    /// The toString() method returns a string representing the object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/toString
    #[wasm_bindgen(method, js_name = toString)]
    pub fn to_string(this: &Object) -> String;


}

// Array
#[wasm_bindgen]
extern {
    pub type Array;

    /// The indexOf() method returns the first index at which a given element can be found in the array, or -1 if it is not present.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/indexOf
    #[wasm_bindgen(method, js_name = indexOf)]
    pub fn index_of(this: &Array, value: JsValue, from_index: i32) -> i32;

    /// The lastIndexOf() method returns the last index at which a given element can be found in the array, or -1 if it is not present.
    /// The array is searched backwards, starting at fromIndex.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/lastIndexOf
    #[wasm_bindgen(method, js_name = lastIndexOf)]
    pub fn last_index_of(this: &Array, value: JsValue, from_index: i32) -> i32;
}