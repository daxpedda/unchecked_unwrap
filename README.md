# unchecked_unwrap

[![Crates][ci]][cl] [![Resolution][iri]][irl] [![Issues][ori]][orl] [![License][li]][ll] [![LoC][mlci]][mlcl]

[ci]: https://img.shields.io/crates/v/unchecked_unwrap.svg
[cl]: https://crates.io/crates/unchecked_unwrap/

[iri]: http://isitmaintained.com/badge/resolution/daxpedda/unchecked_unwrap.svg
[irl]: http://isitmaintained.com/project/daxpedda/unchecked_unwrap

[ori]: http://isitmaintained.com/badge/open/daxpedda/unchecked_unwrap.svg
[orl]: http://isitmaintained.com/project/daxpedda/unchecked_unwrap

[li]: https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg
[ll]: LICENSE

[mlci]: https://tokei.rs/b1/github/daxpedda/unchecked_unwrap/
[mlcl]: https://github.com/daxpedda/unchecked_unwrap/

**[Stable](https://github.com/daxpedda/unchecked_unwrap/tree/stable/):** [![Build][sbi]][sbl] [![Docs][sdi]][sdl] [![Coverage][scci]][sccl]

[sbi]: https://img.shields.io/travis/daxpedda/unchecked_unwrap/stable.svg?label=build:%20stable
[sbl]: https://travis-ci.org/daxpedda/unchecked_unwrap/branches/

[sdi]: https://docs.rs/unchecked_unwrap/badge.svg
[sdl]: https://docs.rs/unchecked_unwrap/

[scci]: https://img.shields.io/codecov/c/github/daxpedda/unchecked_unwrap/stable.svg?label=coverage:%20stable
[sccl]: https://codecov.io/github/daxpedda/unchecked_unwrap/branch/stable/

**[Master](https://github.com/daxpedda/unchecked_unwrap/):** [![Build][mbi]][mbl] [![Docs][mdi]][mdl] [![Coverage][mcci]][mccl]

[mbi]: https://img.shields.io/travis/daxpedda/unchecked_unwrap/stable.svg?label=build:%20master
[mbl]: https://travis-ci.org/daxpedda/unchecked_unwrap/branches/

[mdi]: https://docs.rs/unchecked_unwrap/badge.svg
[mdl]: https://daxpedda.github.io/unchecked_unwrap/master/index.html

[mcci]: https://img.shields.io/codecov/c/github/daxpedda/unchecked_unwrap/master.svg?label=coverage:%20master
[mccl]: https://codecov.io/github/daxpedda/unchecked_unwrap/branch/master/

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

**debug_checks**: On by default. Enables the normal checking behaviour with panics when `debug-assertions` is enabled.

## Testing

Is as simple as `cargo test`.

## Benchmarking

Is as simple as `cargo bench`.
Currently the nightly version of rust is needed for benchmarking.

## Alternatives

* **[unsafe-unwrap-rs](https://github.com/nvzqz/unsafe-unwrap-rs/)** - [![Crates.io](https://img.shields.io/crates/v/unsafe-unwrap.svg)](https://crates.io/crates/unsafe-unwrap/)
* **[unsafe-unwrap](https://github.com/Vurich/unsafe-unwrap/)**

## License

This work is dual-licensed under MIT and Apache 2.0.
You can choose between one of them if you use this work.

See the [LICENSE](LICENSE) file for details

`SPDX-License-Identifier: MIT OR Apache-2.0`
