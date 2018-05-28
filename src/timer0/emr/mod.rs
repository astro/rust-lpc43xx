#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EMR {
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
pub struct EM0R {
    bits: bool,
}
impl EM0R {
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
pub struct EM1R {
    bits: bool,
}
impl EM1R {
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
pub struct EM2R {
    bits: bool,
}
impl EM2R {
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
pub struct EM3R {
    bits: bool,
}
impl EM3R {
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
#[doc = "Possible values of the field `EMC0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMC0R {
    #[doc = "Do Nothing."]
    NOP,
    #[doc = "Clear. Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    CLEAR,
    #[doc = "Set. Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    SET,
    #[doc = "Toggle. Toggle the corresponding External Match bit/output."]
    TOGGLE,
}
impl EMC0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EMC0R::NOP => 0,
            EMC0R::CLEAR => 1,
            EMC0R::SET => 2,
            EMC0R::TOGGLE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EMC0R {
        match value {
            0 => EMC0R::NOP,
            1 => EMC0R::CLEAR,
            2 => EMC0R::SET,
            3 => EMC0R::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline]
    pub fn is_nop(&self) -> bool {
        *self == EMC0R::NOP
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == EMC0R::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == EMC0R::SET
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline]
    pub fn is_toggle(&self) -> bool {
        *self == EMC0R::TOGGLE
    }
}
#[doc = "Possible values of the field `EMC1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMC1R {
    #[doc = "Do Nothing."]
    NOP,
    #[doc = "Clear. Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    CLEAR,
    #[doc = "Set. Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    SET,
    #[doc = "Toggle. Toggle the corresponding External Match bit/output."]
    TOGGLE,
}
impl EMC1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EMC1R::NOP => 0,
            EMC1R::CLEAR => 1,
            EMC1R::SET => 2,
            EMC1R::TOGGLE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EMC1R {
        match value {
            0 => EMC1R::NOP,
            1 => EMC1R::CLEAR,
            2 => EMC1R::SET,
            3 => EMC1R::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline]
    pub fn is_nop(&self) -> bool {
        *self == EMC1R::NOP
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == EMC1R::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == EMC1R::SET
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline]
    pub fn is_toggle(&self) -> bool {
        *self == EMC1R::TOGGLE
    }
}
#[doc = "Possible values of the field `EMC2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMC2R {
    #[doc = "Do Nothing."]
    NOP,
    #[doc = "Clear. Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    CLEAR,
    #[doc = "Set. Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    SET,
    #[doc = "Toggle. Toggle the corresponding External Match bit/output."]
    TOGGLE,
}
impl EMC2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EMC2R::NOP => 0,
            EMC2R::CLEAR => 1,
            EMC2R::SET => 2,
            EMC2R::TOGGLE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EMC2R {
        match value {
            0 => EMC2R::NOP,
            1 => EMC2R::CLEAR,
            2 => EMC2R::SET,
            3 => EMC2R::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline]
    pub fn is_nop(&self) -> bool {
        *self == EMC2R::NOP
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == EMC2R::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == EMC2R::SET
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline]
    pub fn is_toggle(&self) -> bool {
        *self == EMC2R::TOGGLE
    }
}
#[doc = "Possible values of the field `EMC3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMC3R {
    #[doc = "Do Nothing."]
    NOP,
    #[doc = "Clear. Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    CLEAR,
    #[doc = "Set. Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    SET,
    #[doc = "Toggle. Toggle the corresponding External Match bit/output."]
    TOGGLE,
}
impl EMC3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EMC3R::NOP => 0,
            EMC3R::CLEAR => 1,
            EMC3R::SET => 2,
            EMC3R::TOGGLE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EMC3R {
        match value {
            0 => EMC3R::NOP,
            1 => EMC3R::CLEAR,
            2 => EMC3R::SET,
            3 => EMC3R::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline]
    pub fn is_nop(&self) -> bool {
        *self == EMC3R::NOP
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == EMC3R::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == EMC3R::SET
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline]
    pub fn is_toggle(&self) -> bool {
        *self == EMC3R::TOGGLE
    }
}
#[doc = r" Proxy"]
pub struct _EM0W<'a> {
    w: &'a mut W,
}
impl<'a> _EM0W<'a> {
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
pub struct _EM1W<'a> {
    w: &'a mut W,
}
impl<'a> _EM1W<'a> {
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
pub struct _EM2W<'a> {
    w: &'a mut W,
}
impl<'a> _EM2W<'a> {
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
pub struct _EM3W<'a> {
    w: &'a mut W,
}
impl<'a> _EM3W<'a> {
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
#[doc = "Values that can be written to the field `EMC0`"]
pub enum EMC0W {
    #[doc = "Do Nothing."]
    NOP,
    #[doc = "Clear. Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    CLEAR,
    #[doc = "Set. Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    SET,
    #[doc = "Toggle. Toggle the corresponding External Match bit/output."]
    TOGGLE,
}
impl EMC0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMC0W::NOP => 0,
            EMC0W::CLEAR => 1,
            EMC0W::SET => 2,
            EMC0W::TOGGLE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EMC0W<'a> {
    w: &'a mut W,
}
impl<'a> _EMC0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EMC0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Do Nothing."]
    #[inline]
    pub fn nop(self) -> &'a mut W {
        self.variant(EMC0W::NOP)
    }
    #[doc = "Clear. Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(EMC0W::CLEAR)
    }
    #[doc = "Set. Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(EMC0W::SET)
    }
    #[doc = "Toggle. Toggle the corresponding External Match bit/output."]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC0W::TOGGLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EMC1`"]
