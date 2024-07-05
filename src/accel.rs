//! Accelerometer registers.

mod types;

pub use types::*;

use bitfield_struct::bitfield;

/// The I2C bus address.
///
/// For linear acceleration the default (factory) 7-bit slave address is `0011001b`.
///
/// The slave address is completed with a Read/Write bit. If the bit is `1` (read), a repeated
/// `START` (`SR`) condition must be issued after the two sub-address bytes; if the bit is `0` (write)
/// the master transmits to the slave with the direction unchanged.
///
/// When the MSB is set to `1`, multiple bytes can be read.
pub const DEFAULT_DEVICE_ADDRESS: u8 = 0b0011001;

/// Register addresses specific to the accelerometer sensor.
///
/// See also [`DEFAULT_DEVICE_ADDRESS`].
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegisterAddress {
    /// See [`ControlRegister1A`].
    CTRL_REG1_A = 0x20,
    /// See [`ControlRegister2A`].
    CTRL_REG2_A = 0x21,
    /// See [`ControlRegister3A`].
    CTRL_REG3_A = 0x22,
    /// See [`ControlRegister4A`].
    CTRL_REG4_A = 0x23,
    /// See [`ControlRegister5A`].
    CTRL_REG5_A = 0x24,
    /// See [`ControlRegister6A`].
    CTRL_REG6_A = 0x25,
    /// See [`ReferenceRegisterA`].
    REFERENCE_A = 0x26,
    /// See [`StatusRegisterA`].
    STATUS_REG_A = 0x27,
    /// See [`OutXLowA`].
    OUT_X_L_A = 0x28,
    /// See [`OutXHighA`].
    OUT_X_H_A = 0x29,
    /// See [`OutYLowA`].
    OUT_Y_L_A = 0x2A,
    /// See [`OutYHighA`].
    OUT_Y_H_A = 0x2B,
    /// See [`OutZLowA`].
    OUT_Z_L_A = 0x2C,
    /// See [`OutZHighA`].
    OUT_Z_H_A = 0x2D,
    /// See [`FifoControlRegisterA`].
    FIFO_CTRL_REG_A = 0x2E,
    /// See [`FifoSourceRegisterA`].
    FIFO_SRC_REG_A = 0x2F,
    /// See [`Int1ConfigurationRegisterA`].
    INT1_CFG_A = 0x30,
    /// See [`Int1SourceRegisterA`].
    INT1_SRC_A = 0x31,
    /// See [`Int1ThresholdRegisterA`].
    INT1_THS_A = 0x32,
    /// See [`Int1DurationRegisterA`].
    INT1_DURATION_A = 0x33,
    /// See [`Int1ConfigurationRegisterA`].
    INT2_CFG_A = 0x34,
    /// See [`Int2SourceRegisterA`].
    INT2_SRC_A = 0x35,
    /// See [`Int2ThresholdRegisterA`].
    INT2_THS_A = 0x36,
    /// See [`Int2DurationRegisterA`].
    INT2_DURATION_A = 0x37,
    /// See [`ClickConfigurationRegisterA`].
    CLICK_CFG_A = 0x38,
    /// See [`ClickSourceRegisterA`].
    CLICK_SRC_A = 0x39,
    /// See [`ClickThresholdRegisterA`].
    CLICK_THS_A = 0x3A,
    /// See [`ClickTimeLimitRegisterA`].
    TIME_LIMIT_A = 0x3B,
    /// See [`ClickTimeLatencyRegisterA`].
    TIME_LATENCY_A = 0x3C,
    /// See [`ClickTimeWindowRegisterA`].
    TIME_WINDOW_A = 0x3D,
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

/// [`CTRL_REG1_A`](RegisterAddress::CTRL_REG1_A) (20h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ControlRegister1A {
    /// Data rate selection.
    #[bits(4, access = RW)]
    pub output_data_rate: AccelOdr,

    /// Low-power mode enable.
    #[bits(1, access = RW)]
    pub low_power_enable: bool,

    /// Z-axis enable.
    #[bits(1, access = RW, default = true)]
    pub z_enable: bool,

    /// Y-axis enable.
    #[bits(1, access = RW, default = true)]
    pub y_enable: bool,

    /// X-axis enable.
    #[bits(1, access = RW, default = true)]
    pub x_enable: bool,
}

