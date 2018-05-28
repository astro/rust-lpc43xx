#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCR {
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
#[doc = "Possible values of the field `MR0I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR0IR {
    #[doc = "Disabled. Interrupt is disabled"]
    DISABLED,
    #[doc = "Enabled. Interrupt is generated when MR0 matches the value in the TC."]
    ENABLED,
}
impl MR0IR {
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
            MR0IR::DISABLED => false,
            MR0IR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MR0IR {
        match value {
            false => MR0IR::DISABLED,
            true => MR0IR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MR0IR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == MR0IR::ENABLED
    }
}
#[doc = "Possible values of the field `MR0R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR0RR {
    #[doc = "Disabled. Feature disabled."]
    DISABLED,
    #[doc = "Reset. TC will be reset if MR0 matches it."]
    RESET,
}
impl MR0RR {
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
            MR0RR::DISABLED => false,
            MR0RR::RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MR0RR {
        match value {
            false => MR0RR::DISABLED,
            true => MR0RR::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MR0RR::DISABLED
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == MR0RR::RESET
    }
}
#[doc = "Possible values of the field `MR0S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR0SR {
    #[doc = "Disabled. Feature disabled."]
    DISABLED,
    #[doc = "Match. TC and PC will be stopped and TCR[0] will be set to 0 if MR0 matches the TC."]
    MATCH,
}
impl MR0SR {
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
            MR0SR::DISABLED => false,
            MR0SR::MATCH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MR0SR {
        match value {
            false => MR0SR::DISABLED,
            true => MR0SR::MATCH,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MR0SR::DISABLED
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline]
    pub fn is_match_(&self) -> bool {
        *self == MR0SR::MATCH
    }
}
#[doc = "Possible values of the field `MR1I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR1IR {
    #[doc = "Disabled. Interrupt is disabled."]
    DISABLED,
    #[doc = "Match. Interrupt is generated when MR1 matches the value in the TC."]
    MATCH,
}
impl MR1IR {
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
            MR1IR::DISABLED => false,
            MR1IR::MATCH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MR1IR {
        match value {
            false => MR1IR::DISABLED,
            true => MR1IR::MATCH,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MR1IR::DISABLED
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline]
    pub fn is_match_(&self) -> bool {
        *self == MR1IR::MATCH
    }
}
#[doc = "Possible values of the field `MR1R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR1RR {
    #[doc = "Disabled. Feature disabled."]
    DISABLED,
    #[doc = "Reset. TC will be reset if MR1 matches it."]
    RESET,
}
impl MR1RR {
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
            MR1RR::DISABLED => false,
            MR1RR::RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MR1RR {
        match value {
            false => MR1RR::DISABLED,
            true => MR1RR::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MR1RR::DISABLED
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == MR1RR::RESET
    }
}
#[doc = "Possible values of the field `MR1S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR1SR {
    #[doc = "Disabled. Feature disabled."]
    DISABLED,
    #[doc = "Stop. TC and PC will be stopped and TCR[0] will be set to 0 if MR1 matches the TC."]
    STOP,
}
impl MR1SR {
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
            MR1SR::DISABLED => false,
            MR1SR::STOP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MR1SR {
        match value {
            false => MR1SR::DISABLED,
            true => MR1SR::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MR1SR::DISABLED
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == MR1SR::STOP
    }
}
#[doc = "Possible values of the field `MR2I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR2IR {
    #[doc = "Disabled. Interrupt is disabled"]
    DISABLED,
    #[doc = "Match. Interrupt is generated when MR2 matches the value in the TC."]
    MATCH,
}
impl MR2IR {
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
            MR2IR::DISABLED => false,
            MR2IR::MATCH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MR2IR {
        match value {
            false => MR2IR::DISABLED,
            true => MR2IR::MATCH,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MR2IR::DISABLED
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline]
    pub fn is_match_(&self) -> bool {
        *self == MR2IR::MATCH
    }
}
#[doc = "Possible values of the field `MR2R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR2RR {
    #[doc = "Disabled. Feature disabled."]
    DISABLED,
    #[doc = "Match. TC will be reset if MR2 matches it."]
    MATCH,
}
impl MR2RR {
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
            MR2RR::DISABLED => false,
            MR2RR::MATCH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MR2RR {
        match value {
            false => MR2RR::DISABLED,
            true => MR2RR::MATCH,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MR2RR::DISABLED
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline]
    pub fn is_match_(&self) -> bool {
        *self == MR2RR::MATCH
    }
}
#[doc = "Possible values of the field `MR2S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR2SR {
    #[doc = "Disabled. Feature disabled."]
    DISABLED,
    #[doc = "Stop. TC and PC will be stopped and TCR[0] will be set to 0 if MR2 matches the TC."]
    STOP,
}
impl MR2SR {
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
            MR2SR::DISABLED => false,
            MR2SR::STOP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MR2SR {
        match value {
            false => MR2SR::DISABLED,
            true => MR2SR::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MR2SR::DISABLED
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == MR2SR::STOP
    }
}
#[doc = "Possible values of the field `MR3I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR3IR {
    #[doc = "Disabled. This interrupt is disabled."]
    DISABLED,
    #[doc = "Interrupt. Interrupt is generated when MR3 matches the value in the TC."]
    INTERRUPT,
}
impl MR3IR {
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
            MR3IR::DISABLED => false,
            MR3IR::INTERRUPT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MR3IR {
        match value {
            false => MR3IR::DISABLED,
            true => MR3IR::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MR3IR::DISABLED
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline]
    pub fn is_interrupt(&self) -> bool {
        *self == MR3IR::INTERRUPT
    }
}
#[doc = "Possible values of the field `MR3R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR3RR {
    #[doc = "Disabled. Feature disabled."]
    DISABLED,
    #[doc = "Match. TC will be reset if MR3 matches it."]
    MATCH,
}
impl MR3RR {
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
            MR3RR::DISABLED => false,
            MR3RR::MATCH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MR3RR {
        match value {
            false => MR3RR::DISABLED,
            true => MR3RR::MATCH,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MR3RR::DISABLED
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline]
    pub fn is_match_(&self) -> bool {
        *self == MR3RR::MATCH
    }
}
#[doc = "Possible values of the field `MR3S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR3SR {
    #[doc = "Disabled. Feature disabled."]
    DISABLED,
    #[doc = "Stop. TC and PC will be stopped and TCR[0] will be set to 0 if MR3 matches the TC."]
    STOP,
}
impl MR3SR {
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
            MR3SR::DISABLED => false,
            MR3SR::STOP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MR3SR {
        match value {
            false => MR3SR::DISABLED,
            true => MR3SR::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MR3SR::DISABLED
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == MR3SR::STOP
    }
}
#[doc = "Values that can be written to the field `MR0I`"]
pub enum MR0IW {
    #[doc = "Disabled. Interrupt is disabled"]
    DISABLED,
    #[doc = "Enabled. Interrupt is generated when MR0 matches the value in the TC."]
    ENABLED,
}
impl MR0IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MR0IW::DISABLED => false,
            MR0IW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MR0IW<'a> {
    w: &'a mut W,
}
impl<'a> _MR0IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MR0IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Interrupt is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR0IW::DISABLED)
    }
    #[doc = "Enabled. Interrupt is generated when MR0 matches the value in the TC."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MR0IW::ENABLED)
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
#[doc = "Values that can be written to the field `MR0R`"]
pub enum MR0RW {
    #[doc = "Disabled. Feature disabled."]
    DISABLED,
    #[doc = "Reset. TC will be reset if MR0 matches it."]
    RESET,
}
impl MR0RW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MR0RW::DISABLED => false,
            MR0RW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MR0RW<'a> {
    w: &'a mut W,
}
impl<'a> _MR0RW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MR0RW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Feature disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR0RW::DISABLED)
    }
    #[doc = "Reset. TC will be reset if MR0 matches it."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(MR0RW::RESET)
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
#[doc = "Values that can be written to the field `MR0S`"]
pub enum MR0SW {
    #[doc = "Disabled. Feature disabled."]
    DISABLED,
    #[doc = "Match. TC and PC will be stopped and TCR[0] will be set to 0 if MR0 matches the TC."]
    MATCH,
}
impl MR0SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MR0SW::DISABLED => false,
            MR0SW::MATCH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MR0SW<'a> {
    w: &'a mut W,
}
impl<'a> _MR0SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MR0SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Feature disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR0SW::DISABLED)
    }
    #[doc = "Match. TC and PC will be stopped and TCR[0] will be set to 0 if MR0 matches the TC."]
    #[inline]
    pub fn match_(self) -> &'a mut W {
        self.variant(MR0SW::MATCH)
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
#[doc = "Values that can be written to the field `MR1I`"]
pub enum MR1IW {
    #[doc = "Disabled. Interrupt is disabled."]
    DISABLED,
    #[doc = "Match. Interrupt is generated when MR1 matches the value in the TC."]
    MATCH,
}
impl MR1IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MR1IW::DISABLED => false,
            MR1IW::MATCH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MR1IW<'a> {
    w: &'a mut W,
}
impl<'a> _MR1IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MR1IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Interrupt is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR1IW::DISABLED)
    }
    #[doc = "Match. Interrupt is generated when MR1 matches the value in the TC."]
    #[inline]
    pub fn match_(self) -> &'a mut W {
        self.variant(MR1IW::MATCH)
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
#[doc = "Values that can be written to the field `MR1R`"]
pub enum MR1RW {
    #[doc = "Disabled. Feature disabled."]
    DISABLED,
    #[doc = "Reset. TC will be reset if MR1 matches it."]
    RESET,
}
impl MR1RW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MR1RW::DISABLED => false,
            MR1RW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MR1RW<'a> {
    w: &'a mut W,
}
impl<'a> _MR1RW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MR1RW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Feature disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR1RW::DISABLED)
    }
    #[doc = "Reset. TC will be reset if MR1 matches it."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(MR1RW::RESET)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MR1S`"]
