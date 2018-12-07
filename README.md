# unchecked_unwrap

[![Build][bi]][bl] [![Crates][ci]][cl] [![Docs][di]][dl] [![Resolution][iri]][irl] [![Issues][ori]][orl] [![License][li]][ll] ![LoC][lo]

[bi]: https://img.shields.io/travis/com/daxpedda/unchecked_unwrap.svg
[bl]: https://travis-ci.com/daxpedda/unchecked_unwrap/

[ci]: https://img.shields.io/crates/daxpedda/unchecked_unwrap.svg
[cl]: https://crates.io/crates/unchecked_unwrap/

[di]: https://docs.rs/unchecked_unwrap/badge.svg
[dl]: https://docs.rs/unchecked_unwrap/

[iri]: http://isitmaintained.com/badge/resolution/daxpedda/uywi.svg
[irl]: http://isitmaintained.com/project/daxpedda/uywi

[ori]: http://isitmaintained.com/badge/open/daxpedda/uywi.svg
[orl]: http://isitmaintained.com/project/daxpedda/uywi

[li]: https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg
[li]: LICENSE

[lo]: https://tokei.rs/b1/github/daxpedda/unchecked_unwrap

Adds an unchecked version of `unwrap()` and `expect()` to `Option` and `Result` for the rust programmming language.

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

**debug_checks**: Enables `panic`s in debug profile. On by default.

## Testing

Is as simple as `cargo test`.

## Benchmarking

Is as simple as `cargo bench`.
Currently the nightly version of rust is needed for benchmarking.

## Alternatives

* **unsafe-unwrap-rs** - [GitHub](https://github.com/nvzqz/unsafe-unwrap-rs) - [![Crates.io](https://img.shields.io/crates/v/unsafe-unwrap.svg)](https://crates.io/crates/unsafe-unwrap)
* **unsafe-unwrap** - [GitHub](https://github.com/Vurich/unsafe-unwrap)

## License

This work is dual-licensed under MIT and Apache 2.0.
You can choose between one of them if you use this work.

See the [LICENSE](LICENSE) file for details

`SPDX-License-Identifier: NIT OR Apache-2.0`