writable_register!(ControlRegister1A, RegisterAddress::CTRL_REG1_A);

/// [`CTRL_REG2_A`](RegisterAddress::CTRL_REG2_A) (21h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ControlRegister2A {
    /// High-pass filter mode selection.
    #[bits(2, access = RW)]
    pub hpm: HighpassFilterMode,

    /// High-pass filter Cutoff frequency selection
    #[bits(2, access = RW)]
    pub hpcf: u8, // TODO: Add enum

    /// Filter data selection
    #[bits(1, access = RW)]
    pub fds: bool,

    /// High-pass filter enabled for click function
    #[bits(1, access = RW)]
    pub hpclick: bool,

    /// High-pass filter enabled for AOI function on Interrupt 2
    #[bits(1, access = RW)]
    pub hpis2: bool,

    /// High-pass filter enabled for AOI function on Interrupt 1
    #[bits(1, access = RW)]
    pub hpis1: bool,
}

writable_register!(ControlRegister2A, RegisterAddress::CTRL_REG2_A);

/// [`CTRL_REG3_A`](RegisterAddress::CTRL_REG3_A) (22h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ControlRegister3A {
    /// Enable CLICK interrupt on INT1
    #[bits(1, access = RW)]
    pub i1click: bool,

    /// Enable AOI1 interrupt on INT1
    #[bits(1, access = RW)]
    pub i1aoi1: bool,

    /// Enable AOI2 interrupt on INT1
    #[bits(1, access = RW)]
    pub i1aoi2: bool,

    /// Enable the accelerometer data ready (`DRDY1`) interrupt on `INT1`.
    ///
    /// Enabling this on a vanilla sensor will raise an interrupt on `INT1` whenever the
    /// accelerometer reads new data.
    #[bits(1, access = RW)]
    pub i1drdy1: bool,

    /// Enable the accelerometer data ready (`DRDY2`) interrupt on `INT1`.
    #[bits(1, access = RW)]
    pub i1drdy2: bool,

    /// Enable FIFO watermark interrupt on INT1
    #[bits(1, access = RW)]
    pub i1wtm: bool,

    /// Enable FIFO overrun interrupt on INT1
    #[bits(1, access = RW)]
    pub i1overrun: bool,

    #[bits(1)]
    __: bool,
}

writable_register!(ControlRegister3A, RegisterAddress::CTRL_REG3_A);

/// [`CTRL_REG4_A`](RegisterAddress::CTRL_REG4_A) (23h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ControlRegister4A {
    /// Block data update.
    ///
    /// The BDU bit is used to inhibit output register updates between the reading of the upper and
    /// lower register parts. In default mode (BDU = `0`), the lower and upper register parts are
    /// updated continuously. If it is not certain to read faster than output data rate, it is
    /// recommended to set BDU bit to `1`. In this way, after the reading of the lower (upper) register
    /// part, the content of that output register is not updated until the upper (lower) part is read
    /// also. This feature avoids reading LSB and MSB related to different samples.
    ///
    /// * `false` - continuous update
    /// * `true` - output registers not updated until MSB and LSB
    // have been read
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

    /// High-resolution output mode.
    #[bits(1, access = RW)]
    pub high_resolution: bool,

    #[bits(2, default = 0b00)]
    zeros_12: u8,

    /// SPI serial interface mode.
    ///
    /// * `false` - 4-wire interface
    /// * `true` - 3-wire interface
    #[bits(1, access = RW)]
    pub spi_serial_3wire: bool,
}

writable_register!(ControlRegister4A, RegisterAddress::CTRL_REG4_A);

/// [`CTRL_REG5_A`](RegisterAddress::CTRL_REG5_A) (24h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ControlRegister5A {
    /// Reboot memory content
    // have been read
    #[bits(1, access = RW)]
    pub boot: bool,

    /// Enable FIFO
    // have been read
    #[bits(1, access = RW)]
    pub fifo_enable: bool,

    #[bits(2)]
    __: u8,

    /// Latch interrupt request on INT1_SRC register, with INT1_SRC register cleared
    /// by reading INT1_SRC itself.
    // have been read
    #[bits(1, access = RW)]
    pub lir_int1: bool,

    /// 4D enable: 4D detection is enabled on INT1 when 6D bit on INT1_CFG is set to 1.
    // have been read
    #[bits(1, access = RW)]
    pub d4d_int1: bool,

    /// Latch interrupt request on INT2_SRC register, with INT2_SRC register cleared
    /// by reading INT2_SRC itself.
    // have been read
    #[bits(1, access = RW)]
    pub lir_int2: bool,

    /// 4D enable: 4D detection is enabled on INT2 when 6D bit on INT2_CFG is set to 1.
    // have been read
    #[bits(1, access = RW)]
    pub d4d_int2: bool,
}

