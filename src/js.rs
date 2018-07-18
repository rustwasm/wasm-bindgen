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

use core::mem;

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
// * Add a new `#[test]` into the appropriate file in the
// `tests/all/js_globals/` directory. If the imported function or
// method can throw an exception, make sure to also add test coverage
// for that case.

#[wasm_bindgen]
extern "C" {
    /// The `decodeURI()` function decodes a Uniform Resource Identifier (URI)
    /// previously created by `encodeURI` or by a similar routine.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/decodeURI
    #[wasm_bindgen(catch, js_name = decodeURI)]
    pub fn decode_uri(encoded: &str) -> Result<JsString, JsValue>;

    /// The decodeURIComponent() function decodes a Uniform Resource Identifier (URI) component
    /// previously created by encodeURIComponent or by a similar routine.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/decodeURIComponent
    #[wasm_bindgen(catch, js_name = decodeURIComponent)]
    pub fn decode_uri_component(encoded: &str) -> Result<JsString, JsValue>;

    /// The `encodeURI()` function encodes a Uniform Resource Identifier (URI)
    /// by replacing each instance of certain characters by one, two, three, or
    /// four escape sequences representing the UTF-8 encoding of the character
    /// (will only be four escape sequences for characters composed of two
    /// "surrogate" characters).
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURI
    #[wasm_bindgen(js_name = encodeURI)]
    pub fn encode_uri(decoded: &str) -> JsString;

    /// The encodeURIComponent() function encodes a Uniform Resource Identifier (URI) component
    /// by replacing each instance of certain characters by one, two, three, or four escape sequences
    /// representing the UTF-8 encoding of the character
    /// (will only be four escape sequences for characters composed of two "surrogate" characters).
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURIComponent
    #[wasm_bindgen(js_name = encodeURIComponent)]
    pub fn encode_uri_component(decoded: &str) -> JsString;

    /// The `eval()` function evaluates JavaScript code represented as a string.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/eval
    #[wasm_bindgen(catch)]
    pub fn eval(js_source_text: &str) -> Result<JsValue, JsValue>;

    /// The global isFinite() function determines whether the passed value is a finite number.
    /// If  needed, the parameter is first converted to a number.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/isFinite
    #[wasm_bindgen(js_name = isFinite)]
    pub fn is_finite(value: &JsValue) -> bool;

    /// The `parseInt()` function parses a string argument and returns an integer
    /// of the specified radix (the base in mathematical numeral systems), or NaN on error.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/parseInt
    #[wasm_bindgen(js_name = parseInt)]
    pub fn parse_int(text: &str, radix: u8) -> f64;

    /// The parseFloat() function parses an argument and returns a floating point number,
    /// or NaN on error.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/parseFloat
    #[wasm_bindgen(js_name = parseFloat)]
    pub fn parse_float(text: &str) -> f64;

    /// The escape() function computes a new string in which certain characters have been
    /// replaced by a hexadecimal escape sequence.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/escape
    #[wasm_bindgen]
    pub fn escape(string: &str) -> JsString;
}

// Array
#[wasm_bindgen]
extern "C" {
    pub type Array;

    /// The copyWithin() method shallow copies part of an array to another
    /// location in the same array and returns it, without modifying its size.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/copyWithin
    #[wasm_bindgen(method, js_name = copyWithin)]
    pub fn copy_within(this: &Array, target: i32, start: i32, end: i32) -> Array;

    /// The concat() method is used to merge two or more arrays. This method
    /// does not change the existing arrays, but instead returns a new array.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/concat
    #[wasm_bindgen(method)]
    pub fn concat(this: &Array, array: &Array) -> Array;

    /// The every() method tests whether all elements in the array pass the test
    /// implemented by the provided function.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/every
    #[wasm_bindgen(method)]
    pub fn every(this: &Array, predicate: &mut FnMut(JsValue, u32, Array) -> bool) -> bool;

    /// The fill() method fills all the elements of an array from a start index
    /// to an end index with a static value. The end index is not included.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/fill
    #[wasm_bindgen(method)]
    pub fn fill(this: &Array, value: JsValue, start: u32, end: u32) -> Array;

    /// The `filter()` method creates a new array with all elements that pass the
    /// test implemented by the provided function.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/filter
    #[wasm_bindgen(method)]
    pub fn filter(this: &Array, predicate: &mut FnMut(JsValue, u32, Array) -> bool) -> Array;

    /// The `find()` method returns the value of the first element in the array that satisfies
    ///  the provided testing function. Otherwise `undefined` is returned.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/find
    #[wasm_bindgen(method)]
    pub fn find(this: &Array, predicate: &mut FnMut(JsValue, u32, Array) -> bool) -> JsValue;

    /// The findIndex() method returns the index of the first element in the array that
    /// satisfies the provided testing function. Otherwise -1 is returned.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/findIndex
    #[wasm_bindgen(method, js_name = findIndex)]
    pub fn find_index(this: &Array, predicate: &mut FnMut(JsValue, u32, Array) -> bool) -> u32;

    /// The `forEach()` method executes a provided function once for each array element.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/forEach
    #[wasm_bindgen(method, js_name = forEach)]
    pub fn for_each(this: &Array, callback: &mut FnMut(JsValue, u32, Array));

    /// The includes() method determines whether an array includes a certain
    /// element, returning true or false as appropriate.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/includes
    #[wasm_bindgen(method)]
    pub fn includes(this: &Array, value: JsValue, from_index: i32) -> bool;

    /// The indexOf() method returns the first index at which a given element
    /// can be found in the array, or -1 if it is not present.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/indexOf
    #[wasm_bindgen(method, js_name = indexOf)]
    pub fn index_of(this: &Array, value: JsValue, from_index: i32) -> i32;

    /// The Array.isArray() method determines whether the passed value is an Array.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/isArray
    #[wasm_bindgen(static_method_of = Array, js_name = isArray)]
    pub fn is_array(value: &JsValue) -> bool;

    /// The join() method joins all elements of an array (or an array-like object)
    /// into a string and returns this string.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/join
    #[wasm_bindgen(method)]
    pub fn join(this: &Array, delimiter: &str) -> JsString;

    /// The lastIndexOf() method returns the last index at which a given element
    /// can be found in the array, or -1 if it is not present. The array is
    /// searched backwards, starting at fromIndex.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/lastIndexOf
    #[wasm_bindgen(method, js_name = lastIndexOf)]
    pub fn last_index_of(this: &Array, value: JsValue, from_index: i32) -> i32;

    /// The length property of an object which is an instance of type Array
    /// sets or returns the number of elements in that array. The value is an
    /// unsigned, 32-bit integer that is always numerically greater than the
    /// highest index in the array.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/length
    #[wasm_bindgen(method, getter, structural)]
    pub fn length(this: &Array) -> u32;

    /// map calls a provided callback function once for each element in an array,
    /// in order, and constructs a new array from the results. callback is invoked
    /// only for indexes of the array which have assigned values, including undefined.
    /// It is not called for missing elements of the array (that is, indexes that have
    /// never been set, which have been deleted or which have never been assigned a value).
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/map
    #[wasm_bindgen(method)]
    pub fn map(this: &Array, predicate: &mut FnMut(JsValue, u32, Array) -> JsValue) -> Array;

    /// The pop() method removes the last element from an array and returns that
    /// element. This method changes the length of the array.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/pop
    #[wasm_bindgen(method)]
    pub fn pop(this: &Array) -> JsValue;

    /// The push() method adds one or more elements to the end of an array and
    /// returns the new length of the array.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/push
    #[wasm_bindgen(method)]
    pub fn push(this: &Array, value: JsValue) -> u32;

    /// The reduce() method applies a function against an accumulator and each element in
    /// the array (from left to right) to reduce it to a single value.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/Reduce
    #[wasm_bindgen(method)]
    pub fn reduce(this: &Array, predicate: &mut FnMut(JsValue, JsValue, u32, Array) -> JsValue, initial_value: JsValue) -> JsValue;

    /// The reduceRight() method applies a function against an accumulator and each value
    /// of the array (from right-to-left) to reduce it to a single value.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/ReduceRight
    #[wasm_bindgen(method, js_name = reduceRight)]
    pub fn reduce_right(this: &Array, predicate: &mut FnMut(JsValue, JsValue, u32, Array) -> JsValue, initial_value: JsValue) -> JsValue;

    /// The reverse() method reverses an array in place. The first array
    /// element becomes the last, and the last array element becomes the first.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/reverse
    #[wasm_bindgen(method)]
    pub fn reverse(this: &Array) -> Array;

    /// The shift() method removes the first element from an array and returns
    /// that removed element. This method changes the length of the array.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/shift
    #[wasm_bindgen(method)]
    pub fn shift(this: &Array) -> JsValue;

    /// The slice() method returns a shallow copy of a portion of an array into
    /// a new array object selected from begin to end (end not included).
    /// The original array will not be modified.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/slice
    #[wasm_bindgen(method)]
    pub fn slice(this: &Array, start: u32, end: u32) -> Array;

    /// The some() method tests whether at least one element in the array passes the test implemented
    /// by the provided function.
    /// Note: This method returns false for any condition put on an empty array.
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/some
    #[wasm_bindgen(method)]
    pub fn some(this: &Array, predicate: &mut FnMut(JsValue) -> bool) -> bool;

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

    /// The toLocaleString() method returns a string representing the elements of the array.
    /// The elements are converted to Strings using their toLocaleString methods and these
    /// Strings are separated by a locale-specific String (such as a comma “,”).
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/toLocaleString
    #[wasm_bindgen(method, js_name = toLocaleString)]
    pub fn to_locale_string(this: &Array, locales: &JsValue, options: &JsValue) -> JsString;

    /// The toString() method returns a string representing the specified array
    /// and its elements.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/toString
    #[wasm_bindgen(method, js_name = toString)]
    pub fn to_string(this: &Array) -> JsString;

    /// The unshift() method adds one or more elements to the beginning of an
    /// array and returns the new length of the array.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/unshift
    #[wasm_bindgen(method)]
    pub fn unshift(this: &Array, value: JsValue) -> u32;
}

