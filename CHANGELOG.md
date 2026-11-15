# Changelog

All notable changes to this project will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2026-11-15

[0.3.0]: https://github.com/sunsided/l3gd20-registers/releases/tag/v0.3.0

### Added

- `ReadableRegister` for readable registers, alongside `Register` and `WritableRegister`.
- SPI registers implement the `SPIRegister8` marker traits of `hardware-registers`.
- `I2C_AUTO_INCREMENT`, `SPI_READ`, `SPI_AUTO_INCREMENT` and `L3GD20_WHO_AM_I` constants.
- `FifoMode::try_from_bits` and `InvalidFifoMode`, which implements `Display` and
  `core::error::Error`.

### Changed

- [**breaking**] Updated to `hardware-registers` 0.4.1. `Register` no longer implies
  readability; use `ReadableRegister` for reads. `ReadableRegister` and `WritableRegister`
  cover both I²C and SPI.
- [**breaking**] Read-only registers no longer implement `ToBits`; use `Into<u8>`.
- [**breaking**] `FifoControlRegister` decodes through `TryFromBits` and rejects the
  reserved FIFO modes `0b101` to `0b111` with `InvalidFifoMode`.
  `FifoControlRegister::fifo_mode` returns `Result<FifoMode, InvalidFifoMode>` instead of
  panicking on reserved modes.
- [**breaking**] SPI register addresses are `RegisterAddress8` instead of `RegisterAddress6`.
- [**breaking**] Edition 2024; the MSRV is now Rust 1.85.
- Updated `bitfield-struct` to 0.13 and `defmt` to 1.0.

### Fixed

- `WhoAmI` is read-only and no longer implements `WritableRegister`. `WhoAmI::new()`
  holds the expected identification value.
- `Bandwidth::Narrow.sqrt_hz_at(OutputDataRate::Hz380)` returned `25.0` instead of `5.0`.
- [**breaking**] `TemperatureRegister::temp` is `i8`. The register holds a two's complement
  value, so negative readings appeared as values above 127.
- The output register docs no longer claim a fixed little-endian order; the order depends on
  `ControlRegister4::big_endian`.

## [0.2.0] - 2024-07-06

[0.2.0]: https://github.com/sunsided/l3gd20-registers/releases/tag/v0.2.0

### Added

- Added `hz_at` and `sqrt_hz_at` for `Bandwidth`.
- Added `Add` for output data registers to obtain full values.

## [0.1.0] - 2024-07-05

[0.1.0]: https://github.com/sunsided/l3gd20-registers/releases/tag/v0.1.0

### Added

- Initial release.