pub enum MR1SW {
    #[doc = "Disabled. Feature disabled."]
    DISABLED,
    #[doc = "Stop. TC and PC will be stopped and TCR[0] will be set to 0 if MR1 matches the TC."]
    STOP,
}
impl MR1SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MR1SW::DISABLED => false,
            MR1SW::STOP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MR1SW<'a> {
    w: &'a mut W,
}
impl<'a> _MR1SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MR1SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Feature disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR1SW::DISABLED)
    }
    #[doc = "Stop. TC and PC will be stopped and TCR[0] will be set to 0 if MR1 matches the TC."]
    #[inline]
    pub fn stop(self) -> &'a mut W {
        self.variant(MR1SW::STOP)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MR2I`"]
pub enum MR2IW {
    #[doc = "Disabled. Interrupt is disabled"]
    DISABLED,
    #[doc = "Match. Interrupt is generated when MR2 matches the value in the TC."]
    MATCH,
}
impl MR2IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MR2IW::DISABLED => false,
            MR2IW::MATCH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MR2IW<'a> {
    w: &'a mut W,
}
impl<'a> _MR2IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MR2IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Interrupt is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR2IW::DISABLED)
    }
    #[doc = "Match. Interrupt is generated when MR2 matches the value in the TC."]
    #[inline]
    pub fn match_(self) -> &'a mut W {
        self.variant(MR2IW::MATCH)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MR2R`"]
