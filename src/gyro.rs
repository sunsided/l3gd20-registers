//! Gyroscope registers.

use crate::types::{
    Bandwidth, FifoMode, HighpassFilterMode, InvalidFifoMode, OutputDataRate, Sensitivity,
};
use bitfield_struct::bitfield;

/// The I²C bus address.
///
/// The Slave ADdress (SAD) associated with the L3GD20 is `110101xb`. The SDO pin can be
/// used to modify the less significant bit of the device address. If the SDO pin is connected to
/// voltage supply, LSb is `1` (address `1101011b`). Otherwise, if the SDO pin is connected to
/// ground, the LSb value is `0` (address `1101010b`). This solution allows to connect and
/// address two different gyroscopes to the same I²C bus.
pub const DEFAULT_DEVICE_ADDRESS: u8 = 0b0110_1010;

/// Register addresses specific to the Gyroscope sensor.
///
/// See also [`DEFAULT_DEVICE_ADDRESS`].
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum RegisterAddress {
    /// See [`WhoAmI`]. Read-only.
    WHO_AM_I = 0x0F,
    /// See [`ControlRegister1`]. Read-write.
    CTRL_REG1 = 0x20,
    /// See [`ControlRegister2`]. Read-write.
    CTRL_REG2 = 0x21,
    /// See [`ControlRegister3`]. Read-write.
    CTRL_REG3 = 0x22,
    /// See [`ControlRegister4`]. Read-write.
    CTRL_REG4 = 0x23,
    /// See [`ControlRegister5`]. Read-write.
    CTRL_REG5 = 0x24,
    /// See [`ReferenceRegister`]. Read-write.
    REFERENCE = 0x25,
    /// See [`TemperatureRegister`]. Read-only.
    OUT_TEMP = 0x26,
    /// See [`StatusRegister`]. Read-only.
    STATUS_REG = 0x27,
    /// See [`OutXLow`]. Read-only.
    OUT_X_L = 0x28,
    /// See [`OutXHigh`]. Read-only.
    OUT_X_H = 0x29,
    /// See [`OutYLow`]. Read-only.
    OUT_Y_L = 0x2A,
    /// See [`OutYHigh`]. Read-only.
    OUT_Y_H = 0x2B,
    /// See [`OutZLow`]. Read-only.
    OUT_Z_L = 0x2C,
    /// See [`OutZHigh`]. Read-only.
    OUT_Z_H = 0x2D,
    /// See [`FifoControlRegister`]. Read-write.
    FIFO_CTRL_REG = 0x2E,
    /// See [`FifoSourceRegister`]. Read-only.
    FIFO_SRC_REG = 0x2F,
    /// See [`Int1ConfigurationRegister`]. Read-write.
    INT1_CFG = 0x30,
    /// See [`Int1SourceRegisterA`]. Read-only.
    INT1_SRC = 0x31,
    /// See [`Int1ThresholdRegisterXH`]. Read-write.
    INT1_TSH_XH = 0x32,
    /// See [`Int1ThresholdRegisterXL`]. Read-write.
    INT1_TSH_XL = 0x33,
    /// See [`Int1ThresholdRegisterYH`]. Read-write.
    INT1_TSH_YH = 0x34,
    /// See [`Int1ThresholdRegisterYL`]. Read-write.
    INT1_TSH_YL = 0x35,
    /// See [`Int1ThresholdRegisterZH`]. Read-write.
    INT1_TSH_ZH = 0x36,
    /// See [`Int1ThresholdRegisterZL`]. Read-write.
    INT1_TSH_ZL = 0x37,
    /// See [`Int1DurationRegister`]. Read-write.
    INT1_DURATION = 0x38,
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

/// [`WHO_AM_I`](RegisterAddress::WHO_AM_I) (0Fh)
///
/// The register is read-only:
///
/// ```compile_fail
/// fn write<R: l3gd20_registers::WritableRegister>() {}
/// write::<l3gd20_registers::WhoAmI>();
/// ```
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct WhoAmI {
    /// The identification value. Always [`L3GD20_WHO_AM_I`](crate::L3GD20_WHO_AM_I).
    #[bits(8, access = RO, default = crate::L3GD20_WHO_AM_I)]
    pub ident: u8,
}

readable_register!(WhoAmI, RegisterAddress::WHO_AM_I);

/// [`CTRL_REG1`](RegisterAddress::CTRL_REG1) (20h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ControlRegister1 {
    /// Data rate selection.
    #[bits(2, access = RW)]
    pub output_data_rate: OutputDataRate,

    /// Bandwidth selection.
    #[bits(2, access = RW)]
    pub bandwidth: Bandwidth,

    /// "Power-down mode"
    ///
    /// * `false` to enter power-down mode
    /// * `true` to enter normal or sleep mode
    ///
    /// To enter sleep mode, disable the [`ControlRegister1::z_enable`], [`ControlRegister1::x_enable`]
    /// and [`ControlRegister1::y_enable`] flags.
    #[bits(1, access = RW)]
    pub power_up: bool,

    /// Z-axis enable.
    #[bits(1, access = RW, default = true)]
    pub z_enable: bool,

    /// X-axis enable.
    #[bits(1, access = RW, default = true)]
    pub x_enable: bool,

    /// Y-axis enable.
    #[bits(1, access = RW, default = true)]
    pub y_enable: bool,
}

