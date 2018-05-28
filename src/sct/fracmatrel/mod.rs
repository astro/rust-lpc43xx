#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FRACMATREL {
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
pub struct RELFRAC_LR {
    bits: u8,
}
impl RELFRAC_LR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RELFRAC_HR {
    bits: u8,
}
impl RELFRAC_HR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RELFRAC_LW<'a> {
    w: &'a mut W,
}
impl<'a> _RELFRAC_LW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RELFRAC_HW<'a> {
    w: &'a mut W,
}
impl<'a> _RELFRAC_HW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:3 - When UNIFY = 0, read or write the 4-bit value to be loaded into the FRACMATn_L register. When UNIFY = 1, read or write the lower 4 bits to be loaded into the FRACMATn register."]
    #[inline]
    pub fn relfrac_l(&self) -> RELFRAC_LR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RELFRAC_LR { bits }
    }
    #[doc = "Bits 16:19 - When UNIFY = 0, read or write the 4-bit value to be loaded into the FRACMATn_H register. When UNIFY = 1, read or write the upper 4 bits with the 4-bit value to be loaded into the FRACMATn register."]
    #[inline]
    pub fn relfrac_h(&self) -> RELFRAC_HR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RELFRAC_HR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - When UNIFY = 0, read or write the 4-bit value to be loaded into the FRACMATn_L register. When UNIFY = 1, read or write the lower 4 bits to be loaded into the FRACMATn register."]
    #[inline]
    pub fn relfrac_l(&mut self) -> _RELFRAC_LW {
        _RELFRAC_LW { w: self }
    }
    #[doc = "Bits 16:19 - When UNIFY = 0, read or write the 4-bit value to be loaded into the FRACMATn_H register. When UNIFY = 1, read or write the upper 4 bits with the 4-bit value to be loaded into the FRACMATn register."]
    #[inline]
    pub fn relfrac_h(&mut self) -> _RELFRAC_HW {
        _RELFRAC_HW { w: self }
    }
}