writable_register!(ControlRegister5A, RegisterAddress::CTRL_REG5_A);

/// [`CTRL_REG6_A`](RegisterAddress::CTRL_REG6_A) (25h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ControlRegister6A {
    /// CLICK interrupt enable on PAD2.
    // have been read
    #[bits(1, access = RW)]
    pub i2click_en: bool,

    /// Interrupt 1 on PAD2
    // have been read
    #[bits(1, access = RW)]
    pub i2int1: bool,

    /// Interrupt 2 on PAD2
    // have been read
    #[bits(1, access = RW)]
    pub i2int2: bool,

    /// Reboot memory content on PAD2
    // have been read
    #[bits(1, access = RW)]
    pub boot_i1: bool,

    /// Active functions status on PAD2
    // have been read
    #[bits(1, access = RW)]
    pub p2_active: bool,

    #[bits(1)]
    __: u8,

    /// Interrupt active low
    ///
    /// * `false` - Interrupt is active high
    /// * `true` - Interrupt is active low
    // have been read
    #[bits(1, access = RW)]
    pub active_low: bool,

    #[bits(1)]
    __: u8,
}

writable_register!(ControlRegister6A, RegisterAddress::CTRL_REG6_A);

/// This register sets the acceleration value taken as a reference for the high-pass filter output.
/// (See Doc ID 16941 Rev 1. for the LSM303DLH, non -C version)
///
/// [`REFERENCE_A`](RegisterAddress::REFERENCE_A) (26h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ReferenceRegisterA {
    /// Reference value for interrupt generation.
    #[bits(8, access = RO)]
    pub reference: u8,
}

writable_register!(ReferenceRegisterA, RegisterAddress::REFERENCE_A);

/// [`STATUS_REG_A`](RegisterAddress::STATUS_REG_A) (27h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct StatusRegisterA {
    /// X-, Y-, and Z-axis data overrun.
    /// * `false` - no overrun has occurred
    /// * `true` - a new set of data has overwritten the previous data
    #[bits(1, access = RO)]
    pub zyx_overrun: bool,

    /// Z-axis data overrun.
    /// * `false` - no overrun has occurred
    /// * `true` - new data for the Z-axis has overwritten the previous data
    #[bits(1, access = RO)]
    pub z_overrun: bool,

    /// Y-axis data overrun.
    /// * `false` - no overrun has occurred
    /// * `true` - new data for the Y-axis has overwritten the previous data
    #[bits(1, access = RO)]
    pub y_overrun: bool,

    /// X-axis data overrun.
    /// * `false` - no overrun has occurred
    /// * `true` - new data for the X-axis has overwritten the previous data
    #[bits(1, access = RO)]
    pub x_overrun: bool,

    ///  X-, Y-, and Z-axis new data available.
    /// * `false` - a new set of data is not yet available
    /// * `true` - a new set of data is available
    #[bits(1, access = RO)]
    pub xyz_data_available: bool,

    /// Z-axis new data available.
    /// * `false` - new data for the Z-axis is not yet available
    /// * `true` - new data for the Z-axis is available
    #[bits(1, access = RO)]
    pub z_data_available: bool,

    /// Y-axis new data available.
    /// * `false` - new data for the Y-axis is not yet available
    /// * `true` - new data for the Y-axis is available
    #[bits(1, access = RO)]
    pub y_data_available: bool,

    /// X-axis new data available.
    /// * `false` - new data for the X-axis is not yet available
    /// * `true` - new data for the X-axis is available
    #[bits(1, access = RO)]
    pub x_data_available: bool,
}

readable_register!(StatusRegisterA, RegisterAddress::STATUS_REG_A);

