#![cfg(not(debug_assertions))]
extern crate unchecked_unwrap;
use unchecked_unwrap::*;

#[test]
fn option_expect_success() {
    let x = Some(0);

    assert_eq!(unsafe { x.unchecked_expect("test") }, 0);
}

#[test]
fn result_expect_success() {
    let x: Result<_, ()> = Ok(0);

    assert_eq!(unsafe { x.unchecked_expect("test") }, 0);
}

#[test]
fn option_unwrap_success() {
    let x = Some(0);

    assert_eq!(unsafe { x.unchecked_unwrap() }, 0);
}

#[test]
fn result_unwrap_success() {
    let x: Result<_, ()> = Ok(0);

    assert_eq!(unsafe { x.unchecked_unwrap() }, 0);
}
