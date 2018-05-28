#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ENAIO1 {
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
#[doc = "Possible values of the field `ADC1_0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC1_0R {
    #[doc = "Digital function selected on pin PC_3."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Analog function ADC1_0 selected on pin PC_3."]
    ANALOG_FUNCTION_ADC1,
}
impl ADC1_0R {
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
            ADC1_0R::DIGITAL_FUNCTION_SEL => false,
            ADC1_0R::ANALOG_FUNCTION_ADC1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC1_0R {
        match value {
            false => ADC1_0R::DIGITAL_FUNCTION_SEL,
            true => ADC1_0R::ANALOG_FUNCTION_ADC1,
        }
    }
    #[doc = "Checks if the value of the field is `DIGITAL_FUNCTION_SEL`"]
    #[inline]
    pub fn is_digital_function_sel(&self) -> bool {
        *self == ADC1_0R::DIGITAL_FUNCTION_SEL
    }
    #[doc = "Checks if the value of the field is `ANALOG_FUNCTION_ADC1`"]
    #[inline]
    pub fn is_analog_function_adc1(&self) -> bool {
        *self == ADC1_0R::ANALOG_FUNCTION_ADC1
    }
}
#[doc = "Possible values of the field `ADC1_1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC1_1R {
    #[doc = "Digital function selected on pin PC_0."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Analog function ADC1_1 selected on pin PC_0."]
    ANALOG_FUNCTION_ADC1,
}
impl ADC1_1R {
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
            ADC1_1R::DIGITAL_FUNCTION_SEL => false,
            ADC1_1R::ANALOG_FUNCTION_ADC1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC1_1R {
        match value {
            false => ADC1_1R::DIGITAL_FUNCTION_SEL,
            true => ADC1_1R::ANALOG_FUNCTION_ADC1,
        }
    }
    #[doc = "Checks if the value of the field is `DIGITAL_FUNCTION_SEL`"]
    #[inline]
    pub fn is_digital_function_sel(&self) -> bool {
        *self == ADC1_1R::DIGITAL_FUNCTION_SEL
    }
    #[doc = "Checks if the value of the field is `ANALOG_FUNCTION_ADC1`"]
    #[inline]
    pub fn is_analog_function_adc1(&self) -> bool {
        *self == ADC1_1R::ANALOG_FUNCTION_ADC1
    }
}
#[doc = "Possible values of the field `ADC1_2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC1_2R {
    #[doc = "Digital function selected on pin PF_9."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Analog function ADC1_2 selected on pin PF_9."]
    ANALOG_FUNCTION_ADC1,
}
impl ADC1_2R {
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
            ADC1_2R::DIGITAL_FUNCTION_SEL => false,
            ADC1_2R::ANALOG_FUNCTION_ADC1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC1_2R {
        match value {
            false => ADC1_2R::DIGITAL_FUNCTION_SEL,
            true => ADC1_2R::ANALOG_FUNCTION_ADC1,
        }
    }
    #[doc = "Checks if the value of the field is `DIGITAL_FUNCTION_SEL`"]
    #[inline]
    pub fn is_digital_function_sel(&self) -> bool {
        *self == ADC1_2R::DIGITAL_FUNCTION_SEL
    }
    #[doc = "Checks if the value of the field is `ANALOG_FUNCTION_ADC1`"]
    #[inline]
    pub fn is_analog_function_adc1(&self) -> bool {
        *self == ADC1_2R::ANALOG_FUNCTION_ADC1
    }
}
#[doc = "Possible values of the field `ADC1_3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC1_3R {
    #[doc = "Digital function selected on pin PF_6."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Analog function ADC1_3 selected on pin PF_6."]
    ANALOG_FUNCTION_ADC1,
}
impl ADC1_3R {
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
            ADC1_3R::DIGITAL_FUNCTION_SEL => false,
            ADC1_3R::ANALOG_FUNCTION_ADC1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC1_3R {
        match value {
            false => ADC1_3R::DIGITAL_FUNCTION_SEL,
            true => ADC1_3R::ANALOG_FUNCTION_ADC1,
        }
    }
    #[doc = "Checks if the value of the field is `DIGITAL_FUNCTION_SEL`"]
    #[inline]
    pub fn is_digital_function_sel(&self) -> bool {
        *self == ADC1_3R::DIGITAL_FUNCTION_SEL
    }
    #[doc = "Checks if the value of the field is `ANALOG_FUNCTION_ADC1`"]
    #[inline]
    pub fn is_analog_function_adc1(&self) -> bool {
        *self == ADC1_3R::ANALOG_FUNCTION_ADC1
    }
}
#[doc = "Possible values of the field `ADC1_4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC1_4R {
    #[doc = "Digital function selected on pin PF_5."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Analog function ADC1_4 selected on pin PF_5."]
    ANALOG_FUNCTION_ADC1,
}
impl ADC1_4R {
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
            ADC1_4R::DIGITAL_FUNCTION_SEL => false,
            ADC1_4R::ANALOG_FUNCTION_ADC1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC1_4R {
        match value {
            false => ADC1_4R::DIGITAL_FUNCTION_SEL,
            true => ADC1_4R::ANALOG_FUNCTION_ADC1,
        }
    }
    #[doc = "Checks if the value of the field is `DIGITAL_FUNCTION_SEL`"]
    #[inline]
    pub fn is_digital_function_sel(&self) -> bool {
        *self == ADC1_4R::DIGITAL_FUNCTION_SEL
    }
    #[doc = "Checks if the value of the field is `ANALOG_FUNCTION_ADC1`"]
    #[inline]
    pub fn is_analog_function_adc1(&self) -> bool {
        *self == ADC1_4R::ANALOG_FUNCTION_ADC1
    }
}
#[doc = "Possible values of the field `ADC1_5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC1_5R {
    #[doc = "Digital function selected on pin PF_11."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Analog function ADC1_5 selected on pin PF_11."]
    ANALOG_FUNCTION_ADC1,
}
impl ADC1_5R {
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
            ADC1_5R::DIGITAL_FUNCTION_SEL => false,
            ADC1_5R::ANALOG_FUNCTION_ADC1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC1_5R {
        match value {
            false => ADC1_5R::DIGITAL_FUNCTION_SEL,
            true => ADC1_5R::ANALOG_FUNCTION_ADC1,
        }
    }
    #[doc = "Checks if the value of the field is `DIGITAL_FUNCTION_SEL`"]
    #[inline]
    pub fn is_digital_function_sel(&self) -> bool {
        *self == ADC1_5R::DIGITAL_FUNCTION_SEL
    }
    #[doc = "Checks if the value of the field is `ANALOG_FUNCTION_ADC1`"]
    #[inline]
    pub fn is_analog_function_adc1(&self) -> bool {
        *self == ADC1_5R::ANALOG_FUNCTION_ADC1
    }
}
#[doc = "Possible values of the field `ADC1_6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC1_6R {
    #[doc = "Digital function selected on pin P7_7."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Analog function ADC1_6 selected on pin P7_7."]
    ANALOG_FUNCTION_ADC1,
}
impl ADC1_6R {
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
            ADC1_6R::DIGITAL_FUNCTION_SEL => false,
            ADC1_6R::ANALOG_FUNCTION_ADC1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC1_6R {
        match value {
            false => ADC1_6R::DIGITAL_FUNCTION_SEL,
            true => ADC1_6R::ANALOG_FUNCTION_ADC1,
        }
    }
    #[doc = "Checks if the value of the field is `DIGITAL_FUNCTION_SEL`"]
    #[inline]
    pub fn is_digital_function_sel(&self) -> bool {
        *self == ADC1_6R::DIGITAL_FUNCTION_SEL
    }
    #[doc = "Checks if the value of the field is `ANALOG_FUNCTION_ADC1`"]
    #[inline]
    pub fn is_analog_function_adc1(&self) -> bool {
        *self == ADC1_6R::ANALOG_FUNCTION_ADC1
    }
}
#[doc = "Possible values of the field `ADC1_7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC1_7R {
    #[doc = "Digital function selected on pin PF_7."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Analog function ADC1_7 selected on pin PF_7."]
    ANALOG_FUNCTION_ADC1,
}
impl ADC1_7R {
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
            ADC1_7R::DIGITAL_FUNCTION_SEL => false,
            ADC1_7R::ANALOG_FUNCTION_ADC1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC1_7R {
        match value {
            false => ADC1_7R::DIGITAL_FUNCTION_SEL,
            true => ADC1_7R::ANALOG_FUNCTION_ADC1,
        }
    }
    #[doc = "Checks if the value of the field is `DIGITAL_FUNCTION_SEL`"]
    #[inline]
    pub fn is_digital_function_sel(&self) -> bool {
        *self == ADC1_7R::DIGITAL_FUNCTION_SEL
    }
    #[doc = "Checks if the value of the field is `ANALOG_FUNCTION_ADC1`"]
    #[inline]
    pub fn is_analog_function_adc1(&self) -> bool {
        *self == ADC1_7R::ANALOG_FUNCTION_ADC1
    }
}
#[doc = "Values that can be written to the field `ADC1_0`"]
pub enum ADC1_0W {
    #[doc = "Digital function selected on pin PC_3."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Analog function ADC1_0 selected on pin PC_3."]
    ANALOG_FUNCTION_ADC1,
}
impl ADC1_0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC1_0W::DIGITAL_FUNCTION_SEL => false,
            ADC1_0W::ANALOG_FUNCTION_ADC1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC1_0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC1_0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC1_0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital function selected on pin PC_3."]
    #[inline]
    pub fn digital_function_sel(self) -> &'a mut W {
        self.variant(ADC1_0W::DIGITAL_FUNCTION_SEL)
    }
    #[doc = "Analog function ADC1_0 selected on pin PC_3."]
    #[inline]
    pub fn analog_function_adc1(self) -> &'a mut W {
        self.variant(ADC1_0W::ANALOG_FUNCTION_ADC1)
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
#[doc = "Values that can be written to the field `ADC1_1`"]
pub enum ADC1_1W {
    #[doc = "Digital function selected on pin PC_0."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Analog function ADC1_1 selected on pin PC_0."]
    ANALOG_FUNCTION_ADC1,
}
impl ADC1_1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC1_1W::DIGITAL_FUNCTION_SEL => false,
            ADC1_1W::ANALOG_FUNCTION_ADC1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC1_1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC1_1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC1_1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital function selected on pin PC_0."]
    #[inline]
    pub fn digital_function_sel(self) -> &'a mut W {
        self.variant(ADC1_1W::DIGITAL_FUNCTION_SEL)
    }
    #[doc = "Analog function ADC1_1 selected on pin PC_0."]
    #[inline]
    pub fn analog_function_adc1(self) -> &'a mut W {
        self.variant(ADC1_1W::ANALOG_FUNCTION_ADC1)
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
#[doc = "Values that can be written to the field `ADC1_2`"]
pub enum ADC1_2W {
    #[doc = "Digital function selected on pin PF_9."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Analog function ADC1_2 selected on pin PF_9."]
    ANALOG_FUNCTION_ADC1,
}
impl ADC1_2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC1_2W::DIGITAL_FUNCTION_SEL => false,
            ADC1_2W::ANALOG_FUNCTION_ADC1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC1_2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC1_2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC1_2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital function selected on pin PF_9."]
    #[inline]
    pub fn digital_function_sel(self) -> &'a mut W {
        self.variant(ADC1_2W::DIGITAL_FUNCTION_SEL)
    }
    #[doc = "Analog function ADC1_2 selected on pin PF_9."]
    #[inline]
    pub fn analog_function_adc1(self) -> &'a mut W {
        self.variant(ADC1_2W::ANALOG_FUNCTION_ADC1)
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
#[doc = "Values that can be written to the field `ADC1_3`"]
pub enum ADC1_3W {
    #[doc = "Digital function selected on pin PF_6."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Analog function ADC1_3 selected on pin PF_6."]
    ANALOG_FUNCTION_ADC1,
}
impl ADC1_3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC1_3W::DIGITAL_FUNCTION_SEL => false,
            ADC1_3W::ANALOG_FUNCTION_ADC1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC1_3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC1_3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC1_3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital function selected on pin PF_6."]
    #[inline]
    pub fn digital_function_sel(self) -> &'a mut W {
        self.variant(ADC1_3W::DIGITAL_FUNCTION_SEL)
    }
    #[doc = "Analog function ADC1_3 selected on pin PF_6."]
    #[inline]
    pub fn analog_function_adc1(self) -> &'a mut W {
        self.variant(ADC1_3W::ANALOG_FUNCTION_ADC1)
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
#[doc = "Values that can be written to the field `ADC1_4`"]
pub enum ADC1_4W {
    #[doc = "Digital function selected on pin PF_5."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Analog function ADC1_4 selected on pin PF_5."]
    ANALOG_FUNCTION_ADC1,
}
impl ADC1_4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC1_4W::DIGITAL_FUNCTION_SEL => false,
            ADC1_4W::ANALOG_FUNCTION_ADC1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC1_4W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC1_4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC1_4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital function selected on pin PF_5."]
    #[inline]
    pub fn digital_function_sel(self) -> &'a mut W {
        self.variant(ADC1_4W::DIGITAL_FUNCTION_SEL)
    }
    #[doc = "Analog function ADC1_4 selected on pin PF_5."]
    #[inline]
    pub fn analog_function_adc1(self) -> &'a mut W {
        self.variant(ADC1_4W::ANALOG_FUNCTION_ADC1)
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
#[doc = "Values that can be written to the field `ADC1_5`"]
pub enum ADC1_5W {
    #[doc = "Digital function selected on pin PF_11."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Analog function ADC1_5 selected on pin PF_11."]
    ANALOG_FUNCTION_ADC1,
}
impl ADC1_5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC1_5W::DIGITAL_FUNCTION_SEL => false,
            ADC1_5W::ANALOG_FUNCTION_ADC1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC1_5W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC1_5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC1_5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital function selected on pin PF_11."]
    #[inline]
    pub fn digital_function_sel(self) -> &'a mut W {
        self.variant(ADC1_5W::DIGITAL_FUNCTION_SEL)
    }
    #[doc = "Analog function ADC1_5 selected on pin PF_11."]
    #[inline]
    pub fn analog_function_adc1(self) -> &'a mut W {
        self.variant(ADC1_5W::ANALOG_FUNCTION_ADC1)
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
#[doc = "Values that can be written to the field `ADC1_6`"]
pub enum ADC1_6W {
    #[doc = "Digital function selected on pin P7_7."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Analog function ADC1_6 selected on pin P7_7."]
    ANALOG_FUNCTION_ADC1,
}
impl ADC1_6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC1_6W::DIGITAL_FUNCTION_SEL => false,
            ADC1_6W::ANALOG_FUNCTION_ADC1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC1_6W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC1_6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC1_6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital function selected on pin P7_7."]
    #[inline]
    pub fn digital_function_sel(self) -> &'a mut W {
        self.variant(ADC1_6W::DIGITAL_FUNCTION_SEL)
    }
    #[doc = "Analog function ADC1_6 selected on pin P7_7."]
    #[inline]
    pub fn analog_function_adc1(self) -> &'a mut W {
        self.variant(ADC1_6W::ANALOG_FUNCTION_ADC1)
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
#[doc = "Values that can be written to the field `ADC1_7`"]
pub enum ADC1_7W {
    #[doc = "Digital function selected on pin PF_7."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Analog function ADC1_7 selected on pin PF_7."]
    ANALOG_FUNCTION_ADC1,
}
impl ADC1_7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC1_7W::DIGITAL_FUNCTION_SEL => false,
            ADC1_7W::ANALOG_FUNCTION_ADC1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC1_7W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC1_7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC1_7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital function selected on pin PF_7."]
    #[inline]
    pub fn digital_function_sel(self) -> &'a mut W {
        self.variant(ADC1_7W::DIGITAL_FUNCTION_SEL)
    }
    #[doc = "Analog function ADC1_7 selected on pin PF_7."]
    #[inline]
    pub fn analog_function_adc1(self) -> &'a mut W {
        self.variant(ADC1_7W::ANALOG_FUNCTION_ADC1)
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
    #[doc = "Bit 0 - Select ADC1_0"]
    #[inline]
    pub fn adc1_0(&self) -> ADC1_0R {
        ADC1_0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Select ADC1_1"]
    #[inline]
    pub fn adc1_1(&self) -> ADC1_1R {
        ADC1_1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Select ADC1_2"]
    #[inline]
    pub fn adc1_2(&self) -> ADC1_2R {
        ADC1_2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Select ADC1_3"]
    #[inline]
    pub fn adc1_3(&self) -> ADC1_3R {
        ADC1_3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Select ADC1_4"]
    #[inline]
    pub fn adc1_4(&self) -> ADC1_4R {
        ADC1_4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Select ADC1_5"]
    #[inline]
    pub fn adc1_5(&self) -> ADC1_5R {
        ADC1_5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Select ADC1_6"]
    #[inline]
    pub fn adc1_6(&self) -> ADC1_6R {
        ADC1_6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Select ADC1_7."]
    #[inline]
    pub fn adc1_7(&self) -> ADC1_7R {
        ADC1_7R::_from({
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
    #[doc = "Bit 0 - Select ADC1_0"]
    #[inline]
    pub fn adc1_0(&mut self) -> _ADC1_0W {
        _ADC1_0W { w: self }
    }
    #[doc = "Bit 1 - Select ADC1_1"]
    #[inline]
    pub fn adc1_1(&mut self) -> _ADC1_1W {
        _ADC1_1W { w: self }
    }
    #[doc = "Bit 2 - Select ADC1_2"]
    #[inline]
    pub fn adc1_2(&mut self) -> _ADC1_2W {
        _ADC1_2W { w: self }
    }
    #[doc = "Bit 3 - Select ADC1_3"]
    #[inline]
    pub fn adc1_3(&mut self) -> _ADC1_3W {
        _ADC1_3W { w: self }
    }
    #[doc = "Bit 4 - Select ADC1_4"]
    #[inline]
    pub fn adc1_4(&mut self) -> _ADC1_4W {
        _ADC1_4W { w: self }
    }
    #[doc = "Bit 5 - Select ADC1_5"]
    #[inline]
    pub fn adc1_5(&mut self) -> _ADC1_5W {
        _ADC1_5W { w: self }
    }
    #[doc = "Bit 6 - Select ADC1_6"]
    #[inline]
    pub fn adc1_6(&mut self) -> _ADC1_6W {
        _ADC1_6W { w: self }
    }
    #[doc = "Bit 7 - Select ADC1_7."]
    #[inline]
    pub fn adc1_7(&mut self) -> _ADC1_7W {
        _ADC1_7W { w: self }
    }
}
