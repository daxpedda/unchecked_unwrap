#![no_std]
#![warn(
    clippy::cargo,
    clippy::missing_docs_in_private_items,
    clippy::nursery,
    clippy::pedantic,
    missing_docs
)]
#![cfg_attr(
    feature = "nightly",
    feature(external_doc),
    doc(include = "../README.md")
)]
#![cfg_attr(not(feature = "nightly"), doc = "")]

use core::fmt::Debug;

/// Trait for [`unchecked_expect`](UncheckedUnwrap::unchecked_expect) and
/// [`unchecked_unwrap`](UncheckedUnwrap::unchecked_unwrap).
pub trait UncheckedUnwrap<T> {
    /// Unwraps an [`Option`] or [`Result`], yielding the content of a [`Some`]
    /// or [`Ok`].
    ///
    /// This is the unchecked alternative to [`Option::expect`] and
    /// [`Result::expect`].
    ///
    /// # Panics
    ///
    /// Only panics if <span class="module-item"><span class="stab portability"
    /// style="margin-right: 0">`cfg(debug_assertions)`</span></span> and <span
    /// class="module-item"><span class="stab portability" style="margin-right:
    /// 0">`feature="debug_checks"`</span></span> is enabled and the value is a
    /// [`None`] or [`Err`].
    ///
    /// # Safety
    ///
    /// Callers of this function are responsible that [`Option`] or [`Result`]
    /// carries a [`Some`] or [`Ok`].
    ///
    /// Failing that, the returned value may reference invalid memory or cause
    /// undefined behaviour.
    ///
    /// # Examples
    ///
    /// ```
    /// use unchecked_unwrap::UncheckedUnwrap;
    ///
    /// let x = Some("value");
    /// assert_eq!(
    ///     unsafe { x.unchecked_expect("the world is ending") },
    ///     "value"
    /// );
    ///
    /// let x: Result<u32, &str> = Ok(2);
    /// assert_eq!(unsafe { x.unchecked_expect("the sky is falling down") }, 2);
    /// ```
    #[track_caller]
    unsafe fn unchecked_expect(self, msg: &str) -> T;

    /// Unwraps an [`Option`] or [`Result`], yielding the content of a [`Some`]
    /// or [`Ok`].
    ///
    /// This is the unchecked alternative to [`Option::unwrap`] and
    /// [`Result::unwrap`].
    ///
    /// # Panics
    ///
    /// Only panics if <span class="module-item"><span class="stab portability"
    /// style="margin-right: 0">`cfg(debug_assertions)`</span></span> and <span
    /// class="module-item"><span class="stab portability" style="margin-right:
    /// 0">`feature="debug_checks"`</span></span> is enabled and the value is a
    /// [`None`] or [`Err`].
    ///
    /// # Safety
    ///
    /// Callers of this function are responsible that [`Option`] or [`Result`]
    /// carries a [`Some`] or [`Ok`].
    ///
    /// Failing that, the returned value may reference invalid memory or cause
    /// undefined behaviour.
    ///
    /// # Examples
    ///
    /// ```
    /// use unchecked_unwrap::UncheckedUnwrap;
    ///
    /// let x = Some("air");
    /// assert_eq!(unsafe { x.unchecked_unwrap() }, "air");
    ///
    /// let x: Result<u32, &str> = Ok(2);
    /// assert_eq!(unsafe { x.unchecked_unwrap() }, 2);
    /// ```
    #[track_caller]
    unsafe fn unchecked_unwrap(self) -> T;
}

impl<T> UncheckedUnwrap<T> for Option<T> {
    unsafe fn unchecked_unwrap(self) -> T {
        if cfg!(debug_assertions) && cfg!(feature = "debug_checks") {
            self.unwrap()
        } else if let Some(value) = self {
            value
        } else {
            core::hint::unreachable_unchecked()
        }
    }

    unsafe fn unchecked_expect(self, msg: &str) -> T {
        if cfg!(debug_assertions) && cfg!(feature = "debug_checks") {
            self.expect(msg)
        } else if let Some(value) = self {
            value
        } else {
            core::hint::unreachable_unchecked()
        }
    }
}

impl<T, E: Debug> UncheckedUnwrap<T> for Result<T, E> {
    unsafe fn unchecked_unwrap(self) -> T {
        if cfg!(debug_assertions) && cfg!(feature = "debug_checks") {
            self.unwrap()
        } else if let Ok(value) = self {
            value
        } else {
            core::hint::unreachable_unchecked()
        }
    }

    unsafe fn unchecked_expect(self, msg: &str) -> T {
        if cfg!(debug_assertions) && cfg!(feature = "debug_checks") {
            self.expect(msg)
        } else if let Ok(value) = self {
            value
        } else {
            core::hint::unreachable_unchecked()
        }
    }
}
