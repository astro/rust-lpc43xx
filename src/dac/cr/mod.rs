#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR {
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
pub struct VALUER {
    bits: u16,
}
impl VALUER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `BIAS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIASR {
    #[doc = "The settling time of the DAC is 1 micros max, and the maximum current is 700 microA."]
    SHORT,
    #[doc = "The settling time of the DAC is 2.5 micros and the maximum current is 350 microA."]
    LONG,
}
impl BIASR {
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
            BIASR::SHORT => false,
            BIASR::LONG => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BIASR {
        match value {
            false => BIASR::SHORT,
            true => BIASR::LONG,
        }
    }
    #[doc = "Checks if the value of the field is `SHORT`"]
    #[inline]
    pub fn is_short(&self) -> bool {
        *self == BIASR::SHORT
    }
    #[doc = "Checks if the value of the field is `LONG`"]
    #[inline]
    pub fn is_long(&self) -> bool {
        *self == BIASR::LONG
    }
}
#[doc = r" Proxy"]
pub struct _VALUEW<'a> {
    w: &'a mut W,
}
impl<'a> _VALUEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BIAS`"]
pub enum BIASW {
    #[doc = "The settling time of the DAC is 1 micros max, and the maximum current is 700 microA."]
    SHORT,
    #[doc = "The settling time of the DAC is 2.5 micros and the maximum current is 350 microA."]
    LONG,
}
impl BIASW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BIASW::SHORT => false,
            BIASW::LONG => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BIASW<'a> {
    w: &'a mut W,
}
impl<'a> _BIASW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BIASW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The settling time of the DAC is 1 micros max, and the maximum current is 700 microA."]
    #[inline]
    pub fn short(self) -> &'a mut W {
        self.variant(BIASW::SHORT)
    }
    #[doc = "The settling time of the DAC is 2.5 micros and the maximum current is 350 microA."]
    #[inline]
    pub fn long(self) -> &'a mut W {
        self.variant(BIASW::LONG)
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
    #[doc = "Bits 6:15 - After the selected settling time after this field is written with a new VALUE, the voltage on the DACOUT pin (with respect to VSSA) is VALUE/1024 X VDDA."]
    #[inline]
    pub fn value(&self) -> VALUER {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        VALUER { bits }
    }
    #[doc = "Bit 16 - Settling time"]
    #[inline]
    pub fn bias(&self) -> BIASR {
        BIASR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
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
    #[doc = "Bits 6:15 - After the selected settling time after this field is written with a new VALUE, the voltage on the DACOUT pin (with respect to VSSA) is VALUE/1024 X VDDA."]
    #[inline]
    pub fn value(&mut self) -> _VALUEW {
        _VALUEW { w: self }
    }
    #[doc = "Bit 16 - Settling time"]
    #[inline]
    pub fn bias(&mut self) -> _BIASW {
        _BIASW { w: self }
    }
}
