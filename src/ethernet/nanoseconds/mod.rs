#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::NANOSECONDS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct TSSSR {
    bits: u32,
}
impl TSSSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PSNTR {
    bits: bool,
}
impl PSNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:30 - Time stamp sub seconds The value in this field has the sub second representation of time, with an accuracy of 0.46 nano-second. (When TSCTRLSSR in the MAC_TIMESTAMP_CTRL register is set, each bit represents 1 ns and the maximum value will be 0x3B9A_C9FF, after which it rolls-over to zero)."]
    #[inline]
    pub fn tsss(&self) -> TSSSR {
        let bits = {
            const MASK: u32 = 2147483647;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        TSSSR { bits }
    }
    #[doc = "Bit 31 - Positive or negative time This bit indicates positive or negative time value. If the bit is reset, it indicates that the time representation is positive, and if it is set, it indicates negative time value. (This bit represents the 32nd bit of the nanoseconds value when the Advance Time stamp feature is enabled)."]
    #[inline]
    pub fn psnt(&self) -> PSNTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PSNTR { bits }
    }
}
