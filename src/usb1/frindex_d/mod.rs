#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FRINDEX_D {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct FRINDEX2_0R {
    bits: u8,
}
impl FRINDEX2_0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FRINDEX13_3R {
    bits: u16,
}
impl FRINDEX13_3R {
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
    #[doc = "Bits 0:2 - Current micro frame number"]
    #[inline]
    pub fn frindex2_0(&self) -> FRINDEX2_0R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FRINDEX2_0R { bits }
    }
    #[doc = "Bits 3:13 - Current frame number of the last frame transmitted"]
    #[inline]
    pub fn frindex13_3(&self) -> FRINDEX13_3R {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        FRINDEX13_3R { bits }
    }
}
