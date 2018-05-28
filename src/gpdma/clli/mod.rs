#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLLI {
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
#[doc = "Possible values of the field `LM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LMR {
    #[doc = "AHB Master 0."]
    AHB_MASTER_0,
    #[doc = "AHB Master 1."]
    AHB_MASTER_1,
}
impl LMR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            LMR::AHB_MASTER_0 => false,
            LMR::AHB_MASTER_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LMR {
        match value {
            false => LMR::AHB_MASTER_0,
            true => LMR::AHB_MASTER_1,
        }
    }
    #[doc = "Checks if the value of the field is `AHB_MASTER_0`"]
    #[inline]
    pub fn is_ahb_master_0(&self) -> bool {
        *self == LMR::AHB_MASTER_0
    }
    #[doc = "Checks if the value of the field is `AHB_MASTER_1`"]
    #[inline]
    pub fn is_ahb_master_1(&self) -> bool {
        *self == LMR::AHB_MASTER_1
    }
}
#[doc = r" Value of the field"]
pub struct RR {
    bits: bool,
}
impl RR {
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
#[doc = r" Value of the field"]
pub struct LLIR {
    bits: u32,
}
impl LLIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `LM`"]
pub enum LMW {
    #[doc = "AHB Master 0."]
    AHB_MASTER_0,
    #[doc = "AHB Master 1."]
    AHB_MASTER_1,
}
impl LMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LMW::AHB_MASTER_0 => false,
            LMW::AHB_MASTER_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LMW<'a> {
    w: &'a mut W,
}
impl<'a> _LMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "AHB Master 0."]
    #[inline]
    pub fn ahb_master_0(self) -> &'a mut W {
        self.variant(LMW::AHB_MASTER_0)
    }
    #[doc = "AHB Master 1."]
    #[inline]
    pub fn ahb_master_1(self) -> &'a mut W {
        self.variant(LMW::AHB_MASTER_1)
    }
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RW<'a> {
    w: &'a mut W,
}
impl<'a> _RW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LLIW<'a> {
    w: &'a mut W,
}
impl<'a> _LLIW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 1073741823;
        const OFFSET: u8 = 2;
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
    #[doc = "Bit 0 - AHB master select for loading the next LLI:"]
    #[inline]
    pub fn lm(&self) -> LMR {
        LMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Reserved, and must be written as 0, masked on read."]
    #[inline]
    pub fn r(&self) -> RR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RR { bits }
    }
    #[doc = "Bits 2:31 - Linked list item. Bits [31:2] of the address for the next LLI. Address bits [1:0] are 0."]
    #[inline]
    pub fn lli(&self) -> LLIR {
        let bits = {
            const MASK: u32 = 1073741823;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        LLIR { bits }
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
    #[doc = "Bit 0 - AHB master select for loading the next LLI:"]
    #[inline]
    pub fn lm(&mut self) -> _LMW {
        _LMW { w: self }
    }
    #[doc = "Bit 1 - Reserved, and must be written as 0, masked on read."]
    #[inline]
    pub fn r(&mut self) -> _RW {
        _RW { w: self }
    }
    #[doc = "Bits 2:31 - Linked list item. Bits [31:2] of the address for the next LLI. Address bits [1:0] are 0."]
    #[inline]
    pub fn lli(&mut self) -> _LLIW {
        _LLIW { w: self }
    }
}