writable_register!(ControlRegister1, RegisterAddress::CTRL_REG1);

/// [`CTRL_REG2`](RegisterAddress::CTRL_REG2) (21h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ControlRegister2 {
    #[bits(2)]
    __: u8,

    /// High-pass filter mode selection.
    #[bits(2, access = RW)]
    pub hpm: HighpassFilterMode,

    /// High-pass filter Cutoff frequency selection
    #[bits(4, access = RW)]
    pub hpcf: u8, // TODO: Add enum
}

writable_register!(ControlRegister2, RegisterAddress::CTRL_REG2);

/// [`CTRL_REG3`](RegisterAddress::CTRL_REG3) (22h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ControlRegister3 {
    /// Interrupt enable on INT1 pin.
    #[bits(1, access = RW)]
    pub i1int1: bool,

    /// Boot status available on INT1.
    #[bits(1, access = RW)]
    pub i1boot: bool,

    /// Interrupt active configuration on INT1.
    /// * `false` - INT1 is active high (default)
    /// * `true` - INT1 is active low
    #[bits(1, access = RW)]
    pub int1_low: bool,

    /// Push-pull / Open drain selector.
    ///
    /// * `false` - Push-pull (default)
    /// * `true` - Open drain
    #[bits(1, access = RW)]
    pub open_drain: bool,

    /// Date-ready on DRDY/INT2.
    #[bits(1, access = RW)]
    pub i2drdy: bool,

    /// FIFO watermark interrupt on DRDY/INT2.
    #[bits(1, access = RW)]
    pub i2wtm: bool,

    /// FIFO overrun interrupt on DRDY/INT2.
    #[bits(1, access = RW)]
    pub i2orun: bool,

    /// FIFO empty interrupt on DRDY/INT2.
    #[bits(1, access = RW)]
    pub i2empty: bool,
}

writable_register!(ControlRegister3, RegisterAddress::CTRL_REG3);

/// [`CTRL_REG4`](RegisterAddress::CTRL_REG4) (23h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ControlRegister4 {
    /// Block data update.
    ///
    /// * `false` - continuous update
    /// * `true` - output registers not updated until MSb and LSb read
    #[bits(1, access = RW)]
    pub block_data_update: bool,

    /// Big/little endian data selection.
    ///
    /// * `false` - data LSB @ lower address
    /// * `true` - data MSB @ lower address
    #[bits(1, access = RW)]
    pub big_endian: bool,

    /// Full-scale selection
    #[bits(2, access = RW)]
    pub full_scale: Sensitivity,

    #[bits(3, default = 0b000)]
    zeros_12: u8,

    /// SPI serial interface mode.
    ///
    /// * `false` - 4-wire interface (default)
    /// * `true` - 3-wire interface
    #[bits(1, access = RW)]
    pub spi_serial_3wire: bool,
}

writable_register!(ControlRegister4, RegisterAddress::CTRL_REG4);

