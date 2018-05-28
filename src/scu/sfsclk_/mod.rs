#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SFSCLK_ {
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
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Function 0 (default)"]
    FUNCTION_0_DEFAULT,
    #[doc = "Function 1"]
    FUNCTION_1,
    #[doc = "Function 2"]
    FUNCTION_2,
    #[doc = "Function 3"]
    FUNCTION_3,
    #[doc = "Function 4"]
    FUNCTION_4,
    #[doc = "Function 5"]
    FUNCTION_5,
    #[doc = "Function 6"]
    FUNCTION_6,
    #[doc = "Function 7"]
    FUNCTION_7,
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::FUNCTION_0_DEFAULT => 0,
            MODER::FUNCTION_1 => 1,
            MODER::FUNCTION_2 => 2,
            MODER::FUNCTION_3 => 3,
            MODER::FUNCTION_4 => 4,
            MODER::FUNCTION_5 => 5,
            MODER::FUNCTION_6 => 6,
            MODER::FUNCTION_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::FUNCTION_0_DEFAULT,
            1 => MODER::FUNCTION_1,
            2 => MODER::FUNCTION_2,
            3 => MODER::FUNCTION_3,
            4 => MODER::FUNCTION_4,
            5 => MODER::FUNCTION_5,
            6 => MODER::FUNCTION_6,
            7 => MODER::FUNCTION_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FUNCTION_0_DEFAULT`"]
    #[inline]
    pub fn is_function_0_default(&self) -> bool {
        *self == MODER::FUNCTION_0_DEFAULT
    }
    #[doc = "Checks if the value of the field is `FUNCTION_1`"]
    #[inline]
    pub fn is_function_1(&self) -> bool {
        *self == MODER::FUNCTION_1
    }
    #[doc = "Checks if the value of the field is `FUNCTION_2`"]
    #[inline]
    pub fn is_function_2(&self) -> bool {
        *self == MODER::FUNCTION_2
    }
    #[doc = "Checks if the value of the field is `FUNCTION_3`"]
    #[inline]
    pub fn is_function_3(&self) -> bool {
        *self == MODER::FUNCTION_3
    }
    #[doc = "Checks if the value of the field is `FUNCTION_4`"]
    #[inline]
    pub fn is_function_4(&self) -> bool {
        *self == MODER::FUNCTION_4
    }
    #[doc = "Checks if the value of the field is `FUNCTION_5`"]
    #[inline]
    pub fn is_function_5(&self) -> bool {
        *self == MODER::FUNCTION_5
    }
    #[doc = "Checks if the value of the field is `FUNCTION_6`"]
    #[inline]
    pub fn is_function_6(&self) -> bool {
        *self == MODER::FUNCTION_6
    }
    #[doc = "Checks if the value of the field is `FUNCTION_7`"]
    #[inline]
    pub fn is_function_7(&self) -> bool {
        *self == MODER::FUNCTION_7
    }
}
#[doc = "Possible values of the field `EPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPDR {
    #[doc = "Disable pull-down."]
    DISABLE_PULL_DOWN,
    #[doc = "Enable pull-down. Enable both pull-down resistor and pull-up resistor for repeater mode."]
    ENABLE_PULL_DOWN,
}
impl EPDR {
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
            EPDR::DISABLE_PULL_DOWN => false,
            EPDR::ENABLE_PULL_DOWN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPDR {
        match value {
            false => EPDR::DISABLE_PULL_DOWN,
            true => EPDR::ENABLE_PULL_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_PULL_DOWN`"]
    #[inline]
    pub fn is_disable_pull_down(&self) -> bool {
        *self == EPDR::DISABLE_PULL_DOWN
    }
    #[doc = "Checks if the value of the field is `ENABLE_PULL_DOWN`"]
    #[inline]
    pub fn is_enable_pull_down(&self) -> bool {
        *self == EPDR::ENABLE_PULL_DOWN
    }
}
#[doc = "Possible values of the field `EPUN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPUNR {
    #[doc = "Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater mode."]
    ENABLE_PULL_UP,
    #[doc = "Disable pull-up."]
    DISABLE_PULL_UP,
}
impl EPUNR {
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
            EPUNR::ENABLE_PULL_UP => false,
            EPUNR::DISABLE_PULL_UP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPUNR {
        match value {
            false => EPUNR::ENABLE_PULL_UP,
            true => EPUNR::DISABLE_PULL_UP,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE_PULL_UP`"]
    #[inline]
    pub fn is_enable_pull_up(&self) -> bool {
        *self == EPUNR::ENABLE_PULL_UP
    }
    #[doc = "Checks if the value of the field is `DISABLE_PULL_UP`"]
    #[inline]
    pub fn is_disable_pull_up(&self) -> bool {
        *self == EPUNR::DISABLE_PULL_UP
    }
}
#[doc = "Possible values of the field `EHS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EHSR {
    #[doc = "Fast (low noise with fast speed)"]
    FAST_LOW_NOISE_WITH,
    #[doc = "High-speed (medium noise with high speed)"]
    HIGH_SPEED_MEDIUM_N,
}
impl EHSR {
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
            EHSR::FAST_LOW_NOISE_WITH => false,
            EHSR::HIGH_SPEED_MEDIUM_N => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EHSR {
        match value {
            false => EHSR::FAST_LOW_NOISE_WITH,
            true => EHSR::HIGH_SPEED_MEDIUM_N,
        }
    }
    #[doc = "Checks if the value of the field is `FAST_LOW_NOISE_WITH`"]
    #[inline]
    pub fn is_fast_low_noise_with(&self) -> bool {
        *self == EHSR::FAST_LOW_NOISE_WITH
    }
    #[doc = "Checks if the value of the field is `HIGH_SPEED_MEDIUM_N`"]
    #[inline]
    pub fn is_high_speed_medium_n(&self) -> bool {
        *self == EHSR::HIGH_SPEED_MEDIUM_N
    }
}
#[doc = "Possible values of the field `EZI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EZIR {
    #[doc = "Disable input buffer"]
    DISABLE_INPUT_BUFFER,
    #[doc = "Enable input buffer"]
    ENABLE_INPUT_BUFFER,
}
impl EZIR {
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
            EZIR::DISABLE_INPUT_BUFFER => false,
            EZIR::ENABLE_INPUT_BUFFER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EZIR {
        match value {
            false => EZIR::DISABLE_INPUT_BUFFER,
            true => EZIR::ENABLE_INPUT_BUFFER,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_INPUT_BUFFER`"]
    #[inline]
    pub fn is_disable_input_buffer(&self) -> bool {
        *self == EZIR::DISABLE_INPUT_BUFFER
    }
    #[doc = "Checks if the value of the field is `ENABLE_INPUT_BUFFER`"]
    #[inline]
    pub fn is_enable_input_buffer(&self) -> bool {
        *self == EZIR::ENABLE_INPUT_BUFFER
    }
}
#[doc = "Possible values of the field `ZIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZIFR {
    #[doc = "Enable input filter"]
    ENABLE_INPUT_FILTER,
    #[doc = "Disable input filter"]
    DISABLE_INPUT_FILTER,
}
impl ZIFR {
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
            ZIFR::ENABLE_INPUT_FILTER => false,
            ZIFR::DISABLE_INPUT_FILTER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ZIFR {
        match value {
            false => ZIFR::ENABLE_INPUT_FILTER,
            true => ZIFR::DISABLE_INPUT_FILTER,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE_INPUT_FILTER`"]
    #[inline]
    pub fn is_enable_input_filter(&self) -> bool {
        *self == ZIFR::ENABLE_INPUT_FILTER
    }
    #[doc = "Checks if the value of the field is `DISABLE_INPUT_FILTER`"]
    #[inline]
    pub fn is_disable_input_filter(&self) -> bool {
        *self == ZIFR::DISABLE_INPUT_FILTER
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "Function 0 (default)"]
    FUNCTION_0_DEFAULT,
    #[doc = "Function 1"]
    FUNCTION_1,
    #[doc = "Function 2"]
    FUNCTION_2,
    #[doc = "Function 3"]
    FUNCTION_3,
    #[doc = "Function 4"]
    FUNCTION_4,
    #[doc = "Function 5"]
    FUNCTION_5,
    #[doc = "Function 6"]
    FUNCTION_6,
    #[doc = "Function 7"]
    FUNCTION_7,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::FUNCTION_0_DEFAULT => 0,
            MODEW::FUNCTION_1 => 1,
            MODEW::FUNCTION_2 => 2,
            MODEW::FUNCTION_3 => 3,
            MODEW::FUNCTION_4 => 4,
            MODEW::FUNCTION_5 => 5,
            MODEW::FUNCTION_6 => 6,
            MODEW::FUNCTION_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Function 0 (default)"]
    #[inline]
    pub fn function_0_default(self) -> &'a mut W {
        self.variant(MODEW::FUNCTION_0_DEFAULT)
    }
    #[doc = "Function 1"]
    #[inline]
    pub fn function_1(self) -> &'a mut W {
        self.variant(MODEW::FUNCTION_1)
    }
    #[doc = "Function 2"]
    #[inline]
    pub fn function_2(self) -> &'a mut W {
        self.variant(MODEW::FUNCTION_2)
    }
    #[doc = "Function 3"]
    #[inline]
    pub fn function_3(self) -> &'a mut W {
        self.variant(MODEW::FUNCTION_3)
    }
    #[doc = "Function 4"]
    #[inline]
    pub fn function_4(self) -> &'a mut W {
        self.variant(MODEW::FUNCTION_4)
    }
    #[doc = "Function 5"]
    #[inline]
    pub fn function_5(self) -> &'a mut W {
        self.variant(MODEW::FUNCTION_5)
    }
    #[doc = "Function 6"]
    #[inline]
    pub fn function_6(self) -> &'a mut W {
        self.variant(MODEW::FUNCTION_6)
    }
    #[doc = "Function 7"]
    #[inline]
    pub fn function_7(self) -> &'a mut W {
        self.variant(MODEW::FUNCTION_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EPD`"]
pub enum EPDW {
    #[doc = "Disable pull-down."]
    DISABLE_PULL_DOWN,
    #[doc = "Enable pull-down. Enable both pull-down resistor and pull-up resistor for repeater mode."]
    ENABLE_PULL_DOWN,
}
impl EPDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPDW::DISABLE_PULL_DOWN => false,
            EPDW::ENABLE_PULL_DOWN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPDW<'a> {
    w: &'a mut W,
}
impl<'a> _EPDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable pull-down."]
    #[inline]
    pub fn disable_pull_down(self) -> &'a mut W {
        self.variant(EPDW::DISABLE_PULL_DOWN)
    }
    #[doc = "Enable pull-down. Enable both pull-down resistor and pull-up resistor for repeater mode."]
    #[inline]
    pub fn enable_pull_down(self) -> &'a mut W {
        self.variant(EPDW::ENABLE_PULL_DOWN)
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
#[doc = "Values that can be written to the field `EPUN`"]
pub enum EPUNW {
    #[doc = "Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater mode."]
    ENABLE_PULL_UP,
    #[doc = "Disable pull-up."]
    DISABLE_PULL_UP,
}
impl EPUNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPUNW::ENABLE_PULL_UP => false,
            EPUNW::DISABLE_PULL_UP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPUNW<'a> {
    w: &'a mut W,
}
impl<'a> _EPUNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPUNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater mode."]
    #[inline]
    pub fn enable_pull_up(self) -> &'a mut W {
        self.variant(EPUNW::ENABLE_PULL_UP)
    }
    #[doc = "Disable pull-up."]
    #[inline]
    pub fn disable_pull_up(self) -> &'a mut W {
        self.variant(EPUNW::DISABLE_PULL_UP)
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
#[doc = "Values that can be written to the field `EHS`"]
pub enum EHSW {
    #[doc = "Fast (low noise with fast speed)"]
    FAST_LOW_NOISE_WITH,
    #[doc = "High-speed (medium noise with high speed)"]
    HIGH_SPEED_MEDIUM_N,
}
impl EHSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EHSW::FAST_LOW_NOISE_WITH => false,
            EHSW::HIGH_SPEED_MEDIUM_N => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EHSW<'a> {
    w: &'a mut W,
}
impl<'a> _EHSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EHSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Fast (low noise with fast speed)"]
    #[inline]
    pub fn fast_low_noise_with(self) -> &'a mut W {
        self.variant(EHSW::FAST_LOW_NOISE_WITH)
    }
    #[doc = "High-speed (medium noise with high speed)"]
    #[inline]
    pub fn high_speed_medium_n(self) -> &'a mut W {
        self.variant(EHSW::HIGH_SPEED_MEDIUM_N)
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
#[doc = "Values that can be written to the field `EZI`"]
pub enum EZIW {
    #[doc = "Disable input buffer"]
    DISABLE_INPUT_BUFFER,
    #[doc = "Enable input buffer"]
    ENABLE_INPUT_BUFFER,
}
impl EZIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EZIW::DISABLE_INPUT_BUFFER => false,
            EZIW::ENABLE_INPUT_BUFFER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EZIW<'a> {
    w: &'a mut W,
}
impl<'a> _EZIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EZIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable input buffer"]
    #[inline]
    pub fn disable_input_buffer(self) -> &'a mut W {
        self.variant(EZIW::DISABLE_INPUT_BUFFER)
    }
    #[doc = "Enable input buffer"]
    #[inline]
    pub fn enable_input_buffer(self) -> &'a mut W {
        self.variant(EZIW::ENABLE_INPUT_BUFFER)
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
#[doc = "Values that can be written to the field `ZIF`"]
pub enum ZIFW {
    #[doc = "Enable input filter"]
    ENABLE_INPUT_FILTER,
    #[doc = "Disable input filter"]
    DISABLE_INPUT_FILTER,
}
impl ZIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ZIFW::ENABLE_INPUT_FILTER => false,
            ZIFW::DISABLE_INPUT_FILTER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ZIFW<'a> {
    w: &'a mut W,
}
impl<'a> _ZIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ZIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable input filter"]
    #[inline]
    pub fn enable_input_filter(self) -> &'a mut W {
        self.variant(ZIFW::ENABLE_INPUT_FILTER)
    }
    #[doc = "Disable input filter"]
    #[inline]
    pub fn disable_input_filter(self) -> &'a mut W {
        self.variant(ZIFW::DISABLE_INPUT_FILTER)
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
    #[doc = "Bits 0:2 - Select pin function."]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Enable pull-down resistor at pad."]
    #[inline]
    pub fn epd(&self) -> EPDR {
        EPDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at reset."]
    #[inline]
    pub fn epun(&self) -> EPUNR {
        EPUNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Slew rate"]
    #[inline]
    pub fn ehs(&self) -> EHSR {
        EHSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Input buffer enable. The input buffer is disabled by default at reset and must be enabled for receiving."]
    #[inline]
    pub fn ezi(&self) -> EZIR {
        EZIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Input glitch filter. Disable the input glitch filter for clocking signals higher than 30 MHz."]
    #[inline]
    pub fn zif(&self) -> ZIFR {
        ZIFR::_from({
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
    #[doc = "Bits 0:2 - Select pin function."]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 3 - Enable pull-down resistor at pad."]
    #[inline]
    pub fn epd(&mut self) -> _EPDW {
        _EPDW { w: self }
    }
    #[doc = "Bit 4 - Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at reset."]
    #[inline]
    pub fn epun(&mut self) -> _EPUNW {
        _EPUNW { w: self }
    }
    #[doc = "Bit 5 - Slew rate"]
    #[inline]
    pub fn ehs(&mut self) -> _EHSW {
        _EHSW { w: self }
    }
    #[doc = "Bit 6 - Input buffer enable. The input buffer is disabled by default at reset and must be enabled for receiving."]
    #[inline]
    pub fn ezi(&mut self) -> _EZIW {
        _EZIW { w: self }
    }
    #[doc = "Bit 7 - Input glitch filter. Disable the input glitch filter for clocking signals higher than 30 MHz."]
    #[inline]
    pub fn zif(&mut self) -> _ZIFW {
        _ZIFW { w: self }
    }
}
