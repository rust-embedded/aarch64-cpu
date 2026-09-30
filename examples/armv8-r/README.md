# Examples for Armv8-R AArch64 systems

This folder contains example programs for the Armv8-R AArch64 architecture.

## Dependencies

* Arm's Fixed Virtual Platform (FVP) Architecture Envelope Model (AEM) Base R,
  AKA `FVP_BaseR_AEMv8R` [1]
  
  The FVP is used to execute the examples. Ensure that it's part of your
  `$PATH`. Version 'Fast Models [11.30.27 (Nov 14 2025)]' has been tested.

* A Rust toolchain that supports the `aarch64v8r-unknown-none` compilation target.

  At the time of writing that target is not yet available on stable so `nightly-2026-02-13` is
  specified in `rust-toolchain.toml`. Cargo has been configured to build `libcore` from source,
  because this is a Tier 3 target and so not available through `rustup`.

[1]: https://developer.arm.com/Tools%20and%20Software/Fixed%20Virtual%20Platforms/Arm%20Architecture%20FVPs

* (Optional) `defmt-print` to decode the defmt logs produced by some examples.
  The tool can be installed with the following command:

``` console
$ cargo install defmt-print --version 1.0.0 --locked
```

## Running the examples

All commands in this section are meant to be executed using this directory as the working directory.

### `hello`

This program uses semihosting to print to the host's console.
It also validates that static variables are initialized before the start of `main`.

``` console
$ cargo run --bin hello
Hello, world! running from EL2
static variables: X=0, Y=1
```

### `interrupts`

This program showcases nested interrupt handling.

``` console
$ cargo run --bin interrupts
start of main
  > irq_current()
  Handling SGI 1
  SGI1 handler
  before raising SGI0
    > irq_current()
    Handling SGI 0
    SGI0 handler
    Handled SGI 0
    < irq_current()
  after raising SGI0
  Handled SGI 1
  < irq_current()
end of main
```

### `defmt`

This program showcases defmt logging.
The defmt logs are output via UART0 (serial port 0) and not printed to the console.

``` console
$ cargo run --bin defmt
before defmt log
before raise SGI0
after raise SGI0
  > irq_current()
  Handling SGI 0
  Handled SGI 0
  < irq_current()
after defmt log
```

The FVP will create a file named `uart0.out` in the current directory that contains the UART0 output of the program.
This file is defmt-encoded and can be decoded with the following command:

``` console
$ cat uart0.out | defmt-print -e target/aarch64v8r-unknown-none/debug/defmt
INFO  >>> 42 <<<
INFO  SGI0 handler
```

### `timer-cval` and `timer-tval`

These two programs use Arm's Generic Timer, specifically the physical EL1 timer, to periodically fire an interrupt.
See the crate-level doc comment in each example for an explanation on how they arm the timer with different approaches.

``` console
$ cargo run --bin timer-cval
tick(0) at 60000194988
tick(1) at 120000194988
tick(2) at 180000194988
tick(3) at 240000194988
(.. continues ..)
```

``` console
$ cargo run --bin timer-tval
tick(0) at 937695424
tick(1) at 60937731528
tick(2) at 120937767608
tick(3) at 180937803240
(.. continues ..)
```

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
