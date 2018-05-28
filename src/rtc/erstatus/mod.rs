#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ERSTATUS {
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
#[doc = "Possible values of the field `EV0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EV0R {
    #[doc = "No event change on channel 0."]
    NO_CHANGE,
    #[doc = "At least one event has occurred on channel 0."]
    EVENT,
}
impl EV0R {
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
            EV0R::NO_CHANGE => false,
            EV0R::EVENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EV0R {
        match value {
            false => EV0R::NO_CHANGE,
            true => EV0R::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == EV0R::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline]
    pub fn is_event(&self) -> bool {
        *self == EV0R::EVENT
    }
}
#[doc = "Possible values of the field `EV1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EV1R {
    #[doc = "No event change on channel 1."]
    NO_CHANGE,
    #[doc = "At least one event has occurred on channel 1."]
    EVENT,
}
impl EV1R {
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
            EV1R::NO_CHANGE => false,
            EV1R::EVENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EV1R {
        match value {
            false => EV1R::NO_CHANGE,
            true => EV1R::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == EV1R::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline]
    pub fn is_event(&self) -> bool {
        *self == EV1R::EVENT
    }
}
#[doc = "Possible values of the field `EV2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EV2R {
    #[doc = "No event change on channel 2."]
    NO_CHANGE,
    #[doc = "At least one event has occurred on channel 2."]
    EVENT,
}
impl EV2R {
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
            EV2R::NO_CHANGE => false,
            EV2R::EVENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EV2R {
        match value {
            false => EV2R::NO_CHANGE,
            true => EV2R::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == EV2R::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline]
    pub fn is_event(&self) -> bool {
        *self == EV2R::EVENT
    }
}
#[doc = "Possible values of the field `GP_CLEARED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GP_CLEAREDR {
    #[doc = "General purpose registers have not been asynchronous cleared."]
    NO_CHANGE,
    #[doc = "General purpose registers have been asynchronous cleared."]
    EVENT,
}
impl GP_CLEAREDR {
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
            GP_CLEAREDR::NO_CHANGE => false,
            GP_CLEAREDR::EVENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GP_CLEAREDR {
        match value {
            false => GP_CLEAREDR::NO_CHANGE,
            true => GP_CLEAREDR::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == GP_CLEAREDR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline]
    pub fn is_event(&self) -> bool {
        *self == GP_CLEAREDR::EVENT
    }
}
#[doc = "Possible values of the field `WAKEUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUPR {
    #[doc = "No interrupt/wake-up request is pending"]
    NOINTERRUPTWAKEUP,
    #[doc = "An interrupt/wake-up request is pending."]
    INTERRUPTWAKEUP,
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
            WAKEUPR::NOINTERRUPTWAKEUP => false,
            WAKEUPR::INTERRUPTWAKEUP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKEUPR {
        match value {
            false => WAKEUPR::NOINTERRUPTWAKEUP,
            true => WAKEUPR::INTERRUPTWAKEUP,
        }
    }
    #[doc = "Checks if the value of the field is `NOINTERRUPTWAKEUP`"]
    #[inline]
    pub fn is_nointerruptwakeup(&self) -> bool {
        *self == WAKEUPR::NOINTERRUPTWAKEUP
    }
    #[doc = "Checks if the value of the field is `INTERRUPTWAKEUP`"]
    #[inline]
    pub fn is_interruptwakeup(&self) -> bool {
        *self == WAKEUPR::INTERRUPTWAKEUP
    }
}
#[doc = "Values that can be written to the field `EV0`"]
pub enum EV0W {
    #[doc = "No event change on channel 0."]
    NO_CHANGE,
    #[doc = "At least one event has occurred on channel 0."]
    EVENT,
}
impl EV0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EV0W::NO_CHANGE => false,
            EV0W::EVENT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EV0W<'a> {
    w: &'a mut W,
}
impl<'a> _EV0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EV0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No event change on channel 0."]
    #[inline]
    pub fn no_change(self) -> &'a mut W {
        self.variant(EV0W::NO_CHANGE)
    }
    #[doc = "At least one event has occurred on channel 0."]
    #[inline]
    pub fn event(self) -> &'a mut W {
        self.variant(EV0W::EVENT)
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
#[doc = "Values that can be written to the field `EV1`"]
pub enum EV1W {
    #[doc = "No event change on channel 1."]
    NO_CHANGE,
    #[doc = "At least one event has occurred on channel 1."]
    EVENT,
}
impl EV1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EV1W::NO_CHANGE => false,
            EV1W::EVENT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EV1W<'a> {
    w: &'a mut W,
}
impl<'a> _EV1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EV1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No event change on channel 1."]
    #[inline]
    pub fn no_change(self) -> &'a mut W {
        self.variant(EV1W::NO_CHANGE)
    }
    #[doc = "At least one event has occurred on channel 1."]
    #[inline]
    pub fn event(self) -> &'a mut W {
        self.variant(EV1W::EVENT)
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
#[doc = "Values that can be written to the field `EV2`"]
pub enum EV2W {
    #[doc = "No event change on channel 2."]
    NO_CHANGE,
    #[doc = "At least one event has occurred on channel 2."]
    EVENT,
}
impl EV2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EV2W::NO_CHANGE => false,
            EV2W::EVENT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EV2W<'a> {
    w: &'a mut W,
}
impl<'a> _EV2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EV2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No event change on channel 2."]
    #[inline]
    pub fn no_change(self) -> &'a mut W {
        self.variant(EV2W::NO_CHANGE)
    }
    #[doc = "At least one event has occurred on channel 2."]
    #[inline]
    pub fn event(self) -> &'a mut W {
        self.variant(EV2W::EVENT)
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
#[doc = "Values that can be written to the field `GP_CLEARED`"]
pub enum GP_CLEAREDW {
    #[doc = "General purpose registers have not been asynchronous cleared."]
    NO_CHANGE,
    #[doc = "General purpose registers have been asynchronous cleared."]
    EVENT,
}
impl GP_CLEAREDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GP_CLEAREDW::NO_CHANGE => false,
            GP_CLEAREDW::EVENT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GP_CLEAREDW<'a> {
    w: &'a mut W,
}
impl<'a> _GP_CLEAREDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GP_CLEAREDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "General purpose registers have not been asynchronous cleared."]
    #[inline]
    pub fn no_change(self) -> &'a mut W {
        self.variant(GP_CLEAREDW::NO_CHANGE)
    }
    #[doc = "General purpose registers have been asynchronous cleared."]
    #[inline]
    pub fn event(self) -> &'a mut W {
        self.variant(GP_CLEAREDW::EVENT)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WAKEUP`"]
pub enum WAKEUPW {
    #[doc = "No interrupt/wake-up request is pending"]
    NOINTERRUPTWAKEUP,
    #[doc = "An interrupt/wake-up request is pending."]
    INTERRUPTWAKEUP,
}
impl WAKEUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKEUPW::NOINTERRUPTWAKEUP => false,
            WAKEUPW::INTERRUPTWAKEUP => true,
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
    #[doc = "No interrupt/wake-up request is pending"]
    #[inline]
    pub fn nointerruptwakeup(self) -> &'a mut W {
        self.variant(WAKEUPW::NOINTERRUPTWAKEUP)
    }
    #[doc = "An interrupt/wake-up request is pending."]
    #[inline]
    pub fn interruptwakeup(self) -> &'a mut W {
        self.variant(WAKEUPW::INTERRUPTWAKEUP)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Channel0 event flag (WAKEUP0 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
    #[inline]
    pub fn ev0(&self) -> EV0R {
        EV0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Channel1 Event flag (WAKEUP1 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
    #[inline]
    pub fn ev1(&self) -> EV1R {
        EV1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Channel2 Event flag (WAKEUP2 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
    #[inline]
    pub fn ev2(&self) -> EV2R {
        EV2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - General purpose register asynchronous clear flag. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
    #[inline]
    pub fn gp_cleared(&self) -> GP_CLEAREDR {
        GP_CLEAREDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Interrupt/wake-up request flag (Read-only). This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
    #[inline]
    pub fn wakeup(&self) -> WAKEUPR {
        WAKEUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Channel0 event flag (WAKEUP0 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
    #[inline]
    pub fn ev0(&mut self) -> _EV0W {
        _EV0W { w: self }
    }
    #[doc = "Bit 1 - Channel1 Event flag (WAKEUP1 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
    #[inline]
    pub fn ev1(&mut self) -> _EV1W {
        _EV1W { w: self }
    }
    #[doc = "Bit 2 - Channel2 Event flag (WAKEUP2 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
    #[inline]
    pub fn ev2(&mut self) -> _EV2W {
        _EV2W { w: self }
    }
    #[doc = "Bit 3 - General purpose register asynchronous clear flag. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
    #[inline]
    pub fn gp_cleared(&mut self) -> _GP_CLEAREDW {
        _GP_CLEAREDW { w: self }
    }
    #[doc = "Bit 31 - Interrupt/wake-up request flag (Read-only). This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
    #[inline]
    pub fn wakeup(&mut self) -> _WAKEUPW {
        _WAKEUPW { w: self }
    }
}
