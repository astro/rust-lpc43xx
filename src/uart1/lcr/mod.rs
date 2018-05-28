#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LCR {
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
#[doc = "Possible values of the field `WLS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WLSR {
    #[doc = "5-bit character length."]
    _5_BIT_CHARACTER_LENG,
    #[doc = "6-bit character length."]
    _6_BIT_CHARACTER_LENG,
    #[doc = "7-bit character length."]
    _7_BIT_CHARACTER_LENG,
    #[doc = "8-bit character length."]
    _8_BIT_CHARACTER_LENG,
}
impl WLSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WLSR::_5_BIT_CHARACTER_LENG => 0,
            WLSR::_6_BIT_CHARACTER_LENG => 1,
            WLSR::_7_BIT_CHARACTER_LENG => 2,
            WLSR::_8_BIT_CHARACTER_LENG => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WLSR {
        match value {
            0 => WLSR::_5_BIT_CHARACTER_LENG,
            1 => WLSR::_6_BIT_CHARACTER_LENG,
            2 => WLSR::_7_BIT_CHARACTER_LENG,
            3 => WLSR::_8_BIT_CHARACTER_LENG,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_5_BIT_CHARACTER_LENG`"]
    #[inline]
    pub fn is_5_bit_character_leng(&self) -> bool {
        *self == WLSR::_5_BIT_CHARACTER_LENG
    }
    #[doc = "Checks if the value of the field is `_6_BIT_CHARACTER_LENG`"]
    #[inline]
    pub fn is_6_bit_character_leng(&self) -> bool {
        *self == WLSR::_6_BIT_CHARACTER_LENG
    }
    #[doc = "Checks if the value of the field is `_7_BIT_CHARACTER_LENG`"]
    #[inline]
    pub fn is_7_bit_character_leng(&self) -> bool {
        *self == WLSR::_7_BIT_CHARACTER_LENG
    }
    #[doc = "Checks if the value of the field is `_8_BIT_CHARACTER_LENG`"]
    #[inline]
    pub fn is_8_bit_character_leng(&self) -> bool {
        *self == WLSR::_8_BIT_CHARACTER_LENG
    }
}
#[doc = "Possible values of the field `SBS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBSR {
    #[doc = "1 stop bit."]
    _1_STOP_BIT,
    #[doc = "2 stop bits. (1.5 if LCR[1:0]=00)."]
    _2_STOP_BITS,
}
impl SBSR {
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
            SBSR::_1_STOP_BIT => false,
            SBSR::_2_STOP_BITS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SBSR {
        match value {
            false => SBSR::_1_STOP_BIT,
            true => SBSR::_2_STOP_BITS,
        }
    }
    #[doc = "Checks if the value of the field is `_1_STOP_BIT`"]
    #[inline]
    pub fn is_1_stop_bit(&self) -> bool {
        *self == SBSR::_1_STOP_BIT
    }
    #[doc = "Checks if the value of the field is `_2_STOP_BITS`"]
    #[inline]
    pub fn is_2_stop_bits(&self) -> bool {
        *self == SBSR::_2_STOP_BITS
    }
}
#[doc = "Possible values of the field `PE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PER {
    #[doc = "Disable parity generation and checking."]
    DISABLE_PARITY_GENER,
    #[doc = "Enable parity generation and checking."]
    ENABLE_PARITY_GENERA,
}
impl PER {
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
            PER::DISABLE_PARITY_GENER => false,
            PER::ENABLE_PARITY_GENERA => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PER {
        match value {
            false => PER::DISABLE_PARITY_GENER,
            true => PER::ENABLE_PARITY_GENERA,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_PARITY_GENER`"]
    #[inline]
    pub fn is_disable_parity_gener(&self) -> bool {
        *self == PER::DISABLE_PARITY_GENER
    }
    #[doc = "Checks if the value of the field is `ENABLE_PARITY_GENERA`"]
    #[inline]
    pub fn is_enable_parity_genera(&self) -> bool {
        *self == PER::ENABLE_PARITY_GENERA
    }
}
#[doc = "Possible values of the field `PS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSR {
    #[doc = "Odd parity. Number of 1s in the transmitted character and the attached parity bit will be odd."]
    ODD_PARITY,
    #[doc = "Even Parity. Number of 1s in the transmitted character and the attached parity bit will be even."]
    EVEN_PARITY,
    #[doc = "Force HIGH. Forced 1 stick parity."]
    FORCE_HIGH,
    #[doc = "Force LOW. Forced 0 stick parity."]
    FORCE_LOW,
}
impl PSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSR::ODD_PARITY => 0,
            PSR::EVEN_PARITY => 1,
            PSR::FORCE_HIGH => 2,
            PSR::FORCE_LOW => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSR {
        match value {
            0 => PSR::ODD_PARITY,
            1 => PSR::EVEN_PARITY,
            2 => PSR::FORCE_HIGH,
            3 => PSR::FORCE_LOW,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ODD_PARITY`"]
    #[inline]
    pub fn is_odd_parity(&self) -> bool {
        *self == PSR::ODD_PARITY
    }
    #[doc = "Checks if the value of the field is `EVEN_PARITY`"]
    #[inline]
    pub fn is_even_parity(&self) -> bool {
        *self == PSR::EVEN_PARITY
    }
    #[doc = "Checks if the value of the field is `FORCE_HIGH`"]
    #[inline]
    pub fn is_force_high(&self) -> bool {
        *self == PSR::FORCE_HIGH
    }
    #[doc = "Checks if the value of the field is `FORCE_LOW`"]
    #[inline]
    pub fn is_force_low(&self) -> bool {
        *self == PSR::FORCE_LOW
    }
}
#[doc = "Possible values of the field `BC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCR {
    #[doc = "Disabled. Disable break transmission."]
    DISABLED,
    #[doc = "Enabled. Enable break transmission. Output pin UART1 TXD is forced to logic 0 when LCR[6] is active high."]
    ENABLED,
}
impl BCR {
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
            BCR::DISABLED => false,
            BCR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BCR {
        match value {
            false => BCR::DISABLED,
            true => BCR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == BCR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == BCR::ENABLED
    }
}
#[doc = "Possible values of the field `DLAB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLABR {
    #[doc = "Disabled. Disable access to Divisor Latches."]
    DISABLED,
    #[doc = "Enabled. Enable access to Divisor Latches."]
    ENABLED,
}
impl DLABR {
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
            DLABR::DISABLED => false,
            DLABR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DLABR {
        match value {
            false => DLABR::DISABLED,
            true => DLABR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DLABR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DLABR::ENABLED
    }
}
#[doc = "Values that can be written to the field `WLS`"]
pub enum WLSW {
    #[doc = "5-bit character length."]
    _5_BIT_CHARACTER_LENG,
    #[doc = "6-bit character length."]
    _6_BIT_CHARACTER_LENG,
    #[doc = "7-bit character length."]
    _7_BIT_CHARACTER_LENG,
    #[doc = "8-bit character length."]
    _8_BIT_CHARACTER_LENG,
}
impl WLSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WLSW::_5_BIT_CHARACTER_LENG => 0,
            WLSW::_6_BIT_CHARACTER_LENG => 1,
            WLSW::_7_BIT_CHARACTER_LENG => 2,
            WLSW::_8_BIT_CHARACTER_LENG => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WLSW<'a> {
    w: &'a mut W,
}
impl<'a> _WLSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WLSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "5-bit character length."]
    #[inline]
    pub fn _5_bit_character_leng(self) -> &'a mut W {
        self.variant(WLSW::_5_BIT_CHARACTER_LENG)
    }
    #[doc = "6-bit character length."]
    #[inline]
    pub fn _6_bit_character_leng(self) -> &'a mut W {
        self.variant(WLSW::_6_BIT_CHARACTER_LENG)
    }
    #[doc = "7-bit character length."]
    #[inline]
    pub fn _7_bit_character_leng(self) -> &'a mut W {
        self.variant(WLSW::_7_BIT_CHARACTER_LENG)
    }
    #[doc = "8-bit character length."]
    #[inline]
    pub fn _8_bit_character_leng(self) -> &'a mut W {
        self.variant(WLSW::_8_BIT_CHARACTER_LENG)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SBS`"]
pub enum SBSW {
    #[doc = "1 stop bit."]
    _1_STOP_BIT,
    #[doc = "2 stop bits. (1.5 if LCR[1:0]=00)."]
    _2_STOP_BITS,
}
impl SBSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SBSW::_1_STOP_BIT => false,
            SBSW::_2_STOP_BITS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SBSW<'a> {
    w: &'a mut W,
}
impl<'a> _SBSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SBSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "1 stop bit."]
    #[inline]
    pub fn _1_stop_bit(self) -> &'a mut W {
        self.variant(SBSW::_1_STOP_BIT)
    }
    #[doc = "2 stop bits. (1.5 if LCR[1:0]=00)."]
    #[inline]
    pub fn _2_stop_bits(self) -> &'a mut W {
        self.variant(SBSW::_2_STOP_BITS)
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
#[doc = "Values that can be written to the field `PE`"]
pub enum PEW {
    #[doc = "Disable parity generation and checking."]
    DISABLE_PARITY_GENER,
    #[doc = "Enable parity generation and checking."]
    ENABLE_PARITY_GENERA,
}
impl PEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEW::DISABLE_PARITY_GENER => false,
            PEW::ENABLE_PARITY_GENERA => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEW<'a> {
    w: &'a mut W,
}
impl<'a> _PEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable parity generation and checking."]
    #[inline]
    pub fn disable_parity_gener(self) -> &'a mut W {
        self.variant(PEW::DISABLE_PARITY_GENER)
    }
    #[doc = "Enable parity generation and checking."]
    #[inline]
    pub fn enable_parity_genera(self) -> &'a mut W {
        self.variant(PEW::ENABLE_PARITY_GENERA)
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
#[doc = "Values that can be written to the field `PS`"]
pub enum PSW {
    #[doc = "Odd parity. Number of 1s in the transmitted character and the attached parity bit will be odd."]
    ODD_PARITY,
    #[doc = "Even Parity. Number of 1s in the transmitted character and the attached parity bit will be even."]
    EVEN_PARITY,
    #[doc = "Force HIGH. Forced 1 stick parity."]
    FORCE_HIGH,
    #[doc = "Force LOW. Forced 0 stick parity."]
    FORCE_LOW,
}
impl PSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSW::ODD_PARITY => 0,
            PSW::EVEN_PARITY => 1,
            PSW::FORCE_HIGH => 2,
            PSW::FORCE_LOW => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSW<'a> {
    w: &'a mut W,
}
impl<'a> _PSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Odd parity. Number of 1s in the transmitted character and the attached parity bit will be odd."]
    #[inline]
    pub fn odd_parity(self) -> &'a mut W {
        self.variant(PSW::ODD_PARITY)
    }
    #[doc = "Even Parity. Number of 1s in the transmitted character and the attached parity bit will be even."]
    #[inline]
    pub fn even_parity(self) -> &'a mut W {
        self.variant(PSW::EVEN_PARITY)
    }
    #[doc = "Force HIGH. Forced 1 stick parity."]
    #[inline]
    pub fn force_high(self) -> &'a mut W {
        self.variant(PSW::FORCE_HIGH)
    }
    #[doc = "Force LOW. Forced 0 stick parity."]
    #[inline]
    pub fn force_low(self) -> &'a mut W {
        self.variant(PSW::FORCE_LOW)
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
#[doc = "Values that can be written to the field `BC`"]
pub enum BCW {
    #[doc = "Disabled. Disable break transmission."]
    DISABLED,
    #[doc = "Enabled. Enable break transmission. Output pin UART1 TXD is forced to logic 0 when LCR[6] is active high."]
    ENABLED,
}
impl BCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BCW::DISABLED => false,
            BCW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BCW<'a> {
    w: &'a mut W,
}
impl<'a> _BCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Disable break transmission."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BCW::DISABLED)
    }
    #[doc = "Enabled. Enable break transmission. Output pin UART1 TXD is forced to logic 0 when LCR[6] is active high."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BCW::ENABLED)
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
#[doc = "Values that can be written to the field `DLAB`"]
pub enum DLABW {
    #[doc = "Disabled. Disable access to Divisor Latches."]
    DISABLED,
    #[doc = "Enabled. Enable access to Divisor Latches."]
    ENABLED,
}
impl DLABW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DLABW::DISABLED => false,
            DLABW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DLABW<'a> {
    w: &'a mut W,
}
impl<'a> _DLABW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DLABW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Disable access to Divisor Latches."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DLABW::DISABLED)
    }
    #[doc = "Enabled. Enable access to Divisor Latches."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DLABW::ENABLED)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Word Length Select."]
    #[inline]
    pub fn wls(&self) -> WLSR {
        WLSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Stop Bit Select."]
    #[inline]
    pub fn sbs(&self) -> SBSR {
        SBSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Parity Enable."]
    #[inline]
    pub fn pe(&self) -> PER {
        PER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - Parity Select."]
    #[inline]
    pub fn ps(&self) -> PSR {
        PSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - Break Control."]
    #[inline]
    pub fn bc(&self) -> BCR {
        BCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit (DLAB)"]
    #[inline]
    pub fn dlab(&self) -> DLABR {
        DLABR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
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
    #[doc = "Bits 0:1 - Word Length Select."]
    #[inline]
    pub fn wls(&mut self) -> _WLSW {
        _WLSW { w: self }
    }
    #[doc = "Bit 2 - Stop Bit Select."]
    #[inline]
    pub fn sbs(&mut self) -> _SBSW {
        _SBSW { w: self }
    }
    #[doc = "Bit 3 - Parity Enable."]
    #[inline]
    pub fn pe(&mut self) -> _PEW {
        _PEW { w: self }
    }
    #[doc = "Bits 4:5 - Parity Select."]
    #[inline]
    pub fn ps(&mut self) -> _PSW {
        _PSW { w: self }
    }
    #[doc = "Bit 6 - Break Control."]
    #[inline]
    pub fn bc(&mut self) -> _BCW {
        _BCW { w: self }
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit (DLAB)"]
    #[inline]
    pub fn dlab(&mut self) -> _DLABW {
        _DLABW { w: self }
    }
}
