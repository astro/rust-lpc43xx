#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ENAIO2 {
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
#[doc = "Possible values of the field `DAC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACR {
    #[doc = "Digital function selected on pin P4_4."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Analog function DAC selected on pin P4_4."]
    ANALOG_FUNCTION_DAC,
}
impl DACR {
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
            DACR::DIGITAL_FUNCTION_SEL => false,
            DACR::ANALOG_FUNCTION_DAC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DACR {
        match value {
            false => DACR::DIGITAL_FUNCTION_SEL,
            true => DACR::ANALOG_FUNCTION_DAC,
        }
    }
    #[doc = "Checks if the value of the field is `DIGITAL_FUNCTION_SEL`"]
    #[inline]
    pub fn is_digital_function_sel(&self) -> bool {
        *self == DACR::DIGITAL_FUNCTION_SEL
    }
    #[doc = "Checks if the value of the field is `ANALOG_FUNCTION_DAC`"]
    #[inline]
    pub fn is_analog_function_dac(&self) -> bool {
        *self == DACR::ANALOG_FUNCTION_DAC
    }
}
#[doc = "Possible values of the field `BG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGR {
    #[doc = "Digital function selected on pin PF_7."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Band gap output selected for pin PF_7."]
    BAND_GAP_OUTPUT_SELE,
}
impl BGR {
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
            BGR::DIGITAL_FUNCTION_SEL => false,
            BGR::BAND_GAP_OUTPUT_SELE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BGR {
        match value {
            false => BGR::DIGITAL_FUNCTION_SEL,
            true => BGR::BAND_GAP_OUTPUT_SELE,
        }
    }
    #[doc = "Checks if the value of the field is `DIGITAL_FUNCTION_SEL`"]
    #[inline]
    pub fn is_digital_function_sel(&self) -> bool {
        *self == BGR::DIGITAL_FUNCTION_SEL
    }
    #[doc = "Checks if the value of the field is `BAND_GAP_OUTPUT_SELE`"]
    #[inline]
    pub fn is_band_gap_output_sele(&self) -> bool {
        *self == BGR::BAND_GAP_OUTPUT_SELE
    }
}
#[doc = "Values that can be written to the field `DAC`"]
pub enum DACW {
    #[doc = "Digital function selected on pin P4_4."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Analog function DAC selected on pin P4_4."]
    ANALOG_FUNCTION_DAC,
}
impl DACW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DACW::DIGITAL_FUNCTION_SEL => false,
            DACW::ANALOG_FUNCTION_DAC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DACW<'a> {
    w: &'a mut W,
}
impl<'a> _DACW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital function selected on pin P4_4."]
    #[inline]
    pub fn digital_function_sel(self) -> &'a mut W {
        self.variant(DACW::DIGITAL_FUNCTION_SEL)
    }
    #[doc = "Analog function DAC selected on pin P4_4."]
    #[inline]
    pub fn analog_function_dac(self) -> &'a mut W {
        self.variant(DACW::ANALOG_FUNCTION_DAC)
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
#[doc = "Values that can be written to the field `BG`"]
pub enum BGW {
    #[doc = "Digital function selected on pin PF_7."]
    DIGITAL_FUNCTION_SEL,
    #[doc = "Band gap output selected for pin PF_7."]
    BAND_GAP_OUTPUT_SELE,
}
impl BGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BGW::DIGITAL_FUNCTION_SEL => false,
            BGW::BAND_GAP_OUTPUT_SELE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BGW<'a> {
    w: &'a mut W,
}
impl<'a> _BGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital function selected on pin PF_7."]
    #[inline]
    pub fn digital_function_sel(self) -> &'a mut W {
        self.variant(BGW::DIGITAL_FUNCTION_SEL)
    }
    #[doc = "Band gap output selected for pin PF_7."]
    #[inline]
    pub fn band_gap_output_sele(self) -> &'a mut W {
        self.variant(BGW::BAND_GAP_OUTPUT_SELE)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Select DAC"]
    #[inline]
    pub fn dac(&self) -> DACR {
        DACR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Select band gap output. To measure the band gap, disable the pull-up on pin PF_7 and connect PF_7 to the digital pad. Do not use the digital pad nor the ADC1_7 on the board when measuring the band gap (see Section 15.4.8.1)."]
    #[inline]
    pub fn bg(&self) -> BGR {
        BGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - Select DAC"]
    #[inline]
    pub fn dac(&mut self) -> _DACW {
        _DACW { w: self }
    }
    #[doc = "Bit 4 - Select band gap output. To measure the band gap, disable the pull-up on pin PF_7 and connect PF_7 to the digital pad. Do not use the digital pad nor the ADC1_7 on the board when measuring the band gap (see Section 15.4.8.1)."]
    #[inline]
    pub fn bg(&mut self) -> _BGW {
        _BGW { w: self }
    }
}
