#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ENAIO0 {
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
#[doc = "Possible values of the field `ADC0_0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC0_0R {
    #[doc = "Digital function selected on pin P4_3."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Analog function ADC0_0 selected on pin P4_3"]
    ANALOG_FUNCTION_ADC0,
}
impl ADC0_0R {
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
            ADC0_0R::DIGITAL_FUNCTION_SEL => false,
            ADC0_0R::ANALOG_FUNCTION_ADC0 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC0_0R {
        match value {
            false => ADC0_0R::DIGITAL_FUNCTION_SEL,
            true => ADC0_0R::ANALOG_FUNCTION_ADC0,
        }
    }
    #[doc = "Checks if the value of the field is `DIGITAL_FUNCTION_SEL`"]
    #[inline]
    pub fn is_digital_function_sel(&self) -> bool {
        *self == ADC0_0R::DIGITAL_FUNCTION_SEL
    }
    #[doc = "Checks if the value of the field is `ANALOG_FUNCTION_ADC0`"]
    #[inline]
    pub fn is_analog_function_adc0(&self) -> bool {
        *self == ADC0_0R::ANALOG_FUNCTION_ADC0
    }
}
#[doc = "Possible values of the field `ADC0_1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC0_1R {
    #[doc = "Digital function selected on pin P4_1."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Analog function ADC0_1 selected on pin P4_1."]
    ANALOG_FUNCTION_ADC0,
}
impl ADC0_1R {
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
            ADC0_1R::DIGITAL_FUNCTION_SEL => false,
            ADC0_1R::ANALOG_FUNCTION_ADC0 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC0_1R {
        match value {
            false => ADC0_1R::DIGITAL_FUNCTION_SEL,
            true => ADC0_1R::ANALOG_FUNCTION_ADC0,
        }
    }
    #[doc = "Checks if the value of the field is `DIGITAL_FUNCTION_SEL`"]
    #[inline]
    pub fn is_digital_function_sel(&self) -> bool {
        *self == ADC0_1R::DIGITAL_FUNCTION_SEL
    }
    #[doc = "Checks if the value of the field is `ANALOG_FUNCTION_ADC0`"]
    #[inline]
    pub fn is_analog_function_adc0(&self) -> bool {
        *self == ADC0_1R::ANALOG_FUNCTION_ADC0
    }
}
#[doc = "Possible values of the field `ADC0_2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC0_2R {
    #[doc = "Digital function selected on pin PF_8."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Analog function ADC0_2 selected on pin PF_8."]
    ANALOG_FUNCTION_ADC0,
}
impl ADC0_2R {
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
            ADC0_2R::DIGITAL_FUNCTION_SEL => false,
            ADC0_2R::ANALOG_FUNCTION_ADC0 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC0_2R {
        match value {
            false => ADC0_2R::DIGITAL_FUNCTION_SEL,
            true => ADC0_2R::ANALOG_FUNCTION_ADC0,
        }
    }
    #[doc = "Checks if the value of the field is `DIGITAL_FUNCTION_SEL`"]
    #[inline]
    pub fn is_digital_function_sel(&self) -> bool {
        *self == ADC0_2R::DIGITAL_FUNCTION_SEL
    }
    #[doc = "Checks if the value of the field is `ANALOG_FUNCTION_ADC0`"]
    #[inline]
    pub fn is_analog_function_adc0(&self) -> bool {
        *self == ADC0_2R::ANALOG_FUNCTION_ADC0
    }
}
#[doc = "Possible values of the field `ADC0_3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC0_3R {
    #[doc = "Digital function selected on pin P7_5."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Analog function ADC0_3 selected on pin P7_5."]
    ANALOG_FUNCTION_ADC0,
}
impl ADC0_3R {
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
            ADC0_3R::DIGITAL_FUNCTION_SEL => false,
            ADC0_3R::ANALOG_FUNCTION_ADC0 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC0_3R {
        match value {
            false => ADC0_3R::DIGITAL_FUNCTION_SEL,
            true => ADC0_3R::ANALOG_FUNCTION_ADC0,
        }
    }
    #[doc = "Checks if the value of the field is `DIGITAL_FUNCTION_SEL`"]
    #[inline]
    pub fn is_digital_function_sel(&self) -> bool {
        *self == ADC0_3R::DIGITAL_FUNCTION_SEL
    }
    #[doc = "Checks if the value of the field is `ANALOG_FUNCTION_ADC0`"]
    #[inline]
    pub fn is_analog_function_adc0(&self) -> bool {
        *self == ADC0_3R::ANALOG_FUNCTION_ADC0
    }
}
#[doc = "Possible values of the field `ADC0_4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC0_4R {
    #[doc = "Digital function selected on pin P7_4."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Analog function ADC0_4 selected on pin P7_4."]
    ANALOG_FUNCTION_ADC0,
}
impl ADC0_4R {
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
            ADC0_4R::DIGITAL_FUNCTION_SEL => false,
            ADC0_4R::ANALOG_FUNCTION_ADC0 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC0_4R {
        match value {
            false => ADC0_4R::DIGITAL_FUNCTION_SEL,
            true => ADC0_4R::ANALOG_FUNCTION_ADC0,
        }
    }
    #[doc = "Checks if the value of the field is `DIGITAL_FUNCTION_SEL`"]
    #[inline]
    pub fn is_digital_function_sel(&self) -> bool {
        *self == ADC0_4R::DIGITAL_FUNCTION_SEL
    }
    #[doc = "Checks if the value of the field is `ANALOG_FUNCTION_ADC0`"]
    #[inline]
    pub fn is_analog_function_adc0(&self) -> bool {
        *self == ADC0_4R::ANALOG_FUNCTION_ADC0
    }
}
#[doc = "Possible values of the field `ADC0_5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC0_5R {
    #[doc = "Digital function selected on pin PF_10."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Analog function ADC0_5 selected on pin PF_10."]
    ANALOG_FUNCTION_ADC0,
}
impl ADC0_5R {
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
            ADC0_5R::DIGITAL_FUNCTION_SEL => false,
            ADC0_5R::ANALOG_FUNCTION_ADC0 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC0_5R {
        match value {
            false => ADC0_5R::DIGITAL_FUNCTION_SEL,
            true => ADC0_5R::ANALOG_FUNCTION_ADC0,
        }
    }
    #[doc = "Checks if the value of the field is `DIGITAL_FUNCTION_SEL`"]
    #[inline]
    pub fn is_digital_function_sel(&self) -> bool {
        *self == ADC0_5R::DIGITAL_FUNCTION_SEL
    }
    #[doc = "Checks if the value of the field is `ANALOG_FUNCTION_ADC0`"]
    #[inline]
    pub fn is_analog_function_adc0(&self) -> bool {
        *self == ADC0_5R::ANALOG_FUNCTION_ADC0
    }
}
#[doc = "Possible values of the field `ADC0_6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC0_6R {
    #[doc = "Digital function selected on pin PB_6."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Analog function ADC0_6 selected on pin PB_6."]
    ANALOG_FUNCTION_ADC0,
}
impl ADC0_6R {
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
            ADC0_6R::DIGITAL_FUNCTION_SEL => false,
            ADC0_6R::ANALOG_FUNCTION_ADC0 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC0_6R {
        match value {
            false => ADC0_6R::DIGITAL_FUNCTION_SEL,
            true => ADC0_6R::ANALOG_FUNCTION_ADC0,
        }
    }
    #[doc = "Checks if the value of the field is `DIGITAL_FUNCTION_SEL`"]
    #[inline]
    pub fn is_digital_function_sel(&self) -> bool {
        *self == ADC0_6R::DIGITAL_FUNCTION_SEL
    }
    #[doc = "Checks if the value of the field is `ANALOG_FUNCTION_ADC0`"]
    #[inline]
    pub fn is_analog_function_adc0(&self) -> bool {
        *self == ADC0_6R::ANALOG_FUNCTION_ADC0
    }
}
#[doc = "Values that can be written to the field `ADC0_0`"]
pub enum ADC0_0W {
    #[doc = "Digital function selected on pin P4_3."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Analog function ADC0_0 selected on pin P4_3"]
    ANALOG_FUNCTION_ADC0,
}
impl ADC0_0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC0_0W::DIGITAL_FUNCTION_SEL => false,
            ADC0_0W::ANALOG_FUNCTION_ADC0 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC0_0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC0_0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC0_0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital function selected on pin P4_3."]
    #[inline]
    pub fn digital_function_sel(self) -> &'a mut W {
        self.variant(ADC0_0W::DIGITAL_FUNCTION_SEL)
    }
    #[doc = "Analog function ADC0_0 selected on pin P4_3"]
    #[inline]
    pub fn analog_function_adc0(self) -> &'a mut W {
        self.variant(ADC0_0W::ANALOG_FUNCTION_ADC0)
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
#[doc = "Values that can be written to the field `ADC0_1`"]
pub enum ADC0_1W {
    #[doc = "Digital function selected on pin P4_1."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Analog function ADC0_1 selected on pin P4_1."]
    ANALOG_FUNCTION_ADC0,
}
impl ADC0_1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC0_1W::DIGITAL_FUNCTION_SEL => false,
            ADC0_1W::ANALOG_FUNCTION_ADC0 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC0_1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC0_1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC0_1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital function selected on pin P4_1."]
    #[inline]
    pub fn digital_function_sel(self) -> &'a mut W {
        self.variant(ADC0_1W::DIGITAL_FUNCTION_SEL)
    }
    #[doc = "Analog function ADC0_1 selected on pin P4_1."]
    #[inline]
    pub fn analog_function_adc0(self) -> &'a mut W {
        self.variant(ADC0_1W::ANALOG_FUNCTION_ADC0)
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
#[doc = "Values that can be written to the field `ADC0_2`"]
pub enum ADC0_2W {
    #[doc = "Digital function selected on pin PF_8."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Analog function ADC0_2 selected on pin PF_8."]
    ANALOG_FUNCTION_ADC0,
}
impl ADC0_2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC0_2W::DIGITAL_FUNCTION_SEL => false,
            ADC0_2W::ANALOG_FUNCTION_ADC0 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC0_2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC0_2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC0_2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital function selected on pin PF_8."]
    #[inline]
    pub fn digital_function_sel(self) -> &'a mut W {
        self.variant(ADC0_2W::DIGITAL_FUNCTION_SEL)
    }
    #[doc = "Analog function ADC0_2 selected on pin PF_8."]
    #[inline]
    pub fn analog_function_adc0(self) -> &'a mut W {
        self.variant(ADC0_2W::ANALOG_FUNCTION_ADC0)
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
#[doc = "Values that can be written to the field `ADC0_3`"]
pub enum ADC0_3W {
    #[doc = "Digital function selected on pin P7_5."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Analog function ADC0_3 selected on pin P7_5."]
    ANALOG_FUNCTION_ADC0,
}
impl ADC0_3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC0_3W::DIGITAL_FUNCTION_SEL => false,
            ADC0_3W::ANALOG_FUNCTION_ADC0 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC0_3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC0_3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC0_3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital function selected on pin P7_5."]
    #[inline]
    pub fn digital_function_sel(self) -> &'a mut W {
        self.variant(ADC0_3W::DIGITAL_FUNCTION_SEL)
    }
    #[doc = "Analog function ADC0_3 selected on pin P7_5."]
    #[inline]
    pub fn analog_function_adc0(self) -> &'a mut W {
        self.variant(ADC0_3W::ANALOG_FUNCTION_ADC0)
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
#[doc = "Values that can be written to the field `ADC0_4`"]
pub enum ADC0_4W {
    #[doc = "Digital function selected on pin P7_4."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Analog function ADC0_4 selected on pin P7_4."]
    ANALOG_FUNCTION_ADC0,
}
impl ADC0_4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC0_4W::DIGITAL_FUNCTION_SEL => false,
            ADC0_4W::ANALOG_FUNCTION_ADC0 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC0_4W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC0_4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC0_4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital function selected on pin P7_4."]
    #[inline]
    pub fn digital_function_sel(self) -> &'a mut W {
        self.variant(ADC0_4W::DIGITAL_FUNCTION_SEL)
    }
    #[doc = "Analog function ADC0_4 selected on pin P7_4."]
    #[inline]
    pub fn analog_function_adc0(self) -> &'a mut W {
        self.variant(ADC0_4W::ANALOG_FUNCTION_ADC0)
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
#[doc = "Values that can be written to the field `ADC0_5`"]
pub enum ADC0_5W {
    #[doc = "Digital function selected on pin PF_10."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Analog function ADC0_5 selected on pin PF_10."]
    ANALOG_FUNCTION_ADC0,
}
impl ADC0_5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC0_5W::DIGITAL_FUNCTION_SEL => false,
            ADC0_5W::ANALOG_FUNCTION_ADC0 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC0_5W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC0_5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC0_5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital function selected on pin PF_10."]
    #[inline]
    pub fn digital_function_sel(self) -> &'a mut W {
        self.variant(ADC0_5W::DIGITAL_FUNCTION_SEL)
    }
    #[doc = "Analog function ADC0_5 selected on pin PF_10."]
    #[inline]
    pub fn analog_function_adc0(self) -> &'a mut W {
        self.variant(ADC0_5W::ANALOG_FUNCTION_ADC0)
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
#[doc = "Values that can be written to the field `ADC0_6`"]
pub enum ADC0_6W {
    #[doc = "Digital function selected on pin PB_6."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Analog function ADC0_6 selected on pin PB_6."]
    ANALOG_FUNCTION_ADC0,
}
impl ADC0_6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC0_6W::DIGITAL_FUNCTION_SEL => false,
            ADC0_6W::ANALOG_FUNCTION_ADC0 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC0_6W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC0_6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC0_6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital function selected on pin PB_6."]
    #[inline]
    pub fn digital_function_sel(self) -> &'a mut W {
        self.variant(ADC0_6W::DIGITAL_FUNCTION_SEL)
    }
    #[doc = "Analog function ADC0_6 selected on pin PB_6."]
    #[inline]
    pub fn analog_function_adc0(self) -> &'a mut W {
        self.variant(ADC0_6W::ANALOG_FUNCTION_ADC0)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Select ADC0_0"]
    #[inline]
    pub fn adc0_0(&self) -> ADC0_0R {
        ADC0_0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Select ADC0_1"]
    #[inline]
    pub fn adc0_1(&self) -> ADC0_1R {
        ADC0_1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Select ADC0_2"]
    #[inline]
    pub fn adc0_2(&self) -> ADC0_2R {
        ADC0_2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Select ADC0_3"]
    #[inline]
    pub fn adc0_3(&self) -> ADC0_3R {
        ADC0_3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Select ADC0_4"]
    #[inline]
    pub fn adc0_4(&self) -> ADC0_4R {
        ADC0_4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Select ADC0_5"]
    #[inline]
    pub fn adc0_5(&self) -> ADC0_5R {
        ADC0_5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Select ADC0_6"]
    #[inline]
    pub fn adc0_6(&self) -> ADC0_6R {
        ADC0_6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
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
    #[doc = "Bit 0 - Select ADC0_0"]
    #[inline]
    pub fn adc0_0(&mut self) -> _ADC0_0W {
        _ADC0_0W { w: self }
    }
    #[doc = "Bit 1 - Select ADC0_1"]
    #[inline]
    pub fn adc0_1(&mut self) -> _ADC0_1W {
        _ADC0_1W { w: self }
    }
    #[doc = "Bit 2 - Select ADC0_2"]
    #[inline]
    pub fn adc0_2(&mut self) -> _ADC0_2W {
        _ADC0_2W { w: self }
    }
    #[doc = "Bit 3 - Select ADC0_3"]
    #[inline]
    pub fn adc0_3(&mut self) -> _ADC0_3W {
        _ADC0_3W { w: self }
    }
    #[doc = "Bit 4 - Select ADC0_4"]
    #[inline]
    pub fn adc0_4(&mut self) -> _ADC0_4W {
        _ADC0_4W { w: self }
    }
    #[doc = "Bit 5 - Select ADC0_5"]
    #[inline]
    pub fn adc0_5(&mut self) -> _ADC0_5W {
        _ADC0_5W { w: self }
    }
    #[doc = "Bit 6 - Select ADC0_6"]
    #[inline]
    pub fn adc0_6(&mut self) -> _ADC0_6W {
        _ADC0_6W { w: self }
    }
}