/// [`CTRL_REG5`](RegisterAddress::CTRL_REG5) (24h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ControlRegister5 {
    /// Reboot memory content
    // have been read
    #[bits(1, access = RW)]
    pub boot: bool,

    /// Enable FIFO
    // have been read
    #[bits(1, access = RW)]
    pub fifo_enable: bool,

    #[bits(1)]
    __: bool,

    /// High-pass filter enable.
    // have been read
    #[bits(1, access = RW)]
    pub hpen: bool,

    /// INT1 selection configuration. See datasheet.
    // have been read
    #[bits(2, access = RW)]
    pub int1_sel: u8, // TODO: Make enum

    /// Out selection configuration. See datasheet.
    // have been read
    #[bits(2, access = RW)]
    pub out_sel: u8, // TODO: Make enum
}

writable_register!(ControlRegister5, RegisterAddress::CTRL_REG5);

/// ## Reference / Data capture register.
///
/// Reference value for interrupt generation.
///
/// [`REFERENCE`](RegisterAddress::REFERENCE) (25h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ReferenceRegister {
    /// Reference value for interrupt generation.
    #[bits(8, access = RW)]
    pub reference: u8,
}

writable_register!(ReferenceRegister, RegisterAddress::REFERENCE);

/// [`OUT_TEMP`](RegisterAddress::OUT_TEMP) (26h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TemperatureRegister {
    /// Temperature data (1LSB/deg - 8-bit resolution). The value is expressed as two's complement.
    /// Updates at a rate of 1 Hz.
    #[bits(8, access = RO)]
    pub temp: i8,
}

readable_register!(TemperatureRegister, RegisterAddress::OUT_TEMP);

/// [`STATUS_REG`](RegisterAddress::STATUS_REG) (27h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct StatusRegister {
    /// X, Y, Z-axis data overrun.
    #[bits(1, access = RO)]
    pub zyx_overrun: bool,

    /// Z-axis data overrun.
    #[bits(1, access = RO)]
    pub z_overrun: bool,

    /// Y-axis data overrun.
    #[bits(1, access = RO)]
    pub y_overrun: bool,

    /// X-axis data overrun.
    #[bits(1, access = RO)]
    pub x_overrun: bool,

    /// X, Y, Z-axis data available.
    #[bits(1, access = RO)]
    pub zyx_da: bool,

    /// Z-axis data available.
    #[bits(1, access = RO)]
    pub z_da: bool,

    /// Y-axis data available.
    #[bits(1, access = RO)]
    pub y_da: bool,

    /// X-axis data available.
    #[bits(1, access = RO)]
    pub x_da: bool,
}

readable_register!(StatusRegister, RegisterAddress::STATUS_REG);

/// [`OUT_X_L`](RegisterAddress::OUT_X_L) (28h)
///
/// Low byte of the 16-bit angular rate value. See [`OutXHigh`] for the high byte.
///
/// ## Data Order
///
/// With [`ControlRegister4::big_endian`] cleared (the default), the low byte has the lower
/// register address and is read first. Setting it swaps the bytes: `OUT_*_L` then holds
/// the high byte. Adding a low and a high register assumes the default order.
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OutXLow {
    /// Low byte of the X-axis value.
    ///
    /// Together with [`OutXHigh`] this forms a reading expressed in two's complement.
    ///
    /// Sensitivity in mdps/digit as well as error depend on [`Sensitivity`] and [`Bandwidth`].
    #[bits(8, access = RO)]
    pub bits: u8,
}

readable_register!(OutXLow, RegisterAddress::OUT_X_L);

/// [`OUT_X_H`](RegisterAddress::OUT_X_H) (29h)
///
/// High byte of the 16-bit angular rate value. See [`OutXLow`] for the low byte.
///
/// ## Data Order
///
/// With [`ControlRegister4::big_endian`] cleared (the default), the low byte has the lower
/// register address and is read first. Setting it swaps the bytes: `OUT_*_L` then holds
/// the high byte. Adding a low and a high register assumes the default order.
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OutXHigh {
    /// High byte of the X-axis value.
    ///
    /// Together with [`OutXLow`] this forms a reading expressed in two's complement.
    ///
    /// Sensitivity in mdps/digit as well as error depend on [`Sensitivity`] and [`Bandwidth`].
    #[bits(8, access = RO)]
    pub bits: u8,
}

