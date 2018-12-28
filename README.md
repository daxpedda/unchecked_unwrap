# unchecked_unwrap

[![Crates.io](https://img.shields.io/crates/v/unchecked_unwrap.svg)](https://crates.io/crates/unchecked_unwrap/)
[![Deps.rs](https://deps.rs/repo/github/daxpedda/unchecked_unwrap/status.svg)](https://deps.rs/repo/github/daxpedda/unchecked_unwrap)
[![Commits since](https://img.shields.io/github/commits-since/daxpedda/unchecked_unwrap/latest.svg)](https://github.com/daxpedda/unchecked_unwrap/releases/latest/)
[![Resolution](http://isitmaintained.com/badge/resolution/daxpedda/unchecked_unwrap.svg)](http://isitmaintained.com/project/daxpedda/unchecked_unwrap)
[![Issues](http://isitmaintained.com/badge/open/daxpedda/unchecked_unwrap.svg)](http://isitmaintained.com/project/daxpedda/unchecked_unwrap)
[![License](https://img.shields.io/crates/l/unchecked_unwrap.svg)](https://github.com/daxpedda/unchecked_unwrap/blob/master/LICENSE)
[![LoC](https://tokei.rs/b1/github/daxpedda/unchecked_unwrap/)](https://github.com/daxpedda/unchecked_unwrap/)

**[Release](https://github.com/daxpedda/unchecked_unwrap/tree/release/):**
[![Build](https://img.shields.io/travis/daxpedda/unchecked_unwrap/release.svg?label=build:%20release)](https://travis-ci.org/daxpedda/unchecked_unwrap/branches/)
[![Docs](https://docs.rs/unchecked_unwrap/badge.svg)](https://docs.rs/unchecked_unwrap/)
[![Coverage](https://img.shields.io/codecov/c/github/daxpedda/unchecked_unwrap/release.svg?label=coverage:%20release)](https://codecov.io/github/daxpedda/unchecked_unwrap/branch/release/)

**[Master](https://github.com/daxpedda/unchecked_unwrap/):**
[![Build](https://img.shields.io/travis/daxpedda/unchecked_unwrap/master.svg?label=build:%20master)](https://travis-ci.org/daxpedda/unchecked_unwrap/branches/)
[![Docs](https://raw.githack.com/daxpedda/unchecked_unwrap/gh-pages/master/doc/badge.svg)](https://daxpedda.github.io/unchecked_unwrap/master/doc/index.html)
[![Coverage](https://img.shields.io/codecov/c/github/daxpedda/unchecked_unwrap/master.svg?label=coverage:%20master)](https://codecov.io/github/daxpedda/unchecked_unwrap/branch/master/)

Adds an unchecked version of `unwrap()` and `expect()` to `Option` and `Result` for the rust programmming language.
Supports `no_std`.

## Branches

* **[release](https://github.com/daxpedda/unchecked_unwrap/tree/release/)** - For releases only.
* **[master](https://github.com/daxpedda/unchecked_unwrap/)** - For active development, PR's and testing.

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

**debug_checks**: On by default. Enables the normal checking behavior with panics when `debug-assertions` is enabled.

## Documentation

Documentation is available online in the badge links above.
Currently nightly is needed for full documentation: `cargo doc --features doc_include`
If you are not on nightly only use `cargo doc`.

## Tests

Is as simple as `cargo test` and `cargo test --release`.

## Test coverage

Test coverage is currently done by [Tarpaulin](https://github.com/xd009642/tarpaulin), which currently is missing some lines.
I removed the badge from [Crates.io](https://crates.io/crates/unchecked_unwrap/) because it is misleading.

## Benchmarking

Is as simple as `cargo bench`.
Currently the nightly version of rust is needed for benchmarking.

## CI

This crate is checked daily by CI to make sure that it builds successfully with the newest versions of rust stable, beta and nightly.

## Alternatives

* **[unsafe-unwrap-rs](https://github.com/nvzqz/unsafe-unwrap-rs/)** - [![Crates.io](https://img.shields.io/crates/v/unsafe-unwrap.svg)](https://crates.io/crates/unsafe-unwrap/)
* **[unsafe-unwrap](https://github.com/Vurich/unsafe-unwrap/)**

## License

This work is dual-licensed under MIT and Apache 2.0.
You can choose between one of them if you use this work.

See the [LICENSE](https://github.com/daxpedda/unchecked_unwrap/blob/master/LICENSE) file for details

SPDX-License-Identifier: `MIT OR Apache-2.0`
