//! Types used in the Gyroscope registers.

/// Gyroscope Output Data Rate
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum OutputDataRate {
    /// 95 Hz (`0b00`)
    Hz95 = 0b00,
    /// 190 Hz(`0b01`)
    Hz190 = 0b01,
    /// 380 Hz(`0b10`)
    Hz380 = 0b10,
    /// 760 Hz(`0b11`)
    Hz760 = 0b11,
}

impl OutputDataRate {
    /// Converts the value into an `u8`.
    pub const fn into_bits(self) -> u8 {
        self as u8
    }

    pub(crate) const fn from_bits(value: u8) -> Self {
        match value {
            0b00 => OutputDataRate::Hz95,
            0b01 => OutputDataRate::Hz190,
            0b10 => OutputDataRate::Hz380,
            0b11 => OutputDataRate::Hz760,
            _ => unreachable!(),
        }
    }
}

/// Bandwidth
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum Bandwidth {
    /// * 12.5 at 95 Hz ([`OutputDataRate::Hz95`])
    /// * 12.5 at 190 Hz ([`OutputDataRate::Hz190`])
    /// * 20 at 380 Hz ([`OutputDataRate::Hz380`])
    /// * 30 at 760 Hz ([`OutputDataRate::Hz760`])
    Bw00 = 0b00,
    /// * 25 at 95 Hz ([`OutputDataRate::Hz95`])
    /// * 25 at 190 Hz ([`OutputDataRate::Hz190`])
    /// * 25 at 380 Hz ([`OutputDataRate::Hz380`])
    /// * 35 at 760 Hz ([`OutputDataRate::Hz760`])
    Bw01 = 0b01,
    /// * 25 at 95 Hz ([`OutputDataRate::Hz95`])
    /// * 50 at 190 Hz ([`OutputDataRate::Hz190`])
    /// * 50 at 380 Hz ([`OutputDataRate::Hz380`])
    /// * 50 at 760 Hz ([`OutputDataRate::Hz760`])
    Bw10 = 0b10,
    /// * 25 at 95 Hz ([`OutputDataRate::Hz95`])
    /// * 70 at 190 Hz ([`OutputDataRate::Hz190`])
    /// * 100 at 380 Hz ([`OutputDataRate::Hz380`])
    /// * 100 at 760 Hz ([`OutputDataRate::Hz760`])
    Bw11 = 0b11,
}

impl Bandwidth {
    /// Converts the value into an `u8`.
    pub const fn into_bits(self) -> u8 {
        self as u8
    }

    pub(crate) const fn from_bits(value: u8) -> Self {
        match value {
            0b00 => Bandwidth::Bw00,
            0b01 => Bandwidth::Bw01,
            0b10 => Bandwidth::Bw10,
            0b11 => Bandwidth::Bw11,
            _ => unreachable!(),
        }
    }
}

/// High-pass filter mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum HighpassFilterMode {
    /// Normal mode (reset reading `HP_RESET_FILTER`)
    NormalModeResetFilter = 0b00,
    /// Reference signal for filtering
    ReferenceSignal = 0b01,
    /// Normal mode
    NormalMode = 0b10,
    /// Autoreset on interrupt event
    AutoresetOnInterrupt = 0b11,
}

impl HighpassFilterMode {
    /// Converts the value into an `u8`.
    pub const fn into_bits(self) -> u8 {
        self as u8
    }

    pub(crate) const fn from_bits(value: u8) -> Self {
        match value {
            0b00 => HighpassFilterMode::NormalModeResetFilter,
            0b01 => HighpassFilterMode::ReferenceSignal,
            0b10 => HighpassFilterMode::NormalMode,
            0b11 => HighpassFilterMode::AutoresetOnInterrupt,
            _ => unreachable!(),
        }
    }
}

/// Gyroscope sensitivity (full scale selection).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum Sensitivity {
    /// 250 dps
    G250 = 0b00,
    /// 500 dps
    G500 = 0b01,
    /// 2000 dps
    G2000 = 0b10,
    /// 2000 dps
    G2000_11 = 0b11,
}

impl Sensitivity {
    /// Converts the value into an `u8`.
    pub const fn into_bits(self) -> u8 {
        self as u8
    }

    pub(crate) const fn from_bits(value: u8) -> Self {
        match value {
            0b00 => Sensitivity::G250,
            0b01 => Sensitivity::G500,
            0b10 => Sensitivity::G2000,
            0b11 => Sensitivity::G2000_11,
            _ => unreachable!(),
        }
    }
}

/// FIFO mode configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum FifoMode {
    /// Bypass mode (`0b000`)
    ///
    /// Bypass the FIFO and store data directly in the output registers.
    Bypass = 0b000,
    /// FIFO mode (`0b001`)
    #[allow(clippy::upper_case_acronyms)]
    FIFO = 0b001,
    /// Stream mode (`0b010`)
    Stream = 0b010,
    /// Stream-to-FIFO mode (`0b011`)
    StreamToFifo = 0b011,
    /// Bypass-to-Stream mode (`0b100`)
    BypassToStream = 0b100,
}

impl FifoMode {
    /// Converts the value into an `u8`.
    pub const fn into_bits(self) -> u8 {
        self as u8
    }

    pub(crate) const fn from_bits(value: u8) -> Self {
        match value {
            0b000 => FifoMode::Bypass,
            0b001 => FifoMode::FIFO,
            0b010 => FifoMode::Stream,
            0b011 => FifoMode::StreamToFifo,
            0b100 => FifoMode::BypassToStream,
            _ => unreachable!(),
        }
    }
}
