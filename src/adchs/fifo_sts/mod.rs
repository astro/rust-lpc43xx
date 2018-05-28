#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FIFO_STS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct LEVELR {
    bits: u8,
}
impl LEVELR {
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
    #[doc = "Bits 0:4 - 0 = FIFO is empty 1...15 = FIFO is partially full 16 = FIFO is full"]
    #[inline]
    pub fn level(&self) -> LEVELR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LEVELR { bits }
    }
}