/// [`OUT_X_L_A`](RegisterAddress::OUT_X_L_A) (28h)
///
/// Low byte of the 16-bit acceleration value. See [`OutXHighA`] for the high byte.
///
/// ## Little Endian Data Order
///
/// Note that the registers are provided in little endian order, i.e. the low byte
/// has the lower register address and will be read first.
/// While the temperature readings follow the same principle, the magnetometer readings
/// have a different order.
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OutXLowA {
    /// Low byte of the X-axis value.
    ///
    /// Together with [`OutXHighA`] this forms a reading expressed in two's complement.
    #[bits(8, access = RO)]
    pub bits: u8,
}

readable_register!(OutXLowA, RegisterAddress::OUT_X_L_A);

/// [`OUT_X_H_A`](RegisterAddress::OUT_X_H_A) (29h)
///
/// High byte of the 16-bit acceleration value. See [`OutXLowA`] for the low byte.
///
/// ## Little Endian Data Order
///
/// Note that the registers are provided in little endian order, i.e. the low byte
/// has the lower register address and will be read first.
/// While the temperature readings follow the same principle, the magnetometer readings
/// have a different order.
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OutXHighA {
    /// High byte of the X-axis value.
    ///
    /// Together with [`OutXLowA`] this forms a reading expressed in two's complement.
    #[bits(8, access = RO)]
    pub bits: u8,
}

readable_register!(OutXHighA, RegisterAddress::OUT_X_H_A);

/// [`OUT_Y_L_A`](RegisterAddress::OUT_Y_L_A) (2Ah)
///
/// Low byte of the 16-bit acceleration value. See [`OutYHighA`] for the high byte.
///
/// ## Little Endian Data Order
///
/// Note that the registers are provided in little endian order, i.e. the low byte
/// has the lower register address and will be read first.
/// While the temperature readings follow the same principle, the magnetometer readings
/// have a different order.
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OutYLowA {
    /// Low byte of the Y-axis acceleration value.
    ///
    /// Together with [`OutYHighA`] this forms a reading expressed in two's complement.
    #[bits(8, access = RO)]
    pub bits: u8,
}

readable_register!(OutYLowA, RegisterAddress::OUT_Y_L_A);

/// [`OUT_Y_H_A`](RegisterAddress::OUT_Y_H_A) (2Bh)
///
/// High byte of the 16-bit acceleration value. See [`OutYLowA`] for the low byte.
///
/// ## Little Endian Data Order
///
/// Note that the registers are provided in little endian order, i.e. the low byte
/// has the lower register address and will be read first.
/// While the temperature readings follow the same principle, the magnetometer readings
/// have a different order.
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OutYHighA {
    /// High byte of the Y-axis acceleration value.
    ///
    /// Together with [`OutYLowA`] this forms a reading expressed in two's complement.
    #[bits(8, access = RO)]
    pub bits: u8,
}

readable_register!(OutYHighA, RegisterAddress::OUT_Y_H_A);

/// [`OUT_Z_L_A`](RegisterAddress::OUT_Z_L_A) (2Ch)
///
/// Low byte of the 16-bit acceleration value. See [`OutZHighA`] for the high byte.
///
/// ## Little Endian Data Order
///
/// Note that the registers are provided in little endian order, i.e. the low byte
/// has the lower register address and will be read first.
/// While the temperature readings follow the same principle, the magnetometer readings
/// have a different order.
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OutZLowA {
    /// Low byte of the Z-axis acceleration value.
    ///
    /// Together with [`OutZHighA`] this forms a reading expressed in two's complement.
    #[bits(8, access = RO)]
    pub bits: u8,
}

readable_register!(OutZLowA, RegisterAddress::OUT_Z_L_A);

/// [`OUT_Z_H_A`](RegisterAddress::OUT_Z_H_A) (2Dh)
///
/// High byte of the 16-bit acceleration value. See [`OutZLowA`] for the low byte.
///
/// ## Little Endian Data Order
///
/// Note that the registers are provided in little endian order, i.e. the low byte
/// has the lower register address and will be read first.
/// While the temperature readings follow the same principle, the magnetometer readings
/// have a different order.
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OutZHighA {
    /// High byte of the Z-axis acceleration value.
    ///
    /// Together with [`OutZLowA`] this forms a reading expressed in two's complement.
    #[bits(8, access = RO)]
    pub bits: u8,
}

