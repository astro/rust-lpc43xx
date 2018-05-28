#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ICR {
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
#[doc = "Possible values of the field `IRDAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRDAENR {
    #[doc = "Disabled. IrDA mode on USART3 is disabled, USART3 acts as a standard USART."]
    DISABLED,
    #[doc = "Enabled. IrDA mode on USART3 is enabled."]
    ENABLED,
}
impl IRDAENR {
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
            IRDAENR::DISABLED => false,
            IRDAENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IRDAENR {
        match value {
            false => IRDAENR::DISABLED,
            true => IRDAENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == IRDAENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == IRDAENR::ENABLED
    }
}
#[doc = "Possible values of the field `IRDAINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRDAINVR {
    #[doc = "Not inverted. The serial input is not inverted."]
    NOT_INVERTED,
    #[doc = "Inverted. The serial input is inverted. This has no effect on the serial output."]
    INVERTED,
}
impl IRDAINVR {
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
            IRDAINVR::NOT_INVERTED => false,
            IRDAINVR::INVERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IRDAINVR {
        match value {
            false => IRDAINVR::NOT_INVERTED,
            true => IRDAINVR::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INVERTED`"]
    #[inline]
    pub fn is_not_inverted(&self) -> bool {
        *self == IRDAINVR::NOT_INVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline]
    pub fn is_inverted(&self) -> bool {
        *self == IRDAINVR::INVERTED
    }
}
#[doc = "Possible values of the field `FIXPULSEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIXPULSEENR {
    #[doc = "Disabled. IrDA fixed pulse width mode disabled."]
    DISABLED,
    #[doc = "Enabled. IrDA fixed pulse width mode enabled."]
    ENABLED,
}
impl FIXPULSEENR {
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
            FIXPULSEENR::DISABLED => false,
            FIXPULSEENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FIXPULSEENR {
        match value {
            false => FIXPULSEENR::DISABLED,
            true => FIXPULSEENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == FIXPULSEENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == FIXPULSEENR::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct PULSEDIVR {
    bits: u8,
}
impl PULSEDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `IRDAEN`"]
pub enum IRDAENW {
    #[doc = "Disabled. IrDA mode on USART3 is disabled, USART3 acts as a standard USART."]
    DISABLED,
    #[doc = "Enabled. IrDA mode on USART3 is enabled."]
    ENABLED,
}
impl IRDAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IRDAENW::DISABLED => false,
            IRDAENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRDAENW<'a> {
    w: &'a mut W,
}
impl<'a> _IRDAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IRDAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. IrDA mode on USART3 is disabled, USART3 acts as a standard USART."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IRDAENW::DISABLED)
    }
    #[doc = "Enabled. IrDA mode on USART3 is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IRDAENW::ENABLED)
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
#[doc = "Values that can be written to the field `IRDAINV`"]
pub enum IRDAINVW {
    #[doc = "Not inverted. The serial input is not inverted."]
    NOT_INVERTED,
    #[doc = "Inverted. The serial input is inverted. This has no effect on the serial output."]
    INVERTED,
}
impl IRDAINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IRDAINVW::NOT_INVERTED => false,
            IRDAINVW::INVERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRDAINVW<'a> {
    w: &'a mut W,
}
impl<'a> _IRDAINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IRDAINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not inverted. The serial input is not inverted."]
    #[inline]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(IRDAINVW::NOT_INVERTED)
    }
    #[doc = "Inverted. The serial input is inverted. This has no effect on the serial output."]
    #[inline]
    pub fn inverted(self) -> &'a mut W {
        self.variant(IRDAINVW::INVERTED)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FIXPULSEEN`"]
pub enum FIXPULSEENW {
    #[doc = "Disabled. IrDA fixed pulse width mode disabled."]
    DISABLED,
    #[doc = "Enabled. IrDA fixed pulse width mode enabled."]
    ENABLED,
}
impl FIXPULSEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FIXPULSEENW::DISABLED => false,
            FIXPULSEENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FIXPULSEENW<'a> {
    w: &'a mut W,
}
impl<'a> _FIXPULSEENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FIXPULSEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. IrDA fixed pulse width mode disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FIXPULSEENW::DISABLED)
    }
    #[doc = "Enabled. IrDA fixed pulse width mode enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FIXPULSEENW::ENABLED)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PULSEDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _PULSEDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - IrDA mode enable."]
    #[inline]
    pub fn irdaen(&self) -> IRDAENR {
        IRDAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Serial input direction."]
    #[inline]
    pub fn irdainv(&self) -> IRDAINVR {
        IRDAINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - IrDA fixed pulse width mode."]
    #[inline]
    pub fn fixpulseen(&self) -> FIXPULSEENR {
        FIXPULSEENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:5 - Configures the pulse when FixPulseEn = 1. See Table 885 for details."]
    #[inline]
    pub fn pulsediv(&self) -> PULSEDIVR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PULSEDIVR { bits }
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
    #[doc = "Bit 0 - IrDA mode enable."]
    #[inline]
    pub fn irdaen(&mut self) -> _IRDAENW {
        _IRDAENW { w: self }
    }
    #[doc = "Bit 1 - Serial input direction."]
    #[inline]
    pub fn irdainv(&mut self) -> _IRDAINVW {
        _IRDAINVW { w: self }
    }
    #[doc = "Bit 2 - IrDA fixed pulse width mode."]
    #[inline]
    pub fn fixpulseen(&mut self) -> _FIXPULSEENW {
        _FIXPULSEENW { w: self }
    }
    #[doc = "Bits 3:5 - Configures the pulse when FixPulseEn = 1. See Table 885 for details."]
    #[inline]
    pub fn pulsediv(&mut self) -> _PULSEDIVW {
        _PULSEDIVW { w: self }
    }
}
