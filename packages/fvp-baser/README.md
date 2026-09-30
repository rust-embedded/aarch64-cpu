# Tests for the Arm BaseR FVP

This folder contains test programs for the Armv8-R AArch64 architecture that run
on the Arm BaseR Fixed Virtual Platform (FVP).

These tests are executed by running `just test` at the root of this repository.
That command will find all the `packages/*/examples` folders, and execute all
the examples it finds, checking the standard output against a reference file.

There is no `.cargo/config.toml` file here. The runner is specified using comments
at the top of each example.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](../../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC], the maintainer of this crate, the [Embedded Devices Working
Group's Arm Team][team], promises to intervene to uphold that code of conduct.

[CoC]: CODE_OF_CONDUCT.md
[team]: https://github.com/rust-embedded/wg#the-arm-team