// ArrayBuffer
#[wasm_bindgen]
extern "C" {
    pub type ArrayBuffer;

    /// The `ArrayBuffer` object is used to represent a generic,
    /// fixed-length raw binary data buffer. You cannot directly
    /// manipulate the contents of an `ArrayBuffer`; instead, you
    /// create one of the typed array objects or a `DataView` object
    /// which represents the buffer in a specific format, and use that
    /// to read and write the contents of the buffer.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer
    #[wasm_bindgen(constructor)]
    pub fn new(length: u32) -> ArrayBuffer;

    /// The `isView()` method returns true if arg is one of the `ArrayBuffer`
    /// views, such as typed array objects or a DataView; false otherwise.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/isView
    #[wasm_bindgen(static_method_of = ArrayBuffer, js_name = isView)]
    pub fn is_view(value: JsValue) -> bool;

    /// The `slice()` method returns a new `ArrayBuffer` whose contents
    /// are a copy of this `ArrayBuffer`'s bytes from begin, inclusive,
    /// up to end, exclusive.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/slice
    #[wasm_bindgen(method)]
    pub fn slice(this: &ArrayBuffer, begin: u32) -> ArrayBuffer;

    /// Like `slice()` but with the `end` argument.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/slice
    #[wasm_bindgen(method, js_name = slice)]
    pub fn slice_with_end(this: &ArrayBuffer, begin: u32, end: u32) -> ArrayBuffer;
}

// Array Iterator
#[wasm_bindgen]
extern "C" {
    pub type ArrayIterator;

    /// The keys() method returns a new Array Iterator object that contains the
    /// keys for each index in the array.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/keys
    #[wasm_bindgen(method)]
    pub fn keys(this: &Array) -> ArrayIterator;

    /// The entries() method returns a new Array Iterator object that contains
    /// the key/value pairs for each index in the array.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/entries
    #[wasm_bindgen(method)]
    pub fn entries(this: &Array) -> ArrayIterator;

    /// The values() method returns a new Array Iterator object that
    /// contains the values for each index in the array.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/values
    #[wasm_bindgen(method)]
    pub fn values(this: &Array) -> ArrayIterator;
}

// Boolean
#[wasm_bindgen]
extern "C" {
    pub type Boolean;

    /// The `Boolean()` constructor creates an object wrapper for a boolean value.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Boolean
    #[wasm_bindgen(constructor)]
    pub fn new(value: JsValue) -> Boolean;

    /// The `valueOf()` method returns the primitive value of a `Boolean` object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Boolean/valueOf
    #[wasm_bindgen(method, js_name = valueOf)]
    pub fn value_of(this: &Boolean) -> bool;
}

// DataView
#[wasm_bindgen]
extern "C" {
    pub type DataView;

    /// The `DataView` view provides a low-level interface for reading and
    /// writing multiple number types in an `ArrayBuffer` irrespective of the
    /// platform's endianness.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView
    #[wasm_bindgen(constructor)]
    pub fn new(buffer: &ArrayBuffer, byteOffset: usize, byteLength: usize) -> DataView;

    /// The ArrayBuffer referenced by this view. Fixed at construction time and thus read only.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/buffer
    #[wasm_bindgen(method, getter, structural)]
    pub fn buffer(this: &DataView) -> ArrayBuffer;

    /// The length (in bytes) of this view from the start of its ArrayBuffer.
    /// Fixed at construction time and thus read only.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/byteLength
    #[wasm_bindgen(method, getter, structural, js_name = byteLength)]
    pub fn byte_length(this: &DataView) -> usize;

    /// The offset (in bytes) of this view from the start of its ArrayBuffer.
    /// Fixed at construction time and thus read only.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/byteOffset
    #[wasm_bindgen(method, getter, structural, js_name = byteOffset)]
    pub fn byte_offset(this: &DataView) -> usize;

    /// The getInt8() method gets a signed 8-bit integer (byte) at the
    /// specified byte offset from the start of the DataView.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getInt8
    #[wasm_bindgen(method, js_name = getInt8)]
    pub fn get_int8(this: &DataView, byte_offset: usize) -> i8;

    /// The getUint8() method gets a unsigned 8-bit integer (byte) at the specified
    /// byte offset from the start of the DataView.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getUint8
    #[wasm_bindgen(method, js_name = getUint8)]
    pub fn get_uint8(this: &DataView, byte_offset: usize) -> u8;

    /// The getInt16() method gets a signed 16-bit integer (byte) at the specified
    /// byte offset from the start of the DataView.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getInt16
    #[wasm_bindgen(method, js_name = getInt16)]
    pub fn get_int16(this: &DataView, byte_offset: usize) -> i16;

    /// The getUint16() an unsigned 16-bit integer (unsigned byte) at the specified
    /// byte offset from the start of the view.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getUint16
    #[wasm_bindgen(method, js_name = getUint16)]
    pub fn get_uint16(this: &DataView, byte_offset: usize) -> u16;

    /// The getInt32() method gets a signed 16-bit integer (byte) at the specified
    /// byte offset from the start of the DataView.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getInt32
    #[wasm_bindgen(method, js_name = getInt32)]
    pub fn get_int32(this: &DataView, byte_offset: usize) -> i32;

    /// The getUint32() an unsigned 16-bit integer (unsigned byte) at the specified
    /// byte offset from the start of the view.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getUint32
    #[wasm_bindgen(method, js_name = getUint32)]
    pub fn get_uint32(this: &DataView, byte_offset: usize) -> u32;

    /// The getFloat32() method gets a signed 32-bit float (float) at the specified
    /// byte offset from the start of the DataView.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getFloat32
    #[wasm_bindgen(method, js_name = getFloat32)]
    pub fn get_float32(this: &DataView, byte_offset: usize) -> f32;

    /// The getFloat64() method gets a signed 32-bit float (float) at the specified
    /// byte offset from the start of the DataView.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getFloat64
    #[wasm_bindgen(method, js_name = getFloat64)]
    pub fn get_float64(this: &DataView, byte_offset: usize) -> f64;

    /// The setInt8() method stores a signed 8-bit integer (byte) value at the
    /// specified byte offset from the start of the DataView.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setInt8
    #[wasm_bindgen(method, js_name = setInt8)]
    pub fn set_int8(this: &DataView, byte_offset: usize, value: i8);

    /// The setUint8() method stores an unsigned 8-bit integer (byte) value at the
    /// specified byte offset from the start of the DataView.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setUint8
    #[wasm_bindgen(method, js_name = setUint8)]
    pub fn set_uint8(this: &DataView, byte_offset: usize, value: u8);

    /// The setInt16() method stores a signed 16-bit integer (byte) value at the
    /// specified byte offset from the start of the DataView.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setInt16
    #[wasm_bindgen(method, js_name = setInt16)]
    pub fn set_int16(this: &DataView, byte_offset: usize, value: i16);

    /// The setUint16() method stores an unsigned 16-bit integer (byte) value at the
    /// specified byte offset from the start of the DataView.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setUint16
    #[wasm_bindgen(method, js_name = setUint16)]
    pub fn set_uint16(this: &DataView, byte_offset: usize, value: u16);

    /// The setInt32() method stores a signed 32-bit integer (byte) value at the
    /// specified byte offset from the start of the DataView.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setInt32
    #[wasm_bindgen(method, js_name = setInt32)]
    pub fn set_int32(this: &DataView, byte_offset: usize, value: i32);

    /// The setUint32() method stores an unsigned 32-bit integer (byte) value at the
    /// specified byte offset from the start of the DataView.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setUint32
    #[wasm_bindgen(method, js_name = setUint32)]
    pub fn set_uint32(this: &DataView, byte_offset: usize, value: u32);

    /// The setFloat32() method stores a signed 32-bit float (float) value at the
    /// specified byte offset from the start of the DataView.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setFloat32
    #[wasm_bindgen(method, js_name = setFloat32)]
    pub fn set_float32(this: &DataView, byte_offset: usize, value: f32);

    /// The setFloat64() method stores a signed 64-bit float (float) value at the
    /// specified byte offset from the start of the DataView.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setFloat64
    #[wasm_bindgen(method, js_name = setFloat64)]
    pub fn set_float64(this: &DataView, byte_offset: usize, value: f64);
}

// Error
#[wasm_bindgen]
extern "C" {
    pub type Error;

    /// The Error constructor creates an error object.
    /// Instances of Error objects are thrown when runtime errors occur.
    /// The Error object can also be used as a base object for user-defined exceptions.
    /// See below for standard built-in error types.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error
    #[wasm_bindgen(constructor)]
    pub fn new(message: &JsString) -> Error;

    /// The message property is a human-readable description of the error.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/message
    #[wasm_bindgen(method, getter, structural)]
    pub fn message(this: &Error) -> JsString;
    #[wasm_bindgen(method, setter, structural)]
    pub fn set_message(this: &Error, message: &JsString);

    /// The name property represents a name for the type of error. The initial value is "Error".
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/name
    #[wasm_bindgen(method, getter, structural)]
    pub fn name(this: &Error) -> JsString;
    #[wasm_bindgen(method, setter, structural)]
    pub fn set_name(this: &Error, name: &JsString);

    /// The toString() method returns a string representing the specified Error object
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/toString
    #[wasm_bindgen(method, js_name = toString)]
    pub fn to_string(this: &Error) -> JsString;
}

// Float32Array
#[wasm_bindgen]
extern "C" {
    pub type Float32Array;

    /// The `Float32Array()` constructor creates an array of 32-bit floats.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Float32Array
    #[wasm_bindgen(constructor)]
    pub fn new(constructor_arg: JsValue) -> Float32Array;

    /// The fill() method fills all the elements of an array from a start index
    /// to an end index with a static value. The end index is not included.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/fill
    #[wasm_bindgen(method)]
    pub fn fill(this: &Float32Array, value: f32, start: u32, end: u32) -> Float32Array;
}

// Float64Array
#[wasm_bindgen]
extern "C" {
    pub type Float64Array;

    /// The `Float64Array()` constructor creates an array of 64-bit floats.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Float64Array
    #[wasm_bindgen(constructor)]
    pub fn new(constructor_arg: JsValue) -> Float64Array;

    /// The fill() method fills all the elements of an array from a start index
    /// to an end index with a static value. The end index is not included.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/fill
    #[wasm_bindgen(method)]
    pub fn fill(this: &Float64Array, value: f64, start: u32, end: u32) -> Float64Array;
}

