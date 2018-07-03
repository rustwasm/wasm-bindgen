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
}

// UInt8Array
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
    pub fn fill(this: &Uint8Array, value: JsValue, start: u32, end: u32) -> Uint8Array;
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
    pub fn size(this: &Map) -> Number;
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
    pub fn abs(x: f64) -> Number;

    /// The Math.acos() function returns the arccosine (in radians) of a
    /// number, that is ∀x∊[-1;1]
    /// Math.acos(x) = arccos(x) = the unique y∊[0;π] such that cos(y)=x
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/acos
    #[wasm_bindgen(static_method_of = Math)]
    pub fn acos(x: f64) -> Number;

    /// The Math.acosh() function returns the hyperbolic arc-cosine of a
    /// number, that is ∀x ≥ 1
    /// Math.acosh(x) = arcosh(x) = the unique y ≥ 0 such that cosh(y) = x
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/acosh
    #[wasm_bindgen(static_method_of = Math)]
    pub fn acosh(x: f64) -> Number;

    /// The Math.asin() function returns the arcsine (in radians) of a
    /// number, that is ∀x ∊ [-1;1]
    /// Math.asin(x) = arcsin(x) = the unique y∊[-π2;π2] such that sin(y) = x
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/asin
    #[wasm_bindgen(static_method_of = Math)]
    pub fn asin(x: f64) -> Number;

    /// The Math.asinh() function returns the hyperbolic arcsine of a
    /// number, that is Math.asinh(x) = arsinh(x) = the unique y such that sinh(y) = x
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/asinh
    #[wasm_bindgen(static_method_of = Math)]
    pub fn asinh(x: f64) -> Number;

    /// The Math.atan() function returns the arctangent (in radians) of a
    /// number, that is Math.atan(x) = arctan(x) = the unique y ∊ [-π2;π2]such that
    /// tan(y) = x
    #[wasm_bindgen(static_method_of = Math)]
    pub fn atan(x: f64) -> Number;

    /// The Math.atan2() function returns the arctangent of the quotient of
    /// its arguments.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/atan2
    #[wasm_bindgen(static_method_of = Math)]
    pub fn atan2(y: f64, x: f64) -> Number;

    /// The Math.atanh() function returns the hyperbolic arctangent of a number,
    /// that is ∀x ∊ (-1,1), Math.atanh(x) = arctanh(x) = the unique y such that
    /// tanh(y) = x
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/atanh
    #[wasm_bindgen(static_method_of = Math)]
    pub fn atanh(x: f64) -> Number;

    /// The Math.cbrt() function returns the cube root of a number, that is
    /// Math.cbrt(x) = x^3 = the unique y such that y^3 = x
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/cbrt
    #[wasm_bindgen(static_method_of = Math)]
    pub fn cbrt(x: f64) -> Number;

    /// The Math.ceil() function returns the smallest integer greater than
    /// or equal to a given number.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/ceil
    #[wasm_bindgen(static_method_of = Math)]
    pub fn ceil(x: f64) -> Number;

    /// The Math.clz32() function returns the number of leading zero bits in
    /// the 32-bit binary representation of a number.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/clz32
    #[wasm_bindgen(static_method_of = Math)]
    pub fn clz32(x: i32) -> Number;

    /// The Math.cos() static function returns the cosine of the specified angle,
    /// which must be specified in radians. This value is length(adjacent)/length(hypotenuse).
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/cos
    #[wasm_bindgen(static_method_of = Math)]
    pub fn cos(x: f64) -> Number;


    /// The Math.cosh() function returns the hyperbolic cosine of a number,
    /// that can be expressed using the constant e.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/cosh
    #[wasm_bindgen(static_method_of = Math)]
    pub fn cosh(x: f64) -> Number;

    /// The Math.exp() function returns e^x, where x is the argument, and e is Euler's number
    /// (also known as Napier's constant), the base of the natural logarithms.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/exp
    #[wasm_bindgen(static_method_of = Math)]
    pub fn exp(x: f64) -> Number;

    /// The Math.expm1() function returns e^x - 1, where x is the argument, and e the base of the
    /// natural logarithms.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/expm1
    #[wasm_bindgen(static_method_of = Math)]
    pub fn expm1(x: f64) -> Number;

    /// The Math.floor() function returns the largest integer less than or
    /// equal to a given number.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/floor
    #[wasm_bindgen(static_method_of = Math)]
    pub fn floor(x: f64) -> Number;

    /// The Math.fround() function returns the nearest 32-bit single precision float representation
    /// of a Number.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/fround
    #[wasm_bindgen(static_method_of = Math)]
    pub fn fround(x: f64) -> Number;

    /// The Math.imul() function returns the result of the C-like 32-bit multiplication of the
    /// two parameters.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/imul
    #[wasm_bindgen(static_method_of = Math)]
    pub fn imul(x: i32, y: i32) -> Number;

    /// The Math.log() function returns the natural logarithm (base e) of a number.
    /// The JavaScript Math.log() function is equivalent to ln(x) in mathematics.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/log
    #[wasm_bindgen(static_method_of = Math)]
    pub fn log(x: f64) -> Number;

    /// The Math.log10() function returns the base 10 logarithm of a number.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/log10
    #[wasm_bindgen(static_method_of = Math)]
    pub fn log10(x: f64) -> Number;

    /// The Math.log1p() function returns the natural logarithm (base e) of 1 + a number.
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/log1p
    #[wasm_bindgen(static_method_of = Math)]
    pub fn log1p(x: f64) -> Number;

    /// The Math.log2() function returns the base 2 logarithm of a number.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/log2
    #[wasm_bindgen(static_method_of = Math)]
    pub fn log2(x: f64) -> Number;

    /// The Math.pow() function returns the base to the exponent power, that is, base^exponent.
    /// 
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/pow
    #[wasm_bindgen(static_method_of = Math)]
    pub fn pow(base: f64, exponent: f64) -> Number;

    /// The Math.round() function returns the value of a number rounded to the nearest integer.
    /// 
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/round
    #[wasm_bindgen(static_method_of = Math)]
    pub fn round(x: f64) -> Number;

    /// The Math.sign() function returns the sign of a number, indicating whether the number is
    /// positive, negative or zero.
    /// 
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/sign
    #[wasm_bindgen(static_method_of = Math)]
    pub fn sign(x: f64) -> Number;

    /// The Math.sin() function returns the sine of a number.
    /// 
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/sin
    #[wasm_bindgen(static_method_of = Math)]
    pub fn sin(x: f64) -> Number;

    /// The Math.sinh() function returns the hyperbolic sine of a number, that can be expressed
    /// using the constant e: Math.sinh(x) = (e^x - e^-x)/2
    /// 
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/sinh
    #[wasm_bindgen(static_method_of = Math)]
    pub fn sinh(x: f64) -> Number;

    /// The Math.sqrt() function returns the square root of a number, that is
    /// ∀x ≥ 0, Math.sqrt(x) = √x = the unique y ≥ 0 such that y^2 = x
    /// 
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/sqrt
    #[wasm_bindgen(static_method_of = Math)]
    pub fn sqrt(x: f64) -> Number;

    /// The Math.tan() function returns the tangent of a number.
    /// 
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/tan
    #[wasm_bindgen(static_method_of = Math)]
    pub fn tan(x: f64) -> Number;

    /// The Math.tanh() function returns the hyperbolic tangent of a number, that is
    /// tanh x = sinh x / cosh x = (e^x - e^-x)/(e^x + e^-x) = (e^2x - 1)/(e^2x + 1)
    /// 
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/tanh
    #[wasm_bindgen(static_method_of = Math)]
    pub fn tanh(x: f64) -> Number;

    /// The Math.trunc() function returns the integer part of a number by removing any fractional
    /// digits.
    /// 
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/trunc
    #[wasm_bindgen(static_method_of = Math)]
    pub fn trunc(x: f64) -> Number;
}

