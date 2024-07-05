# Changelog

All notable changes to this project will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.3] - 2024-07-02

[0.1.3]: https://github.com/sunsided/lsm303dlhc-registers/releases/tag/v0.1.3

### Internal

- Improved documentation on the `ControlRegister3A::i1drdy1` field (accelerometer data ready interrupt).

## [0.1.2] - 2024-07-02

[0.1.2]: https://github.com/sunsided/lsm303dlhc-registers/releases/tag/v0.1.2

### Fixed

- Applied some field defaults for `MR_REG_M`, `CRA_REG_M` and `CRB_REG_M`.

## [0.1.1] - 2024-07-01

[0.1.1]: https://github.com/sunsided/lsm303dlhc-registers/releases/tag/v0.1.1

### Changed

- Renames some fields and types to more accurately describe functionality.

## [0.1.0] - 2024-07-01

[0.1.0]: https://github.com/sunsided/lsm303dlhc-registers/releases/tag/v0.1.0

### Added

- Initial release providing the register types based on `bitfields` and `hardware-registers`.