readable_register!(OutXHigh, RegisterAddress::OUT_X_H);

/// [`OUT_Y_L`](RegisterAddress::OUT_Y_L) (2Ah)
///
/// Low byte of the 16-bit angular rate value. See [`OutYHigh`] for the high byte.
///
/// ## Data Order
///
/// With [`ControlRegister4::big_endian`] cleared (the default), the low byte has the lower
/// register address and is read first. Setting it swaps the bytes: `OUT_*_L` then holds
/// the high byte. Adding a low and a high register assumes the default order.
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OutYLow {
    /// Low byte of the Y-axis angular rate value.
    ///
    /// Together with [`OutYHigh`] this forms a reading expressed in two's complement.
    ///
    /// Sensitivity in mdps/digit as well as error depend on [`Sensitivity`] and [`Bandwidth`].
    #[bits(8, access = RO)]
    pub bits: u8,
}

readable_register!(OutYLow, RegisterAddress::OUT_Y_L);

/// [`OUT_Y_H`](RegisterAddress::OUT_Y_H) (2Bh)
///
/// High byte of the 16-bit angular rate value. See [`OutYLow`] for the low byte.
///
/// ## Data Order
///
/// With [`ControlRegister4::big_endian`] cleared (the default), the low byte has the lower
/// register address and is read first. Setting it swaps the bytes: `OUT_*_L` then holds
/// the high byte. Adding a low and a high register assumes the default order.
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OutYHigh {
    /// High byte of the Y-axis angular rate value.
    ///
    /// Together with [`OutYLow`] this forms a reading expressed in two's complement.
    ///
    /// Sensitivity in mdps/digit as well as error depend on [`Sensitivity`] and [`Bandwidth`].
    #[bits(8, access = RO)]
    pub bits: u8,
}

readable_register!(OutYHigh, RegisterAddress::OUT_Y_H);

/// [`OUT_Z_L`](RegisterAddress::OUT_Z_L) (2Ch)
///
/// Low byte of the 16-bit angular rate value. See [`OutZHigh`] for the high byte.
///
/// ## Data Order
///
/// With [`ControlRegister4::big_endian`] cleared (the default), the low byte has the lower
/// register address and is read first. Setting it swaps the bytes: `OUT_*_L` then holds
/// the high byte. Adding a low and a high register assumes the default order.
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OutZLow {
    /// Low byte of the Z-axis angular rate value.
    ///
    /// Together with [`OutZHigh`] this forms a reading expressed in two's complement.
    ///
    /// Sensitivity in mdps/digit as well as error depend on [`Sensitivity`] and [`Bandwidth`].
    #[bits(8, access = RO)]
    pub bits: u8,
}

readable_register!(OutZLow, RegisterAddress::OUT_Z_L);

/// [`OUT_Z_H`](RegisterAddress::OUT_Z_H) (2Dh)
///
/// High byte of the 16-bit angular rate value. See [`OutZLow`] for the low byte.
///
/// ## Data Order
///
/// With [`ControlRegister4::big_endian`] cleared (the default), the low byte has the lower
/// register address and is read first. Setting it swaps the bytes: `OUT_*_L` then holds
/// the high byte. Adding a low and a high register assumes the default order.
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OutZHigh {
    /// High byte of the Z-axis angular rate value.
    ///
    /// Together with [`OutZLow`] this forms a reading expressed in two's complement.
    ///
    /// Sensitivity in mdps/digit as well as error depend on [`Sensitivity`] and [`Bandwidth`].
    #[bits(8, access = RO)]
    pub bits: u8,
}

readable_register!(OutZHigh, RegisterAddress::OUT_Z_H);

