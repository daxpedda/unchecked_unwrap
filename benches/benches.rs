#![no_std]
#![cfg(feature = "nightly")]
#![feature(test)]
#![warn(
    clippy::all,
    clippy::cargo,
    clippy::missing_docs_in_private_items,
    clippy::pedantic,
    clippy::nursery,
    missing_docs
)]

mod checked {
    extern crate test;

    use self::test::{black_box, Bencher};

    #[bench]
    fn expect_option(bencher: &mut Bencher) {
        let option = Some("test");

        bencher.iter(|| {
            for _ in 0..1000 {
                black_box(black_box(option).expect(""));
            }
        });
    }

    #[bench]
    fn expect_result(bencher: &mut test::Bencher) {
        let result: Result<_, ()> = Ok("test");

        bencher.iter(|| {
            for _ in 0..1000 {
                black_box(black_box(result).expect(""));
            }
        });
    }

    #[bench]
    fn unwrap_option(bencher: &mut test::Bencher) {
        let option = Some("test");

        bencher.iter(|| {
            for _ in 0..1000 {
                black_box(black_box(option).unwrap());
            }
        });
    }

    #[bench]
    fn unwrap_result(bencher: &mut test::Bencher) {
        let result: Result<_, ()> = Ok("test");

        bencher.iter(|| {
            for _ in 0..1000 {
                #[allow(clippy::unwrap_used)]
                black_box(black_box(result).unwrap());
            }
        });
    }
}

mod unchecked {
    extern crate test;

    use self::test::{black_box, Bencher};
    use unchecked_unwrap::UncheckedUnwrap;

    #[bench]
    fn expect_option(bencher: &mut Bencher) {
        let option = Some("test");

        bencher.iter(|| {
            for _ in 0..1000 {
                unsafe {
                    black_box(black_box(option).unchecked_expect(""));
                }
            }
        });
    }

    #[bench]
    fn expect_result(bencher: &mut Bencher) {
        let result: Result<_, ()> = Ok("test");

        bencher.iter(|| {
            for _ in 0..1000 {
                unsafe {
                    black_box(black_box(result).unchecked_expect(""));
                }
            }
        });
    }

    #[bench]
    fn unwrap_option(bencher: &mut Bencher) {
        let option = Some("test");

        bencher.iter(|| {
            for _ in 0..1000 {
                unsafe {
                    black_box(black_box(option).unchecked_unwrap());
                }
            }
        });
    }

    #[bench]
    fn unwrap_result(bencher: &mut Bencher) {
        let result: Result<_, ()> = Ok("test");

        bencher.iter(|| {
            for _ in 0..1000 {
                unsafe {
                    black_box(black_box(result).unchecked_unwrap());
                }
            }
        });
    }
}