// Function
#[wasm_bindgen]
extern "C" {
    pub type Function;

    /// The apply() method calls a function with a given this value, and arguments provided as an array
    /// (or an array-like object).
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/apply
    #[wasm_bindgen(method)]
    pub fn apply(this: &Function, context: &JsValue, args: &Array) -> Function;

    /// The bind() method creates a new function that, when called, has its this keyword set to the provided value,
    /// with a given sequence of arguments preceding any provided when the new function is called.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/bind
    #[wasm_bindgen(method)]
    pub fn bind(this: &Function, context: &JsValue) -> Function;

    /// The length property indicates the number of arguments expected by the function.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/length
    #[wasm_bindgen(method, getter, structural)]
    pub fn length(this: &Function) -> u32;

    /// A Function object's read-only name property indicates the function's
    /// name as specified when it was created or "anonymous" for functions
    /// created anonymously.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/name
    #[wasm_bindgen(method, getter, structural)]
    pub fn name(this: &Function) -> JsString;

    /// The toString() method returns a string representing the source code of the function.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/toString
    #[wasm_bindgen(method, js_name = toString)]
    pub fn to_string(this: &Function) -> JsString;
}

impl JsValue {
    /// Returns the `Function` value of this JS value if it's an instance of a
    /// function.
    ///
    /// If this JS value is not an instance of a function then this returns
    /// `None`.
    pub fn as_function(&self) -> Option<&Function> {
        if self.is_function() {
            Some(unsafe { mem::transmute(self) })
        } else {
            None
        }
    }
}

// Generator
#[wasm_bindgen]
extern {
    pub type Generator;

    /// The next() method returns an object with two properties done and value.
    /// You can also provide a parameter to the next method to send a value to the generator.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Generator/next
    #[wasm_bindgen(method, structural, catch)]
    pub fn next(this: &Generator, value: &JsValue) -> Result<JsValue, JsValue>;

    /// The return() method returns the given value and finishes the generator.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Generator/return
    #[wasm_bindgen(method, structural, js_name = return)]
    pub fn return_(this: &Generator, value: &JsValue) -> JsValue;

    /// The throw() method resumes the execution of a generator by throwing an error into it
    /// and returns an object with two properties done and value.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Generator/throw
    #[wasm_bindgen(method, structural, catch)]
    pub fn throw(this: &Generator, error: &Error) -> Result<JsValue, JsValue>;
}

// Int8Array
#[wasm_bindgen]
extern "C" {
    pub type Int8Array;

    /// The `Int8Array()` constructor creates an array of signed 8-bit integers.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Int8Array
    #[wasm_bindgen(constructor)]
    pub fn new(constructor_arg: JsValue) -> Int8Array;

    /// The fill() method fills all the elements of an array from a start index
    /// to an end index with a static value. The end index is not included.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/fill
    #[wasm_bindgen(method)]
    pub fn fill(this: &Int8Array, value: i8, start: u32, end: u32) -> Int8Array;
}

// Int16Array
#[wasm_bindgen]
extern "C" {
    pub type Int16Array;

    /// The `Int16Array()` constructor creates an array of signed 16-bit integers.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Int16Array
    #[wasm_bindgen(constructor)]
    pub fn new(constructor_arg: JsValue) -> Int16Array;

    /// The fill() method fills all the elements of an array from a start index
    /// to an end index with a static value. The end index is not included.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/fill
    #[wasm_bindgen(method)]
    pub fn fill(this: &Int16Array, value: i16, start: u32, end: u32) -> Int16Array;
}

// Int32Array
#[wasm_bindgen]
extern "C" {
    pub type Int32Array;

    /// The `Int32Array()` constructor creates an array of signed 32-bit integers.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Int32Array
    #[wasm_bindgen(constructor)]
    pub fn new(constructor_arg: JsValue) -> Int32Array;

    /// The fill() method fills all the elements of an array from a start index
    /// to an end index with a static value. The end index is not included.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/fill
    #[wasm_bindgen(method)]
    pub fn fill(this: &Int32Array, value: i32, start: u32, end: u32) -> Int32Array;
}

// Map
#[wasm_bindgen]
extern {
    pub type Map;

    /// The clear() method removes all elements from a Map object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/clear
    #[wasm_bindgen(method)]
    pub fn clear(this: &Map);

    /// The delete() method removes the specified element from a Map object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/delete
    #[wasm_bindgen(method)]
    pub fn delete(this: &Map, key: &str) -> bool;

    /// The forEach() method executes a provided function once per each
    /// key/value pair in the Map object, in insertion order.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/forEach
    #[wasm_bindgen(method, js_name = forEach)]
    pub fn for_each(this: &Map, callback: &mut FnMut(JsValue, JsValue));

    /// The get() method returns a specified element from a Map object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/get
    #[wasm_bindgen(method)]
    pub fn get(this: &Map, key: &JsValue) -> JsValue;

    /// The has() method returns a boolean indicating whether an element with
    /// the specified key exists or not.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/has
    #[wasm_bindgen(method)]
    pub fn has(this: &Map, key: &JsValue) -> bool;

    /// The Map object holds key-value pairs. Any value (both objects and
    /// primitive values) maybe used as either a key or a value.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map
    #[wasm_bindgen(constructor)]
    pub fn new() -> Map;

    /// The set() method adds or updates an element with a specified key
    /// and value to a Map object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/set
    #[wasm_bindgen(method)]
    pub fn set(this: &Map, key: &JsValue, value: &JsValue) -> Map;

    /// The value of size is an integer representing how many entries
    /// the Map object has. A set accessor function for size is undefined;
    /// you can not change this property.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/size
    #[wasm_bindgen(method, getter, structural)]
    pub fn size(this: &Map) -> u32;
}

// Map Iterator
#[wasm_bindgen]
extern {
    pub type MapIterator;

    /// The entries() method returns a new Iterator object that contains
    /// the [key, value] pairs for each element in the Map object in
    /// insertion order.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/entries
    #[wasm_bindgen(method)]
    pub fn entries(this: &Map) -> MapIterator;

    /// The keys() method returns a new Iterator object that contains the
    /// keys for each element in the Map object in insertion order.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/keys
    #[wasm_bindgen(method)]
    pub fn keys(this: &Map) -> MapIterator;

    /// The values() method returns a new Iterator object that contains the
    /// values for each element in the Map object in insertion order.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/values
    #[wasm_bindgen(method)]
    pub fn values(this: &Map) -> MapIterator;
}

