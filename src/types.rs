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
    /// Selects the narrowest filter bandwidth.
    ///
    /// * 12.5 at 95 Hz ([`OutputDataRate::Hz95`])
    /// * 12.5 at 190 Hz ([`OutputDataRate::Hz190`])
    /// * 20 at 380 Hz ([`OutputDataRate::Hz380`])
    /// * 30 at 760 Hz ([`OutputDataRate::Hz760`])
    ///
    /// Rate noise density: 0.03 dps/√Hz
    Narrowest = 0b00,
    /// Selects a narrow filter bandwidth.
    ///
    /// * 25 at 95 Hz ([`OutputDataRate::Hz95`])
    /// * 25 at 190 Hz ([`OutputDataRate::Hz190`])
    /// * 25 at 380 Hz ([`OutputDataRate::Hz380`])
    /// * 35 at 760 Hz ([`OutputDataRate::Hz760`])
    ///
    /// Rate noise density: 0.03 dps/√Hz
    Narrow = 0b01,
    /// Selects a medium filter bandwidth.
    ///
    /// * 25 at 95 Hz ([`OutputDataRate::Hz95`])
    /// * 50 at 190 Hz ([`OutputDataRate::Hz190`])
    /// * 50 at 380 Hz ([`OutputDataRate::Hz380`])
    /// * 50 at 760 Hz ([`OutputDataRate::Hz760`])
    ///
    /// Rate noise density: 0.03 dps/√Hz
    Medium = 0b10,
    /// Selects a wide filter bandwidth.
    ///
    /// * 25 at 95 Hz ([`OutputDataRate::Hz95`])
    /// * 70 at 190 Hz ([`OutputDataRate::Hz190`])
    /// * 100 at 380 Hz ([`OutputDataRate::Hz380`])
    /// * 100 at 760 Hz ([`OutputDataRate::Hz760`])
    ///
    /// Rate noise density: 0.03 dps/√Hz
    Wide = 0b11,
}

impl Bandwidth {
    /// Determines the bandwidth in Hertz at the given output data rate.
    #[must_use]
    pub fn hz_at(&self, odr: OutputDataRate) -> f32 {
        match self {
            Bandwidth::Narrowest => match odr {
                OutputDataRate::Hz95 => 12.5,
                OutputDataRate::Hz190 => 12.5,
                OutputDataRate::Hz380 => 20.0,
                OutputDataRate::Hz760 => 30.0,
            },
            Bandwidth::Narrow => match odr {
                OutputDataRate::Hz95 => 25.0,
                OutputDataRate::Hz190 => 25.0,
                OutputDataRate::Hz380 => 25.0,
                OutputDataRate::Hz760 => 35.0,
            },
            Bandwidth::Medium => match odr {
                OutputDataRate::Hz95 => 25.0,
                OutputDataRate::Hz190 => 50.0,
                OutputDataRate::Hz380 => 50.0,
                OutputDataRate::Hz760 => 50.0,
            },
            Bandwidth::Wide => match odr {
                OutputDataRate::Hz95 => 25.0,
                OutputDataRate::Hz190 => 70.0,
                OutputDataRate::Hz380 => 100.0,
                OutputDataRate::Hz760 => 100.0,
            },
        }
    }

    /// Determines the square root of the bandwidth at the given output data rate.
    ///
    /// This factor plays a role in determining the rate noise density.
    #[must_use]
    pub fn sqrt_hz_at(&self, odr: OutputDataRate) -> f32 {
        #[allow(clippy::excessive_precision)]
        match self {
            Bandwidth::Narrowest => match odr {
                OutputDataRate::Hz95 => 3.5355339059327378,  // √(12.5 Hz)
                OutputDataRate::Hz190 => 3.5355339059327378, // √(12.5 Hz)
                OutputDataRate::Hz380 => 4.47213595499958,   // √(20.0 Hz)
                OutputDataRate::Hz760 => 5.477225575051661,  // √(30.0 Hz)
            },
            Bandwidth::Narrow => match odr {
                OutputDataRate::Hz95 => 5.0,                // √(25.0 Hz)
                OutputDataRate::Hz190 => 5.0,               // √(25.0 Hz)
                OutputDataRate::Hz380 => 25.0,              // √(25.0 Hz)
                OutputDataRate::Hz760 => 5.916079783099616, // √(35.0 Hz)
            },
            Bandwidth::Medium => match odr {
                OutputDataRate::Hz95 => 5.0,                 // √(25.0 Hz)
                OutputDataRate::Hz190 => 7.0710678118654755, // √(50.0 Hz)
                OutputDataRate::Hz380 => 7.0710678118654755, // √(50.0 Hz)
                OutputDataRate::Hz760 => 7.0710678118654755, // √(50.0 Hz)
            },
            Bandwidth::Wide => match odr {
                OutputDataRate::Hz95 => 5.0,                // √(25.0 Hz)
                OutputDataRate::Hz190 => 8.366600265340756, // √(70.0 Hz)
                OutputDataRate::Hz380 => 10.0,              // √(100.0 Hz)
                OutputDataRate::Hz760 => 10.0,              // √(100.0 Hz)
            },
        }
    }

    /// Converts the value into an `u8`.
    pub const fn into_bits(self) -> u8 {
        self as u8
    }

    pub(crate) const fn from_bits(value: u8) -> Self {
        match value {
            0b00 => Bandwidth::Narrowest,
            0b01 => Bandwidth::Narrow,
            0b10 => Bandwidth::Medium,
            0b11 => Bandwidth::Wide,
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
    /// # 250 dps
    ///
    /// This is usually the most appropriate setting for human interaction applications.
    /// It provides a good balance between sensitivity and range, capturing fine movements with
    /// sufficient detail. Most human motions, including head movements and hand gestures,
    /// fall well within this range.
    ///
    /// ## Resolution and error
    /// 8.75 mdps/digit; ±10 dps at zero-rate level
    D250 = 0b00,

    /// # 500 dps
    ///
    /// This setting is used if there are slightly faster motions involved. It offers a wider
    /// range at the expense of some resolution but still captures typical human movements effectively.
    ///
    /// ## Resolution and error
    /// 17.50 mdps/digit; ±15 dps at zero-rate level
    D500 = 0b01,

    /// # 2000 dps
    ///
    /// This is generally too high for most human interaction applications. It is suitable for
    /// high-speed rotational measurements, such as those found in fast-spinning objects or certain
    /// industrial applications.
    ///
    /// ## Resolution and error
    /// 70 mdps/digit; ±75 dps at zero-rate level
    D2000 = 0b10,

    /// Same as [`Sensitivity::D2000`]
    D2000_11 = 0b11,
}

impl Sensitivity {
    /// Converts the value into an `u8`.
    pub const fn into_bits(self) -> u8 {
        self as u8
    }

    pub(crate) const fn from_bits(value: u8) -> Self {
        match value {
            0b00 => Sensitivity::D250,
            0b01 => Sensitivity::D500,
            0b10 => Sensitivity::D2000,
            0b11 => Sensitivity::D2000_11,
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