// Number.
#[wasm_bindgen]
extern "C" {
    pub type Number;

    /// The `Number` JavaScript object is a wrapper object allowing
    /// you to work with numerical values. A `Number` object is
    /// created using the `Number()` constructor.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number
    #[wasm_bindgen(constructor)]
    pub fn new(value: JsValue) -> Number;

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
    pub fn value_of(this: &Number) -> Number;
}

// Date.
#[wasm_bindgen]
extern "C" {
    pub type Date;

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

    /// The Object.isExtensible() method determines if an object is extensible
    /// (whether it can have new properties added to it).
    /// 
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/isExtensible
    #[wasm_bindgen(static_method_of = Object, js_name = isExtensible)]
    pub fn is_extensible(object: &Object) -> bool;

    /// The Object.isFrozen() determines if an object is frozen.
    /// 
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/isFrozen
    #[wasm_bindgen(static_method_of = Object, js_name = isFrozen)]
    pub fn is_frozen(object: &Object) -> bool;

    /// The Object.isSealed() method determines if an object is sealed.
    /// 
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/isSealed
    #[wasm_bindgen(static_method_of = Object, js_name = isSealed)]
    pub fn is_sealed(object: &Object) -> bool;

    /// The isPrototypeOf() method checks if an object exists in another
    /// object's prototype chain.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/isPrototypeOf
    #[wasm_bindgen(method, js_name = isPrototypeOf)]
    pub fn is_prototype_of(this: &Object, value: &JsValue) -> bool;

    /// The Object.keys() method returns an array of a given object's property
    /// names, in the same order as we get with a normal loop.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/keys
    #[wasm_bindgen(static_method_of = Object)]
    pub fn keys(object: &Object) -> Array;

    /// The Object constructor creates an object wrapper.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object
    #[wasm_bindgen(constructor)]
    pub fn new() -> Object;

    /// The Object.preventExtensions() method prevents
    /// new properties from ever being added to an object
    /// (i.e. prevents future extensions to the object).
    /// 
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/preventExtensions
    #[wasm_bindgen(static_method_of = Object, js_name = preventExtensions)]
    pub fn prevent_extensions(object: &Object);

    /// The propertyIsEnumerable() method returns a Boolean indicating
    /// whether the specified property is enumerable.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/propertyIsEnumerable
    #[wasm_bindgen(method, js_name = propertyIsEnumerable)]
    pub fn property_is_enumerable(this: &Object, property: &JsValue) -> bool;

    /// The Object.seal() method seals an object, preventing new properties
    /// from being added to it and marking all existing properties as non-configurable.
    /// Values of present properties can still be changed as long as they are writable.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/seal
    #[wasm_bindgen(static_method_of = Object)]
    pub fn seal(value: &JsValue) -> JsValue;

    /// The Object.setPrototypeOf() method sets the prototype (i.e., the internal
    /// [[Prototype]] property) of a specified object to another object or null.
    /// 
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/setPrototypeOf
    #[wasm_bindgen(static_method_of = Object, js_name = setPrototypeOf)]
    pub fn set_prototype_of(object: &Object, prototype: &Object) -> Object;

    /// The toLocaleString() method returns a string representing the object.
    /// This method is meant to be overridden by derived objects for
    /// locale-specific purposes.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/toLocaleString
    #[wasm_bindgen(method, js_name = toLocaleString)]
    pub fn to_locale_string(this: &Object) -> JsString;

    /// The toString() method returns a string representing the object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/toString
    #[wasm_bindgen(method, js_name = toString)]
    pub fn to_string(this: &Object) -> JsString;

    /// The valueOf() method returns the primitive value of the
    /// specified object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/valueOf
    #[wasm_bindgen(method, js_name = valueOf)]
    pub fn value_of(this: &Object) -> Object;

    /// The Object.values() method returns an array of a given object's
    /// own enumerable property values, in the same order as that provided
    /// by a for...in loop (the difference being that a for-in loop
    /// enumerates properties in the prototype chain as well).
    /// 
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/values
    #[wasm_bindgen(static_method_of = Object)]
    pub fn values(object: &Object) -> Array;
}