/// [`FIFO_CTRL_REG`](RegisterAddress::FIFO_CTRL_REG) (2Eh)
///
/// The FIFO mode bits have reserved patterns (`0b101` to `0b111`). Decoding through
/// [`TryFromBits`](crate::prelude::TryFromBits) rejects them with [`InvalidFifoMode`];
/// [`FifoControlRegister::fifo_mode`] panics on a value built from such bits with `From<u8>`.
/// `Debug` prints the reserved mode instead of panicking.
#[bitfield(u8, order = Msb, debug = false)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FifoControlRegister {
    /// FIFO mode selection
    #[bits(3, access = RW, default = FifoMode::Bypass)]
    pub fifo_mode: FifoMode,

    /// FIFO threshold. Watermark level setting.
    #[bits(5, access = RW)]
    pub watermark: u8,
}

register!(FifoControlRegister, RegisterAddress::FIFO_CTRL_REG);
write_access!(FifoControlRegister);

impl crate::prelude::TryFromBits<u8> for FifoControlRegister {
    type Error = InvalidFifoMode;

    #[inline]
    fn try_from_bits_ref(value: &u8) -> Result<Self, Self::Error> {
        let mode = *value >> 5;
        match FifoMode::try_from_bits(mode) {
            Some(_) => Ok(Self::from(*value)),
            None => Err(InvalidFifoMode(mode)),
        }
    }
}

impl crate::ReadableRegister for FifoControlRegister {}

impl core::fmt::Debug for FifoControlRegister {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let bits = u8::from(*self);
        let mode = bits >> 5;
        let mut s = f.debug_struct("FifoControlRegister");
        match FifoMode::try_from_bits(mode) {
            Some(fifo_mode) => s.field("fifo_mode", &fifo_mode),
            None => s.field("fifo_mode", &InvalidFifoMode(mode)),
        };
        s.field("watermark", &self.watermark()).finish()
    }
}

/// [`FIFO_SRC_REG`](RegisterAddress::FIFO_SRC_REG) (2Fh)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FifoSourceRegister {
    /// Watermark status
    ///
    /// * `false` - FIFO filling is lower than watermark level.
    /// * `true` - FIFO filling is equal or higher than watermark level.
    #[bits(1, access = RO)]
    pub wtm: bool,

    /// Overrun bit status.
    ///
    /// * `false` - FIFO is not completely filled.
    /// * `true` - FIFO is completely filled.
    #[bits(1, access = RO)]
    pub ovrn_fifo: bool,

    /// FIFO empty bit.
    #[bits(1, access = RO)]
    pub empty: bool,

    /// FIFO-stored data level.
    #[bits(5, access = RO)]
    pub fss: u8,
}

readable_register!(FifoSourceRegister, RegisterAddress::FIFO_SRC_REG);

/// [`INT1_CFG`](RegisterAddress::INT1_CFG) (30h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Int1ConfigurationRegister {
    /// AND/OR combination of interrupt events.
    #[bits(1, access = RW)]
    pub aoi: bool,

    /// Latch interrupt request.
    #[bits(1, access = RW)]
    pub lir: bool,

    /// Enable interrupt generation on Z high event.
    #[bits(1, access = RW)]
    pub zhie: bool,

    /// Enable interrupt generation on Z low event.
    #[bits(1, access = RW)]
    pub zlie: bool,

    /// Enable interrupt generation on Y high event.
    #[bits(1, access = RW)]
    pub yhie: bool,

    /// Enable interrupt generation on Y low event.
    #[bits(1, access = RW)]
    pub ylie: bool,

    /// Enable interrupt generation on X high event.
    #[bits(1, access = RW)]
    pub xhie: bool,

    /// Enable interrupt generation on X low event.
    #[bits(1, access = RW)]
    pub xlie: bool,
}

writable_register!(Int1ConfigurationRegister, RegisterAddress::INT1_CFG);

/// [`INT1_SRC`](RegisterAddress::INT1_SRC) (31h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Int1SourceRegisterA {
    #[bits(1, default = false)]
    zero: bool,

    /// Interrupt active.
    #[bits(1, access = RO)]
    pub ia: bool,

    /// Z high.
    #[bits(1, access = RO)]
    pub z_high: bool,

    /// Z low.
    #[bits(1, access = RO)]
    pub z_low: bool,

    /// Y high.
    #[bits(1, access = RO)]
    pub y_high: bool,

    /// Y low.
    #[bits(1, access = RO)]
    pub y_low: bool,

    /// X high.
    #[bits(1, access = RO)]
    pub x_high: bool,

    /// X low.
    #[bits(1, access = RO)]
    pub x_low: bool,
}

