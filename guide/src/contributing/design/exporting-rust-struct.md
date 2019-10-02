# Exporting a struct to JS

So far we've covered JS objects, importing functions, and exporting functions.
This has given us quite a rich base to build on so far, and that's great! We
sometimes, though, want to go even further and define a JS `class` in Rust. Or
in other words, we want to expose an object with methods from Rust to JS rather
than just importing/exporting free functions.

The `#[wasm_bindgen]` attribute can annotate both a `struct` and `impl` blocks
to allow:

```rust
#[wasm_bindgen(prototype=Bar)]
pub struct Foo {
    internal: i32,
}

#[wasm_bindgen]
impl Foo {
    #[wasm_bindgen(constructor)]
    pub fn new(val: i32) -> WasmType<Foo> {
        instantiate! {
            super("abc", 123);
            Foo { internal: val }
        }
    }

    pub fn new_zero() -> WasmType<Foo> {
        Foo::new(0)
    }

    pub fn get(&self) -> i32 {
        self.internal
    }

    pub fn set(&mut self, val: i32) {
        self.internal = val;
    }
}
```

This is a typical Rust `struct` definition for a type with a constructor and a
few methods. Annotating the struct with `#[wasm_bindgen]` means that we'll
inject an additional field into it and generate necessary trait impls to convert
this type (wrapped as `WasmType<T>`, which is a vendored `Rc<RefCell<T>>`) to/from
the JS boundary.  The `instantiate!` macro within a method marked as `constructor`
will inject a value for this field into the given struct literal, and evaluates
to the wrapped `WasmType<T>`.

The annotated `impl` block here means that the functions inside will also be made
available to JS through generated shims. If we take a look at the generated JS
code for this we'll see:

```js
import * as wasm from './js_hello_world_bg';

const HOST_PTR = Symbol('HOST_PTR');
const BORROW = Symbol('BORROW');
const FREE = Symbol('FREE');

export class Foo extends Bar {
    [BORROW](klass, mutable) {
        // implementation detail omitted - see discussion below
    }

    [FREE]() {
        // implementation detail omitted - see discussion below
    }
    
    constructor(val) {
        const _this = () => HOST_PTR in this ? this[HOST_PTR] : this[HOST_PTR] = addHeapObject(this);
        const _super = (...args) => { super(...args); return _this(); } // derived classes only
        try {
            wasm.foo_new(val, addBorrowedObject(_super /* or `_this` in base classes */));
        } finally {
            heap[stack_pointer++] = undefined;
        }
    }

    static new_zero() {
        const ret = wasm.foo_new_zero();
        return takeObject(ret);
    }

    get() {
        let me = this[BORROW](Foo, false);
        const ret = wasm.foo_get(me);
        return ret;
    }

    set(val) {
        let me = this[BORROW](Foo, true);
        wasm.foo_set(me, val);
    }
}
```

That's actually not much! We can see here though how we've translated from Rust
to JS:

* Associated functions in Rust (those without `self`) turn into `static`
  functions in JS.

* Methods in Rust turn into methods in JS.

* The method annotated as `#[wasm_bindgen(constructor)]` becomes the
  JavaScript class's `constructor()`, enabling you to instantiate objects
  from JavaScript using `new Foo()`.  In addition to the parameters
  declared on the Rust method, this constructor adds to the arguments that
  are sent to Rust either (a) `_this`, a callback that returns the JavaScript
  heap index of the canonical reference to the `this` object (first creating
  it if necessary); or (b) `_super`, a callback that forwards its arguments
  to `super()` in JS and then invokes the `_this` callback, returning the
  latter's result.

* Runtime borrows of the Rust object can be obtained via the `[BORROW]`
  method; this takes a `klass` parameter in order to support inheritance,
  where the desired Rust object might be some other type from the object's
  prototype chain; and also a `mutable` parameter to support both mutable
  (true) and immutable (false) borrows.

* Manual memory management is exposed in JS as well. The `[FREE]` method is
  required to be invoked to deallocate resources on the Rust side of things.

During object construction, the `instantiate!` macro invokes the provided
callback (in the case of `super()`, with any provided arguments), and then
injects into the provided struct literal an additional field (defined by
`wasm_bindgen` on the `struct` declaration) having the following value:

* for base types, the callback's return value wrapped as a `JsValue`—i.e. a
canonical `JsValue` for the JavaScript shim object;

* for derived types:

    * where the prototype type is imported, the callback's return value (which
      in this case will again be the object's canonical JS heap index) wrapped
      as the prototype type; or else

    * where the prototype type is exported, the inner value of the object at
      the address in the `[WASM_PTR]` property of the JavaScript object
      indicated by the callback's return value—this will necessarily be the
      constructed instance of the prototype type owing to the necessarily
      immediately-completed invocation of its own (super) constructor.

Finally, the `instantiate!` macro wraps the instantiated Rust object as a
heap-allocated `WasmType`, the address of which it passes to an imported
intrinsic function for assignment to the `[WASM_PTR]` property of the
JavaScript object.

The `[BORROW]` and `[FREE]` methods then use this `[WASM_PTR]` property when
calling into Rust in order for it to locate the underlying object.  If an
attempt by `[FREE]` to free the Rust memory is successful, it nulls out the
`[WASM_PTR]` property in order to "neuter" the JS object and thereby ensure
that any future use of it will trigger a panic in Rust.

(TODO: Use revocable proxies instead so that freed objects can be fully
disabled through revocation; currently types that derive from imported/JavaScript
types may remain partially functional and very broken.  That said: the WeakMaps
TC39 proposal would negate any need to manually free these resources, instead
enabling them to be automatically freed when the JavaScript object is garbage
collected–and therefore no risk that objects could ever enter such a partially
functional very broken state).

