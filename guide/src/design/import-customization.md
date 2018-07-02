# Customizing import behavior

The `#[wasm_bindgen]` macro supports a good amount of configuration for
controlling precisely how imports are imported and what they map to in JS. This
section is intended to hopefully be an exhaustive reference of the
possibilities!

* `module` and `version` - we've seen `module` so far indicating where we can
  import items from but `version` is also allowed:

  ```rust
  #[wasm_bindgen(module = "moment", version = "^2.0.0")]
  extern {
      type Moment;
      fn moment() -> Moment;
      #[wasm_bindgen(method)]
      fn format(this: &Moment) -> String;
  }
  ```

  The `module` key is used to configure the module that each item is imported
  from. The `version` key does not affect the generated wasm itself but rather
  it's an informative directive for tools like [wasm-pack]. Tools like wasm-pack
  will generate a `package.json` for you and the `version` listed here, when
  `module` is also an NPM package, will correspond to what to write down in
  `package.json`.

  In other words the usage of `module` as the name of an NPM package and
  `version` as the version requirement allows you to, inline in Rust, depend on
  the NPM ecosystem and import functionality from those packages. When bundled
  with a tool like [wasm-pack] everything will automatically get wired up with
  bundlers and you should be good to go!

  Note that the `version` is *required* if `module` doesn't start with `./`. If
  `module` starts with `./` then it is an error to provide a version.

[wasm-pack]: https://github.com/rustwasm/wasm-pack

* `catch` - this attribute allows catching a JS exception. This can be attached
  to any imported function and the function must return a `Result` where the
  `Err` payload is a `JsValue`, like so:

  ```rust
  #[wasm_bindgen]
  extern {
      #[wasm_bindgen(catch)]
      fn foo() -> Result<(), JsValue>;
  }
  ```

  If the imported function throws an exception then `Err` will be returned with
  the exception that was raised, and otherwise `Ok` is returned with the result
  of the function.

  By default `wasm-bindgen` will take no action when wasm calls a JS function
  which ends up throwing an exception. The wasm spec right now doesn't support
  stack unwinding and as a result Rust code **will not execute destructors**.
  This can unfortunately cause memory leaks in Rust right now, but as soon as
  wasm implements catching exceptions we'll be sure to add support as well!

* `constructor` - this is used to indicate that the function being bound should
  actually translate to a `new` constructor in JS. The final argument must be a
  type that's imported from JS, and it's what'll get used in JS:

  ```rust
  #[wasm_bindgen]
  extern {
      type Foo;
      #[wasm_bindgen(constructor)]
      fn new() -> Foo;
  }
  ```

  This will attach the `new` function to the `Foo` type (implied by
  `constructor`) and in JS when this function is called it will be equivalent to
  `new Foo()`.

* `method` - this is the gateway to adding methods to imported objects or
  otherwise accessing properties on objects via methods and such. This should be
  done for doing the equivalent of expressions like `foo.bar()` in JS.

  ```rust
  #[wasm_bindgen]
  extern {
      type Foo;
      #[wasm_bindgen(method)]
      fn work(this: &Foo);
  }
  ```

  The first argument of a `method` annotation must be a borrowed reference (not
  mutable, shared) to the type that the method is attached to. In this case
  we'll be able to call this method like `foo.work()` in JS (where `foo` has
  type `Foo`).

  In JS this invocation will correspond to accessing `Foo.prototype.work` and
  then calling that when the import is called. Note that `method` by default
  implies going through `prototype` to get a function pointer.

* `js_namespace` - this attribute indicates that the JS type is accessed through
  a particular namespace. For example the `WebAssembly.Module` APIs are all
  accessed through the `WebAssembly` namespace. The `js_namespace` can be
  applied to any import and whenever the generated JS attempts to reference a
  name (like a class or function name) it'll be accessed through this namespace.

  ```rust
  #[wasm_bindgen]
  extern {
      #[wasm_bindgen(js_namespace = console)]
      fn log(s: &str);
  }
  ```

  This is an example of how to bind `console.log(x)` in Rust. The `log` function
  will be available in the Rust module and will be invoked as `console.log` in
  JS.

* `getter` and `setter` - these two attributes can be combined with `method` to
  indicate that this is a getter or setter method. A `getter`-tagged function by
  default accesses the JS property with the same name as the getter function. A
  `setter`'s function name is currently required to start with "set\_" and the
  property it accesses is the suffix after "set\_".

  ```rust
  #[wasm_bindgen]
  extern {
      type Foo;
      #[wasm_bindgen(method, getter)]
      fn property(this: &Foo) -> u32;
      #[wasm_bindgen(method, setter)]
      fn set_property(this: &Foo, val: u32);
  }
  ```

  Here we're importing the `Foo` type and defining the ability to access each
  object's `property` property. The first function here is a getter and will be
  available in Rust as `foo.property()`, and the latter is the setter which is
  accessible as `foo.set_property(2)`. Note that both functions have a `this`
  argument as they're tagged with `method`.

  Finally, you can also pass an argument to the `getter` and `setter`
  properties to configure what property is accessed. When the property is
  explicitly specified then there is no restriction on the method name. For
  example the below is equivalent to the above:

  ```rust
  #[wasm_bindgen]
  extern {
      type Foo;
      #[wasm_bindgen(method, getter = property)]
      fn assorted_method_name(this: &Foo) -> u32;
      #[wasm_bindgen(method, setter = "property")]
      fn some_other_method_name(this: &Foo, val: u32);
  }
  ```

  Properties in JS are accessed through `Object.getOwnPropertyDescriptor`. Note
  that this typically only works for class-like-defined properties which aren't
  just attached properties on any old object. For accessing any old property on
  an object we can use...

* `structural` - this is a flag to `method` annotations which indicates that the
  method being accessed (or property with getters/setters) should be accessed in
  a structural fashion. For example methods are *not* accessed through
  `prototype` and properties are accessed on the object directly rather than
  through `Object.getOwnPropertyDescriptor`.

  ```rust
  #[wasm_bindgen]
  extern {
      type Foo;
      #[wasm_bindgen(method, structural)]
      fn bar(this: &Foo);
      #[wasm_bindgen(method, getter, structural)]
      fn baz(this: &Foo) -> u32;
  }
  ```

  The type here, `Foo`, is not required to exist in JS (it's not referenced).
  Instead wasm-bindgen will generate shims that will access the passed in JS
  value's `bar` property to or the `baz` property (depending on the function).

* `js_name = foo` - this can be used to bind to a different function in JS than
  the identifier that's defined in Rust. For example you can also define
  multiple signatures for a polymorphic function in JS as well:

  ```rust
  #[wasm_bindgen]
  extern {
      type Foo;
      #[wasm_bindgen(js_namespace = console, js_name = log)]
      fn log_string(s: &str);
      #[wasm_bindgen(js_namespace = console, js_name = log)]
      fn log_u32(n: u32);
      #[wasm_bindgen(js_namespace = console, js_name = log)]
      fn log_many(a: u32, b: JsValue);
  }
  ```

  All of these functions will call `console.log` in Rust, but each identifier
  will have only one signature in Rust.