readable_register!(Int1SourceRegisterA, RegisterAddress::INT1_SRC);

/// [`INT1_TSH_XH`](RegisterAddress::INT1_TSH_XH) (32h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Int1ThresholdRegisterXH {
    #[bits(1, default = false)]
    zero: bool,

    /// Interrupt 1 threshold.
    #[bits(7, access = RW, default = 0)]
    pub threshold: u8,
}

writable_register!(Int1ThresholdRegisterXH, RegisterAddress::INT1_TSH_XH);

/// [`INT1_TSH_XL`](RegisterAddress::INT1_TSH_XL) (33h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Int1ThresholdRegisterXL {
    /// Interrupt 1 threshold. Low byte.
    #[bits(8, access = RW, default = 0)]
    pub threshold: u8,
}

writable_register!(Int1ThresholdRegisterXL, RegisterAddress::INT1_TSH_XL);

/// [`INT1_TSH_YH`](RegisterAddress::INT1_TSH_YH) (34h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Int1ThresholdRegisterYH {
    #[bits(1, default = false)]
    zero: bool,

    /// Interrupt 1 threshold.
    #[bits(7, access = RW, default = 0)]
    pub threshold: u8,
}

writable_register!(Int1ThresholdRegisterYH, RegisterAddress::INT1_TSH_YH);

/// [`INT1_TSH_YL`](RegisterAddress::INT1_TSH_YL) (35h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Int1ThresholdRegisterYL {
    /// Interrupt 1 threshold. Low byte.
    #[bits(8, access = RW, default = 0)]
    pub threshold: u8,
}

writable_register!(Int1ThresholdRegisterYL, RegisterAddress::INT1_TSH_YL);

/// [`INT1_TSH_ZH`](RegisterAddress::INT1_TSH_ZH) (36h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Int1ThresholdRegisterZH {
    #[bits(1, default = false)]
    zero: bool,

    /// Interrupt 1 threshold.
    #[bits(7, access = RW, default = 0)]
    pub threshold: u8,
}

writable_register!(Int1ThresholdRegisterZH, RegisterAddress::INT1_TSH_ZH);

/// [`INT1_TSH_ZL`](RegisterAddress::INT1_TSH_ZL) (37h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Int1ThresholdRegisterZL {
    /// Interrupt 1 threshold. Low byte.
    #[bits(8, access = RW, default = 0)]
    pub threshold: u8,
}

writable_register!(Int1ThresholdRegisterZL, RegisterAddress::INT1_TSH_ZL);

/// [`INT1_DURATION`](RegisterAddress::INT1_DURATION) (38h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Int1DurationRegister {
    /// Wait bit.
    ///
    /// * `false` - Interrupt falls immediately if the signal crosses the threshold.
    /// * `true` - If the signal crosses the selected threshold, the interrupt falls only after the
    ///      duration has counted the number of samples at the selected data rate, written into the
    ///      duration counter register.
    #[bits(1, access = RW, default = false)]
    pub wait: bool,

    /// The minimum duration of the Interrupt 1 event to be recognized. Duration
    /// steps and maximum values depend on the ODR chosen.
    #[bits(7, access = RW, default = 0)]
    pub duration: u8,
}

