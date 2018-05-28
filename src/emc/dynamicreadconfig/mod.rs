#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DYNAMICREADCONFIG {
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
#[doc = "Possible values of the field `RD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDR {
    #[doc = "Do not use. POR reset value."]
    DO_NOT_USE,
    #[doc = "Command delayed by 1/2 EMC_CCLK."]
    HALF,
    #[doc = "Command delayed by 1/2 EMC_CCLK plus one clock cycle."]
    HALFPLUSONE,
    #[doc = "Command delayed by1/2 EMC_CCLK plus two clock cycles,"]
    HALFPLUSTWO,
}
impl RDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RDR::DO_NOT_USE => 0,
            RDR::HALF => 1,
            RDR::HALFPLUSONE => 2,
            RDR::HALFPLUSTWO => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RDR {
        match value {
            0 => RDR::DO_NOT_USE,
            1 => RDR::HALF,
            2 => RDR::HALFPLUSONE,
            3 => RDR::HALFPLUSTWO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DO_NOT_USE`"]
    #[inline]
    pub fn is_do_not_use(&self) -> bool {
        *self == RDR::DO_NOT_USE
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline]
    pub fn is_half(&self) -> bool {
        *self == RDR::HALF
    }
    #[doc = "Checks if the value of the field is `HALFPLUSONE`"]
    #[inline]
    pub fn is_halfplusone(&self) -> bool {
        *self == RDR::HALFPLUSONE
    }
    #[doc = "Checks if the value of the field is `HALFPLUSTWO`"]
    #[inline]
    pub fn is_halfplustwo(&self) -> bool {
        *self == RDR::HALFPLUSTWO
    }
}
#[doc = "Values that can be written to the field `RD`"]
pub enum RDW {
    #[doc = "Do not use. POR reset value."]
    DO_NOT_USE,
    #[doc = "Command delayed by 1/2 EMC_CCLK."]
    HALF,
    #[doc = "Command delayed by 1/2 EMC_CCLK plus one clock cycle."]
    HALFPLUSONE,
    #[doc = "Command delayed by1/2 EMC_CCLK plus two clock cycles,"]
    HALFPLUSTWO,
}
impl RDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RDW::DO_NOT_USE => 0,
            RDW::HALF => 1,
            RDW::HALFPLUSONE => 2,
            RDW::HALFPLUSTWO => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RDW<'a> {
    w: &'a mut W,
}
impl<'a> _RDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Do not use. POR reset value."]
    #[inline]
    pub fn do_not_use(self) -> &'a mut W {
        self.variant(RDW::DO_NOT_USE)
    }
    #[doc = "Command delayed by 1/2 EMC_CCLK."]
    #[inline]
    pub fn half(self) -> &'a mut W {
        self.variant(RDW::HALF)
    }
    #[doc = "Command delayed by 1/2 EMC_CCLK plus one clock cycle."]
    #[inline]
    pub fn halfplusone(self) -> &'a mut W {
        self.variant(RDW::HALFPLUSONE)
    }
    #[doc = "Command delayed by1/2 EMC_CCLK plus two clock cycles,"]
    #[inline]
    pub fn halfplustwo(self) -> &'a mut W {
        self.variant(RDW::HALFPLUSTWO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 0:1 - Read data strategy."]
    #[inline]
    pub fn rd(&self) -> RDR {
        RDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bits 0:1 - Read data strategy."]
    #[inline]
    pub fn rd(&mut self) -> _RDW {
        _RDW { w: self }
    }
}
