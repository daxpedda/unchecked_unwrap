extern crate unchecked_unwrap;
use unchecked_unwrap::*;

#[test]
fn option_expect_success() {
    let x = Some(0);

    assert_eq!(unsafe { x.unchecked_expect("test") }, 0);
}

#[test]
#[should_panic(expected = "test")]
fn option_expect_failure() {
    let x: Option<()> = None;

    unsafe { x.unchecked_expect("test") }
}

#[test]
fn result_expect_success() {
    let x: Result<_, ()> = Ok(0);

    assert_eq!(unsafe { x.unchecked_expect("test") }, 0);
}

#[test]
#[should_panic(expected = "test")]
fn result_expect_failure() {
    let x: Result<(), _> = Err(());

    unsafe { x.unchecked_expect("test") }
}

#[test]
fn option_unwrap_success() {
    let x = Some(0);

    assert_eq!(unsafe { x.unchecked_unwrap() }, 0);
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn option_unwrap_failure() {
    let x: Option<()> = None;

    unsafe { x.unchecked_unwrap() }
}

#[test]
fn result_unwrap_success() {
    let x: Result<_, ()> = Ok(0);

    assert_eq!(unsafe { x.unchecked_unwrap() }, 0);
}

#[test]
#[should_panic(expected = "called `Result::unwrap()` on an `Err` value: ()")]
fn result_unwrap_failure() {
    let x: Result<(), _> = Err(());

    unsafe { x.unchecked_unwrap() }
}
