//! # STMicroelectronics L3GD20 Gyrscope Registers
//!
//! This crate provides a typed map of the L3GD20's I²C/SPI registers.

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]
#![forbid(unsafe_code)]
// Enables the `doc_cfg` feature when the `docsrs` configuration attribute is defined.
#![cfg_attr(docsrs, feature(doc_cfg))]

/// Exports commonly used traits.
pub mod prelude {
    pub use crate::{Register, WritableRegister};
    pub use hardware_registers::i2c::*;
    pub use hardware_registers::register_address::{RegisterAddress6, RegisterAddress8};
    pub use hardware_registers::sizes::R1;
    pub use hardware_registers::spi::*;
    pub use hardware_registers::{FromBits, HardwareRegister, ToBits, WritableHardwareRegister};
}

macro_rules! readable_register {
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
                $crate::prelude::DeviceAddress7::new(DEFAULT_DEVICE_ADDRESS);
            const REGISTER_ADDRESS: $crate::prelude::RegisterAddress8 =
                $crate::prelude::RegisterAddress8::new(($addr).addr());
        }

        impl $crate::prelude::SPIRegister<$crate::prelude::RegisterAddress6, $crate::prelude::R1>
            for $type
        {
            type Backing = u8;

            const REGISTER_ADDRESS: $crate::prelude::RegisterAddress6 =
                $crate::prelude::RegisterAddress6::new(($addr).addr());
        }

        impl $crate::prelude::ToBits for $type {
            type Target = u8;

            #[inline]
            fn to_bits(&self) -> Self::Target {
                (*self).into()
            }
        }

        impl $crate::prelude::FromBits<u8> for $type {
            #[inline]
            fn from_bits(value: u8) -> Self {
                value.into()
            }

            fn from_bits_ref(value: &u8) -> Self {
                (*value).into()
            }
        }
    };
}

macro_rules! writable_register {
    ($type:ident, $addr:expr) => {
        readable_register!($type, $addr);
        impl $crate::prelude::WritableHardwareRegister<$crate::prelude::R1> for $type {}
        impl $crate::WritableRegister for $type {}
    };
}

pub mod gyro;
mod types;

/// A sensor register.
pub trait Register: prelude::I2CRegister8<prelude::DeviceAddress7> + From<u8> + Into<u8> {}

/// A writable sensor register.
pub trait WritableRegister:
    prelude::WritableI2CRegister8<prelude::DeviceAddress7> + Register
{
}
