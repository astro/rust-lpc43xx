#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STATICCONFIG {
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
#[doc = "Possible values of the field `MW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MWR {
    #[doc = "8 bit (POR reset value)."]
    _8_BIT,
    #[doc = "16 bit."]
    _16_BIT,
    #[doc = "32 bit."]
    _32_BIT,
}
impl MWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MWR::_8_BIT => 0,
            MWR::_16_BIT => 1,
            MWR::_32_BIT => 2,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MWR {
        match value {
            0 => MWR::_8_BIT,
            1 => MWR::_16_BIT,
            2 => MWR::_32_BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline]
    pub fn is_8_bit(&self) -> bool {
        *self == MWR::_8_BIT
    }
    #[doc = "Checks if the value of the field is `_16_BIT`"]
    #[inline]
    pub fn is_16_bit(&self) -> bool {
        *self == MWR::_16_BIT
    }
    #[doc = "Checks if the value of the field is `_32_BIT`"]
    #[inline]
    pub fn is_32_bit(&self) -> bool {
        *self == MWR::_32_BIT
    }
}
#[doc = "Possible values of the field `PM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMR {
    #[doc = "Disabled. (POR reset value.)"]
    DISABLED,
    #[doc = "Enabled. Async page mode enabled (page length four)."]
    ENABLED,
}
impl PMR {
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
            PMR::DISABLED => false,
            PMR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PMR {
        match value {
            false => PMR::DISABLED,
            true => PMR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PMR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PMR::ENABLED
    }
}
#[doc = "Possible values of the field `PC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCR {
    #[doc = "Active LOW chip select."]
    ACTIVE_LOW,
    #[doc = "Active HIGH chip select."]
    ACTIVE_HIGH,
}
impl PCR {
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
            PCR::ACTIVE_LOW => false,
            PCR::ACTIVE_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PCR {
        match value {
            false => PCR::ACTIVE_LOW,
            true => PCR::ACTIVE_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE_LOW`"]
    #[inline]
    pub fn is_active_low(&self) -> bool {
        *self == PCR::ACTIVE_LOW
    }
    #[doc = "Checks if the value of the field is `ACTIVE_HIGH`"]
    #[inline]
    pub fn is_active_high(&self) -> bool {
        *self == PCR::ACTIVE_HIGH
    }
}
#[doc = "Possible values of the field `PB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBR {
    #[doc = "High. For reads all the bits in BLSn[3:0] are HIGH. For writes the respective active bits in BLSn[3:0] are LOW (POR reset value)."]
    HIGH,
    #[doc = "Low. For reads the respective active bits in BLSn[3:0] are LOW. For writes the respective active bits in BLSn[3:0] are LOW."]
    LOW,
}
impl PBR {
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
            PBR::HIGH => false,
            PBR::LOW => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PBR {
        match value {
            false => PBR::HIGH,
            true => PBR::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PBR::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PBR::LOW
    }
}
#[doc = "Possible values of the field `EW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWR {
    #[doc = "Disabled. Extended wait disabled (POR reset value)."]
    DISABLED,
    #[doc = "Enabled. Extended wait enabled."]
    ENABLED,
}
impl EWR {
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
            EWR::DISABLED => false,
            EWR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EWR {
        match value {
            false => EWR::DISABLED,
            true => EWR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == EWR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == EWR::ENABLED
    }
}
#[doc = "Possible values of the field `B`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BR {
    #[doc = "Disabled. Buffer disabled (POR reset value)."]
    DISABLED,
    #[doc = "Enabled. Buffer enabled."]
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
    #[doc = "Protect. Write protected."]
    PROTECT,
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
            PR::PROTECT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PR {
        match value {
            false => PR::NONE,
            true => PR::PROTECT,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == PR::NONE
    }
    #[doc = "Checks if the value of the field is `PROTECT`"]
    #[inline]
    pub fn is_protect(&self) -> bool {
        *self == PR::PROTECT
    }
}
#[doc = "Values that can be written to the field `MW`"]
pub enum MWW {
    #[doc = "8 bit (POR reset value)."]
    _8_BIT,
    #[doc = "16 bit."]
    _16_BIT,
    #[doc = "32 bit."]
    _32_BIT,
}
impl MWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MWW::_8_BIT => 0,
            MWW::_16_BIT => 1,
            MWW::_32_BIT => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MWW<'a> {
    w: &'a mut W,
}
impl<'a> _MWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MWW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "8 bit (POR reset value)."]
    #[inline]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(MWW::_8_BIT)
    }
    #[doc = "16 bit."]
    #[inline]
    pub fn _16_bit(self) -> &'a mut W {
        self.variant(MWW::_16_BIT)
    }
    #[doc = "32 bit."]
    #[inline]
    pub fn _32_bit(self) -> &'a mut W {
        self.variant(MWW::_32_BIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PM`"]
pub enum PMW {
    #[doc = "Disabled. (POR reset value.)"]
    DISABLED,
    #[doc = "Enabled. Async page mode enabled (page length four)."]
    ENABLED,
}
impl PMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PMW::DISABLED => false,
            PMW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PMW<'a> {
    w: &'a mut W,
}
impl<'a> _PMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. (POR reset value.)"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PMW::DISABLED)
    }
    #[doc = "Enabled. Async page mode enabled (page length four)."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PMW::ENABLED)
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
#[doc = "Values that can be written to the field `PC`"]
pub enum PCW {
    #[doc = "Active LOW chip select."]
    ACTIVE_LOW,
    #[doc = "Active HIGH chip select."]
    ACTIVE_HIGH,
}
impl PCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PCW::ACTIVE_LOW => false,
            PCW::ACTIVE_HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCW<'a> {
    w: &'a mut W,
}
impl<'a> _PCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Active LOW chip select."]
    #[inline]
    pub fn active_low(self) -> &'a mut W {
        self.variant(PCW::ACTIVE_LOW)
    }
    #[doc = "Active HIGH chip select."]
    #[inline]
    pub fn active_high(self) -> &'a mut W {
        self.variant(PCW::ACTIVE_HIGH)
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
#[doc = "Values that can be written to the field `PB`"]
pub enum PBW {
    #[doc = "High. For reads all the bits in BLSn[3:0] are HIGH. For writes the respective active bits in BLSn[3:0] are LOW (POR reset value)."]
    HIGH,
    #[doc = "Low. For reads the respective active bits in BLSn[3:0] are LOW. For writes the respective active bits in BLSn[3:0] are LOW."]
    LOW,
}
impl PBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PBW::HIGH => false,
            PBW::LOW => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PBW<'a> {
    w: &'a mut W,
}
impl<'a> _PBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "High. For reads all the bits in BLSn[3:0] are HIGH. For writes the respective active bits in BLSn[3:0] are LOW (POR reset value)."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PBW::HIGH)
    }
    #[doc = "Low. For reads the respective active bits in BLSn[3:0] are LOW. For writes the respective active bits in BLSn[3:0] are LOW."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PBW::LOW)
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
#[doc = "Values that can be written to the field `EW`"]
pub enum EWW {
    #[doc = "Disabled. Extended wait disabled (POR reset value)."]
    DISABLED,
    #[doc = "Enabled. Extended wait enabled."]
    ENABLED,
}
impl EWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EWW::DISABLED => false,
            EWW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EWW<'a> {
    w: &'a mut W,
}
impl<'a> _EWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Extended wait disabled (POR reset value)."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWW::DISABLED)
    }
    #[doc = "Enabled. Extended wait enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWW::ENABLED)
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
#[doc = "Values that can be written to the field `B`"]
pub enum BW {
    #[doc = "Disabled. Buffer disabled (POR reset value)."]
    DISABLED,
    #[doc = "Enabled. Buffer enabled."]
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
    #[doc = "Disabled. Buffer disabled (POR reset value)."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BW::DISABLED)
    }
    #[doc = "Enabled. Buffer enabled."]
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
    #[doc = "Protect. Write protected."]
    PROTECT,
}
impl PW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PW::NONE => false,
            PW::PROTECT => true,
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
    #[doc = "Protect. Write protected."]
    #[inline]
    pub fn protect(self) -> &'a mut W {
        self.variant(PW::PROTECT)
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
    #[doc = "Bits 0:1 - Memory width."]
    #[inline]
    pub fn mw(&self) -> MWR {
        MWR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Page mode. In page mode the EMC can burst up to four external accesses. Therefore devices with asynchronous page mode burst four or higher devices are supported. Asynchronous page mode burst two devices are not supported and must be accessed normally."]
    #[inline]
    pub fn pm(&self) -> PMR {
        PMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Chip select polarity. The value of the chip select polarity on power-on reset is 0."]
    #[inline]
    pub fn pc(&self) -> PCR {
        PCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Byte lane state. The byte lane state bit, PB, enables different types of memory to be connected. For byte-wide static memories the BLSn[3:0] signal from the EMC is usually connected to WE (write enable). In this case for reads all the BLSn[3:0] bits must be HIGH. This means that the byte lane state (PB) bit must be LOW. 16 bit wide static memory devices usually have the BLSn[3:0] signals connected to the UBn and LBn (upper byte and lower byte) signals in the static memory. In this case a write to a particular byte must assert the appropriate UBn or LBn signal LOW. For reads, all the UB and LB signals must be asserted LOW so that the bus is driven. In this case the byte lane state (PB) bit must be HIGH. When PB is set to 0, the WE signal is undefined or 0. You must set PB to 1, to use the WE signal."]
    #[inline]
    pub fn pb(&self) -> PBR {
        PBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Extended wait. Extended wait (EW) uses the StaticExtendedWait register to time both the read and write transfers rather than the StaticWaitRd and StaticWaitWr registers. This enables much longer transactions.[1]"]
    #[inline]
    pub fn ew(&self) -> EWR {
        EWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Buffer enable [2]."]
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
    #[doc = "Bits 0:1 - Memory width."]
    #[inline]
    pub fn mw(&mut self) -> _MWW {
        _MWW { w: self }
    }
    #[doc = "Bit 3 - Page mode. In page mode the EMC can burst up to four external accesses. Therefore devices with asynchronous page mode burst four or higher devices are supported. Asynchronous page mode burst two devices are not supported and must be accessed normally."]
    #[inline]
    pub fn pm(&mut self) -> _PMW {
        _PMW { w: self }
    }
    #[doc = "Bit 6 - Chip select polarity. The value of the chip select polarity on power-on reset is 0."]
    #[inline]
    pub fn pc(&mut self) -> _PCW {
        _PCW { w: self }
    }
    #[doc = "Bit 7 - Byte lane state. The byte lane state bit, PB, enables different types of memory to be connected. For byte-wide static memories the BLSn[3:0] signal from the EMC is usually connected to WE (write enable). In this case for reads all the BLSn[3:0] bits must be HIGH. This means that the byte lane state (PB) bit must be LOW. 16 bit wide static memory devices usually have the BLSn[3:0] signals connected to the UBn and LBn (upper byte and lower byte) signals in the static memory. In this case a write to a particular byte must assert the appropriate UBn or LBn signal LOW. For reads, all the UB and LB signals must be asserted LOW so that the bus is driven. In this case the byte lane state (PB) bit must be HIGH. When PB is set to 0, the WE signal is undefined or 0. You must set PB to 1, to use the WE signal."]
    #[inline]
    pub fn pb(&mut self) -> _PBW {
        _PBW { w: self }
    }
    #[doc = "Bit 8 - Extended wait. Extended wait (EW) uses the StaticExtendedWait register to time both the read and write transfers rather than the StaticWaitRd and StaticWaitWr registers. This enables much longer transactions.[1]"]
    #[inline]
    pub fn ew(&mut self) -> _EWW {
        _EWW { w: self }
    }
    #[doc = "Bit 19 - Buffer enable [2]."]
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