pub enum MR2RW {
    #[doc = "Disabled. Feature disabled."]
    DISABLED,
    #[doc = "Match. TC will be reset if MR2 matches it."]
    MATCH,
}
impl MR2RW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MR2RW::DISABLED => false,
            MR2RW::MATCH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MR2RW<'a> {
    w: &'a mut W,
}
impl<'a> _MR2RW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MR2RW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Feature disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR2RW::DISABLED)
    }
    #[doc = "Match. TC will be reset if MR2 matches it."]
    #[inline]
    pub fn match_(self) -> &'a mut W {
        self.variant(MR2RW::MATCH)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MR2S`"]
pub enum MR2SW {
    #[doc = "Disabled. Feature disabled."]
    DISABLED,
    #[doc = "Stop. TC and PC will be stopped and TCR[0] will be set to 0 if MR2 matches the TC."]
    STOP,
}
impl MR2SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MR2SW::DISABLED => false,
            MR2SW::STOP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MR2SW<'a> {
    w: &'a mut W,
}
impl<'a> _MR2SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MR2SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Feature disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR2SW::DISABLED)
    }
    #[doc = "Stop. TC and PC will be stopped and TCR[0] will be set to 0 if MR2 matches the TC."]
    #[inline]
    pub fn stop(self) -> &'a mut W {
        self.variant(MR2SW::STOP)
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
#[doc = "Values that can be written to the field `MR3I`"]
pub enum MR3IW {
    #[doc = "Disabled. This interrupt is disabled."]
    DISABLED,
    #[doc = "Interrupt. Interrupt is generated when MR3 matches the value in the TC."]
    INTERRUPT,
}
impl MR3IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MR3IW::DISABLED => false,
            MR3IW::INTERRUPT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MR3IW<'a> {
    w: &'a mut W,
}
impl<'a> _MR3IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MR3IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. This interrupt is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR3IW::DISABLED)
    }
    #[doc = "Interrupt. Interrupt is generated when MR3 matches the value in the TC."]
    #[inline]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(MR3IW::INTERRUPT)
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
#[doc = "Values that can be written to the field `MR3R`"]
pub enum MR3RW {
    #[doc = "Disabled. Feature disabled."]
    DISABLED,
    #[doc = "Match. TC will be reset if MR3 matches it."]
    MATCH,
}
impl MR3RW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MR3RW::DISABLED => false,
            MR3RW::MATCH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MR3RW<'a> {
    w: &'a mut W,
}
impl<'a> _MR3RW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MR3RW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Feature disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR3RW::DISABLED)
    }
    #[doc = "Match. TC will be reset if MR3 matches it."]
    #[inline]
    pub fn match_(self) -> &'a mut W {
        self.variant(MR3RW::MATCH)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MR3S`"]
