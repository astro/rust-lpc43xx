#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::INT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct INTID15_0R {
    bits: u16,
}
impl INTID15_0R {
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
    #[doc = "Bits 0:15 - 0x0000= No interrupt is pending 0x0001 to 0x0020 = Number of message object which caused the interrupt. 0x0021 to 0x7FFF = Unused 0x8000 = Status interrupt 0x8001 to 0xFFFF = Unused"]
    #[inline]
    pub fn intid15_0(&self) -> INTID15_0R {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        INTID15_0R { bits }
    }
}
