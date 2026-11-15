//! # STMicroelectronics L3GD20 Gyroscope Registers
//!
//! This crate provides a typed map of the L3GD20's I²C/SPI registers.
//!
//! Multi-byte values (angular rate outputs, interrupt thresholds) are split into
//! one register per byte. Consecutive registers can be read or written in one transfer:
//!
//! - On I²C, the most significant bit of the register address ([`I2C_AUTO_INCREMENT`])
//!   enables address auto-increment.
//! - On SPI, the first byte holds the read bit ([`SPI_READ`]), the auto-increment bit
//!   ([`SPI_AUTO_INCREMENT`]) and the register address.

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]
#![forbid(unsafe_code)]
// bitfield-struct generates `with_*_checked` setters that return `Result<_, ()>`.
#![allow(clippy::result_unit_err)]
// Enables the `doc_cfg` feature when the `docsrs` configuration attribute is defined.
#![cfg_attr(docsrs, feature(doc_cfg))]

/// Exports commonly used traits.
pub mod prelude {
    pub use crate::{ReadableRegister, Register, WritableRegister};
    pub use hardware_registers::i2c::*;
    pub use hardware_registers::register_address::RegisterAddress8;
    pub use hardware_registers::sizes::R1;
    pub use hardware_registers::spi::{
        ReadableSPIRegister8, SPIRegister, SPIRegister8, WritableSPIRegister8,
    };
    pub use hardware_registers::{
        FromBits, HardwareRegister, ToBits, TryFromBits, WritableHardwareRegister,
    };
}

/// Implements the address and size information shared by all registers.
macro_rules! register {
    ($type:ident, $addr:expr) => {
        impl $crate::Register for $type {}
        impl $crate::prelude::HardwareRegister<$crate::prelude::R1> for $type {}

        impl
            $crate::prelude::I2CRegister<
                $crate::prelude::DeviceAddress7,
                $crate::prelude::RegisterAddress8,
                $crate::prelude::R1,
            > for $type
        {
            type Backing = u8;

            const DEFAULT_DEVICE_ADDRESS: $crate::prelude::DeviceAddress7 =
                $crate::prelude::DeviceAddress7::new($crate::DEFAULT_DEVICE_ADDRESS);
            const REGISTER_ADDRESS: $crate::prelude::RegisterAddress8 =
                $crate::prelude::RegisterAddress8::new(($addr).addr());
        }

        impl $crate::prelude::SPIRegister<$crate::prelude::RegisterAddress8, $crate::prelude::R1>
            for $type
        {
            type Backing = u8;

            const REGISTER_ADDRESS: $crate::prelude::RegisterAddress8 =
                $crate::prelude::RegisterAddress8::new(($addr).addr());
        }
    };
}

/// Implements infallible decoding, which makes a register readable.
macro_rules! read_access {
    ($type:ident) => {
        impl $crate::prelude::FromBits<u8> for $type {
            #[inline]
            fn from_bits(value: u8) -> Self {
                value.into()
            }

            #[inline]
            fn from_bits_ref(value: &u8) -> Self {
                (*value).into()
            }
        }

        impl $crate::ReadableRegister for $type {}
    };
}

/// Implements encoding, which makes a register writable.
macro_rules! write_access {
    ($type:ident) => {
        impl $crate::prelude::ToBits for $type {
            type Target = u8;

            #[inline]
            fn to_bits(&self) -> Self::Target {
                (*self).into()
            }
        }

        impl $crate::prelude::WritableHardwareRegister<$crate::prelude::R1> for $type {}
        impl $crate::WritableRegister for $type {}
    };
}

macro_rules! readable_register {
    ($type:ident, $addr:expr) => {
        register!($type, $addr);
        read_access!($type);
    };
}

macro_rules! writable_register {
    ($type:ident, $addr:expr) => {
        register!($type, $addr);
        read_access!($type);
        write_access!($type);
    };
}

mod conversions;
mod gyro;
mod types;

pub use gyro::*;
pub use types::*;

/// The bit in the I²C register address (`SUB`) that enables address auto-increment.
pub const I2C_AUTO_INCREMENT: u8 = 0x80;

/// The read bit (`RW`) in the first byte of an SPI transfer. Writes leave it cleared.
pub const SPI_READ: u8 = 0x80;

/// The bit (`MS`) in the first byte of an SPI transfer that enables address auto-increment.
pub const SPI_AUTO_INCREMENT: u8 = 0x40;

/// The value of the [`WhoAmI`] register for a L3GD20.
pub const L3GD20_WHO_AM_I: u8 = 0b1101_0100;

/// A gyroscope register.
pub trait Register: prelude::I2CRegister8<prelude::DeviceAddress7> + From<u8> + Into<u8> {}

/// A readable gyroscope register.
///
/// Most registers decode every bit pattern. [`FifoControlRegister`] rejects reserved
/// FIFO modes, so generic code reads through [`TryFromBits`](prelude::TryFromBits).
pub trait ReadableRegister:
    Register + prelude::ReadableI2CRegister8<prelude::DeviceAddress7>
{
}

/// A writable gyroscope register.
pub trait WritableRegister:
    Register + prelude::WritableI2CRegister8<prelude::DeviceAddress7>
{
}
