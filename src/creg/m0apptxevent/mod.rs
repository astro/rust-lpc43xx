#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::M0APPTXEVENT {
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
#[doc = "Possible values of the field `TXEVCLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEVCLRR {
    #[doc = "Clear the TXEV event."]
    CLEAR_THE_TXEV_EVENT,
    #[doc = "No effect."]
    NO_EFFECT,
}
impl TXEVCLRR {
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
            TXEVCLRR::CLEAR_THE_TXEV_EVENT => false,
            TXEVCLRR::NO_EFFECT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXEVCLRR {
        match value {
            false => TXEVCLRR::CLEAR_THE_TXEV_EVENT,
            true => TXEVCLRR::NO_EFFECT,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR_THE_TXEV_EVENT`"]
    #[inline]
    pub fn is_clear_the_txev_event(&self) -> bool {
        *self == TXEVCLRR::CLEAR_THE_TXEV_EVENT
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline]
    pub fn is_no_effect(&self) -> bool {
        *self == TXEVCLRR::NO_EFFECT
    }
}
#[doc = "Values that can be written to the field `TXEVCLR`"]
pub enum TXEVCLRW {
    #[doc = "Clear the TXEV event."]
    CLEAR_THE_TXEV_EVENT,
    #[doc = "No effect."]
    NO_EFFECT,
}
impl TXEVCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXEVCLRW::CLEAR_THE_TXEV_EVENT => false,
            TXEVCLRW::NO_EFFECT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXEVCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TXEVCLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXEVCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the TXEV event."]
    #[inline]
    pub fn clear_the_txev_event(self) -> &'a mut W {
        self.variant(TXEVCLRW::CLEAR_THE_TXEV_EVENT)
    }
    #[doc = "No effect."]
    #[inline]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TXEVCLRW::NO_EFFECT)
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
    #[doc = "Bit 0 - Cortex-M0APP TXEV event handling."]
    #[inline]
    pub fn txevclr(&self) -> TXEVCLRR {
        TXEVCLRR::_from({
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
    #[doc = "Bit 0 - Cortex-M0APP TXEV event handling."]
    #[inline]
    pub fn txevclr(&mut self) -> _TXEVCLRW {
        _TXEVCLRW { w: self }
    }
}
