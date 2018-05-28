#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MASK1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct M1R {
    bits: u32,
}
impl M1R {
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
    #[doc = "Bits 0:29 - Interrupt enable"]
    #[inline]
    pub fn m1(&self) -> M1R {
        let bits = {
            const MASK: u32 = 1073741823;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        M1R { bits }
    }
}
