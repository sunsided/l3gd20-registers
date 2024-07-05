//! Types used in the magnetometer registers.

/// Magnetometer Output Data Rate
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum MagOdr {
    /// 0.75 Hz (`0b000`)
    Hz0_75 = 0b000,
    /// 1.5 Hz(`0b001`)
    Hz1_5 = 0b001,
    /// 3 Hz(`0b010`)
    Hz3 = 0b010,
    /// 7.5 Hz(`0b011`)
    Hz7_5 = 0b011,
    /// 15 Hz(`0b100`)
    Hz15 = 0b100,
    /// 30 Hz(`0b101`)
    Hz30 = 0b101,
    /// 75 Hz(`0b110`)
    Hz75 = 0b110,
    /// 220 Hz(`0b111`)
    Hz220 = 0b111,
}

impl MagOdr {
    /// Converts the value into an `u8`.
    pub const fn into_bits(self) -> u8 {
        self as u8
    }

    pub(crate) const fn from_bits(value: u8) -> Self {
        match value {
            0b000 => MagOdr::Hz0_75,
            0b001 => MagOdr::Hz1_5,
            0b010 => MagOdr::Hz3,
            0b011 => MagOdr::Hz7_5,
            0b100 => MagOdr::Hz15,
            0b101 => MagOdr::Hz30,
            0b110 => MagOdr::Hz75,
            0b111 => MagOdr::Hz220,
            _ => unreachable!(),
        }
    }
}

/// Magnetometer gain configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum MagGain {
    /// Sensor input field range ±1.3 Gauss.
    ///
    /// Gain X, Y and Z: 1100 LSB/Gauss
    /// Gain Z: 980 LSB/Gauss
    Gauss1_3 = 0b001,
    /// Sensor input field range ±1.9 Gauss.
    ///
    /// Gain X, Y and Z: 855 LSB/Gauss
    /// Gain Z: 760 LSB/Gauss
    Gauss1_9 = 0b010,
    /// Sensor input field range ±2.5 Gauss.
    ///
    /// Gain X, Y and Z: 670 LSB/Gauss
    /// Gain Z: 600 LSB/Gauss
    Gauss2_5 = 0b011,
    /// Sensor input field range ±4.0 Gauss.
    ///
    /// Gain X, Y and Z: 450 LSB/Gauss
    /// Gain Z: 400 LSB/Gauss
    Gauss4_0 = 0b100,
    /// Sensor input field range ±4.7 Gauss.
    ///
    /// Gain X, Y and Z: 400 LSB/Gauss
    /// Gain Z: 355 LSB/Gauss
    Gauss4_7 = 0b101,
    /// Sensor input field range ±5.6 Gauss.
    ///
    /// Gain X, Y and Z: 330 LSB/Gauss
    /// Gain Z: 295 LSB/Gauss
    Gauss5_6 = 0b110,
    /// Sensor input field range ±8.1 Gauss.
    ///
    /// Gain X, Y and Z: 230 LSB/Gauss
    /// Gain Z: 205 LSB/Gauss
    Gauss8_1 = 0b111,
}

impl MagGain {
    /// Converts the value into an `u8`.
    pub const fn into_bits(self) -> u8 {
        self as u8
    }

    pub(crate) const fn from_bits(value: u8) -> Self {
        match value {
            0b001 => MagGain::Gauss1_3,
            0b010 => MagGain::Gauss1_9,
            0b011 => MagGain::Gauss2_5,
            0b100 => MagGain::Gauss4_0,
            0b101 => MagGain::Gauss4_7,
            0b110 => MagGain::Gauss5_6,
            0b111 => MagGain::Gauss8_1,
            _ => unreachable!(),
        }
    }
}
