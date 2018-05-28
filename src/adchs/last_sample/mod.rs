#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::LAST_SAMPLE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct DONER {
    bits: bool,
}
impl DONER {
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
#[doc = r" Value of the field"]
pub struct OVERRUNR {
    bits: bool,
}
impl OVERRUNR {
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
#[doc = r" Value of the field"]
pub struct THCMP_RANGER {
    bits: u8,
}
impl THCMP_RANGER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct THCMP_CROSSR {
    bits: u8,
}
impl THCMP_CROSSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SAMPLER {
    bits: u16,
}
impl SAMPLER {
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
    #[doc = "Bit 0 - This bit is set to 1 when an A/D conversion on this channel completes. This bit is cleared whenever this register is read."]
    #[inline]
    pub fn done(&self) -> DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DONER { bits }
    }
    #[doc = "Bit 1 - This bit will be set to a 1 if a new conversion on this channel completes and overwrites the previous contents of the RESULT field before it has been read - i.e. while the DONE bit is set. This bit is cleared, along with the DONE bit, whenever this register is read. This bit (in any of the registers) will cause an overrun interrupt request to be asserted if the overrun interrupt is enabled."]
    #[inline]
    pub fn overrun(&self) -> OVERRUNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OVERRUNR { bits }
    }
    #[doc = "Bits 2:3 - Threshold Range Comparison result 00: In Range 01: Below Range 10: Above Range 11: Reserved"]
    #[inline]
    pub fn thcmp_range(&self) -> THCMP_RANGER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        THCMP_RANGER { bits }
    }
    #[doc = "Bits 4:5 - Threshold Crossing Comparison result 00: No Threshold Crossing detected 01: Downward Threshold Crossing detected 10: Upward Threshold Crossing detected 11: Reserved"]
    #[inline]
    pub fn thcmp_cross(&self) -> THCMP_CROSSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        THCMP_CROSSR { bits }
    }
    #[doc = "Bits 6:17 - 12-Bit value of last converted sample for this channel"]
    #[inline]
    pub fn sample(&self) -> SAMPLER {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SAMPLER { bits }
    }
}