pub enum MR3SW {
    #[doc = "Disabled. Feature disabled."]
    DISABLED,
    #[doc = "Stop. TC and PC will be stopped and TCR[0] will be set to 0 if MR3 matches the TC."]
    STOP,
}
impl MR3SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MR3SW::DISABLED => false,
            MR3SW::STOP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MR3SW<'a> {
    w: &'a mut W,
}
impl<'a> _MR3SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MR3SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Feature disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR3SW::DISABLED)
    }
    #[doc = "Stop. TC and PC will be stopped and TCR[0] will be set to 0 if MR3 matches the TC."]
    #[inline]
    pub fn stop(self) -> &'a mut W {
        self.variant(MR3SW::STOP)
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
        const OFFSET: u8 = 11;
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
    #[doc = "Bit 0 - Interrupt on MR0"]
    #[inline]
    pub fn mr0i(&self) -> MR0IR {
        MR0IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Reset on MR0"]
    #[inline]
    pub fn mr0r(&self) -> MR0RR {
        MR0RR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Stop on MR0"]
    #[inline]
    pub fn mr0s(&self) -> MR0SR {
        MR0SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Interrupt on MR1"]
    #[inline]
    pub fn mr1i(&self) -> MR1IR {
        MR1IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Reset on MR1"]
    #[inline]
    pub fn mr1r(&self) -> MR1RR {
        MR1RR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Stop on MR1"]
    #[inline]
    pub fn mr1s(&self) -> MR1SR {
        MR1SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Interrupt on MR2"]
    #[inline]
    pub fn mr2i(&self) -> MR2IR {
        MR2IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Reset on MR2"]
    #[inline]
    pub fn mr2r(&self) -> MR2RR {
        MR2RR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Stop on MR2."]
    #[inline]
    pub fn mr2s(&self) -> MR2SR {
        MR2SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Interrupt on MR3"]
    #[inline]
    pub fn mr3i(&self) -> MR3IR {
        MR3IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Reset on MR3"]
    #[inline]
    pub fn mr3r(&self) -> MR3RR {
        MR3RR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Stop on MR3"]
    #[inline]
    pub fn mr3s(&self) -> MR3SR {
        MR3SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
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
    #[doc = "Bit 0 - Interrupt on MR0"]
    #[inline]
    pub fn mr0i(&mut self) -> _MR0IW {
        _MR0IW { w: self }
    }
    #[doc = "Bit 1 - Reset on MR0"]
    #[inline]
    pub fn mr0r(&mut self) -> _MR0RW {
        _MR0RW { w: self }
    }
    #[doc = "Bit 2 - Stop on MR0"]
    #[inline]
    pub fn mr0s(&mut self) -> _MR0SW {
        _MR0SW { w: self }
    }
    #[doc = "Bit 3 - Interrupt on MR1"]
    #[inline]
    pub fn mr1i(&mut self) -> _MR1IW {
        _MR1IW { w: self }
    }
    #[doc = "Bit 4 - Reset on MR1"]
    #[inline]
    pub fn mr1r(&mut self) -> _MR1RW {
        _MR1RW { w: self }
    }
    #[doc = "Bit 5 - Stop on MR1"]
    #[inline]
    pub fn mr1s(&mut self) -> _MR1SW {
        _MR1SW { w: self }
    }
    #[doc = "Bit 6 - Interrupt on MR2"]
    #[inline]
    pub fn mr2i(&mut self) -> _MR2IW {
        _MR2IW { w: self }
    }
    #[doc = "Bit 7 - Reset on MR2"]
    #[inline]
    pub fn mr2r(&mut self) -> _MR2RW {
        _MR2RW { w: self }
    }
    #[doc = "Bit 8 - Stop on MR2."]
    #[inline]
    pub fn mr2s(&mut self) -> _MR2SW {
        _MR2SW { w: self }
    }
    #[doc = "Bit 9 - Interrupt on MR3"]
    #[inline]
    pub fn mr3i(&mut self) -> _MR3IW {
        _MR3IW { w: self }
    }
    #[doc = "Bit 10 - Reset on MR3"]
    #[inline]
    pub fn mr3r(&mut self) -> _MR3RW {
        _MR3RW { w: self }
    }
    #[doc = "Bit 11 - Stop on MR3"]
    #[inline]
    pub fn mr3s(&mut self) -> _MR3SW {
        _MR3SW { w: self }
    }
}
