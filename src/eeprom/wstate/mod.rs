#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WSTATE {
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
pub struct PHASE3R {
    bits: u8,
}
impl PHASE3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PHASE2R {
    bits: u8,
}
impl PHASE2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PHASE1R {
    bits: u8,
}
impl PHASE1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LCK_PARWEPR {
    bits: bool,
}
impl LCK_PARWEPR {
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
#[doc = r" Proxy"]
pub struct _PHASE3W<'a> {
    w: &'a mut W,
}
impl<'a> _PHASE3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PHASE2W<'a> {
    w: &'a mut W,
}
impl<'a> _PHASE2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PHASE1W<'a> {
    w: &'a mut W,
}
impl<'a> _PHASE1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LCK_PARWEPW<'a> {
    w: &'a mut W,
}
impl<'a> _LCK_PARWEPW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:7 - Wait states for phase 3 (minus 1 encoded). The number of system clock periods to meet a duration equal to TPHASE3."]
    #[inline]
    pub fn phase3(&self) -> PHASE3R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PHASE3R { bits }
    }
    #[doc = "Bits 8:15 - Wait states for phase 2 (minus 1 encoded). The number of system clock periods to meet a duration equal to TPHASE2."]
    #[inline]
    pub fn phase2(&self) -> PHASE2R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PHASE2R { bits }
    }
    #[doc = "Bits 16:23 - Wait states for phase 1 (minus 1 encoded). The number of system clock periods to meet a duration equal to TPHASE1."]
    #[inline]
    pub fn phase1(&self) -> PHASE1R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PHASE1R { bits }
    }
    #[doc = "Bit 31 - Lock timing parameters for write, erase and program operation 0 = WSTATE and CLKDIV registers have R/W access 1 = WSTATE and CLKDIV registers have R only access"]
    #[inline]
    pub fn lck_parwep(&self) -> LCK_PARWEPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LCK_PARWEPR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 132610 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Wait states for phase 3 (minus 1 encoded). The number of system clock periods to meet a duration equal to TPHASE3."]
    #[inline]
    pub fn phase3(&mut self) -> _PHASE3W {
        _PHASE3W { w: self }
    }
    #[doc = "Bits 8:15 - Wait states for phase 2 (minus 1 encoded). The number of system clock periods to meet a duration equal to TPHASE2."]
    #[inline]
    pub fn phase2(&mut self) -> _PHASE2W {
        _PHASE2W { w: self }
    }
    #[doc = "Bits 16:23 - Wait states for phase 1 (minus 1 encoded). The number of system clock periods to meet a duration equal to TPHASE1."]
    #[inline]
    pub fn phase1(&mut self) -> _PHASE1W {
        _PHASE1W { w: self }
    }
    #[doc = "Bit 31 - Lock timing parameters for write, erase and program operation 0 = WSTATE and CLKDIV registers have R/W access 1 = WSTATE and CLKDIV registers have R only access"]
    #[inline]
    pub fn lck_parwep(&mut self) -> _LCK_PARWEPW {
        _LCK_PARWEPW { w: self }
    }
}
