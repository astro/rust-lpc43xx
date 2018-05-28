#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::XTAL_OSC_CTRL {
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
#[doc = "Possible values of the field `ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLER {
    #[doc = "Enable"]
    ENABLE,
    #[doc = "Power-down (default)"]
    POWER_DOWN,
}
impl ENABLER {
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
            ENABLER::ENABLE => false,
            ENABLER::POWER_DOWN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENABLER {
        match value {
            false => ENABLER::ENABLE,
            true => ENABLER::POWER_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ENABLER::ENABLE
    }
    #[doc = "Checks if the value of the field is `POWER_DOWN`"]
    #[inline]
    pub fn is_power_down(&self) -> bool {
        *self == ENABLER::POWER_DOWN
    }
}
#[doc = "Possible values of the field `BYPASS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASSR {
    #[doc = "Crystal. Operation with crystal connected (default)."]
    CRYSTAL,
    #[doc = "Bypass mode. Use this mode when an external clock source is used instead of a crystal."]
    BYPASS_MODE,
}
impl BYPASSR {
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
            BYPASSR::CRYSTAL => false,
            BYPASSR::BYPASS_MODE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BYPASSR {
        match value {
            false => BYPASSR::CRYSTAL,
            true => BYPASSR::BYPASS_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `CRYSTAL`"]
    #[inline]
    pub fn is_crystal(&self) -> bool {
        *self == BYPASSR::CRYSTAL
    }
    #[doc = "Checks if the value of the field is `BYPASS_MODE`"]
    #[inline]
    pub fn is_bypass_mode(&self) -> bool {
        *self == BYPASSR::BYPASS_MODE
    }
}
#[doc = "Possible values of the field `HF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFR {
    #[doc = "Low. Oscillator low-frequency mode (crystal or external clock source 1 to 20 MHz). Between 15 MHz and 20 MHz, the state of the HF bit is don't care."]
    LOW,
    #[doc = "High. Oscillator high-frequency mode; crystal or external clock source 15 to 25 MHz. Between 15 MHz and 20 MHz, the state of the HF bit is don't care."]
    HIGH,
}
impl HFR {
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
            HFR::LOW => false,
            HFR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HFR {
        match value {
            false => HFR::LOW,
            true => HFR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == HFR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == HFR::HIGH
    }
}
#[doc = "Values that can be written to the field `ENABLE`"]
pub enum ENABLEW {
    #[doc = "Enable"]
    ENABLE,
    #[doc = "Power-down (default)"]
    POWER_DOWN,
}
impl ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENABLEW::ENABLE => false,
            ENABLEW::POWER_DOWN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLEW::ENABLE)
    }
    #[doc = "Power-down (default)"]
    #[inline]
    pub fn power_down(self) -> &'a mut W {
        self.variant(ENABLEW::POWER_DOWN)
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
#[doc = "Values that can be written to the field `BYPASS`"]
pub enum BYPASSW {
    #[doc = "Crystal. Operation with crystal connected (default)."]
    CRYSTAL,
    #[doc = "Bypass mode. Use this mode when an external clock source is used instead of a crystal."]
    BYPASS_MODE,
}
impl BYPASSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BYPASSW::CRYSTAL => false,
            BYPASSW::BYPASS_MODE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _BYPASSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BYPASSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Crystal. Operation with crystal connected (default)."]
    #[inline]
    pub fn crystal(self) -> &'a mut W {
        self.variant(BYPASSW::CRYSTAL)
    }
    #[doc = "Bypass mode. Use this mode when an external clock source is used instead of a crystal."]
    #[inline]
    pub fn bypass_mode(self) -> &'a mut W {
        self.variant(BYPASSW::BYPASS_MODE)
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
#[doc = "Values that can be written to the field `HF`"]
pub enum HFW {
    #[doc = "Low. Oscillator low-frequency mode (crystal or external clock source 1 to 20 MHz). Between 15 MHz and 20 MHz, the state of the HF bit is don't care."]
    LOW,
    #[doc = "High. Oscillator high-frequency mode; crystal or external clock source 15 to 25 MHz. Between 15 MHz and 20 MHz, the state of the HF bit is don't care."]
    HIGH,
}
impl HFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HFW::LOW => false,
            HFW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HFW<'a> {
    w: &'a mut W,
}
impl<'a> _HFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low. Oscillator low-frequency mode (crystal or external clock source 1 to 20 MHz). Between 15 MHz and 20 MHz, the state of the HF bit is don't care."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(HFW::LOW)
    }
    #[doc = "High. Oscillator high-frequency mode; crystal or external clock source 15 to 25 MHz. Between 15 MHz and 20 MHz, the state of the HF bit is don't care."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(HFW::HIGH)
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
    #[doc = "Bit 0 - Oscillator-pad enable. Do not change the BYPASS and ENABLE bits in one write-action: this will result in unstable device operation!"]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Configure crystal operation or external-clock input pin XTAL1. Do not change the BYPASS and ENABLE bits in one write-action: this will result in unstable device operation!"]
    #[inline]
    pub fn bypass(&self) -> BYPASSR {
        BYPASSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Select frequency range"]
    #[inline]
    pub fn hf(&self) -> HFR {
        HFR::_from({
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
        W { bits: 5 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Oscillator-pad enable. Do not change the BYPASS and ENABLE bits in one write-action: this will result in unstable device operation!"]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bit 1 - Configure crystal operation or external-clock input pin XTAL1. Do not change the BYPASS and ENABLE bits in one write-action: this will result in unstable device operation!"]
    #[inline]
    pub fn bypass(&mut self) -> _BYPASSW {
        _BYPASSW { w: self }
    }
    #[doc = "Bit 2 - Select frequency range"]
    #[inline]
    pub fn hf(&mut self) -> _HFW {
        _HFW { w: self }
    }
}
