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

// Array
#[wasm_bindgen]
extern {
    pub type Array;

    /// The copyWithin() method shallow copies part of an array to another location in the same
    /// array and returns it, without modifying its size.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/copyWithin
    #[wasm_bindgen(method, js_name = copyWithin)]
    pub fn copy_within(this: &Array, target: i32, start: i32, end: i32) -> Array;

    ///The concat() method is used to merge two or more arrays. This method
    /// does not change the existing arrays, but instead returns a new array.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/concat
    #[wasm_bindgen(method)]
    pub fn concat(this: &Array, array: &Array) -> Array;

    /// The fill() method fills all the elements of an array from a start index to an
    /// end index with a static value. The end index is not included.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/fill
    #[wasm_bindgen(method)]
    pub fn fill(this: &Array, value: JsValue, start: u32, end: u32) -> Array;

    /// The length property of an object which is an instance of type Array sets or returns the number of elements in that array.
    /// The value is an unsigned, 32-bit integer that is always numerically greater than the highest index in the array.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/length
    #[wasm_bindgen(method, getter, structural)]
    pub fn length(this: &Array) -> u32;

    /// The indexOf() method returns the first index at which a given element can be
    /// found in the array, or -1 if it is not present.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/indexOf
    #[wasm_bindgen(method, js_name = indexOf)]
    pub fn index_of(this: &Array, value: JsValue, from_index: i32) -> i32;

    /// The includes() method determines whether an array includes a certain element,
    /// returning true or false as appropriate.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/includes
    #[wasm_bindgen(method)]
    pub fn includes(this: &Array, value: JsValue, from_index: i32) -> bool;

    /// The join() method joins all elements of an array (or an array-like object)
    /// into a string and returns this string.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/join
    #[wasm_bindgen(method)]
    pub fn join(this: &Array, delimiter: &str) -> String;

    /// The lastIndexOf() method returns the last index at which a given element can
    /// be found in the array, or -1 if it is not present. The array is searched
    /// backwards, starting at fromIndex.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/lastIndexOf
    #[wasm_bindgen(method, js_name = lastIndexOf)]
    pub fn last_index_of(this: &Array, value: JsValue, from_index: i32) -> i32;

    /// The pop() method removes the last element from an array and returns that element.
    /// This method changes the length of the array.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/pop
    #[wasm_bindgen(method)]
    pub fn pop(this: &Array) -> JsValue;

    /// The push() method adds one or more elements to the end of an array and returns
    /// the new length of the array.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/push
    #[wasm_bindgen(method)]
    pub fn push(this: &Array, value: JsValue) -> u32;

    /// The reverse() method reverses an array in place.
    /// The first array element becomes the last, and the last array element becomes the first.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/reverse
    #[wasm_bindgen(method)]
    pub fn reverse(this: &Array) -> Array;

    /// The slice() method returns a shallow copy of a portion of an array into a new array
    /// object selected from begin to end (end not included).
    /// The original array will not be modified.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/slice
    #[wasm_bindgen(method)]
    pub fn slice(this: &Array, start: u32, end: u32) -> Array;

    /// The shift() method removes the first element from an array and returns that removed element.
    /// This method changes the length of the array.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/shift
    #[wasm_bindgen(method)]
    pub fn shift(this: &Array) -> JsValue;

    /// The sort() method sorts the elements of an array in place and returns
    /// the array. The sort is not necessarily stable. The default sort
    /// order is according to string Unicode code points.
    ///
    /// The time and space complexity of the sort cannot be guaranteed as it
    /// is implementation dependent.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/sort
    #[wasm_bindgen(method)]
    pub fn sort(this: &Array) -> Array;

    /// The toString() method returns a string representing the specified array and its elements.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/toString
    #[wasm_bindgen(method, js_name = toString)]
    pub fn to_string(this: &Array) -> String;

    /// The unshift() method adds one or more elements to the beginning of an array
    /// and returns the new length of the array.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/unshift
    #[wasm_bindgen(method)]
    pub fn unshift(this: &Array, value: JsValue) -> u32;
}

// Array Iterator
#[wasm_bindgen]
extern {
    pub type ArrayIterator;

    /// The keys() method returns a new Array Iterator object that contains the keys for each index in the array.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/keys
    #[wasm_bindgen(method)]
    pub fn keys(this: &Array) -> ArrayIterator;

    /// The entries() method returns a new Array Iterator object that contains the key/value pairs for each index in the array.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/entries
    #[wasm_bindgen(method)]
    pub fn entries(this: &Array) -> ArrayIterator;
}

// Number.
#[wasm_bindgen]
extern {
    pub type Number;

    /// The toLocaleString() method returns a string with a language sensitive
    /// representation of this number.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/toLocaleString
    #[wasm_bindgen(method, js_name = toLocaleString)]
    pub fn to_locale_string(this: &Number, locale: String) -> String;

    /// The toPrecision() method returns a string representing the Number
    /// object to the specified precision.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/toPrecision
    #[wasm_bindgen(catch, method, js_name = toPrecision)]
    pub fn to_precision(this: &Number, precision: u8) -> Result<String, JsValue>;

    /// The toString() method returns a string representing the
    /// specified Number object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/toString
    #[wasm_bindgen(catch, method, js_name = toString)]
    pub fn to_string(this: &Number, radix: u8) -> Result<String, JsValue>;

    /// The valueOf() method returns the wrapped primitive value of
    /// a Number object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/valueOf
    #[wasm_bindgen(method, js_name = valueOf)]
    pub fn value_of(this: &Number) -> Number;

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
    pub fn has_own_property(this: &Object, property: &JsValue) -> bool;

    /// The toLocaleString() method returns a string representing the object.
    /// This method is meant to be overridden by derived objects for locale-specific
    /// purposes.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/toLocaleString
    #[wasm_bindgen(method, js_name = toLocaleString)]
    pub fn to_locale_string(this: &Object) -> String;

    /// The toString() method returns a string representing the object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/toString
    #[wasm_bindgen(method, js_name = toString)]
    pub fn to_string(this: &Object) -> String;

    /// The isPrototypeOf() method checks if an object exists in another
    /// object's prototype chain.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/isPrototypeOf
    #[wasm_bindgen(method, js_name = isPrototypeOf)]
    pub fn is_prototype_of(this: &Object, value: &JsValue) -> bool;

    /// The propertyIsEnumerable() method returns a Boolean indicating
    /// whether the specified property is enumerable.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/propertyIsEnumerable
    #[wasm_bindgen(method, js_name = propertyIsEnumerable)]
    pub fn property_is_enumerable(this: &Object, property: &JsValue) -> bool;
}

// String
#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_name = String)]
    pub type JsString;

    /// The slice() method extracts a section of a string and returns it as a
    /// new string, without modifying the original string.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/slice
    #[wasm_bindgen(method, js_class = "String")]
    pub fn slice(this: &JsString, start: u32, end: u32) -> JsString;
}
