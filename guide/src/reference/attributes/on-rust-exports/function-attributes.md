# `function-attributes`

By default, exported Rust functions and methods generate function signature from equivalent rust types identifiers without any arguments and return var documentations unless completely handcoded using `skip-jsdoc` and `typescript-custom-section` with `skip_typescript` which does not provide the best solution, specially when one needs to do a lot of those handcodings.
It's fair to say that being able to write specific documentation for each argument and return variable of the function, or override an argument or return variable type to a custom one for generated js/ts bindings and many more use cases are essential for creating a well defined, structured and typed bindings.

Function attributes addresses these issues and limitations by providing an ability to override a function's return type and arguments names and types for generated bindings as well as ability to write specific documentation for each of them individually as desired:
- `#[wasm_bindgen(return_type)]` and `#[wasm_bindgen(return_description)]` used to override function's return type and to specify description for generated js/ts bindings.
- `#[wasm_bindgen(js_name)]`, `#[wasm_bindgen(param_type)]` and `#[wasm_bindgen(param_description)]` applied to a rust function argument to override that argument's name and type and to specify description for generated js/ts bindings.
- `#[wasm_bindgen(optional)]` used to tag a function's argument as optional (typescript `?` operator) for function's generated typescript signature.

For example a rust function can return `JsValue` by serializing a rust type using serde, yet on generated ts bindings instead of `any` as the return type, it can be overriden to the ts interface of the serialized rust type equivalent defined using `typescript-custom-section` (or using [Tsify Crate](https://crates.io/crates/tsify)):
```rust
// we wont use "#[wasm_bindgen]" for this struct
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Person {
    pub name: String;
}

// define a ts interface equivalent of Person struct
// alternatively Tsify crate can be used to generate ts interface from rust types
#[wasm_bindgen(typescript_custom_section)]
const TS_INTERFACE_EXPORT: &'static str = r"
  export interface Person { name: string; }
";

#[wasm_bindgen(return_type = "Person", return_description = "a Person object")]
pub async fn build_person(
    #[wasm_bindgen(js_name = "personName", param_description = "Specifies the person's name")]
    person_name: String,
) -> Result<JsValue, JsValue> {
    //
    // some async operations
    //
    Ok(serde_wasm_bindgen::to_value(&Person { name: person_name })?)
}
```
this will generate the following ts bindings:
```ts
export interface Person { name: string; }

/**
* @param personName - Specifies the person's name
* @returns a Person object
*/
export function build_person(personName: string): Promise<Person>;
```
As you can see, using function attributes, we can now return a js/ts object without ever using `wasm_bindgen` macro on `Person` struct that would have generated a js/ts class object in the bindings which might not be the desired outcome for every case, you can see as well that instead of handcoding the full documentation for `build_person()` with all js_doc/ts_doc tags and syntax, we can just write specific docs for each individual component of the function and as a result end up with a fully typed and documented function in the bindings.

Let's look at some more exmaples in details:
```rust
/// Description for foo
#[wasm_bindgen(return_type = "Foo", return_description = "some description for return type")]
pub async fn foo(
    #[wasm_bindgen(js_name = "firstArg", param_description = "some description for firstArg")]
    arg1: String,
    #[wasm_bindgen(js_name = "secondArg", param_type = "Bar", optional)]
    arg2: JsValue,
) -> Result<JsValue, JsValue> {
    // function body
}
```
This will generate the following js bindings:
```js
/**
* Description for foo
* @param {string} firstArg - some description for firstArg
* @param {Bar} [secondArg]
* @returns {Promise<Foo>} some description for return type
*/
export function foo(firstArg, secondArg) {};
```
And will generate the following ts bindings:
```ts
/**
* Description for foo
* @param firstArg - some description for firstArg
* @param secondArg
* @returns some description for return type
*/
export function foo(firstArg: string, secondArg?: Bar): Promise<Foo>;
```

Same thing applies to rust struct's (and enums) impl methods and their equivalent js/ts class methods:
```rust
/// Description for Foo
#[wasm_bindgen]
pub struct Foo {
    pub Foo: String,
}

#[wasm_bindgen]
impl Foo {
    /// Description for foo
    #[wasm_bindgen(return_type = "Baz", return_description = "some description for return type")]
    pub fn foo(
        &self,
        #[wasm_bindgen(js_name = "firstArg", param_description = "some description for firstArg")]
        arg1: String,
        #[wasm_bindgen(js_name = "secondArg", param_type = "Bar", optional)]
        arg2: JsValue,
    ) -> JsValue {
        // function body
    }
}
```

This will generate the following js bindings:
```js
/**
* Description for Foo
*/
export class Foo {
    /**
    * Description for foo
    * @param {string} firstArg - some description for firstArg
    * @param {Bar} [secondArg]
    * @returns {Baz} some description for return type
    */
    foo(firstArg, secondArg) {};
}
```

And will generate the following ts bindings:
```ts
/**
* Description for Foo
*/
export class Foo {
    /**
    * Description for foo
    * @param firstArg - some description for firstArg
    * @param secondArg
    * @returns some description for return type
    */
    foo(firstArg: string, secondArg?: Bar): Baz;
}
```

As shown in exmaples, these attributes allows for great level of control and customization over generated bindings but note that they can only be used on functions and methods that are being exported to js/ts and cannot be used on `self` argument of rust structs/enums methods.