The real trickery with these bindings ends up happening in Rust, however, so
let's take a look at that.

```rust
// original input to `#[wasm_bindgen]` omitted ...
impl Foo {
    // call an imported intrinsic function to initiate object
    // construction via `new Foo()` in JavaScript, passing arguments
    // as a heap-allocated slice of JsValues; intrinsic returns the
    // value of the constructed object's `[WASM_PTR]` property, from
    // which the WasmType is cloned and returned
    pub fn new(val: i32) -> WasmType<Foo> {
        instantiate_via_js( Box::new([val.into()]) )
    }
}

#[export_name = "foo_new"]
pub extern "C" fn __wasm_bindgen_generated_Foo_new(val: i32, __wbg_callback: u32) {
    let __wbg_callback = unsafe { SuperCallback::from_abi(__wbg_callback) };
    
    // expansion of instantiate! macro
    {
        // wrap the instantiated object in a WasmType
        let wasm = WasmType::new(WasmRefCell::new(Foo {
            internal : val,
        
            // invoke the callback and inject the result into `__proto__` field
            __proto__: __wbg_callback.invoke( Box::new(["abc".into(), 123.into()]) ),
        }));
        
        // update the JS object's `[WASM_PTR]` property
        JsValue::set_wasm_pointer(
            &*wasm.borrow(),
            Box::into_raw(Box::new(WasmType::clone(&wasm))),
        );

        wasm
    }
}

#[no_mangle]
pub unsafe extern "C" fn __wbg_foo_borrow(ptr: u32, id: u64, mutable: bool) -> u32 {
    let ptr = ptr as *mut WasmType<Foo>;
    assert_not_null(ptr);

    // borrow a RefMut (or Ref, per `mutable` argument) and map to
    // object in prototype chain of type with matching `id`, upcast
    // to Any (`get_underlying_*` functions are provided by blanket
    // implementation)
    if mutable {
        Box::into_raw(Box::new(RefMut::map(
            (**ptr).borrow_mut(),
            |me| me.get_underlying_mut(id)
        ))).into_abi()
    } else {
        Box::into_raw(Box::new(Ref::map(
            (**ptr).borrow(),
            |me| me.get_underlying_ref(id)
        ))).into_abi()
    }
}

#[no_mangle]
pub unsafe extern "C" fn __wbindgen_foo_free(me: u32) {
    let ptr = me as *mut WasmType<Foo>;
    assert_not_null(ptr);

    // consume the Rc and, if there are no others, ensure there are no active borrows
    if let Ok(me) = WasmType::try_unwrap(*Box::from_raw(ptr)) {
        me.borrow_mut();
    }
}

impl RefFromWasmAbi for Foo {
    type Abi = u32;
    type Anchor = Ref<'static, Foo>;

    // downcast the received Ref<Any> to Ref<Self>
    unsafe fn ref_from_abi(js: u32) -> Self::Anchor {
        let ptr = js as *mut Ref<'static, dyn Any>;
        assert_not_null(ptr);
        Ref::map(*Box::from_raw(ptr), |val| val.downcast_ref().unwrap())
    }
}

impl RefMutFromWasmAbi for Foo {
    type Abi = u32;
    type Anchor = RefMut<'static, Foo>;

    // downcast the received RefMut<Any> to RefMut<Self>
    unsafe fn ref_from_abi(js: u32) -> Self::Anchor {
        let ptr = js as *mut RefMut<'static, dyn Any>;
        assert_not_null(ptr);
        RefMut::map(*Box::from_raw(ptr), |val| val.downcast_mut().unwrap())
    }
}

#[export_name = "foo_new_zero"]
pub extern "C" fn __wasm_bindgen_generated_Foo_new_zero() -> u32 {
    let _ret = { Foo::new_zero() };
    _ret.return_abi()
}

#[export_name = "foo_get"]
pub extern "C" fn __wasm_bindgen_generated_Foo_get(me: u32) -> i32 {
    let me = unsafe { Foo::ref_from_abi(me) };

    // invoke underlying function
    me.get()
}

#[export_name = "foo_set"]
pub extern "C" fn __wasm_bindgen_generated_Foo_set(me: u32, arg1: i32) {
    let mut me = unsafe { Foo::ref_mut_from_abi(me) };

    // invoke underlying function
    me.set(arg1)
}
```

As with before this is cleaned up from the actual output but it's the same idea
as to what's going on! Here we can see a shim for each function as well as
shims for borrowing and deallocating an instance of `Foo`. Recall that the only
valid wasm types today are numbers, so we're required to shoehorn all of `Foo`
into a `u32`, which is currently done via `Box` (like `std::unique_ptr` in C++).
Note, though, that there's an extra layer here, `WasmType`. This type is the
same as [`Rc<RefCell>`] and can be mostly glossed over.

The purpose for this type, if you're interested though, is to uphold Rust's
guarantees about aliasing in a world where aliasing is rampant (JS).
Specifically the `&Foo` type means that there can be as much aliasing as you'd
like, but crucially `&mut Foo` means that it is the sole pointer to the data
(no other `&Foo` to the same instance exists). The [`RefCell`] type in libstd
is a way of dynamically enforcing this at runtime (as opposed to compile time
where it usually happens). Baking in `WasmType` is the same idea here,
adding runtime checks for aliasing which are typically happening at compile
time. This is currently a Rust-specific feature which isn't actually in the
`wasm-bindgen` tool itself, it's just in the Rust-generated code (aka the
`#[wasm_bindgen]` attribute).

[`RefCell`]: https://doc.rust-lang.org/std/cell/struct.RefCell.html
