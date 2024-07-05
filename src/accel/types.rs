//! Types used in the accelerometer registers.

/// Accelerometer Output Data Rate
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum AccelOdr {
    /// Power-down mode (`0b0000`)
    Disabled = 0b0000,
    /// 1 Hz (`0b0001`)
    Hz1 = 0b0001,
    /// 10 Hz (`0b0010`)
    Hz10 = 0b0010,
    /// 25 Hz (`0b0011`)
    Hz25 = 0b0011,
    /// 50 Hz (`0b0100`)
    Hz50 = 0b0100,
    /// 100 Hz (`0b0101`)
    Hz100 = 0b0101,
    /// 200 Hz (`0b0110`)
    Hz200 = 0b0110,
    /// 400 Hz (`0b0111`)
    Hz400 = 0b0111,
    /// 1.620 kHz when in Low-Power mode (`0b1000`)
    LpHz1620 = 0b1000,
    /// 5.376 kHz when in normal mode, 1.344 kHz when in normal mode (`0b1001`)
    LpHz1620NormalHz5376 = 0b1001,
}

impl AccelOdr {
    /// Converts the value into an `u8`.
    pub const fn into_bits(self) -> u8 {
        self as u8
    }

    pub(crate) const fn from_bits(value: u8) -> Self {
        match value {
            0b0000 => AccelOdr::Disabled,
            0b0001 => AccelOdr::Hz1,
            0b0010 => AccelOdr::Hz10,
            0b0011 => AccelOdr::Hz25,
            0b0100 => AccelOdr::Hz50,
            0b0101 => AccelOdr::Hz100,
            0b0110 => AccelOdr::Hz200,
            0b0111 => AccelOdr::Hz400,
            0b1000 => AccelOdr::LpHz1620,
            0b1001 => AccelOdr::LpHz1620NormalHz5376,
            _ => unreachable!(),
        }
    }
}

/// Acceleration sensitivity (full scale selection).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum Sensitivity {
    /// Range: [-2g, +2g]. Sensitivity ~ 1 g / (1 << 14) LSB (`0b00`)
    G1 = 0b00,
    /// Range: [-4g, +4g]. Sensitivity ~ 2 g / (1 << 14) LSB (`0b01`)
    G2 = 0b01,
    /// Range: [-8g, +8g]. Sensitivity ~ 4 g / (1 << 14) LSB (`0b10`)
    G4 = 0b10,
    /// Range: [-16g, +16g]. Sensitivity ~ 12 g / (1 << 14) LSB (`0b11`)
    G12 = 0b11,
}

impl Sensitivity {
    /// Converts the value into an `u8`.
    pub const fn into_bits(self) -> u8 {
        self as u8
    }

    pub(crate) const fn from_bits(value: u8) -> Self {
        match value {
            0b00 => Sensitivity::G1,
            0b01 => Sensitivity::G2,
            0b10 => Sensitivity::G4,
            0b11 => Sensitivity::G12,
            _ => unreachable!(),
        }
    }
}

/// FIFO mode configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum FifoMode {
    /// Bypass mode (`0b00`)
    ///
    /// Bypass the FIFO and store data directly in the output registers.
    Bypass = 0b00,
    /// FIFO mode (`0b01`)
    #[allow(clippy::upper_case_acronyms)]
    FIFO = 0b01,
    /// Stream mode (`0b10`)
    Stream = 0b10,
    /// Trigger mode (`0b11`)
    Trigger = 0b11,
}

impl FifoMode {
    /// Converts the value into an `u8`.
    pub const fn into_bits(self) -> u8 {
        self as u8
    }

    pub(crate) const fn from_bits(value: u8) -> Self {
        match value {
            0b00 => FifoMode::Bypass,
            0b01 => FifoMode::FIFO,
            0b10 => FifoMode::Stream,
            0b11 => FifoMode::Trigger,
            _ => unreachable!(),
        }
    }
}

/// High-Pass Filter Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum HighpassFilterMode {
    /// Normal mode (`0b00`)
    ///
    /// Reset reading `HP_RESET_FILTER`.
    NormalWithReset = 0b00,
    /// Reference signal for filtering (`0b01`)
    ReferenceSignal = 0b01,
    /// Normal mode (`0b10`)
    Normal = 0b10,
    /// Autoreset on interrupt event (`0b11`)
    AutoresetOnInterrupt = 0b11,
}

impl HighpassFilterMode {
    /// Converts the value into an `u8`.
    pub const fn into_bits(self) -> u8 {
        self as u8
    }

    pub(crate) const fn from_bits(value: u8) -> Self {
        match value {
            0b00 => HighpassFilterMode::NormalWithReset,
            0b01 => HighpassFilterMode::ReferenceSignal,
            0b10 => HighpassFilterMode::Normal,
            0b11 => HighpassFilterMode::AutoresetOnInterrupt,
            _ => unreachable!(),
        }
    }
}