pub enum EMC1W {
    #[doc = "Do Nothing."]
    NOP,
    #[doc = "Clear. Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    CLEAR,
    #[doc = "Set. Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    SET,
    #[doc = "Toggle. Toggle the corresponding External Match bit/output."]
    TOGGLE,
}
impl EMC1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMC1W::NOP => 0,
            EMC1W::CLEAR => 1,
            EMC1W::SET => 2,
            EMC1W::TOGGLE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EMC1W<'a> {
    w: &'a mut W,
}
impl<'a> _EMC1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EMC1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Do Nothing."]
    #[inline]
    pub fn nop(self) -> &'a mut W {
        self.variant(EMC1W::NOP)
    }
    #[doc = "Clear. Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(EMC1W::CLEAR)
    }
    #[doc = "Set. Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(EMC1W::SET)
    }
    #[doc = "Toggle. Toggle the corresponding External Match bit/output."]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC1W::TOGGLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EMC2`"]
pub enum EMC2W {
    #[doc = "Do Nothing."]
    NOP,
    #[doc = "Clear. Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    CLEAR,
    #[doc = "Set. Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    SET,
    #[doc = "Toggle. Toggle the corresponding External Match bit/output."]
    TOGGLE,
}
impl EMC2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMC2W::NOP => 0,
            EMC2W::CLEAR => 1,
            EMC2W::SET => 2,
            EMC2W::TOGGLE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EMC2W<'a> {
    w: &'a mut W,
}
impl<'a> _EMC2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EMC2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Do Nothing."]
    #[inline]
    pub fn nop(self) -> &'a mut W {
        self.variant(EMC2W::NOP)
    }
    #[doc = "Clear. Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(EMC2W::CLEAR)
    }
    #[doc = "Set. Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(EMC2W::SET)
    }
    #[doc = "Toggle. Toggle the corresponding External Match bit/output."]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC2W::TOGGLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EMC3`"]
