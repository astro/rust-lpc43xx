#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ERFIRSTSTAMP {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct SECR {
    bits: u8,
}
impl SECR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MINR {
    bits: u8,
}
impl MINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HOURR {
    bits: u8,
}
impl HOURR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DOYR {
    bits: u16,
}
impl DOYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:5 - Seconds value in the range of 0 to 59."]
    #[inline]
    pub fn sec(&self) -> SECR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SECR { bits }
    }
    #[doc = "Bits 6:11 - Minutes value in the range of 0 to 59."]
    #[inline]
    pub fn min(&self) -> MINR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MINR { bits }
    }
    #[doc = "Bits 12:16 - Hours value in the range of 0 to 23."]
    #[inline]
    pub fn hour(&self) -> HOURR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HOURR { bits }
    }
    #[doc = "Bits 17:25 - Day of Year value in the range of 1 to 366."]
    #[inline]
    pub fn doy(&self) -> DOYR {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DOYR { bits }
    }
}
