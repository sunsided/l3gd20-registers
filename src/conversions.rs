use crate::{OutXHigh, OutXLow, OutYHigh, OutYLow, OutZHigh, OutZLow};
use core::ops::Add;

/// Combines the low and high byte of an axis into its two's complement value.
///
/// Assumes the default data order, i.e. [`ControlRegister4::big_endian`](crate::ControlRegister4::big_endian)
/// cleared.
macro_rules! combine_bytes {
    ($low:ident, $high:ident) => {
        impl Add<$high> for $low {
            type Output = i16;

            fn add(self, hi: $high) -> Self::Output {
                i16::from_le_bytes([self.bits(), hi.bits()])
            }
        }

        impl Add<$low> for $high {
            type Output = i16;

            fn add(self, lo: $low) -> Self::Output {
                lo.add(self)
            }
        }
    };
}

combine_bytes!(OutXLow, OutXHigh);
combine_bytes!(OutYLow, OutYHigh);
combine_bytes!(OutZLow, OutZHigh);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combines_low_and_high_bytes() {
        let (lo, hi) = (OutXLow::from(0x34), OutXHigh::from(0xF2));
        assert_eq!(lo + hi, 0xF234_u16 as i16);
        assert_eq!(hi + lo, 0xF234_u16 as i16);
        assert_eq!(OutYLow::from(0xFF) + OutYHigh::from(0x7F), i16::MAX);
        assert_eq!(OutZLow::from(0x00) + OutZHigh::from(0x80), i16::MIN);
    }
}
