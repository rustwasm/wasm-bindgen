# `function-attributes`

By default, exported Rust functions and methods generate function signature from equivalent Rust types identifiers without any arguments and return variable documentations unless completely handcoded using `skip-jsdoc` and `typescript-custom-section` with `skip_typescript`, however by using function attributes, it's possible to override a function's return type and arguments names and types for generated bindings as well as the ability to write specific documentation for each of them individually as desired:
- `#[wasm_bindgen(unchecked_return_type)]` and `#[wasm_bindgen(return_description)]` used to override a function's return type and to specify descriptions for generated JS/TS bindings.
- `#[wasm_bindgen(js_name)]`, `#[wasm_bindgen(unchecked_param_type)]` and `#[wasm_bindgen(param_description)]` applied to a Rust function argument to override that argument's name and type and to specify descriptions for generated JS/TS bindings.

For example a Rust function can return `JsValue` by serializing a Rust type using serde, yet on generated TS bindings instead of `any` as the return type, it can be overridden to the TS interface of the equivalent of the serialized Rust type defined using `typescript-custom-section`:
```rust
// we wont use "#[wasm_bindgen]" for this struct
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Person {
    pub name: String;
}

// define a TS interface equivalent of Person struct
#[wasm_bindgen(typescript_custom_section)]
const TS_INTERFACE_EXPORT: &'static str = r"
export interface Person { name: string; }
";

#[wasm_bindgen(unchecked_return_type = "Person", return_description = "a Person object")]
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
this will generate the following TS bindings:
```ts
export interface Person { name: string; }

/**
* @param personName - Specifies the person's name
* @returns a Person object
*/
export function build_person(personName: string): Promise<Person>;
```
As you can see, using function attributes, we can now return a JS/TS object without ever using `wasm_bindgen` macro on `Person` struct that would have generated a JS/TS class object in the bindings which might not be the desired outcome for every case, you can see as well that instead of handcoding the full documentation for `build_person()` with all JSDoc/TSDoc tags and syntax, we can just write specific description for each individual component of the function and as a result end up with a fully typed and documented function in the bindings.

**IMPORTANT NOTE**
Types that are provided using `#[wasm_bindgen(unchecked_return_type)]` and `#[wasm_bindgen(unchecked_param_type)]` aren't checked as these attributes' identifiers entail, meaning they will end up in the function's signature and docs bindings exactly as they have been specified and there are no checks/validation for them in place, so only because a user uses '#[wasm_bindgen(unchecked_param_type = "number")]' for example, it doesn't necessarily mean it's actually going to be a value of number type, therefore validation and checks between the value and its type should be handled by the user and the responsibility of using them correctly and carefully relies solely on the user.

Let's look at some more exmaples in details:
```rust
/// Description for foo
#[wasm_bindgen(unchecked_return_type = "Foo", return_description = "some description for return type")]
pub async fn foo(
    #[wasm_bindgen(js_name = "firstArg", param_description = "some description for firstArg")]
    arg1: String,
    #[wasm_bindgen(js_name = "secondArg", unchecked_param_type = "Bar")]
    arg2: JsValue,
) -> Result<JsValue, JsValue> {
    // function body
}
```
This will generate the following JS bindings:
```js
/**
* Description for foo
* @param {string} firstArg - some description for firstArg
* @param {Bar} secondArg
* @returns {Promise<Foo>} some description for return type
*/
export function foo(firstArg, secondArg) {};
```
And will generate the following TS bindings:
```ts
/**
* Description for foo
* @param firstArg - some description for firstArg
* @param secondArg
* @returns some description for return type
*/
export function foo(firstArg: string, secondArg: Bar): Promise<Foo>;
```

Same thing applies to Rust struct's (and enums) impl methods and their equivalent JS/TS/TS class methods:
```rust
/// Description for Foo
#[wasm_bindgen]
pub struct Foo {
    // properties
}

#[wasm_bindgen]
impl Foo {
    /// Description for foo
    #[wasm_bindgen(unchecked_return_type = "Baz", return_description = "some description for return type")]
    pub fn foo(
        &self,
        #[wasm_bindgen(js_name = "firstArg", param_description = "some description for firstArg")]
        arg1: String,
        #[wasm_bindgen(js_name = "secondArg", unchecked_param_type = "Bar")]
        arg2: JsValue,
    ) -> JsValue {
        // function body
    }
}
```

This will generate the following JS bindings:
```js
/**
* Description for Foo
*/
export class Foo {
    /**
    * Description for foo
    * @param {string} firstArg - some description for firstArg
    * @param {Bar} secondArg
    * @returns {Baz} some description for return type
    */
    foo(firstArg, secondArg) {};
}
```

And will generate the following TS bindings:
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
    foo(firstArg: string, secondArg: Bar): Baz;
}
```

As shown in these examples, these attributes allows for a great level of control and customization over generated bindings. But note that they can only be used on functions and methods that are being exported to JS/TS/TS and cannot be used on the `self` argument of Rust struct/enum methods.
