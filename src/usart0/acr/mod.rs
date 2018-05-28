#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ACR {
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
#[doc = "Possible values of the field `START`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTR {
    #[doc = "Stop. Auto-baud stop (auto-baud is not running)."]
    STOP,
    #[doc = "Start. Auto-baud start (auto-baud is running). Auto-baud run bit. This bit is automatically cleared after auto-baud completion."]
    START,
}
impl STARTR {
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
            STARTR::STOP => false,
            STARTR::START => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STARTR {
        match value {
            false => STARTR::STOP,
            true => STARTR::START,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == STARTR::STOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline]
    pub fn is_start(&self) -> bool {
        *self == STARTR::START
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Mode 0."]
    MODE_0,
    #[doc = "Mode 1."]
    MODE_1,
}
impl MODER {
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
            MODER::MODE_0 => false,
            MODER::MODE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MODER {
        match value {
            false => MODER::MODE_0,
            true => MODER::MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `MODE_0`"]
    #[inline]
    pub fn is_mode_0(&self) -> bool {
        *self == MODER::MODE_0
    }
    #[doc = "Checks if the value of the field is `MODE_1`"]
    #[inline]
    pub fn is_mode_1(&self) -> bool {
        *self == MODER::MODE_1
    }
}
#[doc = "Possible values of the field `AUTORESTART`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTORESTARTR {
    #[doc = "No restart."]
    NO_RESTART,
    #[doc = "Restart. Restart in case of time-out (counter restarts at next USART Rx falling edge)"]
    RESTART,
}
impl AUTORESTARTR {
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
            AUTORESTARTR::NO_RESTART => false,
            AUTORESTARTR::RESTART => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUTORESTARTR {
        match value {
            false => AUTORESTARTR::NO_RESTART,
            true => AUTORESTARTR::RESTART,
        }
    }
    #[doc = "Checks if the value of the field is `NO_RESTART`"]
    #[inline]
    pub fn is_no_restart(&self) -> bool {
        *self == AUTORESTARTR::NO_RESTART
    }
    #[doc = "Checks if the value of the field is `RESTART`"]
    #[inline]
    pub fn is_restart(&self) -> bool {
        *self == AUTORESTARTR::RESTART
    }
}
#[doc = "Possible values of the field `ABEOINTCLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABEOINTCLRR {
    #[doc = "No effect. Writing a 0 has no impact."]
    NO_EFFECT,
    #[doc = "Clear. Writing a 1 will clear the corresponding interrupt in the IIR."]
    CLEAR,
}
impl ABEOINTCLRR {
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
            ABEOINTCLRR::NO_EFFECT => false,
            ABEOINTCLRR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ABEOINTCLRR {
        match value {
            false => ABEOINTCLRR::NO_EFFECT,
            true => ABEOINTCLRR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline]
    pub fn is_no_effect(&self) -> bool {
        *self == ABEOINTCLRR::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == ABEOINTCLRR::CLEAR
    }
}
#[doc = "Possible values of the field `ABTOINTCLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABTOINTCLRR {
    #[doc = "No effect. Writing a 0 has no impact."]
    NO_EFFECT,
    #[doc = "Clear. Writing a 1 will clear the corresponding interrupt in the IIR."]
    CLEAR,
}
impl ABTOINTCLRR {
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
            ABTOINTCLRR::NO_EFFECT => false,
            ABTOINTCLRR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ABTOINTCLRR {
        match value {
            false => ABTOINTCLRR::NO_EFFECT,
            true => ABTOINTCLRR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline]
    pub fn is_no_effect(&self) -> bool {
        *self == ABTOINTCLRR::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == ABTOINTCLRR::CLEAR
    }
}
#[doc = "Values that can be written to the field `START`"]
pub enum STARTW {
    #[doc = "Stop. Auto-baud stop (auto-baud is not running)."]
    STOP,
    #[doc = "Start. Auto-baud start (auto-baud is running). Auto-baud run bit. This bit is automatically cleared after auto-baud completion."]
    START,
}
impl STARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STARTW::STOP => false,
            STARTW::START => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Stop. Auto-baud stop (auto-baud is not running)."]
    #[inline]
    pub fn stop(self) -> &'a mut W {
        self.variant(STARTW::STOP)
    }
    #[doc = "Start. Auto-baud start (auto-baud is running). Auto-baud run bit. This bit is automatically cleared after auto-baud completion."]
    #[inline]
    pub fn start(self) -> &'a mut W {
        self.variant(STARTW::START)
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
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "Mode 0."]
    MODE_0,
    #[doc = "Mode 1."]
    MODE_1,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MODEW::MODE_0 => false,
            MODEW::MODE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Mode 0."]
    #[inline]
    pub fn mode_0(self) -> &'a mut W {
        self.variant(MODEW::MODE_0)
    }
    #[doc = "Mode 1."]
    #[inline]
    pub fn mode_1(self) -> &'a mut W {
        self.variant(MODEW::MODE_1)
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
#[doc = "Values that can be written to the field `AUTORESTART`"]
pub enum AUTORESTARTW {
    #[doc = "No restart."]
    NO_RESTART,
    #[doc = "Restart. Restart in case of time-out (counter restarts at next USART Rx falling edge)"]
    RESTART,
}
impl AUTORESTARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUTORESTARTW::NO_RESTART => false,
            AUTORESTARTW::RESTART => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUTORESTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTORESTARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUTORESTARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No restart."]
    #[inline]
    pub fn no_restart(self) -> &'a mut W {
        self.variant(AUTORESTARTW::NO_RESTART)
    }
    #[doc = "Restart. Restart in case of time-out (counter restarts at next USART Rx falling edge)"]
    #[inline]
    pub fn restart(self) -> &'a mut W {
        self.variant(AUTORESTARTW::RESTART)
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
#[doc = "Values that can be written to the field `ABEOINTCLR`"]
pub enum ABEOINTCLRW {
    #[doc = "No effect. Writing a 0 has no impact."]
    NO_EFFECT,
    #[doc = "Clear. Writing a 1 will clear the corresponding interrupt in the IIR."]
    CLEAR,
}
impl ABEOINTCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ABEOINTCLRW::NO_EFFECT => false,
            ABEOINTCLRW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ABEOINTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _ABEOINTCLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ABEOINTCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect. Writing a 0 has no impact."]
    #[inline]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABEOINTCLRW::NO_EFFECT)
    }
    #[doc = "Clear. Writing a 1 will clear the corresponding interrupt in the IIR."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ABEOINTCLRW::CLEAR)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ABTOINTCLR`"]