// Math
#[wasm_bindgen]
extern "C" {
    pub type Math;

    /// The Math.abs() function returns the absolute value of a number, that is
    /// Math.abs(x) = |x|
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/abs
    #[wasm_bindgen(static_method_of = Math)]
    pub fn abs(x: f64) -> f64;

    /// The Math.acos() function returns the arccosine (in radians) of a
    /// number, that is ∀x∊[-1;1]
    /// Math.acos(x) = arccos(x) = the unique y∊[0;π] such that cos(y)=x
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/acos
    #[wasm_bindgen(static_method_of = Math)]
    pub fn acos(x: f64) -> f64;

    /// The Math.acosh() function returns the hyperbolic arc-cosine of a
    /// number, that is ∀x ≥ 1
    /// Math.acosh(x) = arcosh(x) = the unique y ≥ 0 such that cosh(y) = x
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/acosh
    #[wasm_bindgen(static_method_of = Math)]
    pub fn acosh(x: f64) -> f64;

    /// The Math.asin() function returns the arcsine (in radians) of a
    /// number, that is ∀x ∊ [-1;1]
    /// Math.asin(x) = arcsin(x) = the unique y∊[-π2;π2] such that sin(y) = x
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/asin
    #[wasm_bindgen(static_method_of = Math)]
    pub fn asin(x: f64) -> f64;

    /// The Math.asinh() function returns the hyperbolic arcsine of a
    /// number, that is Math.asinh(x) = arsinh(x) = the unique y such that sinh(y) = x
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/asinh
    #[wasm_bindgen(static_method_of = Math)]
    pub fn asinh(x: f64) -> f64;

    /// The Math.atan() function returns the arctangent (in radians) of a
    /// number, that is Math.atan(x) = arctan(x) = the unique y ∊ [-π2;π2]such that
    /// tan(y) = x
    #[wasm_bindgen(static_method_of = Math)]
    pub fn atan(x: f64) -> f64;

    /// The Math.atan2() function returns the arctangent of the quotient of
    /// its arguments.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/atan2
    #[wasm_bindgen(static_method_of = Math)]
    pub fn atan2(y: f64, x: f64) -> f64;

    /// The Math.atanh() function returns the hyperbolic arctangent of a number,
    /// that is ∀x ∊ (-1,1), Math.atanh(x) = arctanh(x) = the unique y such that
    /// tanh(y) = x
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/atanh
    #[wasm_bindgen(static_method_of = Math)]
    pub fn atanh(x: f64) -> f64;

    /// The Math.cbrt() function returns the cube root of a number, that is
    /// Math.cbrt(x) = x^3 = the unique y such that y^3 = x
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/cbrt
    #[wasm_bindgen(static_method_of = Math)]
    pub fn cbrt(x: f64) -> f64;

    /// The Math.ceil() function returns the smallest integer greater than
    /// or equal to a given number.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/ceil
    #[wasm_bindgen(static_method_of = Math)]
    pub fn ceil(x: f64) -> i32;

    /// The Math.clz32() function returns the number of leading zero bits in
    /// the 32-bit binary representation of a number.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/clz32
    #[wasm_bindgen(static_method_of = Math)]
    pub fn clz32(x: i32) -> u32;

    /// The Math.cos() static function returns the cosine of the specified angle,
    /// which must be specified in radians. This value is length(adjacent)/length(hypotenuse).
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/cos
    #[wasm_bindgen(static_method_of = Math)]
    pub fn cos(x: f64) -> f64;


    /// The Math.cosh() function returns the hyperbolic cosine of a number,
    /// that can be expressed using the constant e.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/cosh
    #[wasm_bindgen(static_method_of = Math)]
    pub fn cosh(x: f64) -> f64;

    /// The Math.exp() function returns e^x, where x is the argument, and e is Euler's number
    /// (also known as Napier's constant), the base of the natural logarithms.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/exp
    #[wasm_bindgen(static_method_of = Math)]
    pub fn exp(x: f64) -> f64;

    /// The Math.expm1() function returns e^x - 1, where x is the argument, and e the base of the
    /// natural logarithms.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/expm1
    #[wasm_bindgen(static_method_of = Math)]
    pub fn expm1(x: f64) -> f64;

    /// The Math.floor() function returns the largest integer less than or
    /// equal to a given number.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/floor
    #[wasm_bindgen(static_method_of = Math)]
    pub fn floor(x: f64) -> i32;

    /// The Math.fround() function returns the nearest 32-bit single precision float representation
    /// of a Number.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/fround
    #[wasm_bindgen(static_method_of = Math)]
    pub fn fround(x: f64) -> f32;

    /// The Math.imul() function returns the result of the C-like 32-bit multiplication of the
    /// two parameters.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/imul
    #[wasm_bindgen(static_method_of = Math)]
    pub fn imul(x: i32, y: i32) -> i32;

    /// The Math.log() function returns the natural logarithm (base e) of a number.
    /// The JavaScript Math.log() function is equivalent to ln(x) in mathematics.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/log
    #[wasm_bindgen(static_method_of = Math)]
    pub fn log(x: f64) -> f64;

    /// The Math.log10() function returns the base 10 logarithm of a number.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/log10
    #[wasm_bindgen(static_method_of = Math)]
    pub fn log10(x: f64) -> f64;

    /// The Math.log1p() function returns the natural logarithm (base e) of 1 + a number.
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/log1p
    #[wasm_bindgen(static_method_of = Math)]
    pub fn log1p(x: f64) -> f64;

    /// The Math.log2() function returns the base 2 logarithm of a number.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/log2
    #[wasm_bindgen(static_method_of = Math)]
    pub fn log2(x: f64) -> f64;

    /// The Math.pow() function returns the base to the exponent power, that is, base^exponent.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/pow
    #[wasm_bindgen(static_method_of = Math)]
    pub fn pow(base: f64, exponent: f64) -> f64;

    /// The Math.random() function returns a floating-point, pseudo-random number 
    /// in the range 0–1 (inclusive of 0, but not 1) with approximately uniform distribution 
    /// over that range — which you can then scale to your desired range. 
    /// The implementation selects the initial seed to the random number generation algorithm; 
    /// it cannot be chosen or reset by the user.
    /// 
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/random    
    #[wasm_bindgen(static_method_of = Math)]
    pub fn random() -> f64;

    /// The Math.round() function returns the value of a number rounded to the nearest integer.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/round
    #[wasm_bindgen(static_method_of = Math)]
    pub fn round(x: f64) -> i32;

    /// The Math.sign() function returns the sign of a number, indicating whether the number is
    /// positive, negative or zero.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/sign
    #[wasm_bindgen(static_method_of = Math)]
    pub fn sign(x: f64) -> f64;

    /// The Math.sin() function returns the sine of a number.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/sin
    #[wasm_bindgen(static_method_of = Math)]
    pub fn sin(x: f64) -> f64;

    /// The Math.sinh() function returns the hyperbolic sine of a number, that can be expressed
    /// using the constant e: Math.sinh(x) = (e^x - e^-x)/2
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/sinh
    #[wasm_bindgen(static_method_of = Math)]
    pub fn sinh(x: f64) -> f64;

    /// The Math.sqrt() function returns the square root of a number, that is
    /// ∀x ≥ 0, Math.sqrt(x) = √x = the unique y ≥ 0 such that y^2 = x
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/sqrt
    #[wasm_bindgen(static_method_of = Math)]
    pub fn sqrt(x: f64) -> f64;

    /// The Math.tan() function returns the tangent of a number.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/tan
    #[wasm_bindgen(static_method_of = Math)]
    pub fn tan(x: f64) -> f64;

    /// The Math.tanh() function returns the hyperbolic tangent of a number, that is
    /// tanh x = sinh x / cosh x = (e^x - e^-x)/(e^x + e^-x) = (e^2x - 1)/(e^2x + 1)
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/tanh
    #[wasm_bindgen(static_method_of = Math)]
    pub fn tanh(x: f64) -> f64;

    /// The Math.trunc() function returns the integer part of a number by removing any fractional
    /// digits.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/trunc
    #[wasm_bindgen(static_method_of = Math)]
    pub fn trunc(x: f64) -> i32;
}

// Number.
#[wasm_bindgen]
extern "C" {
    pub type Number;

    /// The Number.isFinite() method determines whether the passed value is a finite number.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/isFinite
    #[wasm_bindgen(static_method_of = Number, js_name = isFinite)]
    pub fn is_finite(value: &JsValue) -> bool;

    /// The Number.isInteger() method determines whether the passed value is an integer.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/isInteger
    #[wasm_bindgen(static_method_of = Number, js_name = isInteger)]
    pub fn is_integer(value: &JsValue) -> bool;

    /// The Number.isSafeInteger() method determines whether the provided value is a number
    /// that is a safe integer.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/isSafeInteger
    #[wasm_bindgen(static_method_of = Number, js_name = isSafeInteger)]
    pub fn is_safe_integer(value: &JsValue) -> bool;

    /// The `Number` JavaScript object is a wrapper object allowing
    /// you to work with numerical values. A `Number` object is
    /// created using the `Number()` constructor.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number
    #[wasm_bindgen(constructor)]
    pub fn new(value: JsValue) -> Number;

    /// The Number.parseInt() method parses a string argument and returns an
    /// integer of the specified radix or base.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/parseInt
    #[wasm_bindgen(static_method_of = Number, js_name = parseInt)]
    pub fn parse_int(text: &str, radix: u8) -> Number;

    /// The Number.parseFloat() method parses a string argument and returns a
    /// floating point number.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/parseFloat
    #[wasm_bindgen(static_method_of = Number, js_name = parseFloat)]
    pub fn parse_float(text: &str) -> Number;

    /// The toLocaleString() method returns a string with a language sensitive
    /// representation of this number.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/toLocaleString
    #[wasm_bindgen(method, js_name = toLocaleString)]
    pub fn to_locale_string(this: &Number, locale: &str) -> JsString;

    /// The toPrecision() method returns a string representing the Number
    /// object to the specified precision.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/toPrecision
    #[wasm_bindgen(catch, method, js_name = toPrecision)]
    pub fn to_precision(this: &Number, precision: u8) -> Result<JsString, JsValue>;

    /// The toFixed() method returns a string representing the Number
    /// object using fixed-point notation.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/toFixed
    #[wasm_bindgen(catch, method, js_name = toFixed)]
    pub fn to_fixed(this: &Number, digits: u8) -> Result<JsString, JsValue>;

    /// The toExponential() method returns a string representing the Number
    /// object in exponential notation.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/toExponential
    #[wasm_bindgen(catch, method, js_name = toExponential)]
    pub fn to_exponential(this: &Number, fraction_digits: u8) -> Result<JsString, JsValue>;

    /// The toString() method returns a string representing the
    /// specified Number object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/toString
    #[wasm_bindgen(catch, method, js_name = toString)]
    pub fn to_string(this: &Number, radix: u8) -> Result<JsString, JsValue>;

    /// The valueOf() method returns the wrapped primitive value of
    /// a Number object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/valueOf
    #[wasm_bindgen(method, js_name = valueOf)]
    pub fn value_of(this: &Number) -> f64;
}

