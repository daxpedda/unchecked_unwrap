#![no_std]
#![warn(
    clippy::all,
    clippy::cargo,
    clippy::missing_docs_in_private_items,
    clippy::pedantic,
    clippy::nursery,
    missing_docs
)]
use unchecked_unwrap::{UncheckedExpect, UncheckedUnwrap};

#[test]
fn option_expect_success() {
    let option = Some(0);

    assert_eq!(unsafe { option.unchecked_expect("test") }, 0);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = "test")]
fn option_expect_failure() {
    let option: Option<()> = None;

    unsafe { option.unchecked_expect("test") }
}

#[test]
fn result_expect_success() {
    let result: Result<_, ()> = Ok(0);

    assert_eq!(unsafe { result.unchecked_expect("test") }, 0);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = "test")]
fn result_expect_failure() {
    let result: Result<(), _> = Err(());

    unsafe { result.unchecked_expect("test") }
}

#[test]
fn option_unwrap_success() {
    let option = Some(0);

    assert_eq!(unsafe { option.unchecked_unwrap() }, 0);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn option_unwrap_failure() {
    let option: Option<()> = None;

    unsafe { option.unchecked_unwrap() }
}

#[test]
fn result_unwrap_success() {
    let result: Result<_, ()> = Ok(0);

    assert_eq!(unsafe { result.unchecked_unwrap() }, 0);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = "called `Result::unwrap()` on an `Err` value: ()")]
fn result_unwrap_failure() {
    let result: Result<(), _> = Err(());

    unsafe { result.unchecked_unwrap() }
}
