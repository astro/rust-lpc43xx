#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONFIG {
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
#[doc = "Possible values of the field `UNIFY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNIFYR {
    #[doc = "16-bit.The SCT operates as two 16-bit counters named L and H."]
    _16_BIT,
    #[doc = "32-bit. The SCT operates as a unified 32-bit counter."]
    _32_BIT,
}
impl UNIFYR {
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
            UNIFYR::_16_BIT => false,
            UNIFYR::_32_BIT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UNIFYR {
        match value {
            false => UNIFYR::_16_BIT,
            true => UNIFYR::_32_BIT,
        }
    }
    #[doc = "Checks if the value of the field is `_16_BIT`"]
    #[inline]
    pub fn is_16_bit(&self) -> bool {
        *self == UNIFYR::_16_BIT
    }
    #[doc = "Checks if the value of the field is `_32_BIT`"]
    #[inline]
    pub fn is_32_bit(&self) -> bool {
        *self == UNIFYR::_32_BIT
    }
}
#[doc = "Possible values of the field `CLKMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKMODER {
    #[doc = "Bus clock. The bus clock clocks the SCT and prescalers."]
    BUS_CLOCK,
    #[doc = "Prescaled bus clock. The SCT clock is the bus clock, but the prescalers are  enabled to count only when sampling of the input selected by  the CKSEL field finds the selected edge. The minimum pulse  width on the clock input is 1 bus clock period. This mode is the high-performance  sampled-clock mode."]
    PRESCALED_BUS_CLOCK,
    #[doc = "SCT Input. The input selected by  CKSEL clocks the SCT and prescalers. The input is synchronized to the bus clock and possibly inverted.  The minimum pulse width on the clock input is 1 bus clock  period. This mode is the low-power sampled-clock mode."]
    SCT_INPUT,
}
impl CLKMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKMODER::BUS_CLOCK => 0,
            CLKMODER::PRESCALED_BUS_CLOCK => 1,
            CLKMODER::SCT_INPUT => 2,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKMODER {
        match value {
            0 => CLKMODER::BUS_CLOCK,
            1 => CLKMODER::PRESCALED_BUS_CLOCK,
            2 => CLKMODER::SCT_INPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BUS_CLOCK`"]
    #[inline]
    pub fn is_bus_clock(&self) -> bool {
        *self == CLKMODER::BUS_CLOCK
    }
    #[doc = "Checks if the value of the field is `PRESCALED_BUS_CLOCK`"]
    #[inline]
    pub fn is_prescaled_bus_clock(&self) -> bool {
        *self == CLKMODER::PRESCALED_BUS_CLOCK
    }
    #[doc = "Checks if the value of the field is `SCT_INPUT`"]
    #[inline]
    pub fn is_sct_input(&self) -> bool {
        *self == CLKMODER::SCT_INPUT
    }
}
#[doc = "Possible values of the field `CKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKSELR {
    #[doc = "Rising edges on input 0."]
    RISING_EDGES_ON_INPU_1,
    #[doc = "Falling edges on input 0."]
    FALLING_EDGES_ON_INP_1,
    #[doc = "Rising edges on input 1."]
    RISING_EDGES_ON_INPU_2,
    #[doc = "Falling edges on input 1."]
    FALLING_EDGES_ON_INP_2,
    #[doc = "Rising edges on input 2."]
    RISING_EDGES_ON_INPU_3,
    #[doc = "Falling edges on input 2."]
    FALLING_EDGES_ON_INP_3,
    #[doc = "Rising edges on input 3."]
    RISING_EDGES_ON_INPU_4,
    #[doc = "Falling edges on input 3."]
    FALLING_EDGES_ON_INP_4,
    #[doc = "Rising edges on input 4."]
    RISING_EDGES_ON_INPU_5,
    #[doc = "Falling edges on input 4."]
    FALLING_EDGES_ON_INP_5,
    #[doc = "Rising edges on input 5."]
    RISING_EDGES_ON_INPU_6,
    #[doc = "Falling edges on input 5."]
    FALLING_EDGES_ON_INP_6,
    #[doc = "Rising edges on input 6."]
    RISING_EDGES_ON_INPU_7,
    #[doc = "Falling edges on input 6."]
    FALLING_EDGES_ON_INP_7,
    #[doc = "Rising edges on input 7."]
    RISING_EDGES_ON_INPU_8,
    #[doc = "Falling edges on input 7."]
    FALLING_EDGES_ON_INP_8,
}
impl CKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CKSELR::RISING_EDGES_ON_INPU_1 => 0,
            CKSELR::FALLING_EDGES_ON_INP_1 => 1,
            CKSELR::RISING_EDGES_ON_INPU_2 => 2,
            CKSELR::FALLING_EDGES_ON_INP_2 => 3,
            CKSELR::RISING_EDGES_ON_INPU_3 => 4,
            CKSELR::FALLING_EDGES_ON_INP_3 => 5,
            CKSELR::RISING_EDGES_ON_INPU_4 => 6,
            CKSELR::FALLING_EDGES_ON_INP_4 => 7,
            CKSELR::RISING_EDGES_ON_INPU_5 => 8,
            CKSELR::FALLING_EDGES_ON_INP_5 => 9,
            CKSELR::RISING_EDGES_ON_INPU_6 => 10,
            CKSELR::FALLING_EDGES_ON_INP_6 => 11,
            CKSELR::RISING_EDGES_ON_INPU_7 => 12,
            CKSELR::FALLING_EDGES_ON_INP_7 => 13,
            CKSELR::RISING_EDGES_ON_INPU_8 => 14,
            CKSELR::FALLING_EDGES_ON_INP_8 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CKSELR {
        match value {
            0 => CKSELR::RISING_EDGES_ON_INPU_1,
            1 => CKSELR::FALLING_EDGES_ON_INP_1,
            2 => CKSELR::RISING_EDGES_ON_INPU_2,
            3 => CKSELR::FALLING_EDGES_ON_INP_2,
            4 => CKSELR::RISING_EDGES_ON_INPU_3,
            5 => CKSELR::FALLING_EDGES_ON_INP_3,
            6 => CKSELR::RISING_EDGES_ON_INPU_4,
            7 => CKSELR::FALLING_EDGES_ON_INP_4,
            8 => CKSELR::RISING_EDGES_ON_INPU_5,
            9 => CKSELR::FALLING_EDGES_ON_INP_5,
            10 => CKSELR::RISING_EDGES_ON_INPU_6,
            11 => CKSELR::FALLING_EDGES_ON_INP_6,
            12 => CKSELR::RISING_EDGES_ON_INPU_7,
            13 => CKSELR::FALLING_EDGES_ON_INP_7,
            14 => CKSELR::RISING_EDGES_ON_INPU_8,
            15 => CKSELR::FALLING_EDGES_ON_INP_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RISING_EDGES_ON_INPU_1`"]
    #[inline]
    pub fn is_rising_edges_on_inpu_1(&self) -> bool {
        *self == CKSELR::RISING_EDGES_ON_INPU_1
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGES_ON_INP_1`"]
    #[inline]
    pub fn is_falling_edges_on_inp_1(&self) -> bool {
        *self == CKSELR::FALLING_EDGES_ON_INP_1
    }
    #[doc = "Checks if the value of the field is `RISING_EDGES_ON_INPU_2`"]
    #[inline]
    pub fn is_rising_edges_on_inpu_2(&self) -> bool {
        *self == CKSELR::RISING_EDGES_ON_INPU_2
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGES_ON_INP_2`"]
    #[inline]
    pub fn is_falling_edges_on_inp_2(&self) -> bool {
        *self == CKSELR::FALLING_EDGES_ON_INP_2
    }
    #[doc = "Checks if the value of the field is `RISING_EDGES_ON_INPU_3`"]
    #[inline]
    pub fn is_rising_edges_on_inpu_3(&self) -> bool {
        *self == CKSELR::RISING_EDGES_ON_INPU_3
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGES_ON_INP_3`"]
    #[inline]
    pub fn is_falling_edges_on_inp_3(&self) -> bool {
        *self == CKSELR::FALLING_EDGES_ON_INP_3
    }
    #[doc = "Checks if the value of the field is `RISING_EDGES_ON_INPU_4`"]
    #[inline]
    pub fn is_rising_edges_on_inpu_4(&self) -> bool {
        *self == CKSELR::RISING_EDGES_ON_INPU_4
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGES_ON_INP_4`"]
    #[inline]
    pub fn is_falling_edges_on_inp_4(&self) -> bool {
        *self == CKSELR::FALLING_EDGES_ON_INP_4
    }
    #[doc = "Checks if the value of the field is `RISING_EDGES_ON_INPU_5`"]
    #[inline]
    pub fn is_rising_edges_on_inpu_5(&self) -> bool {
        *self == CKSELR::RISING_EDGES_ON_INPU_5
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGES_ON_INP_5`"]
    #[inline]
    pub fn is_falling_edges_on_inp_5(&self) -> bool {
        *self == CKSELR::FALLING_EDGES_ON_INP_5
    }
    #[doc = "Checks if the value of the field is `RISING_EDGES_ON_INPU_6`"]
    #[inline]
    pub fn is_rising_edges_on_inpu_6(&self) -> bool {
        *self == CKSELR::RISING_EDGES_ON_INPU_6
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGES_ON_INP_6`"]
    #[inline]
    pub fn is_falling_edges_on_inp_6(&self) -> bool {
        *self == CKSELR::FALLING_EDGES_ON_INP_6
    }
    #[doc = "Checks if the value of the field is `RISING_EDGES_ON_INPU_7`"]
    #[inline]
    pub fn is_rising_edges_on_inpu_7(&self) -> bool {
        *self == CKSELR::RISING_EDGES_ON_INPU_7
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGES_ON_INP_7`"]
    #[inline]
    pub fn is_falling_edges_on_inp_7(&self) -> bool {
        *self == CKSELR::FALLING_EDGES_ON_INP_7
    }
    #[doc = "Checks if the value of the field is `RISING_EDGES_ON_INPU_8`"]
    #[inline]
    pub fn is_rising_edges_on_inpu_8(&self) -> bool {
        *self == CKSELR::RISING_EDGES_ON_INPU_8
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGES_ON_INP_8`"]
    #[inline]
    pub fn is_falling_edges_on_inp_8(&self) -> bool {
        *self == CKSELR::FALLING_EDGES_ON_INP_8
    }
}
#[doc = r" Value of the field"]
pub struct NORELAOD_LR {
    bits: bool,
}
impl NORELAOD_LR {
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
pub struct NORELOAD_HR {
    bits: bool,
}
impl NORELOAD_HR {
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
pub struct INSYNCR {
    bits: u8,
}
impl INSYNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AUTOLIMIT_LR {
    bits: bool,
}
impl AUTOLIMIT_LR {
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
pub struct AUTOLIMIT_HR {
    bits: bool,
}
impl AUTOLIMIT_HR {
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
#[doc = "Values that can be written to the field `UNIFY`"]
pub enum UNIFYW {
    #[doc = "16-bit.The SCT operates as two 16-bit counters named L and H."]
    _16_BIT,
    #[doc = "32-bit. The SCT operates as a unified 32-bit counter."]
    _32_BIT,
}
impl UNIFYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UNIFYW::_16_BIT => false,
            UNIFYW::_32_BIT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UNIFYW<'a> {
    w: &'a mut W,
}
impl<'a> _UNIFYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UNIFYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "16-bit.The SCT operates as two 16-bit counters named L and H."]
    #[inline]
    pub fn _16_bit(self) -> &'a mut W {
        self.variant(UNIFYW::_16_BIT)
    }
    #[doc = "32-bit. The SCT operates as a unified 32-bit counter."]
    #[inline]
    pub fn _32_bit(self) -> &'a mut W {
        self.variant(UNIFYW::_32_BIT)
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
#[doc = "Values that can be written to the field `CLKMODE`"]
pub enum CLKMODEW {
    #[doc = "Bus clock. The bus clock clocks the SCT and prescalers."]
    BUS_CLOCK,
    #[doc = "Prescaled bus clock. The SCT clock is the bus clock, but the prescalers are  enabled to count only when sampling of the input selected by  the CKSEL field finds the selected edge. The minimum pulse  width on the clock input is 1 bus clock period. This mode is the high-performance  sampled-clock mode."]
    PRESCALED_BUS_CLOCK,
    #[doc = "SCT Input. The input selected by  CKSEL clocks the SCT and prescalers. The input is synchronized to the bus clock and possibly inverted.  The minimum pulse width on the clock input is 1 bus clock  period. This mode is the low-power sampled-clock mode."]
    SCT_INPUT,
}
impl CLKMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKMODEW::BUS_CLOCK => 0,
            CLKMODEW::PRESCALED_BUS_CLOCK => 1,
            CLKMODEW::SCT_INPUT => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Bus clock. The bus clock clocks the SCT and prescalers."]
    #[inline]
    pub fn bus_clock(self) -> &'a mut W {
        self.variant(CLKMODEW::BUS_CLOCK)
    }
    #[doc = "Prescaled bus clock. The SCT clock is the bus clock, but the prescalers are enabled to count only when sampling of the input selected by the CKSEL field finds the selected edge. The minimum pulse width on the clock input is 1 bus clock period. This mode is the high-performance sampled-clock mode."]
    #[inline]
    pub fn prescaled_bus_clock(self) -> &'a mut W {
        self.variant(CLKMODEW::PRESCALED_BUS_CLOCK)
    }
    #[doc = "SCT Input. The input selected by CKSEL clocks the SCT and prescalers. The input is synchronized to the bus clock and possibly inverted. The minimum pulse width on the clock input is 1 bus clock period. This mode is the low-power sampled-clock mode."]
    #[inline]
    pub fn sct_input(self) -> &'a mut W {
        self.variant(CLKMODEW::SCT_INPUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CKSEL`"]
pub enum CKSELW {
    #[doc = "Rising edges on input 0."]
    RISING_EDGES_ON_INPU_1,
    #[doc = "Falling edges on input 0."]
    FALLING_EDGES_ON_INP_1,
    #[doc = "Rising edges on input 1."]
    RISING_EDGES_ON_INPU_2,
    #[doc = "Falling edges on input 1."]
    FALLING_EDGES_ON_INP_2,
    #[doc = "Rising edges on input 2."]
    RISING_EDGES_ON_INPU_3,
    #[doc = "Falling edges on input 2."]
    FALLING_EDGES_ON_INP_3,
    #[doc = "Rising edges on input 3."]
    RISING_EDGES_ON_INPU_4,
    #[doc = "Falling edges on input 3."]
    FALLING_EDGES_ON_INP_4,
    #[doc = "Rising edges on input 4."]
    RISING_EDGES_ON_INPU_5,
    #[doc = "Falling edges on input 4."]
    FALLING_EDGES_ON_INP_5,
    #[doc = "Rising edges on input 5."]
    RISING_EDGES_ON_INPU_6,
    #[doc = "Falling edges on input 5."]
    FALLING_EDGES_ON_INP_6,
    #[doc = "Rising edges on input 6."]
    RISING_EDGES_ON_INPU_7,
    #[doc = "Falling edges on input 6."]
    FALLING_EDGES_ON_INP_7,
    #[doc = "Rising edges on input 7."]
    RISING_EDGES_ON_INPU_8,
    #[doc = "Falling edges on input 7."]
    FALLING_EDGES_ON_INP_8,
}
impl CKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CKSELW::RISING_EDGES_ON_INPU_1 => 0,
            CKSELW::FALLING_EDGES_ON_INP_1 => 1,
            CKSELW::RISING_EDGES_ON_INPU_2 => 2,
            CKSELW::FALLING_EDGES_ON_INP_2 => 3,
            CKSELW::RISING_EDGES_ON_INPU_3 => 4,
            CKSELW::FALLING_EDGES_ON_INP_3 => 5,
            CKSELW::RISING_EDGES_ON_INPU_4 => 6,
            CKSELW::FALLING_EDGES_ON_INP_4 => 7,
            CKSELW::RISING_EDGES_ON_INPU_5 => 8,
            CKSELW::FALLING_EDGES_ON_INP_5 => 9,
            CKSELW::RISING_EDGES_ON_INPU_6 => 10,
            CKSELW::FALLING_EDGES_ON_INP_6 => 11,
            CKSELW::RISING_EDGES_ON_INPU_7 => 12,
            CKSELW::FALLING_EDGES_ON_INP_7 => 13,
            CKSELW::RISING_EDGES_ON_INPU_8 => 14,
            CKSELW::FALLING_EDGES_ON_INP_8 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CKSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Rising edges on input 0."]
    #[inline]
    pub fn rising_edges_on_inpu_1(self) -> &'a mut W {
        self.variant(CKSELW::RISING_EDGES_ON_INPU_1)
    }
    #[doc = "Falling edges on input 0."]
    #[inline]
    pub fn falling_edges_on_inp_1(self) -> &'a mut W {
        self.variant(CKSELW::FALLING_EDGES_ON_INP_1)
    }
    #[doc = "Rising edges on input 1."]
    #[inline]
    pub fn rising_edges_on_inpu_2(self) -> &'a mut W {
        self.variant(CKSELW::RISING_EDGES_ON_INPU_2)
    }
    #[doc = "Falling edges on input 1."]
    #[inline]
    pub fn falling_edges_on_inp_2(self) -> &'a mut W {
        self.variant(CKSELW::FALLING_EDGES_ON_INP_2)
    }
    #[doc = "Rising edges on input 2."]
    #[inline]
    pub fn rising_edges_on_inpu_3(self) -> &'a mut W {
        self.variant(CKSELW::RISING_EDGES_ON_INPU_3)
    }
    #[doc = "Falling edges on input 2."]
    #[inline]
    pub fn falling_edges_on_inp_3(self) -> &'a mut W {
        self.variant(CKSELW::FALLING_EDGES_ON_INP_3)
    }
    #[doc = "Rising edges on input 3."]
    #[inline]
    pub fn rising_edges_on_inpu_4(self) -> &'a mut W {
        self.variant(CKSELW::RISING_EDGES_ON_INPU_4)
    }
    #[doc = "Falling edges on input 3."]
    #[inline]
    pub fn falling_edges_on_inp_4(self) -> &'a mut W {
        self.variant(CKSELW::FALLING_EDGES_ON_INP_4)
    }
    #[doc = "Rising edges on input 4."]
    #[inline]
    pub fn rising_edges_on_inpu_5(self) -> &'a mut W {
        self.variant(CKSELW::RISING_EDGES_ON_INPU_5)
    }
    #[doc = "Falling edges on input 4."]
    #[inline]
    pub fn falling_edges_on_inp_5(self) -> &'a mut W {
        self.variant(CKSELW::FALLING_EDGES_ON_INP_5)
    }
    #[doc = "Rising edges on input 5."]
    #[inline]
    pub fn rising_edges_on_inpu_6(self) -> &'a mut W {
        self.variant(CKSELW::RISING_EDGES_ON_INPU_6)
    }
    #[doc = "Falling edges on input 5."]
    #[inline]
    pub fn falling_edges_on_inp_6(self) -> &'a mut W {
        self.variant(CKSELW::FALLING_EDGES_ON_INP_6)
    }
    #[doc = "Rising edges on input 6."]
    #[inline]
    pub fn rising_edges_on_inpu_7(self) -> &'a mut W {
        self.variant(CKSELW::RISING_EDGES_ON_INPU_7)
    }
    #[doc = "Falling edges on input 6."]
    #[inline]
    pub fn falling_edges_on_inp_7(self) -> &'a mut W {
        self.variant(CKSELW::FALLING_EDGES_ON_INP_7)
    }
    #[doc = "Rising edges on input 7."]
    #[inline]
    pub fn rising_edges_on_inpu_8(self) -> &'a mut W {
        self.variant(CKSELW::RISING_EDGES_ON_INPU_8)
    }
    #[doc = "Falling edges on input 7."]
    #[inline]
    pub fn falling_edges_on_inp_8(self) -> &'a mut W {
        self.variant(CKSELW::FALLING_EDGES_ON_INP_8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NORELAOD_LW<'a> {
    w: &'a mut W,
}
impl<'a> _NORELAOD_LW<'a> {
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
#[doc = r" Proxy"]
pub struct _NORELOAD_HW<'a> {
    w: &'a mut W,
}
impl<'a> _NORELOAD_HW<'a> {
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
#[doc = r" Proxy"]
pub struct _INSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _INSYNCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AUTOLIMIT_LW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOLIMIT_LW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AUTOLIMIT_HW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOLIMIT_HW<'a> {
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
        const OFFSET: u8 = 18;
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
    #[doc = "Bit 0 - SCT operation"]
    #[inline]
    pub fn unify(&self) -> UNIFYR {
        UNIFYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:2 - SCT clock mode"]
    #[inline]
    pub fn clkmode(&self) -> CLKMODER {
        CLKMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:6 - SCT clock select"]
    #[inline]
    pub fn cksel(&self) -> CKSELR {
        CKSELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - A 1 in this bit prevents the lower match and fractional match registers from being reloaded from their respective reload registers. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline]
    pub fn norelaod_l(&self) -> NORELAOD_LR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NORELAOD_LR { bits }
    }
    #[doc = "Bit 8 - A 1 in this bit prevents the higher match and fractional match registers from being reloaded from their respective reload registers. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline]
    pub fn noreload_h(&self) -> NORELOAD_HR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NORELOAD_HR { bits }
    }
    #[doc = "Bits 9:16 - Synchronization for input n (bit 9 = input 0, bit 10 = input 1,..., bit 16 = input 7). A 1 in one of these bits subjects the corresponding input to synchronization to the SCT clock, before it is used to create an event. If an input is synchronous to the SCT clock, keep its bit 0 for faster response. When the CKMODE field is 1x, the bit in this field, corresponding to the input selected by the CKSEL field, is not used."]
    #[inline]
    pub fn insync(&self) -> INSYNCR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INSYNCR { bits }
    }
    #[doc = "Bit 17 - A one in this bit causes a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in uni-directional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline]
    pub fn autolimit_l(&self) -> AUTOLIMIT_LR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUTOLIMIT_LR { bits }
    }
    #[doc = "Bit 18 - A one in this bit will cause a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in uni-directional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline]
    pub fn autolimit_h(&self) -> AUTOLIMIT_HR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUTOLIMIT_HR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 32256 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - SCT operation"]
    #[inline]
    pub fn unify(&mut self) -> _UNIFYW {
        _UNIFYW { w: self }
    }
    #[doc = "Bits 1:2 - SCT clock mode"]
    #[inline]
    pub fn clkmode(&mut self) -> _CLKMODEW {
        _CLKMODEW { w: self }
    }
    #[doc = "Bits 3:6 - SCT clock select"]
    #[inline]
    pub fn cksel(&mut self) -> _CKSELW {
        _CKSELW { w: self }
    }
    #[doc = "Bit 7 - A 1 in this bit prevents the lower match and fractional match registers from being reloaded from their respective reload registers. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline]
    pub fn norelaod_l(&mut self) -> _NORELAOD_LW {
        _NORELAOD_LW { w: self }
    }
    #[doc = "Bit 8 - A 1 in this bit prevents the higher match and fractional match registers from being reloaded from their respective reload registers. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline]
    pub fn noreload_h(&mut self) -> _NORELOAD_HW {
        _NORELOAD_HW { w: self }
    }
    #[doc = "Bits 9:16 - Synchronization for input n (bit 9 = input 0, bit 10 = input 1,..., bit 16 = input 7). A 1 in one of these bits subjects the corresponding input to synchronization to the SCT clock, before it is used to create an event. If an input is synchronous to the SCT clock, keep its bit 0 for faster response. When the CKMODE field is 1x, the bit in this field, corresponding to the input selected by the CKSEL field, is not used."]
    #[inline]
    pub fn insync(&mut self) -> _INSYNCW {
        _INSYNCW { w: self }
    }
    #[doc = "Bit 17 - A one in this bit causes a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in uni-directional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline]
    pub fn autolimit_l(&mut self) -> _AUTOLIMIT_LW {
        _AUTOLIMIT_LW { w: self }
    }
    #[doc = "Bit 18 - A one in this bit will cause a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in uni-directional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline]
    pub fn autolimit_h(&mut self) -> _AUTOLIMIT_HW {
        _AUTOLIMIT_HW { w: self }
    }
}
