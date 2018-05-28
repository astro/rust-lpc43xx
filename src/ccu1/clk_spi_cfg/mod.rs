#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLK_SPI_CFG {
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
#[doc = "Possible values of the field `RUN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUNR {
    #[doc = "Clock is disabled."]
    DISABLED,
    #[doc = "Clock is enabled."]
    ENABLED,
}
impl RUNR {
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
            RUNR::DISABLED => false,
            RUNR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RUNR {
        match value {
            false => RUNR::DISABLED,
            true => RUNR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RUNR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RUNR::ENABLED
    }
}
#[doc = "Possible values of the field `AUTO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOR {
    #[doc = "Auto is disabled."]
    AUTO_IS_DISABLED_,
    #[doc = "Auto is enabled."]
    AUTO_IS_ENABLED_,
}
impl AUTOR {
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
            AUTOR::AUTO_IS_DISABLED_ => false,
            AUTOR::AUTO_IS_ENABLED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUTOR {
        match value {
            false => AUTOR::AUTO_IS_DISABLED_,
            true => AUTOR::AUTO_IS_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `AUTO_IS_DISABLED_`"]
    #[inline]
    pub fn is_auto_is_disabled_(&self) -> bool {
        *self == AUTOR::AUTO_IS_DISABLED_
    }
    #[doc = "Checks if the value of the field is `AUTO_IS_ENABLED_`"]
    #[inline]
    pub fn is_auto_is_enabled_(&self) -> bool {
        *self == AUTOR::AUTO_IS_ENABLED_
    }
}
#[doc = "Possible values of the field `WAKEUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUPR {
    #[doc = "Wake-up is disabled."]
    WAKE_UP_IS_DISABLED_,
    #[doc = "Wake-up is enabled."]
    WAKE_UP_IS_ENABLED_,
}
impl WAKEUPR {
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
            WAKEUPR::WAKE_UP_IS_DISABLED_ => false,
            WAKEUPR::WAKE_UP_IS_ENABLED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKEUPR {
        match value {
            false => WAKEUPR::WAKE_UP_IS_DISABLED_,
            true => WAKEUPR::WAKE_UP_IS_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `WAKE_UP_IS_DISABLED_`"]
    #[inline]
    pub fn is_wake_up_is_disabled_(&self) -> bool {
        *self == WAKEUPR::WAKE_UP_IS_DISABLED_
    }
    #[doc = "Checks if the value of the field is `WAKE_UP_IS_ENABLED_`"]
    #[inline]
    pub fn is_wake_up_is_enabled_(&self) -> bool {
        *self == WAKEUPR::WAKE_UP_IS_ENABLED_
    }
}
#[doc = "Values that can be written to the field `RUN`"]
pub enum RUNW {
    #[doc = "Clock is disabled."]
    DISABLED,
    #[doc = "Clock is enabled."]
    ENABLED,
}
impl RUNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RUNW::DISABLED => false,
            RUNW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RUNW<'a> {
    w: &'a mut W,
}
impl<'a> _RUNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RUNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RUNW::DISABLED)
    }
    #[doc = "Clock is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RUNW::ENABLED)
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
#[doc = "Values that can be written to the field `AUTO`"]
pub enum AUTOW {
    #[doc = "Auto is disabled."]
    AUTO_IS_DISABLED_,
    #[doc = "Auto is enabled."]
    AUTO_IS_ENABLED_,
}
impl AUTOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUTOW::AUTO_IS_DISABLED_ => false,
            AUTOW::AUTO_IS_ENABLED_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUTOW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUTOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Auto is disabled."]
    #[inline]
    pub fn auto_is_disabled_(self) -> &'a mut W {
        self.variant(AUTOW::AUTO_IS_DISABLED_)
    }
    #[doc = "Auto is enabled."]
    #[inline]
    pub fn auto_is_enabled_(self) -> &'a mut W {
        self.variant(AUTOW::AUTO_IS_ENABLED_)
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
#[doc = "Values that can be written to the field `WAKEUP`"]
pub enum WAKEUPW {
    #[doc = "Wake-up is disabled."]
    WAKE_UP_IS_DISABLED_,
    #[doc = "Wake-up is enabled."]
    WAKE_UP_IS_ENABLED_,
}
impl WAKEUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKEUPW::WAKE_UP_IS_DISABLED_ => false,
            WAKEUPW::WAKE_UP_IS_ENABLED_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEUPW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up is disabled."]
    #[inline]
    pub fn wake_up_is_disabled_(self) -> &'a mut W {
        self.variant(WAKEUPW::WAKE_UP_IS_DISABLED_)
    }
    #[doc = "Wake-up is enabled."]
    #[inline]
    pub fn wake_up_is_enabled_(self) -> &'a mut W {
        self.variant(WAKEUPW::WAKE_UP_IS_ENABLED_)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Run enable"]
    #[inline]
    pub fn run(&self) -> RUNR {
        RUNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Auto (AHB disable mechanism) enable"]
    #[inline]
    pub fn auto(&self) -> AUTOR {
        AUTOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Wake-up mechanism enable"]
    #[inline]
    pub fn wakeup(&self) -> WAKEUPR {
        WAKEUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Run enable"]
    #[inline]
    pub fn run(&mut self) -> _RUNW {
        _RUNW { w: self }
    }
    #[doc = "Bit 1 - Auto (AHB disable mechanism) enable"]
    #[inline]
    pub fn auto(&mut self) -> _AUTOW {
        _AUTOW { w: self }
    }
    #[doc = "Bit 2 - Wake-up mechanism enable"]
    #[inline]
    pub fn wakeup(&mut self) -> _WAKEUPW {
        _WAKEUPW { w: self }
    }
}
