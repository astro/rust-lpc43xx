#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TCBCNT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct TRANS_CARD_BYTE_COUNTR {
    bits: u32,
}
impl TRANS_CARD_BYTE_COUNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Number of bytes transferred by CIU unit to card. Register should be read only after data transfer completes; during data transfer, register returns 0."]
    #[inline]
    pub fn trans_card_byte_count(&self) -> TRANS_CARD_BYTE_COUNTR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        TRANS_CARD_BYTE_COUNTR { bits }
    }
}
