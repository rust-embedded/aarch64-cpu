# `testing`

> Test suite for `aarch64-*` crates

This needs to be a separate Cargo package or avoiding compiling target
(AArch64) code to the host and vice versa gets tricky fast.

To build all the unit test applcations, enter this directory and run:

```bash
cargo test
```

It will scan for example binaries in the `./packages/*` directories, compile them according to the
comments inside each binary's source file, and then run them each binary on an appropriate virtual
machine.
