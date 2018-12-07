#![feature(test)]

mod checked {
    extern crate test;
    use self::test::{black_box, Bencher};

    #[bench]
    fn option_expect(b: &mut Bencher) {
        let r = Some(0).as_ref();

        b.iter(|| {
            for _ in 0..1000 {
                black_box(black_box(r).expect(""));
            }
        });
    }

    #[bench]
    fn result_expect(b: &mut test::Bencher) {
        let r: Result<_, &()> = Ok(0).as_ref();

        b.iter(|| {
            for _ in 0..1000 {
                black_box(black_box(r).expect(""));
            }
        });
    }

    #[bench]
    fn option_unwrap(b: &mut test::Bencher) {
        let r = Some(0).as_ref();

        b.iter(|| {
            for _ in 0..1000 {
                black_box(black_box(r).unwrap());
            }
        });
    }

    #[bench]
    fn result_unwrap(b: &mut test::Bencher) {
        let r: Result<_, &()> = Ok(0).as_ref();

        b.iter(|| {
            for _ in 0..1000 {
                #[allow(clippy::option_unwrap_used)]
                black_box(black_box(r).unwrap());
            }
        });
    }
}

mod unchecked {
    extern crate test;
    use self::test::{black_box, Bencher};
    extern crate unchecked_unwrap;
    use unchecked_unwrap::*;

    #[bench]
    fn option_unchecked_expect(b: &mut Bencher) {
        let r = Some(0).as_ref();

        b.iter(|| {
            for _ in 0..1000 {
                unsafe {
                    black_box(black_box(r).unchecked_expect(""));
                }
            }
        });
    }

    #[bench]
    fn result_unchecked_expect(b: &mut Bencher) {
        let r: Result<_, &()> = Ok(0).as_ref();

        b.iter(|| {
            for _ in 0..1000 {
                unsafe {
                    black_box(black_box(r).unchecked_expect(""));
                }
            }
        });
    }

    #[bench]
    fn option_unchecked_unwrap(b: &mut Bencher) {
        let r = Some(0).as_ref();

        b.iter(|| {
            for _ in 0..1000 {
                unsafe {
                    black_box(black_box(r).unchecked_unwrap());
                }
            }
        });
    }

    #[bench]
    fn result_unchecked_unwrap(b: &mut Bencher) {
        let r: Result<_, &()> = Ok(0).as_ref();

        b.iter(|| {
            for _ in 0..1000 {
                unsafe {
                    black_box(black_box(r).unchecked_unwrap());
                }
            }
        });
    }
}
