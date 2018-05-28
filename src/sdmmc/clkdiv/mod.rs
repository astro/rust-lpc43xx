#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLKDIV {
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
pub struct CLK_DIVIDER0R {
    bits: u8,
}
impl CLK_DIVIDER0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CLK_DIVIDER1R {
    bits: u8,
}
impl CLK_DIVIDER1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CLK_DIVIDER2R {
    bits: u8,
}
impl CLK_DIVIDER2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CLK_DIVIDER3R {
    bits: u8,
}
impl CLK_DIVIDER3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CLK_DIVIDER0W<'a> {
    w: &'a mut W,
}
impl<'a> _CLK_DIVIDER0W<'a> {
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
pub struct _CLK_DIVIDER1W<'a> {
    w: &'a mut W,
}
impl<'a> _CLK_DIVIDER1W<'a> {
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
pub struct _CLK_DIVIDER2W<'a> {
    w: &'a mut W,
}
impl<'a> _CLK_DIVIDER2W<'a> {
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
pub struct _CLK_DIVIDER3W<'a> {
    w: &'a mut W,
}
impl<'a> _CLK_DIVIDER3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:7 - Clock divider-0 value. Clock division is 2*n. For example, value of 0 means divide by 2*0 = 0 (no division, bypass), value of 1 means divide by 2*1 = 2, value of ff means divide by 2*255 = 510, and so on."]
    #[inline]
    pub fn clk_divider0(&self) -> CLK_DIVIDER0R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CLK_DIVIDER0R { bits }
    }
    #[doc = "Bits 8:15 - Clock divider-1 value. Clock division is 2*n. For example, value of 0 means divide by 2*0 = 0 (no division, bypass), value of 1 means divide by 2*1 = 2, value of ff means divide by 2*255 = 510, and so on. In MMC-Ver3.3-only mode, bits not implemented because only one clock divider is supported."]
    #[inline]
    pub fn clk_divider1(&self) -> CLK_DIVIDER1R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CLK_DIVIDER1R { bits }
    }
    #[doc = "Bits 16:23 - Clock divider-2 value. Clock division is 2*n. For example, value of 0 means divide by 2*0 = 0 (no division, bypass), value of 1 means divide by 2*1 = 2, value of ff means divide by 2*255 = 510, and so on. In MMC-Ver3.3-only mode, bits not implemented because only one clock divider is supported."]
    #[inline]
    pub fn clk_divider2(&self) -> CLK_DIVIDER2R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CLK_DIVIDER2R { bits }
    }
    #[doc = "Bits 24:31 - Clock divider-3 value. Clock division is 2*n. For example, value of 0 means divide by 2*0 = 0 (no division, bypass), a value of 1 means divide by 2*1 = 2, a value of ff means divide by 2*255 = 510, and so on. In MMC-Ver3.3-only mode, bits not implemented because only one clock divider is supported. divide by 2*0 = 0 (no division, bypass), value of 1 means divide by 2*1 = 2, value of ff means divide by 2*255 = 510, and so on. In MMC-Ver3.3-only mode, bits not implemented because only one clock divider is supported."]
    #[inline]
    pub fn clk_divider3(&self) -> CLK_DIVIDER3R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CLK_DIVIDER3R { bits }
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
    #[doc = "Bits 0:7 - Clock divider-0 value. Clock division is 2*n. For example, value of 0 means divide by 2*0 = 0 (no division, bypass), value of 1 means divide by 2*1 = 2, value of ff means divide by 2*255 = 510, and so on."]
    #[inline]
    pub fn clk_divider0(&mut self) -> _CLK_DIVIDER0W {
        _CLK_DIVIDER0W { w: self }
    }
    #[doc = "Bits 8:15 - Clock divider-1 value. Clock division is 2*n. For example, value of 0 means divide by 2*0 = 0 (no division, bypass), value of 1 means divide by 2*1 = 2, value of ff means divide by 2*255 = 510, and so on. In MMC-Ver3.3-only mode, bits not implemented because only one clock divider is supported."]
    #[inline]
    pub fn clk_divider1(&mut self) -> _CLK_DIVIDER1W {
        _CLK_DIVIDER1W { w: self }
    }
    #[doc = "Bits 16:23 - Clock divider-2 value. Clock division is 2*n. For example, value of 0 means divide by 2*0 = 0 (no division, bypass), value of 1 means divide by 2*1 = 2, value of ff means divide by 2*255 = 510, and so on. In MMC-Ver3.3-only mode, bits not implemented because only one clock divider is supported."]
    #[inline]
    pub fn clk_divider2(&mut self) -> _CLK_DIVIDER2W {
        _CLK_DIVIDER2W { w: self }
    }
    #[doc = "Bits 24:31 - Clock divider-3 value. Clock division is 2*n. For example, value of 0 means divide by 2*0 = 0 (no division, bypass), a value of 1 means divide by 2*1 = 2, a value of ff means divide by 2*255 = 510, and so on. In MMC-Ver3.3-only mode, bits not implemented because only one clock divider is supported. divide by 2*0 = 0 (no division, bypass), value of 1 means divide by 2*1 = 2, value of ff means divide by 2*255 = 510, and so on. In MMC-Ver3.3-only mode, bits not implemented because only one clock divider is supported."]
    #[inline]
    pub fn clk_divider3(&mut self) -> _CLK_DIVIDER3W {
        _CLK_DIVIDER3W { w: self }
    }
}