readable_register!(OutZHighA, RegisterAddress::OUT_Z_H_A);

/// [`FIFO_CTRL_REG_A`](RegisterAddress::FIFO_CTRL_REG_A) (2Eh)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FifoControlRegisterA {
    /// FIFO mode selection
    #[bits(2, access = RW, default = FifoMode::Bypass)]
    pub fifo_mode: FifoMode,

    /// Trigger selection
    ///
    /// * `false` - Trigger event linked to trigger signal on INT1
    /// * `true` - Trigger event linked to trigger signal on INT1
    #[bits(1, access = RW)]
    pub trigger_on_int2: bool,

    /// The `fth` field.
    #[bits(5, access = RW)]
    pub fth: u8,
}

writable_register!(FifoControlRegisterA, RegisterAddress::FIFO_CTRL_REG_A);

/// [`FIFO_CTRL_REG_A`](RegisterAddress::FIFO_SRC_REG_A) (2Fh)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FifoSourceRegisterA {
    #[bits(1, access = RO)]
    pub wtm: bool,

    #[bits(1, access = RO)]
    pub ovrn_fifo: bool,

    #[bits(1, access = RO)]
    pub empty: bool,

    #[bits(5, access = RO)]
    pub fss: u8,
}

readable_register!(FifoSourceRegisterA, RegisterAddress::FIFO_SRC_REG_A);

/// [`INT1_CFG_A`](RegisterAddress::INT1_CFG_A) (2Fh)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Int1ConfigurationRegisterA {
    /// AND/OR combination of interrupt events.
    #[bits(1, access = RW)]
    pub aoi: bool,

    /// 6-direction detection function enabled.
    #[bits(1, access = RW)]
    pub six_d: bool,

    /// Enable interrupt generation on Z high event or on direction recognition.
    #[bits(1, access = RW)]
    pub zhie_zupe: bool,

    /// Enable interrupt generation on Z low event or on direction recognition.
    #[bits(1, access = RW)]
    pub zlie_zdowne: bool,

    /// Enable interrupt generation on Y high event or on direction recognition.
    #[bits(1, access = RW)]
    pub yhie_yupe: bool,

    /// Enable interrupt generation on Y low event or on direction recognition.
    #[bits(1, access = RW)]
    pub ylie_ydowne: bool,

    /// Enable interrupt generation on X high event or on direction recognition.
    #[bits(1, access = RW)]
    pub xhie_xupe: bool,

    /// Enable interrupt generation on X low event or on direction recognition.
    #[bits(1, access = RW)]
    pub xlie_xdowne: bool,
}

writable_register!(Int1ConfigurationRegisterA, RegisterAddress::INT1_CFG_A);

/// [`INT1_SRC_A`](RegisterAddress::INT1_SRC_A) (31h)
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

readable_register!(Int1SourceRegisterA, RegisterAddress::INT1_SRC_A);

/// [`INT1_SRC_A`](RegisterAddress::INT1_THS_A) (32h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Int1ThresholdRegisterA {
    #[bits(1, default = false)]
    zero: bool,

    /// Interrupt 1 threshold.
    #[bits(7, access = RW, default = 0)]
    pub threshold: u8,
}

writable_register!(Int1ThresholdRegisterA, RegisterAddress::INT1_THS_A);

/// [`INT1_DURATION_A`](RegisterAddress::INT1_DURATION_A) (33h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Int1DurationRegisterA {
    #[bits(1, default = false)]
    zero: bool,

    /// The minimum duration of the Interrupt 1 event to be recognized. Duration
    /// steps and maximum values depend on the ODR chosen.
    #[bits(7, access = RW, default = 0)]
    pub duration: u8,
}

writable_register!(Int1DurationRegisterA, RegisterAddress::INT1_DURATION_A);