// Date.
#[wasm_bindgen]
extern "C" {
    pub type Date;

    /// The getDate() method returns the day of the month for the
    /// specified date according to local time.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getDate
    #[wasm_bindgen(method, js_name = getDate)]
    pub fn get_date(this: &Date) -> u32;

    /// The getDay() method returns the day of the week for the specified date according to local time,
    /// where 0 represents Sunday. For the day of the month see getDate().
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getDay
    #[wasm_bindgen(method, js_name = getDay)]
    pub fn get_day(this: &Date) -> u32;

    /// The getFullYear() method returns the year of the specified date according to local time.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getFullYear
    #[wasm_bindgen(method, js_name = getFullYear)]
    pub fn get_full_year(this: &Date) -> u32;

    /// The getHours() method returns the hour for the specified date, according to local time.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getHours
    #[wasm_bindgen(method, js_name = getHours)]
    pub fn get_hours(this: &Date) -> u32;

    /// The getMilliseconds() method returns the milliseconds in the specified date according to local time.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getMilliseconds
    #[wasm_bindgen(method, js_name = getMilliseconds)]
    pub fn get_milliseconds(this: &Date) -> u32;

    /// The getMinutes() method returns the minutes in the specified date according to local time.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getMinutes
    #[wasm_bindgen(method, js_name = getMinutes)]
    pub fn get_minutes(this: &Date) -> u32;

    /// The getMonth() method returns the month in the specified date according to local time,
    /// as a zero-based value (where zero indicates the first month of the year).
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getMonth
    #[wasm_bindgen(method, js_name = getMonth)]
    pub fn get_month(this: &Date) -> u32;

    /// The getSeconds() method returns the seconds in the specified date according to local time.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getSeconds
    #[wasm_bindgen(method, js_name = getSeconds)]
    pub fn get_seconds(this: &Date) -> u32;

    /// The getTime() method returns the numeric value corresponding to the time for the specified date
    /// according to universal time.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getTime
    #[wasm_bindgen(method, js_name = getTime)]
    pub fn get_time(this: &Date) -> f64;

    /// The getTimezoneOffset() method returns the time zone difference, in minutes,
    /// from current locale (host system settings) to UTC.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getTimezoneOffset
    #[wasm_bindgen(method, js_name = getTimezoneOffset)]
    pub fn get_timezone_offset(this: &Date) -> f64;

    /// The getUTCDate() method returns the day (date) of the month in the specified date
    /// according to universal time.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCDate
    #[wasm_bindgen(method, js_name = getUTCDate)]
    pub fn get_utc_date(this: &Date) -> u32;

    /// The getUTCDay() method returns the day of the week in the specified date according to universal time,
    /// where 0 represents Sunday.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCDay
    #[wasm_bindgen(method, js_name = getUTCDay)]
    pub fn get_utc_day(this: &Date) -> u32;

    /// The getUTCFullYear() method returns the year in the specified date according to universal time.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCFullYear
    #[wasm_bindgen(method, js_name = getUTCFullYear)]
    pub fn get_utc_full_year(this: &Date) -> u32;

    /// The getUTCHours() method returns the hours in the specified date according to universal time.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCHours
    #[wasm_bindgen(method, js_name = getUTCHours)]
    pub fn get_utc_hours(this: &Date) -> u32;

    /// The getUTCMilliseconds() method returns the milliseconds in the specified date
    /// according to universal time.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCMilliseconds
    #[wasm_bindgen(method, js_name = getUTCMilliseconds)]
    pub fn get_utc_milliseconds(this: &Date) -> u32;

    /// The getUTCMinutes() method returns the minutes in the specified date according to universal time.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCMinutes
    #[wasm_bindgen(method, js_name = getUTCMinutes)]
    pub fn get_utc_minutes(this: &Date) -> u32;

    /// The getUTCMonth() returns the month of the specified date according to universal time,
    /// as a zero-based value (where zero indicates the first month of the year).
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCMonth
    #[wasm_bindgen(method, js_name = getUTCMonth)]
    pub fn get_utc_month(this: &Date) -> u32;

    /// The getUTCSeconds() method returns the seconds in the specified date according to universal time.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCSeconds
    #[wasm_bindgen(method, js_name = getUTCSeconds)]
    pub fn get_utc_seconds(this: &Date) -> u32;

    /// Creates a JavaScript Date instance that represents
    /// a single moment in time. Date objects are based on a time value that is
    /// the number of milliseconds since 1 January 1970 UTC.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date
    #[wasm_bindgen(constructor)]
    pub fn new() -> Date;

    /// The `Date.now()` method returns the number of milliseconds
    /// elapsed since January 1, 1970 00:00:00 UTC.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/now
    #[wasm_bindgen(static_method_of = Date)]
    pub fn now() -> f64;

    /// The Date.parse() method parses a string representation of a date, and returns the number of milliseconds
    /// since January 1, 1970, 00:00:00 UTC or NaN if the string is unrecognized or, in some cases,
    /// contains illegal date values (e.g. 2015-02-31).
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/parse
    #[wasm_bindgen(static_method_of = Date)]
    pub fn parse(date: JsString) -> f64;

    /// The setDate() method sets the day of the Date object relative to the beginning of the currently set month.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setDate
    #[wasm_bindgen(method, js_name = setDate)]
    pub fn set_date(this: &Date, day: u32) -> f64;

    /// The setFullYear() method sets the full year for a specified date according to local time.
    /// Returns new timestamp.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setFullYear
    #[wasm_bindgen(method, js_name = setFullYear)]
    pub fn set_full_year(this: &Date, year: u32) -> f64;

    /// The setHours() method sets the hours for a specified date according to local time,
    /// and returns the number of milliseconds since January 1, 1970 00:00:00 UTC until the time represented
    /// by the updated Date instance.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setHours
    #[wasm_bindgen(method, js_name = setHours)]
    pub fn set_hours(this: &Date, hours: u32) -> f64;

    /// The setMilliseconds() method sets the milliseconds for a specified date according to local time.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setMilliseconds
    #[wasm_bindgen(method, js_name = setMilliseconds)]
    pub fn set_milliseconds(this: &Date, milliseconds: u32) -> f64;

    /// The setMinutes() method sets the minutes for a specified date according to local time.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setMinutes
    #[wasm_bindgen(method, js_name = setMinutes)]
    pub fn set_minutes(this: &Date, minutes: u32) -> f64;

    /// The setMonth() method sets the month for a specified date according to the currently set year.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setMonth
    #[wasm_bindgen(method, js_name = setMonth)]
    pub fn set_month(this: &Date, month: u32) -> f64;

    /// The setSeconds() method sets the seconds for a specified date according to local time.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setSeconds
    #[wasm_bindgen(method, js_name = setSeconds)]
    pub fn set_seconds(this: &Date, seconds: u32) -> f64;

    /// The setTime() method sets the Date object to the time represented by a number of milliseconds
    /// since January 1, 1970, 00:00:00 UTC.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setTime
    #[wasm_bindgen(method, js_name = setTime)]
    pub fn set_time(this: &Date, time: f64) -> f64;

    /// The setUTCDate() method sets the day of the month for a specified date
    /// according to universal time.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCDate
    #[wasm_bindgen(method, js_name = setUTCDate)]
    pub fn set_utc_date(this: &Date, day: u32) -> f64;

    /// The setUTCFullYear() method sets the full year for a specified date according to universal time.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCFullYear
    #[wasm_bindgen(method, js_name = setUTCFullYear)]
    pub fn set_utc_full_year(this: &Date, year: u32) -> f64;

    /// The setUTCHours() method sets the hour for a specified date according to universal time,
    /// and returns the number of milliseconds since  January 1, 1970 00:00:00 UTC until the time
    /// represented by the updated Date instance.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCHours
    #[wasm_bindgen(method, js_name = setUTCHours)]
    pub fn set_utc_hours(this: &Date, hours: u32) -> f64;

    /// The setUTCMilliseconds() method sets the milliseconds for a specified date
    /// according to universal time.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCMilliseconds
    #[wasm_bindgen(method, js_name = setUTCMilliseconds)]
    pub fn set_utc_milliseconds(this: &Date, milliseconds: u32) -> f64;

    /// The setUTCMinutes() method sets the minutes for a specified date according to universal time.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCMinutes
    #[wasm_bindgen(method, js_name = setUTCMinutes)]
    pub fn set_utc_minutes(this: &Date, minutes: u32) -> f64;

    /// The setUTCMonth() method sets the month for a specified date according to universal time.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCMonth
    #[wasm_bindgen(method, js_name = setUTCMonth)]
    pub fn set_utc_month(this: &Date, month: u32) -> f64;

    /// The setUTCSeconds() method sets the seconds for a specified date according to universal time.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCSeconds
    #[wasm_bindgen(method, js_name = setUTCSeconds)]
    pub fn set_utc_seconds(this: &Date, seconds: u32) -> f64;

    /// The toDateString() method returns the date portion of a Date object
    /// in human readable form in American English.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toDateString
    #[wasm_bindgen(method, js_name = toDateString)]
    pub fn to_date_string(this: &Date) -> JsString;

    /// The toISOString() method returns a string in simplified extended ISO format (ISO
    /// 8601), which is always 24 or 27 characters long (YYYY-MM-DDTHH:mm:ss.sssZ or
    /// ±YYYYYY-MM-DDTHH:mm:ss.sssZ, respectively). The timezone is always zero UTC offset,
    /// as denoted by the suffix "Z"
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toISOString
    #[wasm_bindgen(method, js_name = toISOString)]
    pub fn to_iso_string(this: &Date) -> JsString;

    /// The toJSON() method returns a string representation of the Date object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toJSON
    #[wasm_bindgen(method, js_name = toJSON)]
    pub fn to_json(this: &Date) -> JsString;

    /// The toLocaleDateString() method returns a string with a language sensitive
    /// representation of the date portion of this date. The new locales and options
    /// arguments let applications specify the language whose formatting conventions
    /// should be used and allow to customize the behavior of the function.
    /// In older implementations, which ignore the locales and options arguments,
    /// the locale used and the form of the string
    /// returned are entirely implementation dependent.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toLocaleDateString
    #[wasm_bindgen(method, js_name = toLocaleDateString)]
    pub fn to_locale_date_string(this: &Date, locale: JsString, options: JsValue) -> JsString;

    /// The toLocaleString() method returns a string with a language sensitive
    /// representation of this date. The new locales and options arguments
    /// let applications specify the language whose formatting conventions
    /// should be used and customize the behavior of the function.
    /// In older implementations, which ignore the locales
    /// and options arguments, the locale used and the form of the string
    /// returned are entirely implementation dependent.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toLocaleString
    #[wasm_bindgen(method, js_name = toLocaleString)]
    pub fn to_locale_string(this: &Date, locale: JsString, options: JsValue) -> JsString;

    /// The toLocaleTimeString() method returns a string with a language sensitive
    /// representation of the time portion of this date. The new locales and options
    /// arguments let applications specify the language whose formatting conventions should be
    /// used and customize the behavior of the function. In older implementations, which ignore
    /// the locales and options arguments, the locale used and the form of the string
    /// returned are entirely implementation dependent.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toLocaleTimeString
    #[wasm_bindgen(method, js_name = toLocaleTimeString)]
    pub fn to_locale_time_string(this: &Date, locale: JsString) -> JsString;

    /// The toString() method returns a string representing
    /// the specified Date object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toString
    #[wasm_bindgen(method, js_name = toString)]
    pub fn to_string(this: &Date) -> JsString;

    /// The toTimeString() method returns the time portion of a Date object in human
    /// readable form in American English.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toTimeString
    #[wasm_bindgen(method, js_name = toTimeString)]
    pub fn to_time_string(this: &Date) -> JsString;

    /// The toUTCString() method converts a date to a string,
    /// using the UTC time zone.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toUTCString
    #[wasm_bindgen(method, js_name = toUTCString)]
    pub fn to_utc_string(this: &Date) -> JsString;

    /// The `Date.UTC()` method accepts the same parameters as the
    /// longest form of the constructor, and returns the number of
    /// milliseconds in a `Date` object since January 1, 1970,
    /// 00:00:00, universal time.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/UTC
    #[wasm_bindgen(static_method_of = Date, js_name = UTC)]
    pub fn utc(year: f64, month: f64) -> f64;

    /// The valueOf() method  returns the primitive value of
    /// a Date object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/valueOf
    #[wasm_bindgen(method, js_name = valueOf)]
    pub fn value_of(this: &Date) -> Date;
}

