#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PM {
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
#[doc = "Possible values of the field `PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDR {
    #[doc = "Normal operation."]
    NORMAL_OPERATION_,
    #[doc = "Clocks with wake-up mode enabled (W = 1) are disabled."]
    CLOCKS_WITH_WAKE_UP_,
}
impl PDR {
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
            PDR::NORMAL_OPERATION_ => false,
            PDR::CLOCKS_WITH_WAKE_UP_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDR {
        match value {
            false => PDR::NORMAL_OPERATION_,
            true => PDR::CLOCKS_WITH_WAKE_UP_,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_OPERATION_`"]
    #[inline]
    pub fn is_normal_operation_(&self) -> bool {
        *self == PDR::NORMAL_OPERATION_
    }
    #[doc = "Checks if the value of the field is `CLOCKS_WITH_WAKE_UP_`"]
    #[inline]
    pub fn is_clocks_with_wake_up_(&self) -> bool {
        *self == PDR::CLOCKS_WITH_WAKE_UP_
    }
}
#[doc = "Values that can be written to the field `PD`"]
pub enum PDW {
    #[doc = "Normal operation."]
    NORMAL_OPERATION_,
    #[doc = "Clocks with wake-up mode enabled (W = 1) are disabled."]
    CLOCKS_WITH_WAKE_UP_,
}
impl PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDW::NORMAL_OPERATION_ => false,
            PDW::CLOCKS_WITH_WAKE_UP_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDW<'a> {
    w: &'a mut W,
}
impl<'a> _PDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn normal_operation_(self) -> &'a mut W {
        self.variant(PDW::NORMAL_OPERATION_)
    }
    #[doc = "Clocks with wake-up mode enabled (W = 1) are disabled."]
    #[inline]
    pub fn clocks_with_wake_up_(self) -> &'a mut W {
        self.variant(PDW::CLOCKS_WITH_WAKE_UP_)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Initiate power-down mode"]
    #[inline]
    pub fn pd(&self) -> PDR {
        PDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bit 0 - Initiate power-down mode"]
    #[inline]
    pub fn pd(&mut self) -> _PDW {
        _PDW { w: self }
    }
}