/// [`INT2_CFG_A`](RegisterAddress::INT2_CFG_A) (34h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Int2ConfigurationRegisterA {
    /// AND/OR combination of interrupt events.
    #[bits(1, access = RW, default = false)]
    pub aoi: bool,

    /// 6-direction detection function enabled
    #[bits(1, access = RW, default = false)]
    pub six_d: bool,

    /// Enable interrupt generation on Z high event
    ///
    /// * `false` - disable interrupt request
    /// * `true` - enable interrupt request on measured accel. value higher than preset threshold
    #[bits(1, access = RW, default = false)]
    pub zhie: bool,

    /// Enable interrupt generation on Z low event
    ///
    /// * `false` - disable interrupt request
    /// * `true` - enable interrupt request on measured accel. value lower than preset threshold
    #[bits(1, access = RW, default = false)]
    pub zlie: bool,

    /// Enable interrupt generation on Y high event
    ///
    /// * `false` - disable interrupt request
    /// * `true` - enable interrupt request on measured accel. value higher than preset threshold
    #[bits(1, access = RW, default = false)]
    pub yhie: bool,

    /// Enable interrupt generation on Y low event
    ///
    /// * `false` - disable interrupt request
    /// * `true` - enable interrupt request on measured accel. value lower than preset threshold
    #[bits(1, access = RW, default = false)]
    pub ylie: bool,

    /// Enable interrupt generation on X high event
    ///
    /// * `false` - disable interrupt request
    /// * `true` - enable interrupt request on measured accel. value higher than preset threshold
    #[bits(1, access = RW, default = false)]
    pub xhie: bool,

    /// Enable interrupt generation on X low event
    ///
    /// * `false` - disable interrupt request
    /// * `true` - enable interrupt request on measured accel. value lower than preset threshold
    #[bits(1, access = RW, default = false)]
    pub xlie: bool,
}

writable_register!(Int2ConfigurationRegisterA, RegisterAddress::INT2_CFG_A);

/// [`INT2_SRC_A`](RegisterAddress::INT2_SRC_A) (35h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Int2SourceRegisterA {
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

readable_register!(Int2SourceRegisterA, RegisterAddress::INT2_SRC_A);

/// [`INT2_SRC_A`](RegisterAddress::INT2_THS_A) (36h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Int2ThresholdRegisterA {
    #[bits(1, default = false)]
    zero: bool,

    /// Interrupt 2 threshold.
    #[bits(7, access = RW, default = 0)]
    pub threshold: u8,
}

writable_register!(Int2ThresholdRegisterA, RegisterAddress::INT2_THS_A);

/// [`INT2_DURATION_A`](RegisterAddress::INT2_DURATION_A) (37h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Int2DurationRegisterA {
    #[bits(1, default = false)]
    zero: bool,

    /// The minimum duration of the Interrupt 1 event to be recognized. Duration
    /// steps and maximum values depend on the ODR chosen.
    #[bits(7, access = RW, default = 0)]
    pub duration: u8,
}

writable_register!(Int2DurationRegisterA, RegisterAddress::INT2_DURATION_A);

/// [`CLICK_CFG_A`](RegisterAddress::CLICK_CFG_A) (38h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ClickConfigurationRegisterA {
    #[bits(2)]
    __: u8,

    /// Enable interrupt double-click in Z-axis.
    ///
    /// * `false` - disable interrupt request
    /// * `true` - enable interrupt request on measured accel. value higher than preset threshold
    #[bits(1, access = RW)]
    pub zd: bool,

    /// Enable interrupt single-click in Z-axis.
    ///
    /// * `false` - disable interrupt request
    /// * `true` - enable interrupt request on measured accel. value higher than preset threshold
    #[bits(1, access = RW)]
    pub zs: bool,

    /// Enable interrupt double-click in Y-axis.
    ///
    /// * `false` - disable interrupt request
    /// * `true` - enable interrupt request on measured accel. value higher than preset threshold
    #[bits(1, access = RW)]
    pub yd: bool,

    /// Enable interrupt single-click in Y-axis.
    ///
    /// * `false` - disable interrupt request
    /// * `true` - enable interrupt request on measured accel. value higher than preset threshold
    #[bits(1, access = RW)]
    pub ys: bool,

    /// Enable interrupt double-click in X-axis.
    ///
    /// * `false` - disable interrupt request
    /// * `true` - enable interrupt request on measured accel. value higher than preset threshold
    #[bits(1, access = RW)]
    pub xd: bool,

    /// Enable interrupt single-click in X-axis.
    ///
    /// * `false` - disable interrupt request
    /// * `true` - enable interrupt request on measured accel. value higher than preset threshold
    #[bits(1, access = RW)]
    pub xs: bool,
}

