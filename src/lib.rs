//! Adds an unchecked way to yield the content of an [`Ok`] in [`Result`] or [`Some`] in [`Option`]
//! equivalent to [`unwrap`](Result::unwrap) and [`expect`](Result::expect).
//!
//! Compiling with the `Debug` profile the usual checks can be enabled by the `debug_checks` crate
//! feature, which is enabled by default.

#![no_std]
#![warn(clippy::cargo, clippy::pedantic, clippy::restriction)]

/// Trait for [`unchecked_unwrap`](trait.UncheckedUnwrap.html#method.unchecked_unwrap).
pub trait UncheckedUnwrap<T> {
    /// Unwraps an [`Option`] or [`Result`], yielding the content of a [`Some`] or [`Ok`].
    /// This is the unchecked alternative to [`Option::unwrap`] and [`Result::unwrap`].
    ///
    /// # Panics
    ///
    /// Only panics if the crate features `debug_assertions` and `debug_checks` are
    /// on.
    ///
    /// Panics if the value is a [`None`] or [`Err`], if [`Result`] with a panic massage provided by
    /// the [`Err`]'s value.
    ///
    /// # Safety
    ///
    /// Callers of this function are responsible that [`Option`] or [`Result`] carries a [`Some`] or
    /// [`Ok`].
    ///
    /// Failing that, the returned value may reference invalid memory or cause undefined behaviour.
    ///
    /// # Examples
    ///
    /// ```
    /// use unchecked_unwrap::*;
    ///
    /// let x = Some("air");
    /// assert_eq!(unsafe { x.unchecked_unwrap() }, "air");
    ///
    /// let x: Result<u32, &str> = Ok(2);
    /// assert_eq!(unsafe { x.unchecked_unwrap() }, 2);
    /// ```
    unsafe fn unchecked_unwrap(self) -> T;
}

impl<T> UncheckedUnwrap<T> for Option<T> {
    /// Unwraps a [`Option`], yielding the content of an [`Some`].
    /// This is the unchecked alternative to [`unwrap`](Option::unwrap).
    #[inline]
    unsafe fn unchecked_unwrap(self) -> T {
        if cfg!(debug_assertions) {
            #[cfg(feature = "debug_checks")]
            #[allow(clippy::option_unwrap_used)]
            return self.unwrap();
        }

        if let Some(x) = self {
            x
        } else {
            core::hint::unreachable_unchecked()
        }
    }
}

impl<T, E: core::fmt::Debug> UncheckedUnwrap<T> for Result<T, E> {
    /// Unwraps a [`Result`], yielding the content of an [`Ok`].
    /// This is the unchecked alternative to [`unwrap`](Result::unwrap).
    #[inline]
    unsafe fn unchecked_unwrap(self) -> T {
        if cfg!(debug_assertions) {
            #[cfg(feature = "debug_checks")]
            #[allow(clippy::result_unwrap_used)]
            return self.unwrap();
        }

        if let Ok(x) = self {
            x
        } else {
            core::hint::unreachable_unchecked()
        }
    }
}

/// Trait for [`unchecked_expect`](trait.UncheckedExpect.html#method.unchecked_expect).
pub trait UncheckedExpect<T> {
    /// Unwraps an [`Option`] or [`Result`], yielding the content of a [`Some`] or [`Ok`].
    /// This is the unchecked alternative to [`Option::expect`] and [`Result::expect`].
    ///
    /// # Panics
    ///
    /// Only panics if the crate features `debug_assertions` and `debug_checks` are
    /// on.
    ///
    /// Panics if the value is a [`None`] or [`Err`], with a custom panic message provided by `msg`
    /// and if [`Result`] with the content of the [`Err`].
    ///
    /// # Safety
    ///
    /// Callers of this function are responsible that [`Option`] or [`Result`] carries a [`Some`] or
    /// [`Ok`].
    ///
    /// Failing that, the returned value may reference invalid memory or cause undefined behaviour.
    ///
    /// # Examples
    ///
    /// ```
    /// # use unchecked_unwrap::*;
    ///
    /// let x = Some("value");
    /// assert_eq!(unsafe { x.unchecked_expect("the world is ending") }, "value");
    ///
    /// let x: Result<u32, &str> = Ok(2);
    /// assert_eq!(unsafe { x.unchecked_expect("the sky is falling down") }, 2);
    /// ```
    unsafe fn unchecked_expect(self, msg: &str) -> T;
}

impl<T> UncheckedExpect<T> for Option<T> {
    /// Unwraps an [`Option`], yielding the content of a [`Some`].
    /// This is the unchecked alternative to [`expect`](Option::expect).
    #[inline]
    unsafe fn unchecked_expect(self, msg: &str) -> T {
        if cfg!(debug_assertions) {
            #[cfg(feature = "debug_checks")]
            return self.expect(msg);
        }

        if let Some(x) = self {
            x
        } else {
            core::hint::unreachable_unchecked()
        }
    }
}

impl<T, E: core::fmt::Debug> UncheckedExpect<T> for Result<T, E> {
    /// Unwraps a [`Result`], yielding the content of an [`Ok`].
    /// This is the unchecked alternative to [`expect`](Result::expect).
    #[inline]
    unsafe fn unchecked_expect(self, msg: &str) -> T {
        if cfg!(debug_assertions) {
            #[cfg(feature = "debug_checks")]
            return self.expect(msg);
        }

        if let Ok(x) = self {
            x
        } else {
            core::hint::unreachable_unchecked()
        }
    }
}