pub enum ABTOINTCLRW {
    #[doc = "No effect. Writing a 0 has no impact."]
    NO_EFFECT,
    #[doc = "Clear. Writing a 1 will clear the corresponding interrupt in the IIR."]
    CLEAR,
}
impl ABTOINTCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ABTOINTCLRW::NO_EFFECT => false,
            ABTOINTCLRW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ABTOINTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _ABTOINTCLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ABTOINTCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect. Writing a 0 has no impact."]
    #[inline]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABTOINTCLRW::NO_EFFECT)
    }
    #[doc = "Clear. Writing a 1 will clear the corresponding interrupt in the IIR."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ABTOINTCLRW::CLEAR)
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
        const OFFSET: u8 = 9;
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
    #[doc = "Bit 0 - Start bit. This bit is automatically cleared after auto-baud completion."]
    #[inline]
    pub fn start(&self) -> STARTR {
        STARTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Auto-baud mode select bit."]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Restart bit."]
    #[inline]
    pub fn autorestart(&self) -> AUTORESTARTR {
        AUTORESTARTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - End of auto-baud interrupt clear bit (write-only)."]
    #[inline]
    pub fn abeointclr(&self) -> ABEOINTCLRR {
        ABEOINTCLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Auto-baud time-out interrupt clear bit (write-only)."]
    #[inline]
    pub fn abtointclr(&self) -> ABTOINTCLRR {
        ABTOINTCLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
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
    #[doc = "Bit 0 - Start bit. This bit is automatically cleared after auto-baud completion."]
    #[inline]
    pub fn start(&mut self) -> _STARTW {
        _STARTW { w: self }
    }
    #[doc = "Bit 1 - Auto-baud mode select bit."]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 2 - Restart bit."]
    #[inline]
    pub fn autorestart(&mut self) -> _AUTORESTARTW {
        _AUTORESTARTW { w: self }
    }
    #[doc = "Bit 8 - End of auto-baud interrupt clear bit (write-only)."]
    #[inline]
    pub fn abeointclr(&mut self) -> _ABEOINTCLRW {
        _ABEOINTCLRW { w: self }
    }
    #[doc = "Bit 9 - Auto-baud time-out interrupt clear bit (write-only)."]
    #[inline]
    pub fn abtointclr(&mut self) -> _ABTOINTCLRW {
        _ABTOINTCLRW { w: self }
    }
}