// Set
#[wasm_bindgen]
extern {
    pub type Set;

    /// The add() method appends a new element with a specified value to the 
    /// end of a Set object.
    /// 
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/add
    #[wasm_bindgen(method)]
    pub fn add(this: &Set, value: &JsValue) -> Set;

    /// The clear() method removes all elements from a Set object.
    /// 
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/clear
    #[wasm_bindgen(method)]
    pub fn clear(this: &Set);

    /// The delete() method removes the specified element from a Set object.
    /// 
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/delete
    #[wasm_bindgen(method)]
    pub fn delete(this: &Set, value: &JsValue) -> bool;

    /// The has() method returns a boolean indicating whether an element 
    /// with the specified value exists in a Set object or not.
    /// 
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/has
    #[wasm_bindgen(method)]
    pub fn has(this: &Set, value: &JsValue) -> bool;

    /// The Set object lets you store unique values of any type, whether primitive 
    /// values or object references.
    /// 
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set
    #[wasm_bindgen(constructor)]
    pub fn new() -> Set;

    /// The size accessor property returns the number of elements in a Set object.
    /// 
    /// https://developer.mozilla.org/de/docs/Web/JavaScript/Reference/Global_Objects/Set/size
    #[wasm_bindgen(method, getter, structural)]
    pub fn size(this: &Set) -> Number;
}

