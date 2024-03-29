# unchecked_unwrap

[![Crates.io](https://img.shields.io/crates/v/unchecked_unwrap.svg)](https://crates.io/crates/unchecked_unwrap)
[![Libraries.io](https://img.shields.io/librariesio/release/cargo/unchecked_unwrap)](https://libraries.io/cargo/unchecked_unwrap)
[![Commits since](https://img.shields.io/github/commits-since/daxpedda/unchecked_unwrap/latest)](https://github.com/daxpedda/unchecked_unwrap/releases/latest)
[![License](https://img.shields.io/crates/l/unchecked_unwrap)](https://github.com/daxpedda/unchecked_unwrap/blob/master/LICENSE)
[![LoC](https://tokei.rs/b1/github/daxpedda/unchecked_unwrap)](https://github.com/daxpedda/unchecked_unwrap)

**[Release](https://github.com/daxpedda/unchecked_unwrap/tree/release):**
[![Build](https://github.com/daxpedda/unchecked_unwrap/workflows/CI/badge.svg?branch=release)](https://github.com/daxpedda/unchecked_unwrap/actions?query=workflow%3ACI+branch%3Arelease)
[![Docs](https://docs.rs/unchecked_unwrap/badge.svg)](https://docs.rs/unchecked_unwrap)

**[Master](https://github.com/daxpedda/unchecked_unwrap):**
[![Build](https://github.com/daxpedda/unchecked_unwrap/workflows/CI/badge.svg?branch=master)](https://github.com/daxpedda/unchecked_unwrap/actions?query=workflow%3ACI+branch%3Amaster)
[![Docs](https://github.com/daxpedda/unchecked_unwrap/workflows/docs/badge.svg)](https://daxpedda.github.io/unchecked_unwrap/master/doc/index.html)

## WARNING ⚠️

Since Rust v1.58 this crate is useless, see:

- [`Result::unwrap_unchecked`](https://doc.rust-lang.org/core/result/enum.Result.html#method.unwrap_unchecked)
- [`Result::unwrap_err_unchecked`](https://doc.rust-lang.org/core/result/enum.Result.html#method.unwrap_err_unchecked)
- [`Option::unwrap_unchecked`](https://doc.rust-lang.org/core/option/enum.Option.html#method.unwrap_unchecked)

## Table of contents

- [Description](#description)
- [Branches](#branches)
- [Usage](#usage)
- [Crate features](#crate-features)
- [Documentation](#documentation)
- [Tests](#tests)
- [Benchmarks](#benchmarks)
- [Alternatives](#alternatives)
- [Changelog](#changelog)
- [License](#license)
- [Contribution](#contribution)

## Description

Adds an unchecked version of `unwrap()` and `expect()` to `Option` and `Result`
for the Rust programming language. Supports `no_std`.

## Branches

- **[release](https://github.com/daxpedda/unchecked_unwrap/tree/release)** - For
  releases only.
- **[master](https://github.com/daxpedda/unchecked_unwrap)** - For active
  development, PR's and testing.

## Usage

```rust
use unchecked_unwrap::UncheckedUnwrap;

let x = Some("air");
assert_eq!(unsafe { x.unchecked_unwrap() }, "air");

let x: Result<u32, &str> = Ok(2);
assert_eq!(unsafe { x.unchecked_unwrap() }, 2);

let x = Some("value");
assert_eq!(unsafe { x.unchecked_expect("the world is ending") }, "value");

let x: Result<u32, &str> = Ok(2);
assert_eq!(unsafe { x.unchecked_expect("the sky is falling down") }, 2);
```

<table>
  <thead>
    <tr>
      <th>checked</th>
      <th>unchecked</th>
    </tr>
  </thead>
  <tbody>
    <tr valign="top">
      <td>
<pre lang="rust">fn test_checked(value: Option<&str>) -> &str {
    value.unwrap()
}</pre>
      </td>
      <td>
<pre lang="rust">fn test_unchecked(value: Option<&str>) -> &str {
    unsafe { value.unchecked_unwrap() }
}</pre>
      </td>
    </tr>
    <tr valign="top">
      <td>
<pre lang="asm">push    rax
test    rdi, rdi
je      .LBB2_1       // panic handler
mov     rdx, rsi
mov     rax, rdi
pop     rcx
ret</pre>
      </td>
      <td>
<pre lang="asm">mov     rdx, rsi
mov     rax, rdi
ret</pre>
      </td>
    </tr>
  </tbody>
</table>

## Crate features

- **debug_checks** - On by default. Enables the normal checking behavior with
  panics when `cfg(debug-assertions)` is enabled.

## Documentation

Documentation is available online in the badge links above.

## Tests

Is as simple as `cargo test` and `cargo test --release`.

## Benchmarks

Is as simple as `cargo bench`. Currently the nightly version of Rust is needed
for benchmarking.

A sample result from the CI running on Github Actions:

```ignore
test checked::expect_option   ... bench:         798 ns/iter (+/- 90)
test checked::expect_result   ... bench:         724 ns/iter (+/- 109)
test checked::unwrap_option   ... bench:         802 ns/iter (+/- 52)
test checked::unwrap_result   ... bench:         743 ns/iter (+/- 176)
test unchecked::expect_option ... bench:         407 ns/iter (+/- 93)
test unchecked::expect_result ... bench:         374 ns/iter (+/- 48)
test unchecked::unwrap_option ... bench:         345 ns/iter (+/- 53)
test unchecked::unwrap_result ... bench:         407 ns/iter (+/- 22)
```

## Alternatives

- **[unsafe-unwrap-rs](https://github.com/nvzqz/unsafe-unwrap-rs)** -
  [![Crates.io](https://img.shields.io/crates/v/unsafe-unwrap.svg)](https://crates.io/crates/unsafe-unwrap)
- **[unsafe-unwrap](https://github.com/Vurich/unsafe-unwrap)**

Both alternatives and this crate are quite the same except that this crate
provides additional features that can be toggled with cargo features. See
[crate features](#crate-features) for details.

## Changelog

See the
[CHANGELOG](https://github.com/daxpedda/unchecked_unwrap/blob/master/CHANGELOG.md)
file for details

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
