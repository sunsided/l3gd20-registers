//! Magnetometer and Temperature registers.

// Resolves a code-generation issue with the bitfield macro.
#![allow(clippy::unnecessary_cast)]

mod types;

use bitfield_struct::bitfield;
pub use types::*;

/// The I2C bus address.
///
/// For magnetic sensors the default (factory) 7-bit slave address is 0011110xb.
///
/// The slave address is completed with a Read/Write bit. If the bit is `1` (read), a repeated
/// `START` (`SR`) condition must be issued after the two sub-address bytes. If the bit is `0` (write)
/// the master transmits to the slave with the direction unchanged.
pub const DEFAULT_DEVICE_ADDRESS: u8 = 0b0011110;

/// Register addresses specific to the magnetometer sensor.
///
/// See also [`DEFAULT_DEVICE_ADDRESS`].
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegisterAddress {
    /// See [`ConfigurationARegisterM`].
    CRA_REG_M = 0x00,
    /// See [`ConfigurationBRegisterM`].
    CRB_REG_M = 0x01,
    /// See [`ModeRegisterM`].
    MR_REG_M = 0x02,
    /// See [`OutXHighM`].
    OUT_X_H_M = 0x03,
    /// See [`OutXLowM`].
    OUT_X_L_M = 0x04,
    /// See [`OutZHighM`].
    OUT_Z_H_M = 0x05,
    /// See [`OutZLowM`].
    OUT_Z_L_M = 0x06,
    /// See [`OutYHighM`].
    OUT_Y_H_M = 0x07,
    /// See [`OutYLowM`].
    OUT_Y_L_M = 0x08,
    /// See [`StatusRegisterM`].
    SR_REG_M = 0x09,
    /// See [`IdentificationARegisterM`].
    IRA_REG_M = 0x0A,
    /// See [`IdentificationBRegisterM`].
    IRB_REG_M = 0x0B,
    /// See [`IdentificationCRegisterM`].
    IRC_REG_M = 0x0C,
    /// See [`TemperatureOutHighM`].
    TEMP_OUT_H_M = 0x31,
    /// See [`TemperatureOutLowM`].
    TEMP_OUT_L_M = 0x32,
}

impl RegisterAddress {
    /// Returns the address of a register.
    pub const fn addr(&self) -> u8 {
        *self as u8
    }
}

impl From<RegisterAddress> for u8 {
    fn from(value: RegisterAddress) -> Self {
        value.addr()
    }
}

/// [`CRA_REG_M`](RegisterAddress::CRA_REG_M) (00h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ConfigurationARegisterM {
    /// Temperature sensor enabled.
    #[bits(1, access = RW)]
    pub temp_en: bool,

    /// Must be zero for correct operation of the device.
    #[bits(2, default = 0)]
    zeros_56: u8,

    /// Data output rate bits. These bits set the rate at which data is written to all three data
    /// output registers.
    ///
    /// Note that the datasheet has inconsistent information about this field.
    /// While section 7.2.1 reports default of `100`, i.e. 15 Hz, the register mapping
    /// table 17 appears to be missing a zero.
    #[bits(3, access = RW, default = MagOdr::Hz15)]
    pub data_output_rate: MagOdr,

    /// Must be zero for correct operation of the device.
    #[bits(2, default = 0)]
    zeros_01: u8,
}

writable_register!(ConfigurationARegisterM, RegisterAddress::CRA_REG_M);

/// Magnetometer gain configuration.
///
/// [`CRB_REG_M`](RegisterAddress::CRB_REG_M) (01h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ConfigurationBRegisterM {
    /// Gain configuration.
    #[bits(3, access = RW, default = MagGain::Gauss1_3)]
    pub gain: MagGain,

    /// Must be zero for correct operation of the device.
    #[bits(5, default = 0)]
    zeros_04: u8,
}

writable_register!(ConfigurationBRegisterM, RegisterAddress::CRB_REG_M);

/// Magnetometer mode select.
///
/// [`MR_REG_M`](RegisterAddress::MR_REG_M) (02h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ModeRegisterM {
    /// Must be zero for correct operation of the device.
    #[bits(6, default = 0)]
    zeros_27: u8,

    /// Device is placed in sleep mode.
    #[bits(1, access = RW, default = true)]
    pub sleep_mode: bool,

    /// Enables single conversion mode.
    ///
    /// * `false` - Continuous conversion mode.
    /// * `true` - Single conversion mode.
    #[bits(1, access = RW, default = true)]
    pub single_conversion: bool,
}