// SetIterator
#[wasm_bindgen]
extern {
    pub type SetIterator;

    /// The entries() method returns a new Iterator object that contains 
    /// an array of [value, value] for each element in the Set object, 
    /// in insertion order. For Set objects there is no key like in 
    /// Map objects. However, to keep the API similar to the Map object,
    /// each entry has the same value for its key and value here, so that 
    /// an array [value, value] is returned.
    /// 
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/entries
    #[wasm_bindgen(method)]
    pub fn entries(set: &Set) -> SetIterator;

    /// The keys() method is an alias for this method (for similarity with 
    /// Map objects); it behaves exactly the same and returns values 
    /// of Set elements.
    /// 
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/values
    #[wasm_bindgen(method)]
    pub fn keys(set: &Set) -> SetIterator;

    /// The values() method returns a new Iterator object that contains the 
    /// values for each element in the Set object in insertion order.
    /// 
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/values
    #[wasm_bindgen(method)]
    pub fn values(set: &Set) -> SetIterator;
}

// WeakMap
#[wasm_bindgen]
extern "C" {
    pub type WeakMap;

    /// The WeakMap object is a collection of key/value pairs in which the keys are weakly referenced.  
    /// The keys must be objects and the values can be arbitrary values.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakMap
    #[wasm_bindgen(constructor)]
    pub fn new() -> WeakMap;

    /// The set() method sets the value for the key in the WeakMap object. Returns
    /// the WeakMap object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakMap/set
    #[wasm_bindgen(method, js_class = "WeakMap")]
    pub fn set(this: &WeakMap, key: Object, value: JsValue) -> WeakMap;

    /// The get() method returns a specified by key element
    /// from a WeakMap object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakMap/get
    #[wasm_bindgen(method)]
    pub fn get(this: &WeakMap, key: Object) -> JsValue;

    /// The has() method returns a boolean indicating whether an element with
    /// the specified key exists in the WeakMap object or not.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakMap/has
    #[wasm_bindgen(method)]
    pub fn has(this: &WeakMap, key: Object) -> bool;

    /// The delete() method removes the specified element from a WeakMap object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakMap/delete
    #[wasm_bindgen(method)]
    pub fn delete(this: &WeakMap, key: Object) -> bool;
}

