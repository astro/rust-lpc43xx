#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USB1FLADJ {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct FLTVR {
    bits: u8,
}
impl FLTVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _FLTVW<'a> {
    w: &'a mut W,
}
impl<'a> _FLTVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:5 - Frame length timing value The frame length is given in the number of high-speed bit times in decimal format. Each decimal value change to this register corresponds to 16 high-speed bit times. The SOF cycle time (number of SOF counter clock periods to generate a SOF micro-frame length) is equal to 59488 + value in this field. The default value is decimal 32 (0x20), which results in a SOF cycle time of 60000. 0x00 = 59488 (= 59488 + 0 x 16) 0x01 = 59504 (= 59488 + 1 x 16) 0x02 = 59520 (= 59488 + 2 x 16) ... 0x1F = 59984 (= 59488 + 31 x 16) 0x20 = 60000 (= 59488 + 32 x 16) ... 0x3E = 60480 (= 59488 + 62 x 16) 0x3F = 60496 (= 59488 + 63 x 16)"]
    #[inline]
    pub fn fltv(&self) -> FLTVR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FLTVR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 32 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:5 - Frame length timing value The frame length is given in the number of high-speed bit times in decimal format. Each decimal value change to this register corresponds to 16 high-speed bit times. The SOF cycle time (number of SOF counter clock periods to generate a SOF micro-frame length) is equal to 59488 + value in this field. The default value is decimal 32 (0x20), which results in a SOF cycle time of 60000. 0x00 = 59488 (= 59488 + 0 x 16) 0x01 = 59504 (= 59488 + 1 x 16) 0x02 = 59520 (= 59488 + 2 x 16) ... 0x1F = 59984 (= 59488 + 31 x 16) 0x20 = 60000 (= 59488 + 32 x 16) ... 0x3E = 60480 (= 59488 + 62 x 16) 0x3F = 60496 (= 59488 + 63 x 16)"]
    #[inline]
    pub fn fltv(&mut self) -> _FLTVW {
        _FLTVW { w: self }
    }
}
