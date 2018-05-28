#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CHIPID {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct IDR {
    bits: u32,
}
impl IDR {
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
    #[doc = "Bits 0:31 - Boundary scan ID code 0x5906 002B or 0x6906 002B = LPC4350/30/20/10 (flashless parts) 0x4906 002B = LPC4357/53 (parts with on-chip flash)"]
    #[inline]
    pub fn id(&self) -> IDR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        IDR { bits }
    }
}