writable_register!(ClickConfigurationRegisterA, RegisterAddress::CLICK_CFG_A);

/// [`CLICK_SRC_A`](RegisterAddress::CLICK_SRC_A) (39h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ClickSourceRegisterA {
    #[bits(1)]
    __: bool,

    /// Interrupt active.
    #[bits(1, access = RO)]
    pub ia: bool,

    /// Double-click enable.
    ///
    /// * `false` - double-click detection disable
    /// * `true` - double-click detection enable
    #[bits(1, access = RO)]
    pub dclick: bool,

    /// Single-click enable.
    ///
    /// * `false` - single-click detection disable
    /// * `true` - signle-click detection enable
    #[bits(1, access = RO)]
    pub sclick: bool,

    /// Click sign.
    #[bits(1, access = RO)]
    pub sign_negative: bool,

    /// Z-click detection
    ///
    /// * `false` - no interrupt
    /// * `true` - Z high event has occurred
    #[bits(1, access = RO)]
    pub z: bool,

    /// Y-click detection
    ///
    /// * `false` - no interrupt
    /// * `true` - Y high event has occurred
    #[bits(1, access = RO)]
    pub y: bool,

    /// X-click detection
    ///
    /// * `false` - no interrupt
    /// * `true` - X high event has occurred
    #[bits(1, access = RO)]
    pub x: bool,
}

readable_register!(ClickSourceRegisterA, RegisterAddress::CLICK_SRC_A);

/// [`CLICK_THS_A`](RegisterAddress::CLICK_THS_A) (3Ah)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ClickThresholdRegisterA {
    #[bits(1, default = false)]
    zero: bool,

    /// Click threshold.
    ///
    /// 1 LSB = full-scale / 128.
    /// Ths6 through Ths0 define the threshold which is used by the system to start the
    /// click-detection procedure. The threshold value is expressed over 7 bits
    /// as an unsigned number.
    #[bits(7, access = RW, default = 0)]
    pub threshold: u8,
}

writable_register!(ClickThresholdRegisterA, RegisterAddress::CLICK_THS_A);

/// [`TIME_LIMIT_A`](RegisterAddress::TIME_LIMIT_A) (3Bh)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ClickTimeLimitRegisterA {
    #[bits(1, default = false)]
    zero: bool,

    /// Click time limit.
    ///
    /// 1 LSB = 1/ODR. TLI6 through TLI0 define the maximum time interval that can elapse
    /// between the start of the click-detection procedure (the acceleration on the selected channel
    /// exceeds the programmed threshold) and when the acceleration falls below the threshold.
    #[bits(7, access = RW, default = 0)]
    pub time_limit: u8,
}

writable_register!(ClickTimeLimitRegisterA, RegisterAddress::TIME_LIMIT_A);

/// [`TIME_LATENCY_A`](RegisterAddress::TIME_LATENCY_A) (3Ch)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ClickTimeLatencyRegisterA {
    /// Double-click time latency.
    ///
    /// 1 LSB = 1/ODR. TLA7 through TLA0 define the time interval that starts after the first click
    /// detection where the click-detection procedure is disabled, in cases where the device is
    /// configured for double-click detection.
    #[bits(8, access = RW, default = 0)]
    pub time_latency: u8,
}

writable_register!(ClickTimeLatencyRegisterA, RegisterAddress::TIME_LATENCY_A);

/// [`TIME_WINDOW_A`](RegisterAddress::TIME_WINDOW_A) (3Dh)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ClickTimeWindowRegisterA {
    /// Double-click time window.
    ///
    /// 1 LSB = 1/ODR. TW7 through TW0 define the maximum interval of time that can elapse
    /// after the end of the latency interval in which the click detection procedure can start, in cases
    /// where the device is configured for double-click detection.
    #[bits(8, access = RW, default = 0)]
    pub time_window: u8,
}

writable_register!(ClickTimeWindowRegisterA, RegisterAddress::TIME_WINDOW_A);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::unusual_byte_groupings)]
    fn status_register_1a() {
        let reg = ControlRegister1A::new()
            .with_output_data_rate(AccelOdr::Hz400)
            .with_low_power_enable(false)
            .with_x_enable(true)
            .with_y_enable(true)
            .with_z_enable(true);

        assert_eq!(reg.into_bits(), 0b0111_0_111);
    }
}
