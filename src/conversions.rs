use crate::{OutXHigh, OutXLow, OutYHigh, OutYLow, OutZHigh, OutZLow};
use core::ops::Add;

impl Add<OutXHigh> for OutXLow {
    type Output = i16;

    fn add(self, hi: OutXHigh) -> Self::Output {
        (hi.bits() as i16) << 8 | (self.bits() as i16)
    }
}

impl Add<OutXLow> for OutXHigh {
    type Output = i16;

    fn add(self, lo: OutXLow) -> Self::Output {
        lo.add(self)
    }
}

impl Add<OutYHigh> for OutYLow {
    type Output = i16;

    fn add(self, hi: OutYHigh) -> Self::Output {
        (hi.bits() as i16) << 8 | (self.bits() as i16)
    }
}

impl Add<OutYLow> for OutYHigh {
    type Output = i16;

    fn add(self, lo: OutYLow) -> Self::Output {
        lo.add(self)
    }
}

impl Add<OutZHigh> for OutZLow {
    type Output = i16;

    fn add(self, hi: OutZHigh) -> Self::Output {
        (hi.bits() as i16) << 8 | (self.bits() as i16)
    }
}

impl Add<OutZLow> for OutZHigh {
    type Output = i16;

    fn add(self, lo: OutZLow) -> Self::Output {
        lo.add(self)
    }
}
