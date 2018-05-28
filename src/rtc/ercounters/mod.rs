#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ERCOUNTERS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct COUNTER0R {
    bits: u8,
}
impl COUNTER0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct COUNTER1R {
    bits: u8,
}
impl COUNTER1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct COUNTER2R {
    bits: u8,
}
impl COUNTER2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Value of the counter for Event 0. If the counter reaches full count (the value 7), it remains there if additional events occur. This counter is cleared when the corresponding EVx bit in the ERSTATUS register is cleared by software."]
    #[inline]
    pub fn counter0(&self) -> COUNTER0R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        COUNTER0R { bits }
    }
    #[doc = "Bits 8:10 - Value of the counter for event 1. See description for COUNTER0."]
    #[inline]
    pub fn counter1(&self) -> COUNTER1R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        COUNTER1R { bits }
    }
    #[doc = "Bits 16:18 - Value of the counter for event 2. See description for COUNTER0."]
    #[inline]
    pub fn counter2(&self) -> COUNTER2R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        COUNTER2R { bits }
    }
}