// Object.
#[wasm_bindgen]
extern "C" {
    pub type Object;

    /// The `hasOwnProperty()` method returns a boolean indicating whether the
    /// object has the specified property as its own property (as opposed to
    /// inheriting it).
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/hasOwnProperty
    #[wasm_bindgen(method, js_name = hasOwnProperty)]
    pub fn has_own_property(this: &Object, property: &JsValue) -> bool;

    /// The `Object.isExtensible()` method determines if an object is extensible
    /// (whether it can have new properties added to it).
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/isExtensible
    #[wasm_bindgen(static_method_of = Object, js_name = isExtensible)]
    pub fn is_extensible(object: &Object) -> bool;

    /// The `Object.isFrozen()` determines if an object is frozen.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/isFrozen
    #[wasm_bindgen(static_method_of = Object, js_name = isFrozen)]
    pub fn is_frozen(object: &Object) -> bool;

    /// The `Object.isSealed()` method determines if an object is sealed.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/isSealed
    #[wasm_bindgen(static_method_of = Object, js_name = isSealed)]
    pub fn is_sealed(object: &Object) -> bool;

    /// The `isPrototypeOf()` method checks if an object exists in another
    /// object's prototype chain.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/isPrototypeOf
    #[wasm_bindgen(method, js_name = isPrototypeOf)]
    pub fn is_prototype_of(this: &Object, value: &JsValue) -> bool;

    /// The `Object.keys()` method returns an array of a given object's property
    /// names, in the same order as we get with a normal loop.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/keys
    #[wasm_bindgen(static_method_of = Object)]
    pub fn keys(object: &Object) -> Array;

    /// The [`Object`] constructor creates an object wrapper.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object
    #[wasm_bindgen(constructor)]
    pub fn new() -> Object;

    /// The `Object.preventExtensions()` method prevents new properties from
    /// ever being added to an object (i.e. prevents future extensions to the
    /// object).
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/preventExtensions
    #[wasm_bindgen(static_method_of = Object, js_name = preventExtensions)]
    pub fn prevent_extensions(object: &Object);

    /// The `propertyIsEnumerable()` method returns a Boolean indicating
    /// whether the specified property is enumerable.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/propertyIsEnumerable
    #[wasm_bindgen(method, js_name = propertyIsEnumerable)]
    pub fn property_is_enumerable(this: &Object, property: &JsValue) -> bool;

    /// The `Object.seal()` method seals an object, preventing new properties
    /// from being added to it and marking all existing properties as
    /// non-configurable.  Values of present properties can still be changed as
    /// long as they are writable.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/seal
    #[wasm_bindgen(static_method_of = Object)]
    pub fn seal(value: &JsValue) -> JsValue;

    /// The `Object.setPrototypeOf()` method sets the prototype (i.e., the
    /// internal `[[Prototype]]` property) of a specified object to another
    /// object or `null`.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/setPrototypeOf
    #[wasm_bindgen(static_method_of = Object, js_name = setPrototypeOf)]
    pub fn set_prototype_of(object: &Object, prototype: &Object) -> Object;

    /// The `toLocaleString()` method returns a string representing the object.
    /// This method is meant to be overridden by derived objects for
    /// locale-specific purposes.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/toLocaleString
    #[wasm_bindgen(method, js_name = toLocaleString)]
    pub fn to_locale_string(this: &Object) -> JsString;

    /// The `toString()` method returns a string representing the object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/toString
    #[wasm_bindgen(method, js_name = toString)]
    pub fn to_string(this: &Object) -> JsString;

    /// The `valueOf()` method returns the primitive value of the
    /// specified object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/valueOf
    #[wasm_bindgen(method, js_name = valueOf)]
    pub fn value_of(this: &Object) -> Object;

    /// The `Object.values()` method returns an array of a given object's own
    /// enumerable property values, in the same order as that provided by a
    /// `for...in` loop (the difference being that a for-in loop enumerates
    /// properties in the prototype chain as well).
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/values
    #[wasm_bindgen(static_method_of = Object)]
    pub fn values(object: &Object) -> Array;
}

impl JsValue {
    /// Returns the `Object` value of this JS value if it's an instance of an
    /// object.
    ///
    /// If this JS value is not an instance of an object then this returns
    /// `None`.
    pub fn as_object(&self) -> Option<&Object> {
        if self.is_object() {
            Some(unsafe { mem::transmute(self) })
        } else {
            None
        }
    }
}

// Proxy
#[wasm_bindgen]
extern {
    pub type Proxy;

    /// The [`Proxy`] object is used to define custom behavior for fundamental
    /// operations (e.g. property lookup, assignment, enumeration, function
    /// invocation, etc).
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Proxy
    #[wasm_bindgen(constructor)]
    pub fn new(target: &JsValue, handler: &Object) -> Proxy;

    /// The `Proxy.revocable()` method is used to create a revocable [`Proxy`]
    /// object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Proxy/revocable
    #[wasm_bindgen(static_method_of = Proxy)]
    pub fn revocable(target: &JsValue, handler: &Object) -> Object;
}

// Reflect
#[wasm_bindgen]
extern "C" {
    pub type Reflect;

    /// The static `Reflect.apply()` method calls a target function with
    /// arguments as specified.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/apply
    #[wasm_bindgen(static_method_of = Reflect)]
    pub fn apply(target: &Function, this_argument: &JsValue, arguments_list: &Array) -> JsValue;

    /// The static `Reflect.construct()` method acts like the new operator, but
    /// as a function.  It is equivalent to calling `new target(...args)`. It
    /// gives also the added option to specify a different prototype.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/construct
    #[wasm_bindgen(static_method_of = Reflect)]
    pub fn construct(target: &Function, arguments_list: &Array) -> JsValue;
    #[wasm_bindgen(static_method_of = Reflect, js_name = construct)]
    pub fn construct_with_new_target(target: &Function, arguments_list: &Array, new_target: &Function) -> JsValue;

    /// The static `Reflect.defineProperty()` method is like
    /// `Object.defineProperty()` but returns a `Boolean`.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/defineProperty
    #[wasm_bindgen(static_method_of = Reflect, js_name = defineProperty)]
    pub fn define_property(target: &Object, property_key: &JsValue, attributes: &Object) -> bool;

    /// The static `Reflect.deleteProperty()` method allows to delete
    /// properties.  It is like the `delete` operator as a function.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/deleteProperty
    #[wasm_bindgen(static_method_of = Reflect, js_name = deleteProperty)]
    pub fn delete_property(target: &Object, key: &JsValue) -> bool;

    /// The static `Reflect.get()` method works like getting a property from
    /// an object (`target[propertyKey]`) as a function.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/get
    #[wasm_bindgen(static_method_of = Reflect)]
    pub fn get(target: &Object, key: &JsValue) -> JsValue;

    /// The static `Reflect.getOwnPropertyDescriptor()` method is similar to
    /// `Object.getOwnPropertyDescriptor()`. It returns a property descriptor
    /// of the given property if it exists on the object, `undefined` otherwise.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/getOwnPropertyDescriptor
    #[wasm_bindgen(static_method_of = Reflect, js_name = getOwnPropertyDescriptor)]
    pub fn get_own_property_descriptor(target: &Object, property_key: &JsValue) -> JsValue;

    /// The static `Reflect.getPrototypeOf()` method is almost the same
    /// method as `Object.getPrototypeOf()`. It returns the prototype
    /// (i.e. the value of the internal `[[Prototype]]` property) of
    /// the specified object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/getPrototypeOf
    #[wasm_bindgen(static_method_of = Reflect, js_name = getPrototypeOf)]
    pub fn get_prototype_of(target: &Object) -> Object;

    /// The static `Reflect.has()` method works like the in operator as a
    /// function.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/has
    #[wasm_bindgen(static_method_of = Reflect)]
    pub fn has(target: &Object, property_key: &JsValue) -> bool;

    /// The static `Reflect.isExtensible()` method determines if an object is
    /// extensible (whether it can have new properties added to it). It is
    /// similar to `Object.isExtensible()`, but with some differences.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/isExtensible
    #[wasm_bindgen(static_method_of = Reflect, js_name = isExtensible)]
    pub fn is_extensible(target: &Object) -> bool;

    /// The static `Reflect.ownKeys()` method returns an array of the
    /// target object's own property keys.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/ownKeys
    #[wasm_bindgen(static_method_of = Reflect, js_name = ownKeys)]
    pub fn own_keys(target: &Object) -> Array;

    /// The static `Reflect.preventExtensions()` method prevents new
    /// properties from ever being added to an object (i.e. prevents
    /// future extensions to the object). It is similar to
    /// `Object.preventExtensions()`, but with some differences.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/preventExtensions
    #[wasm_bindgen(static_method_of = Reflect, js_name = preventExtensions)]
    pub fn prevent_extensions(target: &Object) -> bool;

    /// The static `Reflect.set()` method works like setting a
    /// property on an object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/set
    #[wasm_bindgen(static_method_of = Reflect)]
    pub fn set(target: &Object, property_key: &JsValue, value: &JsValue) -> bool;
    #[wasm_bindgen(static_method_of = Reflect, js_name = set)]
    pub fn set_with_receiver(target: &Object, property_key: &JsValue, value: &JsValue, receiver: &Object) -> bool;

    /// The static `Reflect.setPrototypeOf()` method is the same
    /// method as `Object.setPrototypeOf()`. It sets the prototype
    /// (i.e., the internal `[[Prototype]]` property) of a specified
    /// object to another object or to null.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/setPrototypeOf
    #[wasm_bindgen(static_method_of = Reflect, js_name = setPrototypeOf)]
    pub fn set_prototype_of(target: &Object, prototype: &JsValue) -> bool;
}

// Set
#[wasm_bindgen]
extern {
    pub type Set;

    /// The `add()` method appends a new element with a specified value to the
    /// end of a [`Set`] object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/add
    #[wasm_bindgen(method)]
    pub fn add(this: &Set, value: &JsValue) -> Set;

    /// The `clear()` method removes all elements from a [`Set`] object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/clear
    #[wasm_bindgen(method)]
    pub fn clear(this: &Set);

    /// The `delete()` method removes the specified element from a [`Set`]
    /// object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/delete
    #[wasm_bindgen(method)]
    pub fn delete(this: &Set, value: &JsValue) -> bool;

    /// The forEach() method executes a provided function once for each value
    /// in the Set object, in insertion order.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/forEach
    #[wasm_bindgen(method, js_name = forEach)]
    pub fn for_each(this: &Set, callback: &mut FnMut(JsValue, JsValue, Set));

    /// The `has()` method returns a boolean indicating whether an element with
    /// the specified value exists in a [`Set`] object or not.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/has
    #[wasm_bindgen(method)]
    pub fn has(this: &Set, value: &JsValue) -> bool;

    /// The [`Set`] object lets you store unique values of any type, whether
    /// primitive values or object references.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set
    #[wasm_bindgen(constructor)]
    pub fn new() -> Set;

    /// The size accessor property returns the number of elements in a [`Set`]
    /// object.
    ///
    /// https://developer.mozilla.org/de/docs/Web/JavaScript/Reference/Global_Objects/Set/size
    #[wasm_bindgen(method, getter, structural)]
    pub fn size(this: &Set) -> u32;
}