writable_register!(ModeRegisterM, RegisterAddress::MR_REG_M);

/// [`OUT_X_H_M`](RegisterAddress::OUT_X_H_M) (03h)
///
/// High byte of the 16-bit acceleration value. See [`OutXLowM`] for the low byte.
///
/// ## X-Z-Y Order
///
/// Note that the reading registers are provided in X-Z-Y order, not X, then Y, then Z.
///
/// ## Big Endian Data Order
///
/// Note that the registers are provided in big endian order, i.e. the high byte
/// has the lower register address and will be read first.
/// This is different from the accelerometer and temperature reading registers.
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OutXHighM {
    /// High byte of the X-axis magnetic field value.
    ///
    /// Together with [`OutXLowM`] this forms a reading expressed in two's complement.
    #[bits(8, access = RO)]
    pub bits: u8,
}

readable_register!(OutXHighM, RegisterAddress::OUT_X_H_M);

/// [`OUT_X_L_M`](RegisterAddress::OUT_X_L_M) (04h)
///
/// Low byte of the 16-bit acceleration value. See [`OutXHighM`] for the high byte.
///
/// ## X-Z-Y Order
///
/// Note that the reading registers are provided in X-Z-Y order, not X, then Y, then Z.
///
/// ## Big Endian Data Order
///
/// Note that the registers are provided in big endian order, i.e. the high byte
/// has the lower register address and will be read first.
/// This is different from the accelerometer and temperature reading registers.
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OutXLowM {
    /// Low byte of the X-axis magnetic field value.
    ///
    /// Together with [`OutXHighM`] this forms a reading expressed in two's complement.
    #[bits(8, access = RO)]
    pub bits: u8,
}

readable_register!(OutXLowM, RegisterAddress::OUT_X_L_M);

/// [`OUT_Z_H_M`](RegisterAddress::OUT_Z_H_M) (05h)
///
/// High byte of the 16-bit acceleration value. See [`OutZLowM`] for the low byte.
///
/// ## X-Z-Y Order
///
/// Note that the reading registers are provided in X-Z-Y order, not X, then Y, then Z.
///
/// ## Big Endian Data Order
///
/// Note that the registers are provided in big endian order, i.e. the high byte
/// has the lower register address and will be read first.
/// This is different from the accelerometer and temperature reading registers.
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OutZHighM {
    /// High byte of the Z-axis magnetic field value.
    ///
    /// Together with [`OutZLowM`] this forms a reading expressed in two's complement.
    #[bits(8, access = RO)]
    pub bits: u8,
}

readable_register!(OutZHighM, RegisterAddress::OUT_Z_H_M);

/// [`OUT_Z_L_M`](RegisterAddress::OUT_Z_L_M) (06h)
///
/// Low byte of the 16-bit acceleration value. See [`OutZHighM`] for the high byte.
///
/// ## X-Z-Y Order
///
/// Note that the reading registers are provided in X-Z-Y order, not X, then Y, then Z.
///
/// ## Big Endian Data Order
///
/// Note that the registers are provided in big endian order, i.e. the high byte
/// has the lower register address and will be read first.
/// This is different from the accelerometer and temperature reading registers.
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OutZLowM {
    /// Low byte of the Z-axis magnetic field value.
    ///
    /// Together with [`OutZHighM`] this forms a reading expressed in two's complement.
    #[bits(8, access = RO)]
    pub bits: u8,
}

readable_register!(OutZLowM, RegisterAddress::OUT_Z_L_M);

/// [`OUT_Y_H_M`](RegisterAddress::OUT_Y_H_M) (07h)
///
/// High byte of the 16-bit acceleration value. See [`OutYLowM`] for the low byte.
///
/// ## X-Z-Y Order
///
/// Note that the reading registers are provided in X-Z-Y order, not X, then Y, then Z.
///
/// ## Big Endian Data Order
///
/// Note that the registers are provided in big endian order, i.e. the high byte
/// has the lower register address and will be read first.
/// This is different from the accelerometer and temperature reading registers.
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OutYHighM {
    /// High byte of the Y-axis magnetic field value.
    ///
    /// Together with [`OutYLowM`] this forms a reading expressed in two's complement.
    #[bits(8, access = RO)]
    pub bits: u8,
}