pub enum EMC3W {
    #[doc = "Do Nothing."]
    NOP,
    #[doc = "Clear. Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    CLEAR,
    #[doc = "Set. Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    SET,
    #[doc = "Toggle. Toggle the corresponding External Match bit/output."]
    TOGGLE,
}
impl EMC3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMC3W::NOP => 0,
            EMC3W::CLEAR => 1,
            EMC3W::SET => 2,
            EMC3W::TOGGLE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EMC3W<'a> {
    w: &'a mut W,
}
impl<'a> _EMC3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EMC3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Do Nothing."]
    #[inline]
    pub fn nop(self) -> &'a mut W {
        self.variant(EMC3W::NOP)
    }
    #[doc = "Clear. Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(EMC3W::CLEAR)
    }
    #[doc = "Set. Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(EMC3W::SET)
    }
    #[doc = "Toggle. Toggle the corresponding External Match bit/output."]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC3W::TOGGLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
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
    #[doc = "Bit 0 - External Match 0. When a match occurs between the TC and MR0, this bit can either toggle, go low, go high, or do nothing, depending on bits 5:4 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline]
    pub fn em0(&self) -> EM0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EM0R { bits }
    }
    #[doc = "Bit 1 - External Match 1. When a match occurs between the TC and MR1, this bit can either toggle, go low, go high, or do nothing, depending on bits 7:6 of this register. This bit can be driven onto a MATn.1 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline]
    pub fn em1(&self) -> EM1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EM1R { bits }
    }
    #[doc = "Bit 2 - External Match 2. When a match occurs between the TC and MR2, this bit can either toggle, go low, go high, or do nothing, depending on bits 9:8 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline]
    pub fn em2(&self) -> EM2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EM2R { bits }
    }
    #[doc = "Bit 3 - External Match 3. When a match occurs between the TC and MR3, this bit can either toggle, go low, go high, or do nothing, depending on bits 11:10 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline]
    pub fn em3(&self) -> EM3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EM3R { bits }
    }
    #[doc = "Bits 4:5 - External Match Control 0. Determines the functionality of External Match 0."]
    #[inline]
    pub fn emc0(&self) -> EMC0R {
        EMC0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - External Match Control 1. Determines the functionality of External Match 1."]
    #[inline]
    pub fn emc1(&self) -> EMC1R {
        EMC1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - External Match Control 2. Determines the functionality of External Match 2."]
    #[inline]
    pub fn emc2(&self) -> EMC2R {
        EMC2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - External Match Control 3. Determines the functionality of External Match 3."]
    #[inline]
    pub fn emc3(&self) -> EMC3R {
        EMC3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
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
    #[doc = "Bit 0 - External Match 0. When a match occurs between the TC and MR0, this bit can either toggle, go low, go high, or do nothing, depending on bits 5:4 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline]
    pub fn em0(&mut self) -> _EM0W {
        _EM0W { w: self }
    }
    #[doc = "Bit 1 - External Match 1. When a match occurs between the TC and MR1, this bit can either toggle, go low, go high, or do nothing, depending on bits 7:6 of this register. This bit can be driven onto a MATn.1 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline]
    pub fn em1(&mut self) -> _EM1W {
        _EM1W { w: self }
    }
    #[doc = "Bit 2 - External Match 2. When a match occurs between the TC and MR2, this bit can either toggle, go low, go high, or do nothing, depending on bits 9:8 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline]
    pub fn em2(&mut self) -> _EM2W {
        _EM2W { w: self }
    }
    #[doc = "Bit 3 - External Match 3. When a match occurs between the TC and MR3, this bit can either toggle, go low, go high, or do nothing, depending on bits 11:10 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline]
    pub fn em3(&mut self) -> _EM3W {
        _EM3W { w: self }
    }
    #[doc = "Bits 4:5 - External Match Control 0. Determines the functionality of External Match 0."]
    #[inline]
    pub fn emc0(&mut self) -> _EMC0W {
        _EMC0W { w: self }
    }
    #[doc = "Bits 6:7 - External Match Control 1. Determines the functionality of External Match 1."]
    #[inline]
    pub fn emc1(&mut self) -> _EMC1W {
        _EMC1W { w: self }
    }
    #[doc = "Bits 8:9 - External Match Control 2. Determines the functionality of External Match 2."]
    #[inline]
    pub fn emc2(&mut self) -> _EMC2W {
        _EMC2W { w: self }
    }
    #[doc = "Bits 10:11 - External Match Control 3. Determines the functionality of External Match 3."]
    #[inline]
    pub fn emc3(&mut self) -> _EMC3W {
        _EMC3W { w: self }
    }
}
