# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Breaking changes
### Added
### Fixed
### Changed
### Removed

## [v9.4.0] - 2023-09-19

Minor version bump due to re-export of tock-registers dependency (#20)

### Added

- Add register `FAR_EL3` (#11)
- Add register `ESR_EL3` (#10)
- Add register `SCTLR_EL3` (#9)
- Add field `TSC` to register `HCR_EL2` (#17)
- Add register `CNTKCTL_EL1` (#12)
- Add registers `APDAKEYHI_EL1`, `APDAKEYLO_EL1`, `APDBKEYHI_EL1`, `APDBKEYLO_EL1`, `APGAKEYHI_EL1`, `APGAKEYLO_EL1`, `APIAKEYHI_EL1`, `APIAKEYLO_EL1`, `APIBKEYHI_EL1`, `APIBKEYLO_EL1` (#14)
- Add registers `RVBAR_EL1`, `RVBAR_EL2`, `RVBAR_EL3` (#8)
- Add registers `MPIDR_EL1` (#6)
- Add registers `TPIDR_EL2` (#7)

### Fixed

- Fix bitwidth of field `BADDR` of register `VTTBR_EL2` (#18)

### Changed

- Re-export of tock-registers dependency (#20)
- Enable write to registers `ESR_EL1` and `ESR_EL2` (#15)

[Unreleased]: https://github.com/rust-embedded/aarch64-cpu/compare/v9.4.0...HEAD
[v9.4.0]: https://github.com/rust-embedded/aarch64-cpu/compare/v9.3.1...v9.4.0