writable_register!(Int1DurationRegister, RegisterAddress::INT1_DURATION);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn power_on_defaults() {
        assert_eq!(u8::from(WhoAmI::new()), 0xD4);
        assert_eq!(u8::from(ControlRegister1::new()), 0x07);
        assert_eq!(u8::from(ControlRegister4::new()), 0x00);
        assert_eq!(u8::from(FifoControlRegister::new()), 0x00);
    }

    #[test]
    fn control_register_1_layout() {
        let reg = ControlRegister1::new()
            .with_output_data_rate(OutputDataRate::Hz760)
            .with_bandwidth(Bandwidth::Narrow)
            .with_power_up(true);
        assert_eq!(u8::from(reg), 0b1101_1111);
    }

    #[test]
    fn addresses() {
        assert_eq!(
            <WhoAmI as I2CRegister<DeviceAddress7, RegisterAddress8, R1>>::REGISTER_ADDRESS,
            RegisterAddress8::new(0x0F)
        );
        assert_eq!(
            <OutZHigh as SPIRegister<RegisterAddress8, R1>>::REGISTER_ADDRESS,
            RegisterAddress8::new(0x2D)
        );
        assert_eq!(
            <WhoAmI as I2CRegister<DeviceAddress7, RegisterAddress8, R1>>::DEFAULT_DEVICE_ADDRESS,
            DeviceAddress7::new(0x6A)
        );
    }

    #[test]
    fn fifo_control_rejects_reserved_modes() {
        let reg = FifoControlRegister::try_from_bits(0b0100_0101).unwrap();
        assert_eq!(reg.fifo_mode(), FifoMode::Stream);
        assert_eq!(reg.watermark(), 0b0_0101);
        assert_eq!(
            FifoControlRegister::try_from_bits(0b1010_0000),
            Err(InvalidFifoMode(0b101))
        );
        assert_eq!(
            FifoControlRegister::try_from_bits(0b1111_1111),
            Err(InvalidFifoMode(0b111))
        );
    }

    #[test]
    fn fifo_control_debug_does_not_panic() {
        extern crate std;
        use std::format;
        assert_eq!(
            format!("{:?}", FifoControlRegister::from(0b1010_0011)),
            "FifoControlRegister { fifo_mode: InvalidFifoMode(5), watermark: 3 }"
        );
        assert_eq!(
            format!("{:?}", FifoControlRegister::from(0b0100_0001)),
            "FifoControlRegister { fifo_mode: Stream, watermark: 1 }"
        );
    }

    #[test]
    fn temperature_is_signed() {
        assert_eq!(TemperatureRegister::from(0xFB).temp(), -5);
        assert_eq!(TemperatureRegister::from(0x19).temp(), 25);
    }

    /// Bit positions follow the register tables of the L3GD20 datasheet (DocID022116).
    #[test]
    fn flag_layouts() {
        let s = StatusRegister::from(0b1010_0101);
        assert!(s.zyx_overrun() && !s.z_overrun() && s.y_overrun() && !s.x_overrun());
        assert!(!s.zyx_da() && s.z_da() && !s.y_da() && s.x_da());

        let c3 = ControlRegister3::from(0b1001_0110);
        assert!(c3.i1int1() && !c3.i1boot() && !c3.int1_low() && c3.open_drain());
        assert!(!c3.i2drdy() && c3.i2wtm() && c3.i2orun() && !c3.i2empty());

        let c4 = ControlRegister4::from(0b1101_0001);
        assert!(c4.block_data_update() && c4.big_endian() && c4.spi_serial_3wire());
        assert_eq!(c4.full_scale(), Sensitivity::D500);

        let c5 = ControlRegister5::from(0b0101_1001);
        assert!(!c5.boot() && c5.fifo_enable() && c5.hpen());
        assert_eq!((c5.int1_sel(), c5.out_sel()), (0b10, 0b01));

        let cfg = Int1ConfigurationRegister::from(0b1001_0110);
        assert!(cfg.aoi() && !cfg.lir() && !cfg.zhie() && cfg.zlie());
        assert!(!cfg.yhie() && cfg.ylie() && cfg.xhie() && !cfg.xlie());

        let src = Int1SourceRegisterA::from(0b0110_1001);
        assert!(src.ia() && src.z_high() && !src.z_low() && src.y_high());
        assert!(!src.y_low() && !src.x_high() && src.x_low());
    }

    #[test]
    fn readable_and_writable_bounds() {
        fn readable<R: ReadableRegister + ReadableSPIRegister8>() {}
        fn writable<R: WritableRegister + WritableSPIRegister8>() {}
        readable::<WhoAmI>();
        readable::<FifoControlRegister>();
        readable::<OutXLow>();
        writable::<FifoControlRegister>();
        writable::<ControlRegister1>();
    }
}
