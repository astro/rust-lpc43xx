#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EMCDELAYCLK {
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
pub struct CLK_DELAYR {
    bits: u16,
}
impl CLK_DELAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CLK_DELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _CLK_DELAYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
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
    #[doc = "Bits 0:15 - EMC_CLKn SDRAM clock output delay. 0x0 = no delay 0x1111 approximately 0.5 ns delay 0x2222 approximately 1.0 ns delay 0x3333 approximately 1.5 ns delay 0x4444 approximately 2.0 ns delay 0x5555 approximately 2.5 ns delay 0x6666 approximately 3.0 ns delay 0x7777 approximately 3.5 ns delay"]
    #[inline]
    pub fn clk_delay(&self) -> CLK_DELAYR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CLK_DELAYR { bits }
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
    #[doc = "Bits 0:15 - EMC_CLKn SDRAM clock output delay. 0x0 = no delay 0x1111 approximately 0.5 ns delay 0x2222 approximately 1.0 ns delay 0x3333 approximately 1.5 ns delay 0x4444 approximately 2.0 ns delay 0x5555 approximately 2.5 ns delay 0x6666 approximately 3.0 ns delay 0x7777 approximately 3.5 ns delay"]
    #[inline]
    pub fn clk_delay(&mut self) -> _CLK_DELAYW {
        _CLK_DELAYW { w: self }
    }
}