// WeakSet
#[wasm_bindgen]
extern "C" {
    pub type WeakSet;

    /// The WeakSet object lets you store weakly held objects in a collection.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakSet
    #[wasm_bindgen(constructor)]
    pub fn new() -> WeakSet;

    /// The has() method returns a boolean indicating whether an object exists in a WeakSet or not.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakSet/has
    #[wasm_bindgen(method)]
    pub fn has(this: &WeakSet, value: Object) -> bool;

    /// The add() method appends a new object to the end of a WeakSet object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakSet/add
    #[wasm_bindgen(method)]
    pub fn add(this: &WeakSet, value: Object) -> WeakSet;

    /// The delete() method removes the specified element from a WeakSet object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakSet/delete
    #[wasm_bindgen(method)]
    pub fn delete(this: &WeakSet, value: Object) -> bool;
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

    /// The String object's charAt() method returns a new string consisting of the single
    /// UTF-16 code unit located at the specified offset into the string.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/charAt
    #[wasm_bindgen(method, js_class = "String", js_name = charAt)]
    pub fn char_at(this: &JsString, index: u32) -> JsString;

    /// The charCodeAt() method returns an integer between 0 and 65535 representing the UTF-16 code unit at
    /// the given index (the UTF-16 code unit matches the Unicode code point for code points representable in
    /// a single UTF-16 code unit, but might also be the first code unit of a surrogate pair for
    /// code points not representable in a single UTF-16 code unit, e.g. Unicode code points > 0x10000).
    /// If you want the entire code point value, use codePointAt().
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/charCodeAt
    #[wasm_bindgen(method, js_class = "String", js_name = charCodeAt)]
    pub fn char_code_at(this: &JsString, index: u32) -> Number;

    /// The codePointAt() method returns a non-negative integer that is the Unicode code point value.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/codePointAt
    #[wasm_bindgen(method, js_class = "String", js_name = codePointAt)]
    pub fn code_point_at(this: &JsString, pos: u32) -> JsValue;

    /// The concat() method concatenates the string arguments to the calling string and returns a new string.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/concat
    #[wasm_bindgen(method, js_class = "String")]
    pub fn concat(this: &JsString, string_2: &JsString) -> JsString;

    /// The includes() method determines whether one string may be found within another string, returning true or false as appropriate.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/includes
    #[wasm_bindgen(method, js_class = "String")]
    pub fn includes(this: &JsString, search_string: &JsString, position: i32) -> bool;

    /// The indexOf() method returns the index within the calling String object of
    /// the first occurrence of the specified value, starting the search at fromIndex.
    /// Returns -1 if the value is not found.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/indexOf
    #[wasm_bindgen(method, js_class = "String", js_name = indexOf)]
    pub fn index_of(this: &JsString, search_value: &JsString, from_index: i32) -> i32;

    /// The slice() method extracts a section of a string and returns it as a
    /// new string, without modifying the original string.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/slice
    #[wasm_bindgen(method, js_class = "String")]
    pub fn slice(this: &JsString, start: u32, end: u32) -> JsString;

    /// The startsWith() method determines whether a string begins with the characters
    /// of a specified string, returning true or false as appropriate.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/startsWith
    #[wasm_bindgen(method, js_class = "String", js_name = startsWith)]
    pub fn starts_with(this: &JsString, search_string: &JsString, position: u32) -> bool;

    /// The substring() method returns the part of the string between the start and end indexes,
    /// or to the end of the string.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/substring
    #[wasm_bindgen(method, js_class = "String")]
    pub fn substring(this: &JsString, index_start: u32, index_end: u32) -> JsString;

    /// The substr() method returns the part of a string between
    /// the start index and a number of characters after it.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/substr
    #[wasm_bindgen(method, js_class = "String")]
    pub fn substr(this: &JsString, start: i32, length: i32) -> JsString;

    /// The toLowerCase() method returns the calling string value
    /// converted to lower case.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/toLowerCase
    #[wasm_bindgen(method, js_class = "String", js_name = toLowerCase)]
    pub fn to_lower_case(this: &JsString) -> JsString;

    /// The toString() method returns a string representing the specified object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/toString
    #[wasm_bindgen(method, js_class = "String", js_name = toString)]
    pub fn to_string(this: &JsString) -> JsString;

    /// The toUpperCase() method returns the calling string value
    /// converted to uppercase (the value will be converted to a
    /// string if it isn't one).
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/toUpperCase
    #[wasm_bindgen(method, js_class = "String", js_name = toUpperCase)]
    pub fn to_upper_case(this: &JsString) -> JsString;

    /// The trim() method removes whitespace from both ends of a string.
    /// Whitespace in this context is all the whitespace characters
    /// (space, tab, no-break space, etc.) and all the line terminator characters (LF, CR, etc.).
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/trim
    #[wasm_bindgen(method, js_class = "String")]
    pub fn trim(this: &JsString) -> JsString;

    /// The trimEnd() method removes whitespace from the end of a string.
    /// trimRight() is an alias of this method.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/trimEnd
    #[wasm_bindgen(method, js_class = "String", js_name = trimEnd)]
    pub fn trim_end(this: &JsString) -> JsString;

    /// The trimEnd() method removes whitespace from the end of a string.
    /// trimRight() is an alias of this method.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/trimEnd
    #[wasm_bindgen(method, js_class = "String", js_name = trimRight)]
    pub fn trim_right(this: &JsString) -> JsString;

    /// The trimStart() method removes whitespace from the beginning of a string.
    /// trimLeft() is an alias of this method.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/trimStart
    #[wasm_bindgen(method, js_class = "String", js_name = trimStart)]
    pub fn trim_start(this: &JsString) -> JsString;

    /// The trimStart() method removes whitespace from the beginning of a string.
    /// trimLeft() is an alias of this method.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/trimStart
    #[wasm_bindgen(method, js_class = "String", js_name = trimLeft)]
    pub fn trim_left(this: &JsString) -> JsString;

    /// The valueOf() method returns the primitive value of a String object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/valueOf
    #[wasm_bindgen(method, js_class = "String", js_name = valueOf)]
    pub fn value_of(this: &JsString) -> JsString;
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
