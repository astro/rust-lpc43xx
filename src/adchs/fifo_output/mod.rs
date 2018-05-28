#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FIFO_OUTPUT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
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
#[doc = r" Value of the field"]
pub struct CHAN_IDR {
    bits: u8,
}
impl CHAN_IDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EMPTYR {
    bits: bool,
}
impl EMPTYR {
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
pub struct SAMPLE2R {
    bits: u16,
}
impl SAMPLE2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CHAN_ID2R {
    bits: u8,
}
impl CHAN_ID2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EMPTY2R {
    bits: bool,
}
impl EMPTY2R {
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
    #[doc = "Bits 0:11 - Value of first converted sample"]
    #[inline]
    pub fn sample(&self) -> SAMPLER {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SAMPLER { bits }
    }
    #[doc = "Bits 12:14 - Channel number of first converted sample: 000: channel _0 or CHANNEL_ID_EN =0 001: channel _1 010: channel _2 011: channel _3 100: channel _4 101: channel _5 110: reserved 111: recovery_ error"]
    #[inline]
    pub fn chan_id(&self) -> CHAN_IDR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CHAN_IDR { bits }
    }
    #[doc = "Bit 15 - 0: FIFO not empty 1: FIFO empty"]
    #[inline]
    pub fn empty(&self) -> EMPTYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EMPTYR { bits }
    }
    #[doc = "Bits 16:27 - Value of second converted sample. This field is only valid if PACKED_READ is set else it is 0x0"]
    #[inline]
    pub fn sample2(&self) -> SAMPLE2R {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SAMPLE2R { bits }
    }
    #[doc = "Bits 28:30 - Channel number of second converted sample This field is only valid if CHANNEL_ID_EN and PACKED_READ are set else it is 0x0"]
    #[inline]
    pub fn chan_id2(&self) -> CHAN_ID2R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CHAN_ID2R { bits }
    }
    #[doc = "Bit 31 - 0: FIFO not empty 1: FIFO empty and PACKED_READ is set"]
    #[inline]
    pub fn empty2(&self) -> EMPTY2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EMPTY2R { bits }
    }
}