// SetIterator
#[wasm_bindgen]
extern {
    pub type SetIterator;

    /// The `entries()` method returns a new Iterator object that contains an
    /// array of [value, value] for each element in the Set object, in insertion
    /// order. For Set objects there is no key like in Map objects. However, to
    /// keep the API similar to the Map object, each entry has the same value
    /// for its key and value here, so that an array [value, value] is returned.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/entries
    #[wasm_bindgen(method)]
    pub fn entries(set: &Set) -> SetIterator;

    /// The `keys()` method is an alias for this method (for similarity with
    /// Map objects); it behaves exactly the same and returns values
    /// of Set elements.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/values
    #[wasm_bindgen(method)]
    pub fn keys(set: &Set) -> SetIterator;

    /// The `values()` method returns a new Iterator object that contains the
    /// values for each element in the Set object in insertion order.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/values
    #[wasm_bindgen(method)]
    pub fn values(set: &Set) -> SetIterator;
}

// Uint8Array
#[wasm_bindgen]
extern "C" {
    pub type Uint8Array;

    /// The `Uint8Array()` constructor creates an array of unsigned 8-bit integers.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array
    #[wasm_bindgen(constructor)]
    pub fn new(constructor_arg: JsValue) -> Uint8Array;

    /// The fill() method fills all the elements of an array from a start index
    /// to an end index with a static value. The end index is not included.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/fill
    #[wasm_bindgen(method)]
    pub fn fill(this: &Uint8Array, value: u8, start: u32, end: u32) -> Uint8Array;
}

// Uint8ClampedArray
#[wasm_bindgen]
extern "C" {
    pub type Uint8ClampedArray;

    /// The `Uint8ClampedArray()` constructor creates an array of unsigned 8-bit integers clamped
    /// to 0-255; if you specified a value that is out of the range of [0,255], 0 or 255 will be
    /// set instead.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8ClampedArray
    #[wasm_bindgen(constructor)]
    pub fn new(constructor_arg: JsValue) -> Uint8ClampedArray;

    /// The fill() method fills all the elements of an array from a start index
    /// to an end index with a static value. The end index is not included.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/fill
    #[wasm_bindgen(method)]
    pub fn fill(this: &Uint8ClampedArray, value: u8, start: u32, end: u32) -> Uint8ClampedArray;
}

// Uint16Array
#[wasm_bindgen]
extern "C" {
    pub type Uint16Array;

    /// The `Uint16Array()` constructor creates an array of unsigned 16-bit integers.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint16Array
    #[wasm_bindgen(constructor)]
    pub fn new(constructor_arg: JsValue) -> Uint16Array;

    /// The fill() method fills all the elements of an array from a start index
    /// to an end index with a static value. The end index is not included.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/fill
    #[wasm_bindgen(method)]
    pub fn fill(this: &Uint16Array, value: u16, start: u32, end: u32) -> Uint16Array;
}

// Uint32Array
#[wasm_bindgen]
extern "C" {
    pub type Uint32Array;

    /// The `Uint32Array()` constructor creates an array of unsigned 32-bit integers.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint32Array
    #[wasm_bindgen(constructor)]
    pub fn new(constructor_arg: JsValue) -> Uint32Array;

    /// The fill() method fills all the elements of an array from a start index
    /// to an end index with a static value. The end index is not included.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/fill
    #[wasm_bindgen(method)]
    pub fn fill(this: &Uint32Array, value: u32, start: u32, end: u32) -> Uint32Array;
}

// WeakMap
#[wasm_bindgen]
extern "C" {
    pub type WeakMap;

    /// The [`WeakMap`] object is a collection of key/value pairs in which the
    /// keys are weakly referenced.  The keys must be objects and the values can
    /// be arbitrary values.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakMap
    #[wasm_bindgen(constructor)]
    pub fn new() -> WeakMap;

    /// The `set()` method sets the value for the key in the [`WeakMap`] object.
    /// Returns the [`WeakMap`] object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakMap/set
    #[wasm_bindgen(method, js_class = "WeakMap")]
    pub fn set(this: &WeakMap, key: Object, value: JsValue) -> WeakMap;

    /// The get() method returns a specified by key element
    /// from a [`WeakMap`] object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakMap/get
    #[wasm_bindgen(method)]
    pub fn get(this: &WeakMap, key: Object) -> JsValue;

    /// The `has()` method returns a boolean indicating whether an element with
    /// the specified key exists in the [`WeakMap`] object or not.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakMap/has
    #[wasm_bindgen(method)]
    pub fn has(this: &WeakMap, key: Object) -> bool;

    /// The `delete()` method removes the specified element from a [`WeakMap`]
    /// object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakMap/delete
    #[wasm_bindgen(method)]
    pub fn delete(this: &WeakMap, key: Object) -> bool;
}

// WeakSet
#[wasm_bindgen]
extern "C" {
    pub type WeakSet;

    /// The `WeakSet` object lets you store weakly held objects in a collection.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakSet
    #[wasm_bindgen(constructor)]
    pub fn new() -> WeakSet;

    /// The `has()` method returns a boolean indicating whether an object exists
    /// in a WeakSet or not.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakSet/has
    #[wasm_bindgen(method)]
    pub fn has(this: &WeakSet, value: Object) -> bool;

    /// The `add()` method appends a new object to the end of a WeakSet object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakSet/add
    #[wasm_bindgen(method)]
    pub fn add(this: &WeakSet, value: Object) -> WeakSet;

    /// The `delete()` method removes the specified element from a WeakSet
    /// object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakSet/delete
    #[wasm_bindgen(method)]
    pub fn delete(this: &WeakSet, value: Object) -> bool;
}

// WebAssembly
#[wasm_bindgen]
extern "C" {
    pub type WebAssembly;

    /// The `WebAssembly.validate()` function validates a given typed
    /// array of WebAssembly binary code, returning whether the bytes
    /// form a valid wasm module (`true`) or not (`false`).
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WebAssembly/validate
    #[wasm_bindgen(static_method_of = WebAssembly, catch)]
    pub fn validate(bufferSource: JsValue) -> Result<bool, JsValue>;
}