readable_register!(OutYHighM, RegisterAddress::OUT_Y_H_M);

/// [`OUT_Y_L_M`](RegisterAddress::OUT_Y_L_M) (08h)
///
/// Low byte of the 16-bit acceleration value. See [`OutYHighM`] for the high byte.
///
/// ## X-Z-Y Order
///
/// Note that the reading registers are provided in X-Z-Y order, not X, then Y, then Z.
///
/// ## Big Endian Data Order
///
/// Note that the registers are provided in big endian order, i.e. the high byte
/// has the lower register address and will be read first.
/// This is different from the accelerometer and temperature reading registers.
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OutYLowM {
    /// Low byte of the Y-axis magnetic field value.
    ///
    /// Together with [`OutYHighM`] this forms a reading expressed in two's complement.
    #[bits(8, access = RO)]
    pub bits: u8,
}

readable_register!(OutYLowM, RegisterAddress::OUT_Y_L_M);

/// [`SR_REG_M`](RegisterAddress::SR_REG_M) (09h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct StatusRegisterM {
    #[bits(6)]
    __: u8,

    /// Data output register lock. Once a new set of measurements is available, this bit is
    /// set when the first magnetic file data register has been read.
    #[bits(1, access = RO)]
    pub do_lock: bool,

    /// Data-ready bit. This bit is when a new set of measurements is available.
    #[bits(1, access = RO)]
    pub data_ready: bool,
}

readable_register!(StatusRegisterM, RegisterAddress::SR_REG_M);

/// The identification registers (IR) are used to identify the device.
/// (See Doc ID 16941 Rev 1. for the LSM303DLH, non -C version)
///
/// [`IRA_REG_M`](RegisterAddress::IRA_REG_M) (0Ah)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct IdentificationARegisterM {
    /// Undocumented. Always `01001000`, ASCII `H`.
    #[bits(8, access = RO, default = 0b01001000_u8)]
    pub value: u8,
}

readable_register!(IdentificationARegisterM, RegisterAddress::IRA_REG_M);

/// The identification registers (IR) are used to identify the device.
/// (See Doc ID 16941 Rev 1. for the LSM303DLH, non -C version)
///
/// [`IRB_REG_M`](RegisterAddress::IRB_REG_M) (0Bh)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct IdentificationBRegisterM {
    /// Undocumented. Always `00110100`, ASCII `4`.
    #[bits(8, access = RO, default = 0b000110100_u8)]
    pub value: u8,
}

readable_register!(IdentificationBRegisterM, RegisterAddress::IRB_REG_M);

/// The identification registers (IR) are used to identify the device.
/// (See Doc ID 16941 Rev 1. for the LSM303DLH, non -C version)
///
/// [`IRC_REG_M`](RegisterAddress::IRC_REG_M) (0Ch)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct IdentificationCRegisterM {
    /// Undocumented. Always `00110011`, ASCII `3`.
    #[bits(8, access = RO, default = 0b00110011_u8)]
    pub value: u8,
}

readable_register!(IdentificationCRegisterM, RegisterAddress::IRC_REG_M);

/// [`TEMP_OUT_H_M`](RegisterAddress::TEMP_OUT_H_M) (0Ch)
///
/// High byte of the 12-bit temperature reading.
///
/// Together with [`TemperatureOutLowM`], the value is expressed as two's complement with
/// 8 LSB/deg at 12-bit resolution.
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TemperatureOutHighM {
    /// The upper
    #[bits(8, access = RO)]
    pub value: u8,
}

readable_register!(TemperatureOutHighM, RegisterAddress::TEMP_OUT_H_M);

/// [`TEMP_OUT_L_M`](RegisterAddress::TEMP_OUT_L_M) (0Ch)
///
/// Low byte of the 12-bit temperature reading.
///
/// Together with [`TemperatureOutLowM`], the value is expressed as two's complement with
/// 8 LSB/deg at 12-bit resolution.
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TemperatureOutLowM {
    /// The lower nibble of the temperature reading.
    #[bits(4, access = RO)]
    pub value: u8,

    #[bits(4)]
    __: u8,
}

readable_register!(TemperatureOutLowM, RegisterAddress::TEMP_OUT_L_M);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cra_defaults() {
        let value = ConfigurationARegisterM::new();
        assert_eq!(value.into_bits(), 0b0010000);
    }
}
