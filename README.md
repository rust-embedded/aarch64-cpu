# Rust on Arm AArch64

This repository provides support for:

- Armv8-A AArch64 Processors, like the Arm Cortex-A53
- Armv8-R AArch64 Processors, like the Arm Cortex-R82

It does not support any M-Profile Processors (like the Arm Cortex-M3) as they
have a fundamentally different interrupt vector table.

It also does not support processors running in AArch32 mode - A32 machine
code uses different instructions for reading/writing system registers.

There are currently two libraries here:

* [aarch64-cpu](./packages/aarch64-cpu/) - Low level access to processors using the AArch64 execution state
* [aarch64-pmsa-rt](./packages/aarch64-pmsa-rt/) - startup library for AArch64 systems without a MMU

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
