use wasm_bindgen::prelude::*;
use wasm_bindgen::JsError;
use wasm_bindgen_test::*;

use std::fmt;

#[derive(Clone, Debug)]
enum MyError {
    Variant,
    InflightShouldBeFalse,
}
// shouldn't technically need this, surely
impl std::error::Error for MyError {}
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyError::{:?}", self)
    }
}

macro_rules! call_test {
    ($test_fn:ident, $js_fn:ident) => {
        #[wasm_bindgen_test]
        fn $test_fn() {
            #[wasm_bindgen(module = "tests/wasm/result_jserror.js")]
            extern "C" {
                fn $js_fn();
            }
            $js_fn();
        }
    };
}

#[wasm_bindgen]
pub fn return_err() -> Result<i32, JsError> {
    let e = Err(MyError::Variant)?;
    Ok(e)
}
call_test!(test_err, call_err);

#[wasm_bindgen]
pub fn return_ok() -> Result<i32, JsError> {
    Ok(5)
}
call_test!(test_ok, call_ok);

#[wasm_bindgen]
pub fn make_an_error() -> JsError {
    JsError::new("un-thrown error").into()
}
call_test!(test_make_an_error, call_make_an_error);

#[wasm_bindgen]
pub struct MyStruct {
    inflight: bool,
}

call_test!(test_struct, all_struct_methods);

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn console_log(s: &str);

}

struct ResetOnDrop<'a> {
    flag: &'a mut bool,
}

impl<'a> Drop for ResetOnDrop<'a> {
    fn drop(&mut self) {
        // console_log("resetting flag => false");
        *self.flag = false;
    }
}

impl<'a> ResetOnDrop<'a> {
    fn new(flag: &'a mut bool) -> Result<Self, MyError> {
        if *flag {
            return Err(MyError::InflightShouldBeFalse);
        }
        // console_log("setting flag => true");
        Ok(Self { flag })
    }
}

#[wasm_bindgen]
impl MyStruct {
    #[wasm_bindgen]
    pub fn new() -> Result<MyStruct, JsError> {
        Ok(MyStruct { inflight: false })
    }

    #[wasm_bindgen]
    pub fn new_err() -> Result<MyStruct, JsError> {
        Err(MyError::Variant.into())
    }

    #[wasm_bindgen]
    pub fn return_ok(&mut self) -> Result<i32, JsError> {
        let _guard = ResetOnDrop::new(&mut self.inflight)?;
        Ok(5)
    }

    #[wasm_bindgen]
    pub fn return_err(&mut self) -> Result<i32, JsError> {
        let guard = ResetOnDrop::new(&mut self.inflight)?;
        let err = Err(MyError::Variant);
        let nope = err?;
        // we are checking both for the flag being reset (from js, via is_inflight)
        // and for the running of drop code
        drop(guard);
        Ok(nope)
    }

    #[wasm_bindgen]
    pub fn is_inflight(&self) -> bool {
        self.inflight
    }
}

// check some more Ok types
#[wasm_bindgen]
pub fn return_string() -> Result<String, JsError> {
    Ok("string here".into())
}
call_test!(test_return_string, call_return_string);

// now we check that jsvalue works, as it did before (it will still leak though, as long as its
// ReturnWasmAbi impl still uses crate::throw_val)

#[wasm_bindgen]
pub fn return_jsvalue_ok() -> Result<i32, JsValue> {
    Ok(5)
}
call_test!(test_jsvalue_ok, call_jsvalue_ok);

#[wasm_bindgen]
pub fn return_jsvalue_err() -> Result<i32, JsValue> {
    Err(JsValue::from(-1i32))
}
call_test!(test_jsvalue_err, call_jsvalue_err);
