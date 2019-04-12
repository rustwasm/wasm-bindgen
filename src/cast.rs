use crate::JsValue;

/// A trait for checked and unchecked casting between JS types.
///
/// Specified [in an RFC][rfc] this trait is intended to provide support for
/// casting JS values between differnet types of one another. In JS there aren't
/// many static types but we've ascribed JS values with static types in Rust,
/// yet they often need to be switched to other types temporarily! This trait
/// provides both checked and unchecked casting into various kinds of values.
///
/// This trait is automatically implemented for any type imported in a
/// `#[wasm_bindgen]` `extern` block.
///
/// [rfc]: https://github.com/rustwasm/rfcs/pull/2
pub trait JsCast
where
    Self: AsRef<JsValue> + Into<JsValue>,
{
    /// Test whether this JS value is an instance of the type `T`.
    ///
    /// This method performs a dynamic check (at runtime) using the JS
    /// `instanceof` operator. This method returns `self instanceof T`.
    ///
    /// Note that `instanceof` does not work with primitive values or across
    /// different realms (e.g. iframes). Prefer using `has_type` instead.
    fn is_instance_of<T>(&self) -> bool
    where
        T: JsCast,
    {
        T::instanceof(self.as_ref())
    }

    /// Test whether this JS value has a type `T`.
    ///
    /// Unlike `is_instance_of`, the type can override this to a specialised
    /// check that works reliably with primitives and across realms.
    fn has_type<T>(&self) -> bool
    where
        T: JsCast,
    {
        T::is_type_of(self.as_ref())
    }

    /// Performs a dynamic cast (checked at runtime) of this value into the
    /// target type `T`.
    ///
    /// This method will return `Err(self)` if `self.has_type::<T>()`
    /// returns `false`, and otherwise it will return `Ok(T)` manufactured with
    /// an unchecked cast (verified correct via the `instanceof` operation).
    fn dyn_into<T>(self) -> Result<T, Self>
    where
        T: JsCast,
    {
        if self.has_type::<T>() {
            Ok(self.unchecked_into())
        } else {
            Err(self)
        }
    }

    /// Performs a dynamic cast (checked at runtime) of this value into the
    /// target type `T`.
    ///
    /// This method will return `None` if `self.has_type::<T>()`
    /// returns `false`, and otherwise it will return `Some(&T)` manufactured
    /// with an unchecked cast (verified correct via the `instanceof` operation).
    fn dyn_ref<T>(&self) -> Option<&T>
    where
        T: JsCast,
    {
        if self.has_type::<T>() {
            Some(self.unchecked_ref())
        } else {
            None
        }
    }

    /// Performs a zero-cost unchecked cast into the specified type.
    ///
    /// This method will convert the `self` value to the type `T`, where both
    /// `self` and `T` are simple wrappers around `JsValue`. This method **does
    /// not check whether `self` is an instance of `T`**. If used incorrectly
    /// then this method may cause runtime exceptions in both Rust and JS, this
    /// should be used with caution.
    fn unchecked_into<T>(self) -> T
    where
        T: JsCast,
    {
        T::unchecked_from_js(self.into())
    }

    /// Performs a zero-cost unchecked cast into a reference to the specified
    /// type.
    ///
    /// This method will convert the `self` value to the type `T`, where both
    /// `self` and `T` are simple wrappers around `JsValue`. This method **does
    /// not check whether `self` is an instance of `T`**. If used incorrectly
    /// then this method may cause runtime exceptions in both Rust and JS, this
    /// should be used with caution.
    ///
    /// This method, unlike `unchecked_into`, does not consume ownership of
    /// `self` and instead works over a shared reference.
    fn unchecked_ref<T>(&self) -> &T
    where
        T: JsCast,
    {
        T::unchecked_from_js_ref(self.as_ref())
    }

    /// Performs a dynamic `instanceof` check to see whether the `JsValue`
    /// provided is an instance of this type.
    ///
    /// This is intended to be an internal implementation detail, you likely
    /// won't need to call this.
    fn instanceof(val: &JsValue) -> bool;

    /// Performs a dynamic check to see whether the `JsValue` provided
    /// is a value of this type.
    ///
    /// Unlike `instanceof`, this can be specialised to use a custom check by
    /// adding a `#[wasm_bindgen(is_type_of = callback)]` attribute to the
    /// type import declaration.
    ///
    /// Other than that, this is intended to be an internal implementation
    /// detail of `has_type` and you likely won't need to call this.
    fn is_type_of(val: &JsValue) -> bool {
        Self::instanceof(val)
    }

    /// Performs a zero-cost unchecked conversion from a `JsValue` into an
    /// instance of `Self`
    ///
    /// This is intended to be an internal implementation detail, you likely
    /// won't need to call this.
    fn unchecked_from_js(val: JsValue) -> Self;

    /// Performs a zero-cost unchecked conversion from a `&JsValue` into an
    /// instance of `&Self`.
    ///
    /// Note the safety of this method, which basically means that `Self` must
    /// be a newtype wrapper around `JsValue`.
    ///
    /// This is intended to be an internal implementation detail, you likely
    /// won't need to call this.
    fn unchecked_from_js_ref(val: &JsValue) -> &Self;
}
