#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DMA_CURHOST_TRANS_DES {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct HTDR {
    bits: u32,
}
impl HTDR {
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
    #[doc = "Bits 0:31 - Host Transmit Descriptor Address Pointer Cleared on Reset. Pointer updated by DMA during operation."]
    #[inline]
    pub fn htd(&self) -> HTDR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        HTDR { bits }
    }
}
