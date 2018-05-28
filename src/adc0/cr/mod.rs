#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR {
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
pub struct SELR {
    bits: u8,
}
impl SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CLKDIVR {
    bits: u8,
}
impl CLKDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `BURST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURSTR {
    #[doc = "Conversions are software controlled and require 11 clocks."]
    SOFTWARE,
    #[doc = "The AD converter does repeated conversions at the rate selected by the CLKS field, scanning (if necessary) through the pins selected by 1s in the SEL field. The first conversion after the start corresponds to the least-significant 1 in the SEL field, then higher numbered 1 bits (pins) if applicable. Repeated conversions can be terminated by clearing this bit, but the conversion that is  in progress when this bit is cleared will be completed. Important: START bits must be 000 when BURST = 1 or conversions will not start."]
    BURST,
}
impl BURSTR {
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
            BURSTR::SOFTWARE => false,
            BURSTR::BURST => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BURSTR {
        match value {
            false => BURSTR::SOFTWARE,
            true => BURSTR::BURST,
        }
    }
    #[doc = "Checks if the value of the field is `SOFTWARE`"]
    #[inline]
    pub fn is_software(&self) -> bool {
        *self == BURSTR::SOFTWARE
    }
    #[doc = "Checks if the value of the field is `BURST`"]
    #[inline]
    pub fn is_burst(&self) -> bool {
        *self == BURSTR::BURST
    }
}
#[doc = "Possible values of the field `CLKS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSR {
    #[doc = "11 clocks / 10 bits"]
    _11_CLOCKS_10_BITS,
    #[doc = "10 clocks / 9 bits"]
    _10_CLOCKS_9_BITS,
    #[doc = "9 clocks / 8 bits"]
    _9_CLOCKS_8_BITS,
    #[doc = "8 clocks / 7 bits"]
    _8_CLOCKS_7_BITS,
    #[doc = "7 clocks / 6 bits"]
    _7_CLOCKS_6_BITS,
    #[doc = "6 clocks / 5 bits"]
    _6_CLOCKS_5_BITS,
    #[doc = "5 clocks / 4 bits"]
    _5_CLOCKS_4_BITS,
    #[doc = "4 clocks / 3 bits"]
    _4_CLOCKS_3_BITS,
}
impl CLKSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKSR::_11_CLOCKS_10_BITS => 0,
            CLKSR::_10_CLOCKS_9_BITS => 1,
            CLKSR::_9_CLOCKS_8_BITS => 2,
            CLKSR::_8_CLOCKS_7_BITS => 3,
            CLKSR::_7_CLOCKS_6_BITS => 4,
            CLKSR::_6_CLOCKS_5_BITS => 5,
            CLKSR::_5_CLOCKS_4_BITS => 6,
            CLKSR::_4_CLOCKS_3_BITS => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKSR {
        match value {
            0 => CLKSR::_11_CLOCKS_10_BITS,
            1 => CLKSR::_10_CLOCKS_9_BITS,
            2 => CLKSR::_9_CLOCKS_8_BITS,
            3 => CLKSR::_8_CLOCKS_7_BITS,
            4 => CLKSR::_7_CLOCKS_6_BITS,
            5 => CLKSR::_6_CLOCKS_5_BITS,
            6 => CLKSR::_5_CLOCKS_4_BITS,
            7 => CLKSR::_4_CLOCKS_3_BITS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_11_CLOCKS_10_BITS`"]
    #[inline]
    pub fn is_11_clocks_10_bits(&self) -> bool {
        *self == CLKSR::_11_CLOCKS_10_BITS
    }
    #[doc = "Checks if the value of the field is `_10_CLOCKS_9_BITS`"]
    #[inline]
    pub fn is_10_clocks_9_bits(&self) -> bool {
        *self == CLKSR::_10_CLOCKS_9_BITS
    }
    #[doc = "Checks if the value of the field is `_9_CLOCKS_8_BITS`"]
    #[inline]
    pub fn is_9_clocks_8_bits(&self) -> bool {
        *self == CLKSR::_9_CLOCKS_8_BITS
    }
    #[doc = "Checks if the value of the field is `_8_CLOCKS_7_BITS`"]
    #[inline]
    pub fn is_8_clocks_7_bits(&self) -> bool {
        *self == CLKSR::_8_CLOCKS_7_BITS
    }
    #[doc = "Checks if the value of the field is `_7_CLOCKS_6_BITS`"]
    #[inline]
    pub fn is_7_clocks_6_bits(&self) -> bool {
        *self == CLKSR::_7_CLOCKS_6_BITS
    }
    #[doc = "Checks if the value of the field is `_6_CLOCKS_5_BITS`"]
    #[inline]
    pub fn is_6_clocks_5_bits(&self) -> bool {
        *self == CLKSR::_6_CLOCKS_5_BITS
    }
    #[doc = "Checks if the value of the field is `_5_CLOCKS_4_BITS`"]
    #[inline]
    pub fn is_5_clocks_4_bits(&self) -> bool {
        *self == CLKSR::_5_CLOCKS_4_BITS
    }
    #[doc = "Checks if the value of the field is `_4_CLOCKS_3_BITS`"]
    #[inline]
    pub fn is_4_clocks_3_bits(&self) -> bool {
        *self == CLKSR::_4_CLOCKS_3_BITS
    }
}
#[doc = "Possible values of the field `PDN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDNR {
    #[doc = "The A/D converter is in Power-down mode."]
    POWERDOWN,
    #[doc = "The A/D converter is operational."]
    RUNNING,
}
impl PDNR {
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
            PDNR::POWERDOWN => false,
            PDNR::RUNNING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDNR {
        match value {
            false => PDNR::POWERDOWN,
            true => PDNR::RUNNING,
        }
    }
    #[doc = "Checks if the value of the field is `POWERDOWN`"]
    #[inline]
    pub fn is_powerdown(&self) -> bool {
        *self == PDNR::POWERDOWN
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline]
    pub fn is_running(&self) -> bool {
        *self == PDNR::RUNNING
    }
}
#[doc = "Possible values of the field `START`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTR {
    #[doc = "No start (this value should be used when clearing PDN to 0)."]
    NO_START,
    #[doc = "Start conversion now."]
    START_CONVERSION_NOW,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CTOUT_15 (combined timer output 15)."]
    CTOUT_15,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CTOUT_8 (combined timer output 8)."]
    CTOUT_8,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on ADCTRIG0 input."]
    ADCTRIG0,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on ADCTRIG1 input."]
    ADCTRIG1,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on Motocon PWM output MCOA2."]
    MCOA2,
    #[doc = "Reserved."]
    RESERVED,
}
impl STARTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STARTR::NO_START => 0,
            STARTR::START_CONVERSION_NOW => 1,
            STARTR::CTOUT_15 => 2,
            STARTR::CTOUT_8 => 3,
            STARTR::ADCTRIG0 => 4,
            STARTR::ADCTRIG1 => 5,
            STARTR::MCOA2 => 6,
            STARTR::RESERVED => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STARTR {
        match value {
            0 => STARTR::NO_START,
            1 => STARTR::START_CONVERSION_NOW,
            2 => STARTR::CTOUT_15,
            3 => STARTR::CTOUT_8,
            4 => STARTR::ADCTRIG0,
            5 => STARTR::ADCTRIG1,
            6 => STARTR::MCOA2,
            7 => STARTR::RESERVED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_START`"]
    #[inline]
    pub fn is_no_start(&self) -> bool {
        *self == STARTR::NO_START
    }
    #[doc = "Checks if the value of the field is `START_CONVERSION_NOW`"]
    #[inline]
    pub fn is_start_conversion_now(&self) -> bool {
        *self == STARTR::START_CONVERSION_NOW
    }
    #[doc = "Checks if the value of the field is `CTOUT_15`"]
    #[inline]
    pub fn is_ctout_15(&self) -> bool {
        *self == STARTR::CTOUT_15
    }
    #[doc = "Checks if the value of the field is `CTOUT_8`"]
    #[inline]
    pub fn is_ctout_8(&self) -> bool {
        *self == STARTR::CTOUT_8
    }
    #[doc = "Checks if the value of the field is `ADCTRIG0`"]
    #[inline]
    pub fn is_adctrig0(&self) -> bool {
        *self == STARTR::ADCTRIG0
    }
    #[doc = "Checks if the value of the field is `ADCTRIG1`"]
    #[inline]
    pub fn is_adctrig1(&self) -> bool {
        *self == STARTR::ADCTRIG1
    }
    #[doc = "Checks if the value of the field is `MCOA2`"]
    #[inline]
    pub fn is_mcoa2(&self) -> bool {
        *self == STARTR::MCOA2
    }
    #[doc = "Checks if the value of the field is `RESERVED`"]
    #[inline]
    pub fn is_reserved(&self) -> bool {
        *self == STARTR::RESERVED
    }
}
#[doc = "Possible values of the field `EDGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGER {
    #[doc = "Start conversion on a rising edge on the selected signal."]
    RISING,
    #[doc = "Start conversion on a falling edge on the selected signal."]
    FALLING,
}
impl EDGER {
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
            EDGER::RISING => false,
            EDGER::FALLING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDGER {
        match value {
            false => EDGER::RISING,
            true => EDGER::FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == EDGER::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == EDGER::FALLING
    }
}
#[doc = r" Proxy"]
pub struct _SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLKDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BURST`"]
pub enum BURSTW {
    #[doc = "Conversions are software controlled and require 11 clocks."]
    SOFTWARE,
    #[doc = "The AD converter does repeated conversions at the rate selected by the CLKS field, scanning (if necessary) through the pins selected by 1s in the SEL field. The first conversion after the start corresponds to the least-significant 1 in the SEL field, then higher numbered 1 bits (pins) if applicable. Repeated conversions can be terminated by clearing this bit, but the conversion that is  in progress when this bit is cleared will be completed. Important: START bits must be 000 when BURST = 1 or conversions will not start."]
    BURST,
}
impl BURSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BURSTW::SOFTWARE => false,
            BURSTW::BURST => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BURSTW<'a> {
    w: &'a mut W,
}
impl<'a> _BURSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BURSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Conversions are software controlled and require 11 clocks."]
    #[inline]
    pub fn software(self) -> &'a mut W {
        self.variant(BURSTW::SOFTWARE)
    }
    #[doc = "The AD converter does repeated conversions at the rate selected by the CLKS field, scanning (if necessary) through the pins selected by 1s in the SEL field. The first conversion after the start corresponds to the least-significant 1 in the SEL field, then higher numbered 1 bits (pins) if applicable. Repeated conversions can be terminated by clearing this bit, but the conversion that is in progress when this bit is cleared will be completed. Important: START bits must be 000 when BURST = 1 or conversions will not start."]
    #[inline]
    pub fn burst(self) -> &'a mut W {
        self.variant(BURSTW::BURST)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKS`"]
pub enum CLKSW {
    #[doc = "11 clocks / 10 bits"]
    _11_CLOCKS_10_BITS,
    #[doc = "10 clocks / 9 bits"]
    _10_CLOCKS_9_BITS,
    #[doc = "9 clocks / 8 bits"]
    _9_CLOCKS_8_BITS,
    #[doc = "8 clocks / 7 bits"]
    _8_CLOCKS_7_BITS,
    #[doc = "7 clocks / 6 bits"]
    _7_CLOCKS_6_BITS,
    #[doc = "6 clocks / 5 bits"]
    _6_CLOCKS_5_BITS,
    #[doc = "5 clocks / 4 bits"]
    _5_CLOCKS_4_BITS,
    #[doc = "4 clocks / 3 bits"]
    _4_CLOCKS_3_BITS,
}
impl CLKSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKSW::_11_CLOCKS_10_BITS => 0,
            CLKSW::_10_CLOCKS_9_BITS => 1,
            CLKSW::_9_CLOCKS_8_BITS => 2,
            CLKSW::_8_CLOCKS_7_BITS => 3,
            CLKSW::_7_CLOCKS_6_BITS => 4,
            CLKSW::_6_CLOCKS_5_BITS => 5,
            CLKSW::_5_CLOCKS_4_BITS => 6,
            CLKSW::_4_CLOCKS_3_BITS => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKSW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "11 clocks / 10 bits"]
    #[inline]
    pub fn _11_clocks_10_bits(self) -> &'a mut W {
        self.variant(CLKSW::_11_CLOCKS_10_BITS)
    }
    #[doc = "10 clocks / 9 bits"]
    #[inline]
    pub fn _10_clocks_9_bits(self) -> &'a mut W {
        self.variant(CLKSW::_10_CLOCKS_9_BITS)
    }
    #[doc = "9 clocks / 8 bits"]
    #[inline]
    pub fn _9_clocks_8_bits(self) -> &'a mut W {
        self.variant(CLKSW::_9_CLOCKS_8_BITS)
    }
    #[doc = "8 clocks / 7 bits"]
    #[inline]
    pub fn _8_clocks_7_bits(self) -> &'a mut W {
        self.variant(CLKSW::_8_CLOCKS_7_BITS)
    }
    #[doc = "7 clocks / 6 bits"]
    #[inline]
    pub fn _7_clocks_6_bits(self) -> &'a mut W {
        self.variant(CLKSW::_7_CLOCKS_6_BITS)
    }
    #[doc = "6 clocks / 5 bits"]
    #[inline]
    pub fn _6_clocks_5_bits(self) -> &'a mut W {
        self.variant(CLKSW::_6_CLOCKS_5_BITS)
    }
    #[doc = "5 clocks / 4 bits"]
    #[inline]
    pub fn _5_clocks_4_bits(self) -> &'a mut W {
        self.variant(CLKSW::_5_CLOCKS_4_BITS)
    }
    #[doc = "4 clocks / 3 bits"]
    #[inline]
    pub fn _4_clocks_3_bits(self) -> &'a mut W {
        self.variant(CLKSW::_4_CLOCKS_3_BITS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDN`"]
pub enum PDNW {
    #[doc = "The A/D converter is in Power-down mode."]
    POWERDOWN,
    #[doc = "The A/D converter is operational."]
    RUNNING,
}
impl PDNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDNW::POWERDOWN => false,
            PDNW::RUNNING => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDNW<'a> {
    w: &'a mut W,
}
impl<'a> _PDNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The A/D converter is in Power-down mode."]
    #[inline]
    pub fn powerdown(self) -> &'a mut W {
        self.variant(PDNW::POWERDOWN)
    }
    #[doc = "The A/D converter is operational."]
    #[inline]
    pub fn running(self) -> &'a mut W {
        self.variant(PDNW::RUNNING)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `START`"]
pub enum STARTW {
    #[doc = "No start (this value should be used when clearing PDN to 0)."]
    NO_START,
    #[doc = "Start conversion now."]
    START_CONVERSION_NOW,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CTOUT_15 (combined timer output 15)."]
    CTOUT_15,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CTOUT_8 (combined timer output 8)."]
    CTOUT_8,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on ADCTRIG0 input."]
    ADCTRIG0,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on ADCTRIG1 input."]
    ADCTRIG1,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on Motocon PWM output MCOA2."]
    MCOA2,
    #[doc = "Reserved."]
    RESERVED,
}
impl STARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STARTW::NO_START => 0,
            STARTW::START_CONVERSION_NOW => 1,
            STARTW::CTOUT_15 => 2,
            STARTW::CTOUT_8 => 3,
            STARTW::ADCTRIG0 => 4,
            STARTW::ADCTRIG1 => 5,
            STARTW::MCOA2 => 6,
            STARTW::RESERVED => 7,
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
            self.bits(variant._bits())
        }
    }
    #[doc = "No start (this value should be used when clearing PDN to 0)."]
    #[inline]
    pub fn no_start(self) -> &'a mut W {
        self.variant(STARTW::NO_START)
    }
    #[doc = "Start conversion now."]
    #[inline]
    pub fn start_conversion_now(self) -> &'a mut W {
        self.variant(STARTW::START_CONVERSION_NOW)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CTOUT_15 (combined timer output 15)."]
    #[inline]
    pub fn ctout_15(self) -> &'a mut W {
        self.variant(STARTW::CTOUT_15)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CTOUT_8 (combined timer output 8)."]
    #[inline]
    pub fn ctout_8(self) -> &'a mut W {
        self.variant(STARTW::CTOUT_8)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on ADCTRIG0 input."]
    #[inline]
    pub fn adctrig0(self) -> &'a mut W {
        self.variant(STARTW::ADCTRIG0)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on ADCTRIG1 input."]
    #[inline]
    pub fn adctrig1(self) -> &'a mut W {
        self.variant(STARTW::ADCTRIG1)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on Motocon PWM output MCOA2."]
    #[inline]
    pub fn mcoa2(self) -> &'a mut W {
        self.variant(STARTW::MCOA2)
    }
    #[doc = "Reserved."]
    #[inline]
    pub fn reserved(self) -> &'a mut W {
        self.variant(STARTW::RESERVED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EDGE`"]
pub enum EDGEW {
    #[doc = "Start conversion on a rising edge on the selected signal."]
    RISING,
    #[doc = "Start conversion on a falling edge on the selected signal."]
    FALLING,
}
impl EDGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDGEW::RISING => false,
            EDGEW::FALLING => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDGEW<'a> {
    w: &'a mut W,
}
impl<'a> _EDGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Start conversion on a rising edge on the selected signal."]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(EDGEW::RISING)
    }
    #[doc = "Start conversion on a falling edge on the selected signal."]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(EDGEW::FALLING)
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
        const OFFSET: u8 = 27;
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
    #[doc = "Bits 0:7 - Selects which of the ADC[7:0] pins are to be sampled and converted. Bit 0 selects Pin ADC0, bit 1 selects pin AD1,..., and bit 7 selects pin ADC7. In software-controlled mode, only one of these bits should be 1. In hardware scan mode, any value containing 1 to 8 ones. All zeroes is equivalent to 0x01."]
    #[inline]
    pub fn sel(&self) -> SELR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SELR { bits }
    }
    #[doc = "Bits 8:15 - The ADC clock is divided by the CLKDIV value plus one to produce the clock for the A/D converter, which should be less than or equal to 4.5 MHz. Typically, software should program the smallest value in this field that yields a clock of 4.5 MHz or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable."]
    #[inline]
    pub fn clkdiv(&self) -> CLKDIVR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CLKDIVR { bits }
    }
    #[doc = "Bit 16 - Burst mode"]
    #[inline]
    pub fn burst(&self) -> BURSTR {
        BURSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 17:19 - This field selects the number of clocks used for each conversion in Burst mode, and the number of bits of accuracy of the result in the LS bits of ADDR, between 11 clocks (10 bits) and 4 clocks (3 bits)."]
    #[inline]
    pub fn clks(&self) -> CLKSR {
        CLKSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 21 - Power mode"]
    #[inline]
    pub fn pdn(&self) -> PDNR {
        PDNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:26 - When the BURST bit is 0, these bits control whether and when an A/D conversion is started (also see Figure 56):"]
    #[inline]
    pub fn start(&self) -> STARTR {
        STARTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 27 - This bit is significant only when the START field contains 0x2 -0x6. In these cases:"]
    #[inline]
    pub fn edge(&self) -> EDGER {
        EDGER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
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
    #[doc = "Bits 0:7 - Selects which of the ADC[7:0] pins are to be sampled and converted. Bit 0 selects Pin ADC0, bit 1 selects pin AD1,..., and bit 7 selects pin ADC7. In software-controlled mode, only one of these bits should be 1. In hardware scan mode, any value containing 1 to 8 ones. All zeroes is equivalent to 0x01."]
    #[inline]
    pub fn sel(&mut self) -> _SELW {
        _SELW { w: self }
    }
    #[doc = "Bits 8:15 - The ADC clock is divided by the CLKDIV value plus one to produce the clock for the A/D converter, which should be less than or equal to 4.5 MHz. Typically, software should program the smallest value in this field that yields a clock of 4.5 MHz or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable."]
    #[inline]
    pub fn clkdiv(&mut self) -> _CLKDIVW {
        _CLKDIVW { w: self }
    }
    #[doc = "Bit 16 - Burst mode"]
    #[inline]
    pub fn burst(&mut self) -> _BURSTW {
        _BURSTW { w: self }
    }
    #[doc = "Bits 17:19 - This field selects the number of clocks used for each conversion in Burst mode, and the number of bits of accuracy of the result in the LS bits of ADDR, between 11 clocks (10 bits) and 4 clocks (3 bits)."]
    #[inline]
    pub fn clks(&mut self) -> _CLKSW {
        _CLKSW { w: self }
    }
    #[doc = "Bit 21 - Power mode"]
    #[inline]
    pub fn pdn(&mut self) -> _PDNW {
        _PDNW { w: self }
    }
    #[doc = "Bits 24:26 - When the BURST bit is 0, these bits control whether and when an A/D conversion is started (also see Figure 56):"]
    #[inline]
    pub fn start(&mut self) -> _STARTW {
        _STARTW { w: self }
    }
    #[doc = "Bit 27 - This bit is significant only when the START field contains 0x2 -0x6. In these cases:"]
    #[inline]
    pub fn edge(&mut self) -> _EDGEW {
        _EDGEW { w: self }
    }
}