// JsString
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = JsString)]
    pub type JsString;

    /// The length property of a String object indicates the length of a string,
    /// in UTF-16 code units.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/length
    #[wasm_bindgen(method, getter, structural)]
    pub fn length(this: &JsString) -> u32;

    /// The String object's `charAt()` method returns a new string consisting of
    /// the single UTF-16 code unit located at the specified offset into the
    /// string.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/charAt
    #[wasm_bindgen(method, js_class = "String", js_name = charAt)]
    pub fn char_at(this: &JsString, index: u32) -> JsString;

    /// The `charCodeAt()` method returns an integer between 0 and 65535
    /// representing the UTF-16 code unit at the given index (the UTF-16 code
    /// unit matches the Unicode code point for code points representable in a
    /// single UTF-16 code unit, but might also be the first code unit of a
    /// surrogate pair for code points not representable in a single UTF-16 code
    /// unit, e.g. Unicode code points > 0x10000).  If you want the entire code
    /// point value, use `codePointAt()`.
    ///
    /// Returns `NaN` if index is out of range.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/charCodeAt
    #[wasm_bindgen(method, js_class = "String", js_name = charCodeAt)]
    pub fn char_code_at(this: &JsString, index: u32) -> f64;

    /// The `codePointAt()` method returns a non-negative integer that is the
    /// Unicode code point value.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/codePointAt
    #[wasm_bindgen(method, js_class = "String", js_name = codePointAt)]
    pub fn code_point_at(this: &JsString, pos: u32) -> JsValue;

    /// The `concat()` method concatenates the string arguments to the calling
    /// string and returns a new string.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/concat
    #[wasm_bindgen(method, js_class = "String")]
    pub fn concat(this: &JsString, string_2: &JsString) -> JsString;

    /// The endsWith() method determines whether a string ends with the characters of a 
    /// specified string, returning true or false as appropriate.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/endsWith
    #[wasm_bindgen(method, js_class = "String", js_name = endsWith)]
    pub fn ends_with(this: &JsString, search_string: &JsString, length: i32) -> bool;

    /// The `includes()` method determines whether one string may be found
    /// within another string, returning true or false as appropriate.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/includes
    #[wasm_bindgen(method, js_class = "String")]
    pub fn includes(this: &JsString, search_string: &JsString, position: i32) -> bool;

    /// The `indexOf()` method returns the index within the calling String
    /// object of the first occurrence of the specified value, starting the
    /// search at fromIndex.  Returns -1 if the value is not found.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/indexOf
    #[wasm_bindgen(method, js_class = "String", js_name = indexOf)]
    pub fn index_of(this: &JsString, search_value: &JsString, from_index: i32) -> i32;

    /// The `lastIndexOf()` method returns the index within the calling String
    /// object of the last occurrence of the specified value, searching
    /// backwards from fromIndex.  Returns -1 if the value is not found.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/lastIndexOf
    #[wasm_bindgen(method, js_class = "String", js_name = lastIndexOf)]
    pub fn last_index_of(this: &JsString, search_value: &JsString, from_index: i32) -> i32;

    /// The normalize() method returns the Unicode Normalization Form 
    /// of a given string (if the value isn't a string, it will be converted to one first).
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/normalize
    #[wasm_bindgen(method, js_class = "String")]
    pub fn normalize(this: &JsString, form: &JsString) -> JsString;

    /// The `padEnd()` method pads the current string with a given string
    /// (repeated, if needed) so that the resulting string reaches a given
    /// length. The padding is applied from the end (right) of the current
    /// string.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/padEnd
    #[wasm_bindgen(method, js_class = "String", js_name = padEnd)]
    pub fn pad_end(this: &JsString, target_length: u32, pad_string: &JsString) -> JsString;

    /// The `padStart()` method pads the current string with another string
    /// (repeated, if needed) so that the resulting string reaches the given
    /// length. The padding is applied from the start (left) of the current
    /// string.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/padStart
    #[wasm_bindgen(method, js_class = "String", js_name = padStart)]
    pub fn pad_start(this: &JsString, target_length: u32, pad_string: &JsString) -> JsString;

    /// The repeat() method constructs and returns a new string which contains the specified 
    /// number of copies of the string on which it was called, concatenated together.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/repeat
    #[wasm_bindgen(method, js_class = "String")]
    pub fn repeat(this: &JsString, count: i32) -> JsString;

    /// The `slice()` method extracts a section of a string and returns it as a
    /// new string, without modifying the original string.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/slice
    #[wasm_bindgen(method, js_class = "String")]
    pub fn slice(this: &JsString, start: u32, end: u32) -> JsString;

    /// The `startsWith()` method determines whether a string begins with the
    /// characters of a specified string, returning true or false as
    /// appropriate.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/startsWith
    #[wasm_bindgen(method, js_class = "String", js_name = startsWith)]
    pub fn starts_with(this: &JsString, search_string: &JsString, position: u32) -> bool;

    /// The `substring()` method returns the part of the string between the
    /// start and end indexes, or to the end of the string.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/substring
    #[wasm_bindgen(method, js_class = "String")]
    pub fn substring(this: &JsString, index_start: u32, index_end: u32) -> JsString;

    /// The `substr()` method returns the part of a string between
    /// the start index and a number of characters after it.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/substr
    #[wasm_bindgen(method, js_class = "String")]
    pub fn substr(this: &JsString, start: i32, length: i32) -> JsString;

    /// The `toLowerCase()` method returns the calling string value
    /// converted to lower case.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/toLowerCase
    #[wasm_bindgen(method, js_class = "String", js_name = toLowerCase)]
    pub fn to_lower_case(this: &JsString) -> JsString;

    /// The `toString()` method returns a string representing the specified
    /// object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/toString
    #[wasm_bindgen(method, js_class = "String", js_name = toString)]
    pub fn to_string(this: &JsString) -> JsString;

    /// The `toUpperCase()` method returns the calling string value converted to
    /// uppercase (the value will be converted to a string if it isn't one).
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/toUpperCase
    #[wasm_bindgen(method, js_class = "String", js_name = toUpperCase)]
    pub fn to_upper_case(this: &JsString) -> JsString;

    /// The `trim()` method removes whitespace from both ends of a string.
    /// Whitespace in this context is all the whitespace characters (space, tab,
    /// no-break space, etc.) and all the line terminator characters (LF, CR,
    /// etc.).
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/trim
    #[wasm_bindgen(method, js_class = "String")]
    pub fn trim(this: &JsString) -> JsString;

    /// The `trimEnd()` method removes whitespace from the end of a string.
    /// `trimRight()` is an alias of this method.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/trimEnd
    #[wasm_bindgen(method, js_class = "String", js_name = trimEnd)]
    pub fn trim_end(this: &JsString) -> JsString;

    /// The `trimEnd()` method removes whitespace from the end of a string.
    /// `trimRight()` is an alias of this method.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/trimEnd
    #[wasm_bindgen(method, js_class = "String", js_name = trimRight)]
    pub fn trim_right(this: &JsString) -> JsString;

    /// The `trimStart()` method removes whitespace from the beginning of a
    /// string.  `trimLeft()` is an alias of this method.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/trimStart
    #[wasm_bindgen(method, js_class = "String", js_name = trimStart)]
    pub fn trim_start(this: &JsString) -> JsString;

    /// The `trimStart()` method removes whitespace from the beginning of a
    /// string.  `trimLeft()` is an alias of this method.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/trimStart
    #[wasm_bindgen(method, js_class = "String", js_name = trimLeft)]
    pub fn trim_left(this: &JsString) -> JsString;

    /// The `valueOf()` method returns the primitive value of a `String` object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/valueOf
    #[wasm_bindgen(method, js_class = "String", js_name = valueOf)]
    pub fn value_of(this: &JsString) -> JsString;
}

impl JsValue {
    /// Returns the `JsString` value of this JS value if it's an instance of a
    /// string.
    ///
    /// If this JS value is not an instance of a string then this returns
    /// `None`.
    pub fn as_js_string(&self) -> Option<&JsString> {
        if self.is_string() {
            Some(unsafe { mem::transmute(self) })
        } else {
            None
        }
    }
}

impl<'a> From<&'a str> for JsString {
    fn from(s: &'a str) -> Self {
        JsString {
            obj: JsValue::from_str(s),
        }
    }
}

if_std! {
    impl From<String> for JsString {
        fn from(s: String) -> Self {
            From::from(&*s)
        }
    }

    impl<'a> From<&'a JsString> for String {
        fn from(s: &'a JsString) -> Self {
            s.obj.as_string().unwrap()
        }
    }

    impl From<JsString> for String {
        fn from(s: JsString) -> Self {
            From::from(&s)
        }
    }
}

// Symbol
#[wasm_bindgen]
extern "C" {
    pub type Symbol;

    /// The `Symbol.hasInstance` well-known symbol is used to determine
    /// if a constructor object recognizes an object as its instance.
    /// The `instanceof` operator's behavior can be customized by this symbol.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/hasInstance
    #[wasm_bindgen(static_method_of = Symbol, getter, structural, js_name = hasInstance)]
    pub fn has_instance() -> Symbol;

    /// The `Symbol.isConcatSpreadable` well-known symbol is used to configure
    /// if an object should be flattened to its array elements when using the
    /// `Array.prototype.concat()` method.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/isConcatSpreadable
    #[wasm_bindgen(static_method_of = Symbol, getter, structural, js_name = isConcatSpreadable)]
    pub fn is_concat_spreadable() -> Symbol;

    /// The `Symbol.iterator` well-known symbol specifies the default iterator
    /// for an object.  Used by `for...of`.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/iterator
    #[wasm_bindgen(static_method_of = Symbol, getter, structural)]
    pub fn iterator() -> Symbol;

    /// The `Symbol.match` well-known symbol specifies the matching of a regular
    /// expression against a string. This function is called by the
    /// `String.prototype.match()` method.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/match
    #[wasm_bindgen(static_method_of = Symbol, getter, structural, js_name = match)]
    pub fn match_() -> Symbol;

    /// The `Symbol.replace` well-known symbol specifies the method that
    /// replaces matched substrings of a string.  This function is called by the
    /// `String.prototype.replace()` method.
    ///
    /// For more information, see `RegExp.prototype[@@replace]()` and
    /// `String.prototype.replace()`.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/replace
    #[wasm_bindgen(static_method_of = Symbol, getter, structural)]
    pub fn replace() -> Symbol;

    /// The `Symbol.search` well-known symbol specifies the method that returns
    /// the index within a string that matches the regular expression.  This
    /// function is called by the `String.prototype.search()` method.
    ///
    /// For more information, see `RegExp.prototype[@@search]()` and
    /// `String.prototype.search()`.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/search
    #[wasm_bindgen(static_method_of = Symbol, getter, structural)]
    pub fn search() -> Symbol;

    /// The well-known symbol `Symbol.species` specifies a function-valued
    /// property that the constructor function uses to create derived objects.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/species
    #[wasm_bindgen(static_method_of = Symbol, getter, structural)]
    pub fn species() -> Symbol;

    /// The `Symbol.split` well-known symbol specifies the method that splits a
    /// string at the indices that match a regular expression.  This function is
    /// called by the `String.prototype.split()` method.
    ///
    /// For more information, see `RegExp.prototype[@@split]()` and
    /// `String.prototype.split()`.
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/split
    #[wasm_bindgen(static_method_of = Symbol, getter, structural)]
    pub fn split() -> Symbol;

    /// The `Symbol.toPrimitive` is a symbol that specifies a function valued
    /// property that is called to convert an object to a corresponding
    /// primitive value.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/toPrimitive
    #[wasm_bindgen(static_method_of = Symbol, getter, structural, js_name = toPrimitive)]
    pub fn to_primitive() -> Symbol;

    /// The `Symbol.toStringTag` well-known symbol is a string valued property
    /// that is used in the creation of the default string description of an
    /// object.  It is accessed internally by the `Object.prototype.toString()`
    /// method.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/toString
    #[wasm_bindgen(static_method_of = Symbol, getter, structural, js_name = toStringTag)]
    pub fn to_string_tag() -> Symbol;

    /// The Symbol.for(key) method searches for existing symbols in a runtime-wide symbol registry with
    /// the given key and returns it if found.
    /// Otherwise a new symbol gets created in the global symbol registry with this key.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/for
    #[wasm_bindgen(static_method_of = Symbol, js_name = for)]
    pub fn for_(key: &JsString) -> Symbol;

    /// The Symbol.keyFor(sym) method retrieves a shared symbol key from the global symbol registry for the given symbol.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/keyFor
    #[wasm_bindgen(static_method_of = Symbol, js_name = keyFor)]
    pub fn key_for(sym: &Symbol) -> JsString;

    /// The toString() method returns a string representing the specified Symbol object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/toString
    #[wasm_bindgen(method, js_name = toString)]
    pub fn to_string(this: &Symbol) -> JsString;

    /// The valueOf() method returns the primitive value of a Symbol object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/valueOf
    #[wasm_bindgen(method, js_name = valueOf)]
    pub fn value_of(this: &Symbol) -> Symbol;
}

// Intl
#[wasm_bindgen]
extern "C" {
    pub type Intl;

    /// The `Intl.getCanonicalLocales()` method returns an array containing
    /// the canonical locale names. Duplicates will be omitted and elements
    /// will be validated as structurally valid language tags.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/getCanonicalLocales
    #[wasm_bindgen(static_method_of = Intl, js_name = getCanonicalLocales)]
    pub fn get_canonical_locales(s: &JsValue) -> Array;
}
