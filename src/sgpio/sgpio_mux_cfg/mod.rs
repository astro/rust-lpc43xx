#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SGPIO_MUX_CFG {
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
#[doc = "Possible values of the field `EXT_CLK_ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXT_CLK_ENABLER {
    #[doc = "Internal clock signal (slice)"]
    INTERNAL_CLOCK_SIGNA,
    #[doc = "External clock signal (pin)"]
    EXTERNAL_CLOCK_SIGNA,
}
impl EXT_CLK_ENABLER {
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
            EXT_CLK_ENABLER::INTERNAL_CLOCK_SIGNA => false,
            EXT_CLK_ENABLER::EXTERNAL_CLOCK_SIGNA => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EXT_CLK_ENABLER {
        match value {
            false => EXT_CLK_ENABLER::INTERNAL_CLOCK_SIGNA,
            true => EXT_CLK_ENABLER::EXTERNAL_CLOCK_SIGNA,
        }
    }
    #[doc = "Checks if the value of the field is `INTERNAL_CLOCK_SIGNA`"]
    #[inline]
    pub fn is_internal_clock_signa(&self) -> bool {
        *self == EXT_CLK_ENABLER::INTERNAL_CLOCK_SIGNA
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_CLOCK_SIGNA`"]
    #[inline]
    pub fn is_external_clock_signa(&self) -> bool {
        *self == EXT_CLK_ENABLER::EXTERNAL_CLOCK_SIGNA
    }
}
#[doc = "Possible values of the field `CLK_SOURCE_PIN_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_SOURCE_PIN_MODER {
    #[doc = "SGPIO8"]
    SGPIO8,
    #[doc = "SGPIO9"]
    SGPIO9,
    #[doc = "SGPIO10"]
    SGPIO10,
    #[doc = "SGPIO11"]
    SGPIO11,
}
impl CLK_SOURCE_PIN_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLK_SOURCE_PIN_MODER::SGPIO8 => 0,
            CLK_SOURCE_PIN_MODER::SGPIO9 => 1,
            CLK_SOURCE_PIN_MODER::SGPIO10 => 2,
            CLK_SOURCE_PIN_MODER::SGPIO11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLK_SOURCE_PIN_MODER {
        match value {
            0 => CLK_SOURCE_PIN_MODER::SGPIO8,
            1 => CLK_SOURCE_PIN_MODER::SGPIO9,
            2 => CLK_SOURCE_PIN_MODER::SGPIO10,
            3 => CLK_SOURCE_PIN_MODER::SGPIO11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SGPIO8`"]
    #[inline]
    pub fn is_sgpio8(&self) -> bool {
        *self == CLK_SOURCE_PIN_MODER::SGPIO8
    }
    #[doc = "Checks if the value of the field is `SGPIO9`"]
    #[inline]
    pub fn is_sgpio9(&self) -> bool {
        *self == CLK_SOURCE_PIN_MODER::SGPIO9
    }
    #[doc = "Checks if the value of the field is `SGPIO10`"]
    #[inline]
    pub fn is_sgpio10(&self) -> bool {
        *self == CLK_SOURCE_PIN_MODER::SGPIO10
    }
    #[doc = "Checks if the value of the field is `SGPIO11`"]
    #[inline]
    pub fn is_sgpio11(&self) -> bool {
        *self == CLK_SOURCE_PIN_MODER::SGPIO11
    }
}
#[doc = "Possible values of the field `CLK_SOURCE_SLICE_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_SOURCE_SLICE_MODER {
    #[doc = "Slice D"]
    SLICE_D,
    #[doc = "Slice H"]
    SLICE_H,
    #[doc = "Slice O"]
    SLICE_O,
    #[doc = "Slice P"]
    SLICE_P,
}
impl CLK_SOURCE_SLICE_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLK_SOURCE_SLICE_MODER::SLICE_D => 0,
            CLK_SOURCE_SLICE_MODER::SLICE_H => 1,
            CLK_SOURCE_SLICE_MODER::SLICE_O => 2,
            CLK_SOURCE_SLICE_MODER::SLICE_P => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLK_SOURCE_SLICE_MODER {
        match value {
            0 => CLK_SOURCE_SLICE_MODER::SLICE_D,
            1 => CLK_SOURCE_SLICE_MODER::SLICE_H,
            2 => CLK_SOURCE_SLICE_MODER::SLICE_O,
            3 => CLK_SOURCE_SLICE_MODER::SLICE_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SLICE_D`"]
    #[inline]
    pub fn is_slice_d(&self) -> bool {
        *self == CLK_SOURCE_SLICE_MODER::SLICE_D
    }
    #[doc = "Checks if the value of the field is `SLICE_H`"]
    #[inline]
    pub fn is_slice_h(&self) -> bool {
        *self == CLK_SOURCE_SLICE_MODER::SLICE_H
    }
    #[doc = "Checks if the value of the field is `SLICE_O`"]
    #[inline]
    pub fn is_slice_o(&self) -> bool {
        *self == CLK_SOURCE_SLICE_MODER::SLICE_O
    }
    #[doc = "Checks if the value of the field is `SLICE_P`"]
    #[inline]
    pub fn is_slice_p(&self) -> bool {
        *self == CLK_SOURCE_SLICE_MODER::SLICE_P
    }
}
#[doc = "Possible values of the field `QUALIFIER_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QUALIFIER_MODER {
    #[doc = "Enable"]
    ENABLE,
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Slice (see bits QUALIFIER_SLICE_MODE in this register)"]
    SLICE_SEE_BITS_QUAL,
    #[doc = "External SGPIO pin (SGPIO8, SGPIO9, SGPIO10, or SGPIO11)"]
    EXTERNAL_SGPIO_PIN,
}
impl QUALIFIER_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            QUALIFIER_MODER::ENABLE => 0,
            QUALIFIER_MODER::DISABLE => 1,
            QUALIFIER_MODER::SLICE_SEE_BITS_QUAL => 2,
            QUALIFIER_MODER::EXTERNAL_SGPIO_PIN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> QUALIFIER_MODER {
        match value {
            0 => QUALIFIER_MODER::ENABLE,
            1 => QUALIFIER_MODER::DISABLE,
            2 => QUALIFIER_MODER::SLICE_SEE_BITS_QUAL,
            3 => QUALIFIER_MODER::EXTERNAL_SGPIO_PIN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == QUALIFIER_MODER::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == QUALIFIER_MODER::DISABLE
    }
    #[doc = "Checks if the value of the field is `SLICE_SEE_BITS_QUAL`"]
    #[inline]
    pub fn is_slice_see_bits_qual(&self) -> bool {
        *self == QUALIFIER_MODER::SLICE_SEE_BITS_QUAL
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_SGPIO_PIN`"]
    #[inline]
    pub fn is_external_sgpio_pin(&self) -> bool {
        *self == QUALIFIER_MODER::EXTERNAL_SGPIO_PIN
    }
}
#[doc = "Possible values of the field `QUALIFIER_PIN_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QUALIFIER_PIN_MODER {
    #[doc = "SGPIO8"]
    SGPIO8,
    #[doc = "SGPIO9"]
    SGPIO9,
    #[doc = "SGPIO10"]
    SGPIO10,
    #[doc = "SGPIO11"]
    SGPIO11,
}
impl QUALIFIER_PIN_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            QUALIFIER_PIN_MODER::SGPIO8 => 0,
            QUALIFIER_PIN_MODER::SGPIO9 => 1,
            QUALIFIER_PIN_MODER::SGPIO10 => 2,
            QUALIFIER_PIN_MODER::SGPIO11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> QUALIFIER_PIN_MODER {
        match value {
            0 => QUALIFIER_PIN_MODER::SGPIO8,
            1 => QUALIFIER_PIN_MODER::SGPIO9,
            2 => QUALIFIER_PIN_MODER::SGPIO10,
            3 => QUALIFIER_PIN_MODER::SGPIO11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SGPIO8`"]
    #[inline]
    pub fn is_sgpio8(&self) -> bool {
        *self == QUALIFIER_PIN_MODER::SGPIO8
    }
    #[doc = "Checks if the value of the field is `SGPIO9`"]
    #[inline]
    pub fn is_sgpio9(&self) -> bool {
        *self == QUALIFIER_PIN_MODER::SGPIO9
    }
    #[doc = "Checks if the value of the field is `SGPIO10`"]
    #[inline]
    pub fn is_sgpio10(&self) -> bool {
        *self == QUALIFIER_PIN_MODER::SGPIO10
    }
    #[doc = "Checks if the value of the field is `SGPIO11`"]
    #[inline]
    pub fn is_sgpio11(&self) -> bool {
        *self == QUALIFIER_PIN_MODER::SGPIO11
    }
}
#[doc = "Possible values of the field `QUALIFIER_SLICE_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QUALIFIER_SLICE_MODER {
    #[doc = "Slice A, but for slice A slice D is used."]
    SLICE_A,
    #[doc = "Slice H, but for slice H slice O is used."]
    SLICE_H,
    #[doc = "Slice I, but for slice I slice D is used."]
    SLICE_I,
    #[doc = "Slice P, but for slice P slice O is used."]
    SLICE_P,
}
impl QUALIFIER_SLICE_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            QUALIFIER_SLICE_MODER::SLICE_A => 0,
            QUALIFIER_SLICE_MODER::SLICE_H => 1,
            QUALIFIER_SLICE_MODER::SLICE_I => 2,
            QUALIFIER_SLICE_MODER::SLICE_P => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> QUALIFIER_SLICE_MODER {
        match value {
            0 => QUALIFIER_SLICE_MODER::SLICE_A,
            1 => QUALIFIER_SLICE_MODER::SLICE_H,
            2 => QUALIFIER_SLICE_MODER::SLICE_I,
            3 => QUALIFIER_SLICE_MODER::SLICE_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SLICE_A`"]
    #[inline]
    pub fn is_slice_a(&self) -> bool {
        *self == QUALIFIER_SLICE_MODER::SLICE_A
    }
    #[doc = "Checks if the value of the field is `SLICE_H`"]
    #[inline]
    pub fn is_slice_h(&self) -> bool {
        *self == QUALIFIER_SLICE_MODER::SLICE_H
    }
    #[doc = "Checks if the value of the field is `SLICE_I`"]
    #[inline]
    pub fn is_slice_i(&self) -> bool {
        *self == QUALIFIER_SLICE_MODER::SLICE_I
    }
    #[doc = "Checks if the value of the field is `SLICE_P`"]
    #[inline]
    pub fn is_slice_p(&self) -> bool {
        *self == QUALIFIER_SLICE_MODER::SLICE_P
    }
}
#[doc = "Possible values of the field `CONCAT_ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONCAT_ENABLER {
    #[doc = "External data pin"]
    EXTERNAL_DATA_PIN,
    #[doc = "Concatenate data"]
    CONCATENATE_DATA,
}
impl CONCAT_ENABLER {
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
            CONCAT_ENABLER::EXTERNAL_DATA_PIN => false,
            CONCAT_ENABLER::CONCATENATE_DATA => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CONCAT_ENABLER {
        match value {
            false => CONCAT_ENABLER::EXTERNAL_DATA_PIN,
            true => CONCAT_ENABLER::CONCATENATE_DATA,
        }
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_DATA_PIN`"]
    #[inline]
    pub fn is_external_data_pin(&self) -> bool {
        *self == CONCAT_ENABLER::EXTERNAL_DATA_PIN
    }
    #[doc = "Checks if the value of the field is `CONCATENATE_DATA`"]
    #[inline]
    pub fn is_concatenate_data(&self) -> bool {
        *self == CONCAT_ENABLER::CONCATENATE_DATA
    }
}
#[doc = "Possible values of the field `CONCAT_ORDER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONCAT_ORDERR {
    #[doc = "Self-loop"]
    SELF_LOOP,
    #[doc = "2 slices"]
    _2_SLICES,
    #[doc = "4 slices"]
    _4_SLICES,
    #[doc = "8 slices"]
    _8_SLICES,
}
impl CONCAT_ORDERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CONCAT_ORDERR::SELF_LOOP => 0,
            CONCAT_ORDERR::_2_SLICES => 1,
            CONCAT_ORDERR::_4_SLICES => 2,
            CONCAT_ORDERR::_8_SLICES => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CONCAT_ORDERR {
        match value {
            0 => CONCAT_ORDERR::SELF_LOOP,
            1 => CONCAT_ORDERR::_2_SLICES,
            2 => CONCAT_ORDERR::_4_SLICES,
            3 => CONCAT_ORDERR::_8_SLICES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SELF_LOOP`"]
    #[inline]
    pub fn is_self_loop(&self) -> bool {
        *self == CONCAT_ORDERR::SELF_LOOP
    }
    #[doc = "Checks if the value of the field is `_2_SLICES`"]
    #[inline]
    pub fn is_2_slices(&self) -> bool {
        *self == CONCAT_ORDERR::_2_SLICES
    }
    #[doc = "Checks if the value of the field is `_4_SLICES`"]
    #[inline]
    pub fn is_4_slices(&self) -> bool {
        *self == CONCAT_ORDERR::_4_SLICES
    }
    #[doc = "Checks if the value of the field is `_8_SLICES`"]
    #[inline]
    pub fn is_8_slices(&self) -> bool {
        *self == CONCAT_ORDERR::_8_SLICES
    }
}
#[doc = "Values that can be written to the field `EXT_CLK_ENABLE`"]
pub enum EXT_CLK_ENABLEW {
    #[doc = "Internal clock signal (slice)"]
    INTERNAL_CLOCK_SIGNA,
    #[doc = "External clock signal (pin)"]
    EXTERNAL_CLOCK_SIGNA,
}
impl EXT_CLK_ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EXT_CLK_ENABLEW::INTERNAL_CLOCK_SIGNA => false,
            EXT_CLK_ENABLEW::EXTERNAL_CLOCK_SIGNA => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXT_CLK_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _EXT_CLK_ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXT_CLK_ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Internal clock signal (slice)"]
    #[inline]
    pub fn internal_clock_signa(self) -> &'a mut W {
        self.variant(EXT_CLK_ENABLEW::INTERNAL_CLOCK_SIGNA)
    }
    #[doc = "External clock signal (pin)"]
    #[inline]
    pub fn external_clock_signa(self) -> &'a mut W {
        self.variant(EXT_CLK_ENABLEW::EXTERNAL_CLOCK_SIGNA)
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
#[doc = "Values that can be written to the field `CLK_SOURCE_PIN_MODE`"]
pub enum CLK_SOURCE_PIN_MODEW {
    #[doc = "SGPIO8"]
    SGPIO8,
    #[doc = "SGPIO9"]
    SGPIO9,
    #[doc = "SGPIO10"]
    SGPIO10,
    #[doc = "SGPIO11"]
    SGPIO11,
}
impl CLK_SOURCE_PIN_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLK_SOURCE_PIN_MODEW::SGPIO8 => 0,
            CLK_SOURCE_PIN_MODEW::SGPIO9 => 1,
            CLK_SOURCE_PIN_MODEW::SGPIO10 => 2,
            CLK_SOURCE_PIN_MODEW::SGPIO11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLK_SOURCE_PIN_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CLK_SOURCE_PIN_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLK_SOURCE_PIN_MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "SGPIO8"]
    #[inline]
    pub fn sgpio8(self) -> &'a mut W {
        self.variant(CLK_SOURCE_PIN_MODEW::SGPIO8)
    }
    #[doc = "SGPIO9"]
    #[inline]
    pub fn sgpio9(self) -> &'a mut W {
        self.variant(CLK_SOURCE_PIN_MODEW::SGPIO9)
    }
    #[doc = "SGPIO10"]
    #[inline]
    pub fn sgpio10(self) -> &'a mut W {
        self.variant(CLK_SOURCE_PIN_MODEW::SGPIO10)
    }
    #[doc = "SGPIO11"]
    #[inline]
    pub fn sgpio11(self) -> &'a mut W {
        self.variant(CLK_SOURCE_PIN_MODEW::SGPIO11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLK_SOURCE_SLICE_MODE`"]
pub enum CLK_SOURCE_SLICE_MODEW {
    #[doc = "Slice D"]
    SLICE_D,
    #[doc = "Slice H"]
    SLICE_H,
    #[doc = "Slice O"]
    SLICE_O,
    #[doc = "Slice P"]
    SLICE_P,
}
impl CLK_SOURCE_SLICE_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLK_SOURCE_SLICE_MODEW::SLICE_D => 0,
            CLK_SOURCE_SLICE_MODEW::SLICE_H => 1,
            CLK_SOURCE_SLICE_MODEW::SLICE_O => 2,
            CLK_SOURCE_SLICE_MODEW::SLICE_P => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLK_SOURCE_SLICE_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CLK_SOURCE_SLICE_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLK_SOURCE_SLICE_MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Slice D"]
    #[inline]
    pub fn slice_d(self) -> &'a mut W {
        self.variant(CLK_SOURCE_SLICE_MODEW::SLICE_D)
    }
    #[doc = "Slice H"]
    #[inline]
    pub fn slice_h(self) -> &'a mut W {
        self.variant(CLK_SOURCE_SLICE_MODEW::SLICE_H)
    }
    #[doc = "Slice O"]
    #[inline]
    pub fn slice_o(self) -> &'a mut W {
        self.variant(CLK_SOURCE_SLICE_MODEW::SLICE_O)
    }
    #[doc = "Slice P"]
    #[inline]
    pub fn slice_p(self) -> &'a mut W {
        self.variant(CLK_SOURCE_SLICE_MODEW::SLICE_P)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `QUALIFIER_MODE`"]
pub enum QUALIFIER_MODEW {
    #[doc = "Enable"]
    ENABLE,
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Slice (see bits QUALIFIER_SLICE_MODE in this register)"]
    SLICE_SEE_BITS_QUAL,
    #[doc = "External SGPIO pin (SGPIO8, SGPIO9, SGPIO10, or SGPIO11)"]
    EXTERNAL_SGPIO_PIN,
}
impl QUALIFIER_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            QUALIFIER_MODEW::ENABLE => 0,
            QUALIFIER_MODEW::DISABLE => 1,
            QUALIFIER_MODEW::SLICE_SEE_BITS_QUAL => 2,
            QUALIFIER_MODEW::EXTERNAL_SGPIO_PIN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QUALIFIER_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _QUALIFIER_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QUALIFIER_MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(QUALIFIER_MODEW::ENABLE)
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(QUALIFIER_MODEW::DISABLE)
    }
    #[doc = "Slice (see bits QUALIFIER_SLICE_MODE in this register)"]
    #[inline]
    pub fn slice_see_bits_qual(self) -> &'a mut W {
        self.variant(QUALIFIER_MODEW::SLICE_SEE_BITS_QUAL)
    }
    #[doc = "External SGPIO pin (SGPIO8, SGPIO9, SGPIO10, or SGPIO11)"]
    #[inline]
    pub fn external_sgpio_pin(self) -> &'a mut W {
        self.variant(QUALIFIER_MODEW::EXTERNAL_SGPIO_PIN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `QUALIFIER_PIN_MODE`"]
pub enum QUALIFIER_PIN_MODEW {
    #[doc = "SGPIO8"]
    SGPIO8,
    #[doc = "SGPIO9"]
    SGPIO9,
    #[doc = "SGPIO10"]
    SGPIO10,
    #[doc = "SGPIO11"]
    SGPIO11,
}
impl QUALIFIER_PIN_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            QUALIFIER_PIN_MODEW::SGPIO8 => 0,
            QUALIFIER_PIN_MODEW::SGPIO9 => 1,
            QUALIFIER_PIN_MODEW::SGPIO10 => 2,
            QUALIFIER_PIN_MODEW::SGPIO11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QUALIFIER_PIN_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _QUALIFIER_PIN_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QUALIFIER_PIN_MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "SGPIO8"]
    #[inline]
    pub fn sgpio8(self) -> &'a mut W {
        self.variant(QUALIFIER_PIN_MODEW::SGPIO8)
    }
    #[doc = "SGPIO9"]
    #[inline]
    pub fn sgpio9(self) -> &'a mut W {
        self.variant(QUALIFIER_PIN_MODEW::SGPIO9)
    }
    #[doc = "SGPIO10"]
    #[inline]
    pub fn sgpio10(self) -> &'a mut W {
        self.variant(QUALIFIER_PIN_MODEW::SGPIO10)
    }
    #[doc = "SGPIO11"]
    #[inline]
    pub fn sgpio11(self) -> &'a mut W {
        self.variant(QUALIFIER_PIN_MODEW::SGPIO11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `QUALIFIER_SLICE_MODE`"]
pub enum QUALIFIER_SLICE_MODEW {
    #[doc = "Slice A, but for slice A slice D is used."]
    SLICE_A,
    #[doc = "Slice H, but for slice H slice O is used."]
    SLICE_H,
    #[doc = "Slice I, but for slice I slice D is used."]
    SLICE_I,
    #[doc = "Slice P, but for slice P slice O is used."]
    SLICE_P,
}
impl QUALIFIER_SLICE_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            QUALIFIER_SLICE_MODEW::SLICE_A => 0,
            QUALIFIER_SLICE_MODEW::SLICE_H => 1,
            QUALIFIER_SLICE_MODEW::SLICE_I => 2,
            QUALIFIER_SLICE_MODEW::SLICE_P => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QUALIFIER_SLICE_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _QUALIFIER_SLICE_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QUALIFIER_SLICE_MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Slice A, but for slice A slice D is used."]
    #[inline]
    pub fn slice_a(self) -> &'a mut W {
        self.variant(QUALIFIER_SLICE_MODEW::SLICE_A)
    }
    #[doc = "Slice H, but for slice H slice O is used."]
    #[inline]
    pub fn slice_h(self) -> &'a mut W {
        self.variant(QUALIFIER_SLICE_MODEW::SLICE_H)
    }
    #[doc = "Slice I, but for slice I slice D is used."]
    #[inline]
    pub fn slice_i(self) -> &'a mut W {
        self.variant(QUALIFIER_SLICE_MODEW::SLICE_I)
    }
    #[doc = "Slice P, but for slice P slice O is used."]
    #[inline]
    pub fn slice_p(self) -> &'a mut W {
        self.variant(QUALIFIER_SLICE_MODEW::SLICE_P)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CONCAT_ENABLE`"]
pub enum CONCAT_ENABLEW {
    #[doc = "External data pin"]
    EXTERNAL_DATA_PIN,
    #[doc = "Concatenate data"]
    CONCATENATE_DATA,
}
impl CONCAT_ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CONCAT_ENABLEW::EXTERNAL_DATA_PIN => false,
            CONCAT_ENABLEW::CONCATENATE_DATA => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CONCAT_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _CONCAT_ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CONCAT_ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External data pin"]
    #[inline]
    pub fn external_data_pin(self) -> &'a mut W {
        self.variant(CONCAT_ENABLEW::EXTERNAL_DATA_PIN)
    }
    #[doc = "Concatenate data"]
    #[inline]
    pub fn concatenate_data(self) -> &'a mut W {
        self.variant(CONCAT_ENABLEW::CONCATENATE_DATA)
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
#[doc = "Values that can be written to the field `CONCAT_ORDER`"]
pub enum CONCAT_ORDERW {
    #[doc = "Self-loop"]
    SELF_LOOP,
    #[doc = "2 slices"]
    _2_SLICES,
    #[doc = "4 slices"]
    _4_SLICES,
    #[doc = "8 slices"]
    _8_SLICES,
}
impl CONCAT_ORDERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CONCAT_ORDERW::SELF_LOOP => 0,
            CONCAT_ORDERW::_2_SLICES => 1,
            CONCAT_ORDERW::_4_SLICES => 2,
            CONCAT_ORDERW::_8_SLICES => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CONCAT_ORDERW<'a> {
    w: &'a mut W,
}
impl<'a> _CONCAT_ORDERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CONCAT_ORDERW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Self-loop"]
    #[inline]
    pub fn self_loop(self) -> &'a mut W {
        self.variant(CONCAT_ORDERW::SELF_LOOP)
    }
    #[doc = "2 slices"]
    #[inline]
    pub fn _2_slices(self) -> &'a mut W {
        self.variant(CONCAT_ORDERW::_2_SLICES)
    }
    #[doc = "4 slices"]
    #[inline]
    pub fn _4_slices(self) -> &'a mut W {
        self.variant(CONCAT_ORDERW::_4_SLICES)
    }
    #[doc = "8 slices"]
    #[inline]
    pub fn _8_slices(self) -> &'a mut W {
        self.variant(CONCAT_ORDERW::_8_SLICES)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
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
    #[doc = "Bit 0 - Select clock signal."]
    #[inline]
    pub fn ext_clk_enable(&self) -> EXT_CLK_ENABLER {
        EXT_CLK_ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:2 - Select source clock pin."]
    #[inline]
    pub fn clk_source_pin_mode(&self) -> CLK_SOURCE_PIN_MODER {
        CLK_SOURCE_PIN_MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:4 - Select clock source slice. Note that slices D, H, O and P do not support this mode."]
    #[inline]
    pub fn clk_source_slice_mode(&self) -> CLK_SOURCE_SLICE_MODER {
        CLK_SOURCE_SLICE_MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 5:6 - Select qualifier mode."]
    #[inline]
    pub fn qualifier_mode(&self) -> QUALIFIER_MODER {
        QUALIFIER_MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 7:8 - Select qualifier pin."]
    #[inline]
    pub fn qualifier_pin_mode(&self) -> QUALIFIER_PIN_MODER {
        QUALIFIER_PIN_MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 9:10 - Select qualifier slice."]
    #[inline]
    pub fn qualifier_slice_mode(&self) -> QUALIFIER_SLICE_MODER {
        QUALIFIER_SLICE_MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - Enable concatenation."]
    #[inline]
    pub fn concat_enable(&self) -> CONCAT_ENABLER {
        CONCAT_ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:13 - Select concatenation order"]
    #[inline]
    pub fn concat_order(&self) -> CONCAT_ORDERR {
        CONCAT_ORDERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
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
    #[doc = "Bit 0 - Select clock signal."]
    #[inline]
    pub fn ext_clk_enable(&mut self) -> _EXT_CLK_ENABLEW {
        _EXT_CLK_ENABLEW { w: self }
    }
    #[doc = "Bits 1:2 - Select source clock pin."]
    #[inline]
    pub fn clk_source_pin_mode(&mut self) -> _CLK_SOURCE_PIN_MODEW {
        _CLK_SOURCE_PIN_MODEW { w: self }
    }
    #[doc = "Bits 3:4 - Select clock source slice. Note that slices D, H, O and P do not support this mode."]
    #[inline]
    pub fn clk_source_slice_mode(&mut self) -> _CLK_SOURCE_SLICE_MODEW {
        _CLK_SOURCE_SLICE_MODEW { w: self }
    }
    #[doc = "Bits 5:6 - Select qualifier mode."]
    #[inline]
    pub fn qualifier_mode(&mut self) -> _QUALIFIER_MODEW {
        _QUALIFIER_MODEW { w: self }
    }
    #[doc = "Bits 7:8 - Select qualifier pin."]
    #[inline]
    pub fn qualifier_pin_mode(&mut self) -> _QUALIFIER_PIN_MODEW {
        _QUALIFIER_PIN_MODEW { w: self }
    }
    #[doc = "Bits 9:10 - Select qualifier slice."]
    #[inline]
    pub fn qualifier_slice_mode(&mut self) -> _QUALIFIER_SLICE_MODEW {
        _QUALIFIER_SLICE_MODEW { w: self }
    }
    #[doc = "Bit 11 - Enable concatenation."]
    #[inline]
    pub fn concat_enable(&mut self) -> _CONCAT_ENABLEW {
        _CONCAT_ENABLEW { w: self }
    }
    #[doc = "Bits 12:13 - Select concatenation order"]
    #[inline]
    pub fn concat_order(&mut self) -> _CONCAT_ORDERW {
        _CONCAT_ORDERW { w: self }
    }
}
