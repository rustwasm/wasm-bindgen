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
    #[wasm_bindgen(catch, js_name = decodeURI)]
    pub fn decode_uri(encoded: &str) -> Result<JsString, JsValue>;

    /// The `encodeURI()` function encodes a Uniform Resource Identifier (URI)
    /// by replacing each instance of certain characters by one, two, three, or
    /// four escape sequences representing the UTF-8 encoding of the character
    /// (will only be four escape sequences for characters composed of two
    /// "surrogate" characters).
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURI
    #[wasm_bindgen(js_name = encodeURI)]
    pub fn encode_uri(decoded: &str) -> JsString;

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

    /// The length property of an object which is an instance of type Array
    /// sets or returns the number of elements in that array. The value is an
    /// unsigned, 32-bit integer that is always numerically greater than the
    /// highest index in the array.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/length
    #[wasm_bindgen(method, getter, structural)]
    pub fn length(this: &Array) -> u32;

    /// The indexOf() method returns the first index at which a given element
    /// can be found in the array, or -1 if it is not present.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/indexOf
    #[wasm_bindgen(method, js_name = indexOf)]
    pub fn index_of(this: &Array, value: JsValue, from_index: i32) -> i32;

    /// The includes() method determines whether an array includes a certain
    /// element, returning true or false as appropriate.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/includes
    #[wasm_bindgen(method)]
    pub fn includes(this: &Array, value: JsValue, from_index: i32) -> bool;

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

    /// The slice() method returns a shallow copy of a portion of an array into
    /// a new array object selected from begin to end (end not included).
    /// The original array will not be modified.
    ///
    /// http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/slice
    #[wasm_bindgen(method)]
    pub fn slice(this: &Array, start: u32, end: u32) -> Array;

    /// The shift() method removes the first element from an array and returns
    /// that removed element. This method changes the length of the array.
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
extern {
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

// Function
#[wasm_bindgen]
extern {
    pub type JsFunction;

    /// The length property indicates the number of arguments expected by the function.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/length
    #[wasm_bindgen(method, getter, structural)]
    pub fn length(this: &JsFunction) -> u32;

    /// A Function object's read-only name property indicates the function's
    /// name as specified when it was created or "anonymous" for functions
    /// created anonymously.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/name
    #[wasm_bindgen(method, getter, structural)]
    pub fn name(this: &JsFunction) -> JsString;
}

// Math
#[wasm_bindgen]
extern {
    pub type Math;
    /// The Math.abs() function returns the absolute value of a number, that is
    /// Math.abs(x) = |x|
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/abs
    #[wasm_bindgen(static_method_of = Math)]
    pub fn abs(number: i32) -> Number;

    /// The Math.acos() function returns the arccosine (in radians) of a
    /// number, that is ∀x∊[-1;1]
    /// Math.acos(x) = arccos(x) = the unique y∊[0;π] such that cos(y)=x
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/acos
    #[wasm_bindgen(static_method_of = Math)]
    pub fn acos(adjacent: i32, hypotenuse: i32) -> Number;

    /// The Math.acosh() function returns the hyperbolic arc-cosine of a
    /// number, that is ∀x ≥ 1
    /// Math.acosh(x) = arcosh(x) = the unique y ≥ 0 such that cosh(y) = x
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/acosh
    #[wasm_bindgen(static_method_of = Math)]
    pub fn acosh(number: i32) -> Number;

    /// The Math.asin() function returns the arcsine (in radians) of a
    /// number, that is ∀x ∊ [-1;1]
    /// Math.asin(x) = arcsin(x) = the unique y∊[-π2;π2] such that sin(y) = x
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/asin
    #[wasm_bindgen(static_method_of = Math)]
    pub fn asin(number: i32) -> Number;

    /// The Math.asinh() function returns the hyperbolic arcsine of a
    /// number, that is Math.asinh(x) = arsinh(x) = the unique y such that sinh(y) = x
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/asinh
    #[wasm_bindgen(static_method_of = Math)]
    pub fn asinh(number: i32) -> Number;

    /// The Math.atan() function returns the arctangent (in radians) of a
    /// number, that is Math.atan(x) = arctan(x) = the unique y ∊ [-π2;π2]such that
    /// tan(y) = x
    #[wasm_bindgen(static_method_of = Math)]
    pub fn atan(number: i32) -> Number;

    /// The Math.atan2() function returns the arctangent of the quotient of
    /// its arguments.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/atan2
    #[wasm_bindgen(static_method_of = Math)]
    pub fn atan2(y: i32, x: i32) -> Number;

    /// The Math.atanh() function returns the hyperbolic arctangent of a number,
    /// that is ∀x ∊ (-1,1), Math.atanh(x) = arctanh(x) = the unique y such that
    /// tanh(y) = x
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/atanh
    #[wasm_bindgen(static_method_of = Math)]
    pub fn atanh(x: i32) -> Number;


    /// The Math.cbrt() function returns the cube root of a number, that is
    /// Math.cbrt(x) = x^3 = the unique y such that y^3 = x
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/cbrt
    #[wasm_bindgen(static_method_of = Math)]
    pub fn cbrt(x: i32) -> Number;

    /// The Math.ceil() function returns the smallest integer greater than
    /// or equal to a given number.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/ceil
    #[wasm_bindgen(static_method_of = Math)]
    pub fn ceil(x: f32) -> Number;

    /// The Math.clz32() function returns the number of leading zero bits in
    /// the 32-bit binary representation of a number.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/clz32
    #[wasm_bindgen(static_method_of = Math)]
    pub fn clz32(x: i32) -> Number;
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
extern {
    pub type Date;

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

    /// The valueOf() method  returns the primitive value of
    /// a Date object.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/valueOf
    #[wasm_bindgen(method, js_name = valueOf)]
    pub fn value_of(this: &Date) -> Date;
}

// Object.
#[wasm_bindgen]
extern {
    pub type Object;

    /// The `hasOwnProperty()` method returns a boolean indicating whether the
    /// object has the specified property as its own property (as opposed to
    /// inheriting it).
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/hasOwnProperty
    #[wasm_bindgen(method, js_name = hasOwnProperty)]
    pub fn has_own_property(this: &Object, property: &JsValue) -> bool;

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

    /// The propertyIsEnumerable() method returns a Boolean indicating
    /// whether the specified property is enumerable.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/propertyIsEnumerable
    #[wasm_bindgen(method, js_name = propertyIsEnumerable)]
    pub fn property_is_enumerable(this: &Object, property: &JsValue) -> bool;

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
}

// JsString
#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_name = JsString)]
    pub type JsString;

    /// The String object's charAt() method returns a new string consisting of the single
    /// UTF-16 code unit located at the specified offset into the string.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/charAt
    #[wasm_bindgen(method, js_class = "String", js_name = charAt)]
    pub fn char_at(this: &JsString, index: u32) -> JsString;

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
