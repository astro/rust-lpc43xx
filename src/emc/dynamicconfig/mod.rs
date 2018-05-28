#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DYNAMICCONFIG {
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
#[doc = "Possible values of the field `MD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDR {
    #[doc = "SDRAM (POR reset value)."]
    SDRAM,
}
impl MDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MDR::SDRAM => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MDR {
        match value {
            0 => MDR::SDRAM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SDRAM`"]
    #[inline]
    pub fn is_sdram(&self) -> bool {
        *self == MDR::SDRAM
    }
}
#[doc = r" Value of the field"]
pub struct AM0R {
    bits: u8,
}
impl AM0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AM1R {
    bits: bool,
}
impl AM1R {
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
#[doc = "Possible values of the field `B`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BR {
    #[doc = "Disabled. Buffer disabled for accesses to this chip select (POR reset value)."]
    DISABLED,
    #[doc = "Enabled. Buffer enabled for accesses to this chip select. After configuration of the dynamic memory, the buffer must be enabled for normal operation. [2]"]
    ENABLED,
}
impl BR {
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
            BR::DISABLED => false,
            BR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BR {
        match value {
            false => BR::DISABLED,
            true => BR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == BR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == BR::ENABLED
    }
}
#[doc = "Possible values of the field `P`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PR {
    #[doc = "None. Writes not protected (POR reset value)."]
    NONE,
    #[doc = "Protected. Writes protected."]
    PROTECTED,
}
impl PR {
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
            PR::NONE => false,
            PR::PROTECTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PR {
        match value {
            false => PR::NONE,
            true => PR::PROTECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == PR::NONE
    }
    #[doc = "Checks if the value of the field is `PROTECTED`"]
    #[inline]
    pub fn is_protected(&self) -> bool {
        *self == PR::PROTECTED
    }
}
#[doc = "Values that can be written to the field `MD`"]
pub enum MDW {
    #[doc = "SDRAM (POR reset value)."]
    SDRAM,
}
impl MDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MDW::SDRAM => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MDW<'a> {
    w: &'a mut W,
}
impl<'a> _MDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SDRAM (POR reset value)."]
    #[inline]
    pub fn sdram(self) -> &'a mut W {
        self.variant(MDW::SDRAM)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AM0W<'a> {
    w: &'a mut W,
}
impl<'a> _AM0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AM1W<'a> {
    w: &'a mut W,
}
impl<'a> _AM1W<'a> {
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `B`"]
pub enum BW {
    #[doc = "Disabled. Buffer disabled for accesses to this chip select (POR reset value)."]
    DISABLED,
    #[doc = "Enabled. Buffer enabled for accesses to this chip select. After configuration of the dynamic memory, the buffer must be enabled for normal operation. [2]"]
    ENABLED,
}
impl BW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BW::DISABLED => false,
            BW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BW<'a> {
    w: &'a mut W,
}
impl<'a> _BW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Buffer disabled for accesses to this chip select (POR reset value)."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BW::DISABLED)
    }
    #[doc = "Enabled. Buffer enabled for accesses to this chip select. After configuration of the dynamic memory, the buffer must be enabled for normal operation. [2]"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BW::ENABLED)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P`"]
pub enum PW {
    #[doc = "None. Writes not protected (POR reset value)."]
    NONE,
    #[doc = "Protected. Writes protected."]
    PROTECTED,
}
impl PW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PW::NONE => false,
            PW::PROTECTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PW<'a> {
    w: &'a mut W,
}
impl<'a> _PW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "None. Writes not protected (POR reset value)."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(PW::NONE)
    }
    #[doc = "Protected. Writes protected."]
    #[inline]
    pub fn protected(self) -> &'a mut W {
        self.variant(PW::PROTECTED)
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
        const OFFSET: u8 = 20;
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
    #[doc = "Bits 3:4 - Memory device."]
    #[inline]
    pub fn md(&self) -> MDR {
        MDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 7:12 - Address mapping. See Table 382. 000000 = reset value.[1]"]
    #[inline]
    pub fn am0(&self) -> AM0R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AM0R { bits }
    }
    #[doc = "Bit 14 - Address mapping See Table 382. 0 = reset value."]
    #[inline]
    pub fn am1(&self) -> AM1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AM1R { bits }
    }
    #[doc = "Bit 19 - Buffer enable."]
    #[inline]
    pub fn b(&self) -> BR {
        BR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Write protect."]
    #[inline]
    pub fn p(&self) -> PR {
        PR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
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
    #[doc = "Bits 3:4 - Memory device."]
    #[inline]
    pub fn md(&mut self) -> _MDW {
        _MDW { w: self }
    }
    #[doc = "Bits 7:12 - Address mapping. See Table 382. 000000 = reset value.[1]"]
    #[inline]
    pub fn am0(&mut self) -> _AM0W {
        _AM0W { w: self }
    }
    #[doc = "Bit 14 - Address mapping See Table 382. 0 = reset value."]
    #[inline]
    pub fn am1(&mut self) -> _AM1W {
        _AM1W { w: self }
    }
    #[doc = "Bit 19 - Buffer enable."]
    #[inline]
    pub fn b(&mut self) -> _BW {
        _BW { w: self }
    }
    #[doc = "Bit 20 - Write protect."]
    #[inline]
    pub fn p(&mut self) -> _PW {
        _PW { w: self }
    }
}
