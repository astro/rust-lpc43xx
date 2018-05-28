#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCR {
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
#[doc = "Possible values of the field `CAP0RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0RER {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED,
    #[doc = "Low to high. A sequence of 0 then 1 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    LOW_TO_HIGH,
}
impl CAP0RER {
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
            CAP0RER::DISABLED => false,
            CAP0RER::LOW_TO_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAP0RER {
        match value {
            false => CAP0RER::DISABLED,
            true => CAP0RER::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CAP0RER::DISABLED
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline]
    pub fn is_low_to_high(&self) -> bool {
        *self == CAP0RER::LOW_TO_HIGH
    }
}
#[doc = "Possible values of the field `CAP0FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0FER {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED,
    #[doc = "High to low. A sequence of 1 then 0 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    HIGH_TO_LOW,
}
impl CAP0FER {
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
            CAP0FER::DISABLED => false,
            CAP0FER::HIGH_TO_LOW => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAP0FER {
        match value {
            false => CAP0FER::DISABLED,
            true => CAP0FER::HIGH_TO_LOW,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CAP0FER::DISABLED
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline]
    pub fn is_high_to_low(&self) -> bool {
        *self == CAP0FER::HIGH_TO_LOW
    }
}
#[doc = "Possible values of the field `CAP0I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0IR {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED,
    #[doc = "Load. A CR0 load due to a CAPn.0 event will generate an interrupt."]
    LOAD,
}
impl CAP0IR {
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
            CAP0IR::DISABLED => false,
            CAP0IR::LOAD => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAP0IR {
        match value {
            false => CAP0IR::DISABLED,
            true => CAP0IR::LOAD,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CAP0IR::DISABLED
    }
    #[doc = "Checks if the value of the field is `LOAD`"]
    #[inline]
    pub fn is_load(&self) -> bool {
        *self == CAP0IR::LOAD
    }
}
#[doc = "Possible values of the field `CAP1RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1RER {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED,
    #[doc = "Low to high. A sequence of 0 then 1 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    LOW_TO_HIGH,
}
impl CAP1RER {
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
            CAP1RER::DISABLED => false,
            CAP1RER::LOW_TO_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAP1RER {
        match value {
            false => CAP1RER::DISABLED,
            true => CAP1RER::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CAP1RER::DISABLED
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline]
    pub fn is_low_to_high(&self) -> bool {
        *self == CAP1RER::LOW_TO_HIGH
    }
}
#[doc = "Possible values of the field `CAP1FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1FER {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED,
    #[doc = "High to low. A sequence of 1 then 0 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    HIGH_TO_LOW,
}
impl CAP1FER {
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
            CAP1FER::DISABLED => false,
            CAP1FER::HIGH_TO_LOW => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAP1FER {
        match value {
            false => CAP1FER::DISABLED,
            true => CAP1FER::HIGH_TO_LOW,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CAP1FER::DISABLED
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline]
    pub fn is_high_to_low(&self) -> bool {
        *self == CAP1FER::HIGH_TO_LOW
    }
}
#[doc = "Possible values of the field `CAP1I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1IR {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED,
    #[doc = "Load. A CR1 load due to a CAPn.1 event will generate an interrupt."]
    LOAD,
}
impl CAP1IR {
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
            CAP1IR::DISABLED => false,
            CAP1IR::LOAD => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAP1IR {
        match value {
            false => CAP1IR::DISABLED,
            true => CAP1IR::LOAD,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CAP1IR::DISABLED
    }
    #[doc = "Checks if the value of the field is `LOAD`"]
    #[inline]
    pub fn is_load(&self) -> bool {
        *self == CAP1IR::LOAD
    }
}
#[doc = "Possible values of the field `CAP2RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP2RER {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED,
    #[doc = "Low to high. A sequence of 0 then 1 on CAPn.2 will cause CR2 to be loaded with the contents of TC."]
    LOW_TO_HIGH,
}
impl CAP2RER {
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
            CAP2RER::DISABLED => false,
            CAP2RER::LOW_TO_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAP2RER {
        match value {
            false => CAP2RER::DISABLED,
            true => CAP2RER::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CAP2RER::DISABLED
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline]
    pub fn is_low_to_high(&self) -> bool {
        *self == CAP2RER::LOW_TO_HIGH
    }
}
#[doc = "Possible values of the field `CAP2FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP2FER {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED,
    #[doc = "High to low. A sequence of 1 then 0 on CAPn.2 will cause CR2 to be loaded with the contents of TC."]
    HIGH_TO_LOW,
}
impl CAP2FER {
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
            CAP2FER::DISABLED => false,
            CAP2FER::HIGH_TO_LOW => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAP2FER {
        match value {
            false => CAP2FER::DISABLED,
            true => CAP2FER::HIGH_TO_LOW,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CAP2FER::DISABLED
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline]
    pub fn is_high_to_low(&self) -> bool {
        *self == CAP2FER::HIGH_TO_LOW
    }
}
#[doc = "Possible values of the field `CAP2I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP2IR {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED,
    #[doc = "Load. A CR2 load due to a CAPn.2 event will generate an interrupt."]
    LOAD,
}
impl CAP2IR {
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
            CAP2IR::DISABLED => false,
            CAP2IR::LOAD => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAP2IR {
        match value {
            false => CAP2IR::DISABLED,
            true => CAP2IR::LOAD,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CAP2IR::DISABLED
    }
    #[doc = "Checks if the value of the field is `LOAD`"]
    #[inline]
    pub fn is_load(&self) -> bool {
        *self == CAP2IR::LOAD
    }
}
#[doc = "Possible values of the field `CAP3RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP3RER {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED,
    #[doc = "Low to high. A sequence of 0 then 1 on CAPn.3 will cause CR3 to be loaded with the contents of TC."]
    LOW_TO_HIGH,
}
impl CAP3RER {
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
            CAP3RER::DISABLED => false,
            CAP3RER::LOW_TO_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAP3RER {
        match value {
            false => CAP3RER::DISABLED,
            true => CAP3RER::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CAP3RER::DISABLED
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline]
    pub fn is_low_to_high(&self) -> bool {
        *self == CAP3RER::LOW_TO_HIGH
    }
}
#[doc = "Possible values of the field `CAP3FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP3FER {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED,
    #[doc = "A sequence of 1 then 0 on CAPn.3 will cause CR3 to be loaded with the contents of TC."]
    HIGH_TO_LOW,
}
impl CAP3FER {
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
            CAP3FER::DISABLED => false,
            CAP3FER::HIGH_TO_LOW => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAP3FER {
        match value {
            false => CAP3FER::DISABLED,
            true => CAP3FER::HIGH_TO_LOW,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CAP3FER::DISABLED
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline]
    pub fn is_high_to_low(&self) -> bool {
        *self == CAP3FER::HIGH_TO_LOW
    }
}
#[doc = "Possible values of the field `CAP3I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP3IR {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED,
    #[doc = "Load. A CR3 load due to a CAPn.3 event will generate an interrupt."]
    LOAD,
}
impl CAP3IR {
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
            CAP3IR::DISABLED => false,
            CAP3IR::LOAD => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAP3IR {
        match value {
            false => CAP3IR::DISABLED,
            true => CAP3IR::LOAD,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CAP3IR::DISABLED
    }
    #[doc = "Checks if the value of the field is `LOAD`"]
    #[inline]
    pub fn is_load(&self) -> bool {
        *self == CAP3IR::LOAD
    }
}
#[doc = "Values that can be written to the field `CAP0RE`"]
pub enum CAP0REW {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED,
    #[doc = "Low to high. A sequence of 0 then 1 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    LOW_TO_HIGH,
}
impl CAP0REW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP0REW::DISABLED => false,
            CAP0REW::LOW_TO_HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAP0REW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP0REW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAP0REW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAP0REW::DISABLED)
    }
    #[doc = "Low to high. A sequence of 0 then 1 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    #[inline]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(CAP0REW::LOW_TO_HIGH)
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
#[doc = "Values that can be written to the field `CAP0FE`"]
pub enum CAP0FEW {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED,
    #[doc = "High to low. A sequence of 1 then 0 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    HIGH_TO_LOW,
}
impl CAP0FEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP0FEW::DISABLED => false,
            CAP0FEW::HIGH_TO_LOW => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAP0FEW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP0FEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAP0FEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAP0FEW::DISABLED)
    }
    #[doc = "High to low. A sequence of 1 then 0 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    #[inline]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(CAP0FEW::HIGH_TO_LOW)
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
#[doc = "Values that can be written to the field `CAP0I`"]
pub enum CAP0IW {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED,
    #[doc = "Load. A CR0 load due to a CAPn.0 event will generate an interrupt."]
    LOAD,
}
impl CAP0IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP0IW::DISABLED => false,
            CAP0IW::LOAD => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAP0IW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP0IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAP0IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAP0IW::DISABLED)
    }
    #[doc = "Load. A CR0 load due to a CAPn.0 event will generate an interrupt."]
    #[inline]
    pub fn load(self) -> &'a mut W {
        self.variant(CAP0IW::LOAD)
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
#[doc = "Values that can be written to the field `CAP1RE`"]
pub enum CAP1REW {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED,
    #[doc = "Low to high. A sequence of 0 then 1 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    LOW_TO_HIGH,
}
impl CAP1REW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP1REW::DISABLED => false,
            CAP1REW::LOW_TO_HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAP1REW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP1REW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAP1REW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAP1REW::DISABLED)
    }
    #[doc = "Low to high. A sequence of 0 then 1 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    #[inline]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(CAP1REW::LOW_TO_HIGH)
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
#[doc = "Values that can be written to the field `CAP1FE`"]
pub enum CAP1FEW {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED,
    #[doc = "High to low. A sequence of 1 then 0 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    HIGH_TO_LOW,
}
impl CAP1FEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP1FEW::DISABLED => false,
            CAP1FEW::HIGH_TO_LOW => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAP1FEW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP1FEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAP1FEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAP1FEW::DISABLED)
    }
    #[doc = "High to low. A sequence of 1 then 0 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    #[inline]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(CAP1FEW::HIGH_TO_LOW)
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
#[doc = "Values that can be written to the field `CAP1I`"]
pub enum CAP1IW {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED,
    #[doc = "Load. A CR1 load due to a CAPn.1 event will generate an interrupt."]
    LOAD,
}
impl CAP1IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP1IW::DISABLED => false,
            CAP1IW::LOAD => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAP1IW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP1IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAP1IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAP1IW::DISABLED)
    }
    #[doc = "Load. A CR1 load due to a CAPn.1 event will generate an interrupt."]
    #[inline]
    pub fn load(self) -> &'a mut W {
        self.variant(CAP1IW::LOAD)
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
#[doc = "Values that can be written to the field `CAP2RE`"]
pub enum CAP2REW {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED,
    #[doc = "Low to high. A sequence of 0 then 1 on CAPn.2 will cause CR2 to be loaded with the contents of TC."]
    LOW_TO_HIGH,
}
impl CAP2REW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP2REW::DISABLED => false,
            CAP2REW::LOW_TO_HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAP2REW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP2REW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAP2REW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAP2REW::DISABLED)
    }
    #[doc = "Low to high. A sequence of 0 then 1 on CAPn.2 will cause CR2 to be loaded with the contents of TC."]
    #[inline]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(CAP2REW::LOW_TO_HIGH)
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
#[doc = "Values that can be written to the field `CAP2FE`"]
pub enum CAP2FEW {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED,
    #[doc = "High to low. A sequence of 1 then 0 on CAPn.2 will cause CR2 to be loaded with the contents of TC."]
    HIGH_TO_LOW,
}
impl CAP2FEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP2FEW::DISABLED => false,
            CAP2FEW::HIGH_TO_LOW => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAP2FEW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP2FEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAP2FEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAP2FEW::DISABLED)
    }
    #[doc = "High to low. A sequence of 1 then 0 on CAPn.2 will cause CR2 to be loaded with the contents of TC."]
    #[inline]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(CAP2FEW::HIGH_TO_LOW)
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
#[doc = "Values that can be written to the field `CAP2I`"]
pub enum CAP2IW {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED,
    #[doc = "Load. A CR2 load due to a CAPn.2 event will generate an interrupt."]
    LOAD,
}
impl CAP2IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP2IW::DISABLED => false,
            CAP2IW::LOAD => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAP2IW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP2IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAP2IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAP2IW::DISABLED)
    }
    #[doc = "Load. A CR2 load due to a CAPn.2 event will generate an interrupt."]
    #[inline]
    pub fn load(self) -> &'a mut W {
        self.variant(CAP2IW::LOAD)
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
#[doc = "Values that can be written to the field `CAP3RE`"]
pub enum CAP3REW {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED,
    #[doc = "Low to high. A sequence of 0 then 1 on CAPn.3 will cause CR3 to be loaded with the contents of TC."]
    LOW_TO_HIGH,
}
impl CAP3REW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP3REW::DISABLED => false,
            CAP3REW::LOW_TO_HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAP3REW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP3REW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAP3REW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAP3REW::DISABLED)
    }
    #[doc = "Low to high. A sequence of 0 then 1 on CAPn.3 will cause CR3 to be loaded with the contents of TC."]
    #[inline]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(CAP3REW::LOW_TO_HIGH)
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
#[doc = "Values that can be written to the field `CAP3FE`"]
pub enum CAP3FEW {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED,
    #[doc = "A sequence of 1 then 0 on CAPn.3 will cause CR3 to be loaded with the contents of TC."]
    HIGH_TO_LOW,
}
impl CAP3FEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP3FEW::DISABLED => false,
            CAP3FEW::HIGH_TO_LOW => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAP3FEW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP3FEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAP3FEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAP3FEW::DISABLED)
    }
    #[doc = "A sequence of 1 then 0 on CAPn.3 will cause CR3 to be loaded with the contents of TC."]
    #[inline]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(CAP3FEW::HIGH_TO_LOW)
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
#[doc = "Values that can be written to the field `CAP3I`"]
pub enum CAP3IW {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED,
    #[doc = "Load. A CR3 load due to a CAPn.3 event will generate an interrupt."]
    LOAD,
}
impl CAP3IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP3IW::DISABLED => false,
            CAP3IW::LOAD => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAP3IW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP3IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAP3IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAP3IW::DISABLED)
    }
    #[doc = "Load. A CR3 load due to a CAPn.3 event will generate an interrupt."]
    #[inline]
    pub fn load(self) -> &'a mut W {
        self.variant(CAP3IW::LOAD)
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
    #[doc = "Bit 0 - Capture on CAPn.0 rising edge"]
    #[inline]
    pub fn cap0re(&self) -> CAP0RER {
        CAP0RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Capture on CAPn.0 falling edge"]
    #[inline]
    pub fn cap0fe(&self) -> CAP0FER {
        CAP0FER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Interrupt on CAPn.0 event"]
    #[inline]
    pub fn cap0i(&self) -> CAP0IR {
        CAP0IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Capture on CAPn.1 rising edge"]
    #[inline]
    pub fn cap1re(&self) -> CAP1RER {
        CAP1RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Capture on CAPn.1 falling edge"]
    #[inline]
    pub fn cap1fe(&self) -> CAP1FER {
        CAP1FER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Interrupt on CAPn.1 event"]
    #[inline]
    pub fn cap1i(&self) -> CAP1IR {
        CAP1IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Capture on CAPn.2 rising edge"]
    #[inline]
    pub fn cap2re(&self) -> CAP2RER {
        CAP2RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Capture on CAPn.2 falling edge:"]
    #[inline]
    pub fn cap2fe(&self) -> CAP2FER {
        CAP2FER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Interrupt on CAPn.2 event"]
    #[inline]
    pub fn cap2i(&self) -> CAP2IR {
        CAP2IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Capture on CAPn.3 rising edge"]
    #[inline]
    pub fn cap3re(&self) -> CAP3RER {
        CAP3RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - High to low. Capture on CAPn.3 falling edge"]
    #[inline]
    pub fn cap3fe(&self) -> CAP3FER {
        CAP3FER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Interrupt on CAPn.3 event:"]
    #[inline]
    pub fn cap3i(&self) -> CAP3IR {
        CAP3IR::_from({
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
    #[doc = "Bit 0 - Capture on CAPn.0 rising edge"]
    #[inline]
    pub fn cap0re(&mut self) -> _CAP0REW {
        _CAP0REW { w: self }
    }
    #[doc = "Bit 1 - Capture on CAPn.0 falling edge"]
    #[inline]
    pub fn cap0fe(&mut self) -> _CAP0FEW {
        _CAP0FEW { w: self }
    }
    #[doc = "Bit 2 - Interrupt on CAPn.0 event"]
    #[inline]
    pub fn cap0i(&mut self) -> _CAP0IW {
        _CAP0IW { w: self }
    }
    #[doc = "Bit 3 - Capture on CAPn.1 rising edge"]
    #[inline]
    pub fn cap1re(&mut self) -> _CAP1REW {
        _CAP1REW { w: self }
    }
    #[doc = "Bit 4 - Capture on CAPn.1 falling edge"]
    #[inline]
    pub fn cap1fe(&mut self) -> _CAP1FEW {
        _CAP1FEW { w: self }
    }
    #[doc = "Bit 5 - Interrupt on CAPn.1 event"]
    #[inline]
    pub fn cap1i(&mut self) -> _CAP1IW {
        _CAP1IW { w: self }
    }
    #[doc = "Bit 6 - Capture on CAPn.2 rising edge"]
    #[inline]
    pub fn cap2re(&mut self) -> _CAP2REW {
        _CAP2REW { w: self }
    }
    #[doc = "Bit 7 - Capture on CAPn.2 falling edge:"]
    #[inline]
    pub fn cap2fe(&mut self) -> _CAP2FEW {
        _CAP2FEW { w: self }
    }
    #[doc = "Bit 8 - Interrupt on CAPn.2 event"]
    #[inline]
    pub fn cap2i(&mut self) -> _CAP2IW {
        _CAP2IW { w: self }
    }
    #[doc = "Bit 9 - Capture on CAPn.3 rising edge"]
    #[inline]
    pub fn cap3re(&mut self) -> _CAP3REW {
        _CAP3REW { w: self }
    }
    #[doc = "Bit 10 - High to low. Capture on CAPn.3 falling edge"]
    #[inline]
    pub fn cap3fe(&mut self) -> _CAP3FEW {
        _CAP3FEW { w: self }
    }
    #[doc = "Bit 11 - Interrupt on CAPn.3 event:"]
    #[inline]
    pub fn cap3i(&mut self) -> _CAP3IW {
        _CAP3IW { w: self }
    }
}
