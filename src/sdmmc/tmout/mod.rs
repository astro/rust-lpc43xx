#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TMOUT {
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
pub struct RESPONSE_TIMEOUTR {
    bits: u8,
}
impl RESPONSE_TIMEOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DATA_TIMEOUTR {
    bits: u32,
}
impl DATA_TIMEOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RESPONSE_TIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _RESPONSE_TIMEOUTW<'a> {
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
pub struct _DATA_TIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_TIMEOUTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:7 - Response time-out value. Value is in number of card output clocks - cclk_out."]
    #[inline]
    pub fn response_timeout(&self) -> RESPONSE_TIMEOUTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESPONSE_TIMEOUTR { bits }
    }
    #[doc = "Bits 8:31 - Value for card Data Read time-out; same value also used for Data Starvation by Host time-out. Value is in number of card output clocks - cclk_out of selected card. Starvation by Host time-out. Value is in number of card output clocks - cclk_out of selected card."]
    #[inline]
    pub fn data_timeout(&self) -> DATA_TIMEOUTR {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        DATA_TIMEOUTR { bits }
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
    #[doc = "Bits 0:7 - Response time-out value. Value is in number of card output clocks - cclk_out."]
    #[inline]
    pub fn response_timeout(&mut self) -> _RESPONSE_TIMEOUTW {
        _RESPONSE_TIMEOUTW { w: self }
    }
    #[doc = "Bits 8:31 - Value for card Data Read time-out; same value also used for Data Starvation by Host time-out. Value is in number of card output clocks - cclk_out of selected card. Starvation by Host time-out. Value is in number of card output clocks - cclk_out of selected card."]
    #[inline]
    pub fn data_timeout(&mut self) -> _DATA_TIMEOUTW {
        _DATA_TIMEOUTW { w: self }
    }
}
