use JsValue;

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
    fn is_instance_of<T>(&self) -> bool
    where
        T: JsCast,
    {
        T::instanceof(self.as_ref())
    }

    /// Performs a dynamic cast (checked at runtime) of this value into the
    /// target type `T`.
    ///
    /// This method will return `Err(self)` is `self.is_instance_of::<T>()`
    /// returns `false`, and otherwise it will return `Ok(T)` manufactured with
    /// an unchecked cast (verified correct via the `instanceof` operation).
    fn dyn_into<T>(self) -> Result<T, Self>
    where
        T: JsCast,
    {
        if self.is_instance_of::<T>() {
            Ok(self.unchecked_into())
        } else {
            Err(self)
        }
    }

    /// Performs a dynamic cast (checked at runtime) of this value into the
    /// target type `T`.
    ///
    /// This method will return `None` is `self.is_instance_of::<T>()`
    /// returns `false`, and otherwise it will return `Some(&T)` manufactured
    /// with an unchecked cast (verified correct via the `instanceof` operation).
    fn dyn_ref<T>(&self) -> Option<&T>
    where
        T: JsCast,
    {
        if self.is_instance_of::<T>() {
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
