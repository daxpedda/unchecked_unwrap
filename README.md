# unchecked_unwrap

[![Crates.io](https://img.shields.io/crates/v/unchecked_unwrap.svg)](https://crates.io/crates/unchecked_unwrap/)
[![Libraries.io](https://img.shields.io/librariesio/release/cargo/unchecked_unwrap)](https://libraries.io/cargo/unchecked_unwrap)
[![Resolution](https://isitmaintained.com/badge/resolution/daxpedda/unchecked_unwrap.svg)](http://isitmaintained.com/project/daxpedda/unchecked_unwrap)
[![Issues](https://isitmaintained.com/badge/open/daxpedda/unchecked_unwrap.svg)](http://isitmaintained.com/project/daxpedda/unchecked_unwrap)
[![License](https://img.shields.io/crates/l/unchecked_unwrap)](https://github.com/daxpedda/unchecked_unwrap/blob/master/LICENSE)
[![LoC](https://tokei.rs/b1/github/daxpedda/unchecked_unwrap)](https://github.com/daxpedda/unchecked_unwrap/)

**[Release](https://github.com/daxpedda/unchecked_unwrap/tree/release/):**
[![Build](https://github.com/daxpedda/unchecked_unwrap/workflows/CI/badge.svg?branch=release)](https://github.com/daxpedda/unchecked_unwrap/actions?query=workflow%3ACI+branch%3Arelease)
[![Docs](https://docs.rs/unchecked_unwrap/badge.svg)](https://docs.rs/unchecked_unwrap/)

**[Master](https://github.com/daxpedda/unchecked_unwrap/):**
[![Build](https://github.com/daxpedda/unchecked_unwrap/workflows/CI/badge.svg?branch=master)](https://github.com/daxpedda/unchecked_unwrap/actions?query=workflow%3ACI+branch%3Amaster)
[![Docs](https://github.com/daxpedda/unchecked_unwrap/workflows/docs/badge.svg)](https://daxpedda.github.io/unchecked_unwrap/master/doc/index.html)

Adds an unchecked version of `unwrap()` and `expect()` to `Option` and `Result` for the rust programming language.
Supports `no_std`.

## Branches

- **[release](https://github.com/daxpedda/unchecked_unwrap/tree/release/)** - For releases only.
- **[master](https://github.com/daxpedda/unchecked_unwrap/)** - For active development, PR's and testing.

## Usage

```rust
use unchecked_unwrap::*;

let x = Some("air");
assert_eq!(unsafe { x.unchecked_unwrap() }, "air");

let x: Result<u32, &str> = Ok(2);
assert_eq!(unsafe { x.unchecked_unwrap() }, 2);

let x = Some("value");
assert_eq!(unsafe { x.unchecked_expect("the world is ending") }, "value");

let x: Result<u32, &str> = Ok(2);
assert_eq!(unsafe { x.unchecked_expect("the sky is falling down") }, 2);
```

## Crate features

- **debug_checks** - On by default. Enables the normal checking behavior with panics when `debug-assertions` is enabled.
- **nightly**
  - Enables full documentation through [`feature(external_doc)`](https://doc.rust-lang.org/unstable-book/language-features/external-doc.html).
  - Enables benchmarking through [`feature(test)`](https://doc.rust-lang.org/unstable-book/library-features/test.html).
  - Enables implicit caller location through [`feature(track_caller)`](https://doc.rust-lang.org/unstable-book/language-features/track-caller.html).

## Documentation

Documentation is available online in the badge links above.
Currently, nightly is needed for full documentation: `cargo doc --features nightly`
If you are not using nightly, use `cargo doc` as usual.

## Tests

Is as simple as `cargo test` and `cargo test --release`.

## Benchmarking

Is as simple as `cargo bench`.
Currently the nightly version of rust and the `feature="nightly"` is needed for benchmarking.

## CI

This crate is checked daily by CI to make sure that it builds successfully with the newest versions of rust stable, beta and nightly.

## Alternatives

- **[unsafe-unwrap-rs](https://github.com/nvzqz/unsafe-unwrap-rs/)** - [![Crates.io](https://img.shields.io/crates/v/unsafe-unwrap.svg)](https://crates.io/crates/unsafe-unwrap/)
- **[unsafe-unwrap](https://github.com/Vurich/unsafe-unwrap/)**

## Changelog

See the [CHANGELOG](https://github.com/daxpedda/unchecked_unwrap/blob/master/CHANGELOG.md) file for details

## License

This work is dual-licensed under MIT and Apache 2.0.
You can choose between one of them if you use this work.

See the [LICENSE](https://github.com/daxpedda/unchecked_unwrap/blob/master/LICENSE) file for details

SPDX-License-Identifier: `MIT OR Apache-2.0`
