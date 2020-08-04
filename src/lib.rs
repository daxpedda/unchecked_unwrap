#![no_std]
#![warn(
    clippy::all,
    clippy::cargo,
    clippy::missing_docs_in_private_items,
    clippy::pedantic,
    clippy::nursery,
    missing_docs
)]
// Until `track_caller` trickles down to stable.
#![allow(stable_features)]
#![cfg_attr(
    feature = "nightly",
    feature(external_doc, track_caller),
    doc(include = "../README.md")
)]
#![cfg_attr(not(feature = "nightly"), doc = "")]

use core::fmt::Debug;

/// Trait for [`unchecked_expect`](UncheckedExpect::unchecked_expect).
pub trait UncheckedExpect<T> {
    /// Unwraps an [`Option`] or [`Result`], yielding the content of a [`Some`]
    /// or [`Ok`]. This is the unchecked alternative to [`Option::expect`]
    /// and [`Result::expect`].
    ///
    /// # Panics
    ///
    /// Only panics if `cfg(debug_assertions)` and <span
    /// class="module-item"><span class="stab portability" style="margin-right:
    /// 0">`feature="debug_checks"`</span></span> is enabled.
    ///
    /// Panics if the value is a [`None`] or [`Err`], with a custom panic
    /// message provided by `msg` and if [`Result`] with the content of the
    /// [`Err`].
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
    /// use unchecked_unwrap::UncheckedExpect;
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
    #[cfg_attr(feature = "nightly", track_caller)]
    unsafe fn unchecked_expect(self, msg: &str) -> T;
}

/// Trait for [`unchecked_unwrap`](UncheckedUnwrap::unchecked_unwrap).
pub trait UncheckedUnwrap<T> {
    /// Unwraps an [`Option`] or [`Result`], yielding the content of a [`Some`]
    /// or [`Ok`]. This is the unchecked alternative to [`Option::unwrap`]
    /// and [`Result::unwrap`].
    ///
    /// # Panics
    ///
    /// Only panics if `cfg(debug_assertions)` and <span
    /// class="module-item"><span class="stab portability" style="margin-right:
    /// 0">`feature="debug_checks"`</span></span> is enabled.
    ///
    /// Panics if the value is a [`None`] or [`Err`], if [`Result`] with a panic
    /// massage provided by the [`Err`]'s value.
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
    #[cfg_attr(feature = "nightly", track_caller)]
    unsafe fn unchecked_unwrap(self) -> T;
}

impl<T> UncheckedExpect<T> for Option<T> {
    /// Unwraps an [`Option`], yielding the content of a [`Some`].
    /// This is the unchecked alternative to [`expect`](Option::expect).
    unsafe fn unchecked_expect(self, msg: &str) -> T {
        #[allow(clippy::unknown_clippy_lints, clippy::option_if_let_else)]
        if cfg!(debug_assertions) && cfg!(feature = "debug_checks") {
            self.expect(msg)
        } else if let Some(value) = self {
            value
        } else {
            core::hint::unreachable_unchecked()
        }
    }
}

impl<T, E: Debug> UncheckedExpect<T> for Result<T, E> {
    /// Unwraps a [`Result`], yielding the content of an [`Ok`].
    /// This is the unchecked alternative to [`expect`](Result::expect).
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

impl<T> UncheckedUnwrap<T> for Option<T> {
    /// Unwraps a [`Option`], yielding the content of an [`Some`].
    /// This is the unchecked alternative to [`unwrap`](Option::unwrap).
    unsafe fn unchecked_unwrap(self) -> T {
        #[allow(clippy::unknown_clippy_lints, clippy::option_if_let_else)]
        if cfg!(debug_assertions) && cfg!(feature = "debug_checks") {
            self.unwrap()
        } else if let Some(value) = self {
            value
        } else {
            core::hint::unreachable_unchecked()
        }
    }
}

impl<T, E: Debug> UncheckedUnwrap<T> for Result<T, E> {
    /// Unwraps a [`Result`], yielding the content of an [`Ok`].
    /// This is the unchecked alternative to [`unwrap`](Result::unwrap).
    unsafe fn unchecked_unwrap(self) -> T {
        if cfg!(debug_assertions) && cfg!(feature = "debug_checks") {
            self.unwrap()
        } else if let Ok(value) = self {
            value
        } else {
            core::hint::unreachable_unchecked()
        }
    }
}
