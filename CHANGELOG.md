# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [3.0.0] - 2021-01-29

### Changed

- **BREAKING**: merged `UncheckedExpect` into `UncheckedUnwrap`
- some general improvements to the README
- some general improvements to the Documentation

## [2.0.2] - 2020-06-21

### Added

- added "Table of contents" to the README
- added ASM comparison between checked and unchecked code to the README
- added sample benchmark results to the README

### Changed

- some general improvements to the README

### Fixed

- fixed repository link in `Cargo.toml`
- changed benchmarks to use `&str` instead of `&i32`

## [2.0.1] - 2020-05-08

### Fixed

- fixed maintenance status 🤦‍♂️

## [2.0.0] - 2020-05-08

### Added

- this CHANGELOG file
- [implicit caller location](https://doc.rust-lang.org/unstable-book/language-features/track-caller.html)
  support
- improved documentation slightly

### Changed

- **BREAKING**: renamed `feature="doc_include"` to `feature="nightly"`

[3.0.0]: https://github.com/daxpedda/unchecked_unwrap/releases/tag/v3.0.0
[2.0.2]: https://github.com/daxpedda/unchecked_unwrap/releases/tag/v2.0.2
[2.0.1]: https://github.com/daxpedda/unchecked_unwrap/releases/tag/v2.0.1
[2.0.0]: https://github.com/daxpedda/unchecked_unwrap/releases/tag/v2.0.0
