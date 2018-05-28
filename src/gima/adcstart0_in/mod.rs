#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ADCSTART0_IN {
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
#[doc = "Possible values of the field `INV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVR {
    #[doc = "Not inverted."]
    NOT_INVERTED,
    #[doc = "Input inverted."]
    INPUT_INVERTED,
}
impl INVR {
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
            INVR::NOT_INVERTED => false,
            INVR::INPUT_INVERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INVR {
        match value {
            false => INVR::NOT_INVERTED,
            true => INVR::INPUT_INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INVERTED`"]
    #[inline]
    pub fn is_not_inverted(&self) -> bool {
        *self == INVR::NOT_INVERTED
    }
    #[doc = "Checks if the value of the field is `INPUT_INVERTED`"]
    #[inline]
    pub fn is_input_inverted(&self) -> bool {
        *self == INVR::INPUT_INVERTED
    }
}
#[doc = "Possible values of the field `EDGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGER {
    #[doc = "No edge detection."]
    NO_EDGE_DETECTION,
    #[doc = "Rising edge detection enabled."]
    RISING_EDGE_DETECTIO,
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
            EDGER::NO_EDGE_DETECTION => false,
            EDGER::RISING_EDGE_DETECTIO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDGER {
        match value {
            false => EDGER::NO_EDGE_DETECTION,
            true => EDGER::RISING_EDGE_DETECTIO,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EDGE_DETECTION`"]
    #[inline]
    pub fn is_no_edge_detection(&self) -> bool {
        *self == EDGER::NO_EDGE_DETECTION
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE_DETECTIO`"]
    #[inline]
    pub fn is_rising_edge_detectio(&self) -> bool {
        *self == EDGER::RISING_EDGE_DETECTIO
    }
}
#[doc = "Possible values of the field `SYNCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCHR {
    #[doc = "Disable   synchronization."]
    DISABLE__SYNCHRONIZ,
    #[doc = "Enable   synchronization."]
    ENABLE__SYNCHRONIZA,
}
impl SYNCHR {
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
            SYNCHR::DISABLE__SYNCHRONIZ => false,
            SYNCHR::ENABLE__SYNCHRONIZA => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYNCHR {
        match value {
            false => SYNCHR::DISABLE__SYNCHRONIZ,
            true => SYNCHR::ENABLE__SYNCHRONIZA,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE__SYNCHRONIZ`"]
    #[inline]
    pub fn is_disable__synchroniz(&self) -> bool {
        *self == SYNCHR::DISABLE__SYNCHRONIZ
    }
    #[doc = "Checks if the value of the field is `ENABLE__SYNCHRONIZA`"]
    #[inline]
    pub fn is_enable__synchroniza(&self) -> bool {
        *self == SYNCHR::ENABLE__SYNCHRONIZA
    }
}
#[doc = "Possible values of the field `PULSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PULSER {
    #[doc = "Disable single pulse generation."]
    DISABLE_SINGLE_PULSE,
    #[doc = "Enable single pulse generation."]
    ENABLE_SINGLE_PULSE,
}
impl PULSER {
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
            PULSER::DISABLE_SINGLE_PULSE => false,
            PULSER::ENABLE_SINGLE_PULSE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PULSER {
        match value {
            false => PULSER::DISABLE_SINGLE_PULSE,
            true => PULSER::ENABLE_SINGLE_PULSE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_SINGLE_PULSE`"]
    #[inline]
    pub fn is_disable_single_pulse(&self) -> bool {
        *self == PULSER::DISABLE_SINGLE_PULSE
    }
    #[doc = "Checks if the value of the field is `ENABLE_SINGLE_PULSE`"]
    #[inline]
    pub fn is_enable_single_pulse(&self) -> bool {
        *self == PULSER::ENABLE_SINGLE_PULSE
    }
}
#[doc = "Possible values of the field `SELECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELECTR {
    #[doc = "CTOUT_15 or T3_MAT3"]
    CTOUT_15_OR_T3_MAT3,
    #[doc = "T0_MAT0"]
    T0_MAT0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SELECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SELECTR::CTOUT_15_OR_T3_MAT3 => 0,
            SELECTR::T0_MAT0 => 1,
            SELECTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SELECTR {
        match value {
            0 => SELECTR::CTOUT_15_OR_T3_MAT3,
            1 => SELECTR::T0_MAT0,
            i => SELECTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CTOUT_15_OR_T3_MAT3`"]
    #[inline]
    pub fn is_ctout_15_or_t3_mat3(&self) -> bool {
        *self == SELECTR::CTOUT_15_OR_T3_MAT3
    }
    #[doc = "Checks if the value of the field is `T0_MAT0`"]
    #[inline]
    pub fn is_t0_mat0(&self) -> bool {
        *self == SELECTR::T0_MAT0
    }
}
#[doc = "Values that can be written to the field `INV`"]
pub enum INVW {
    #[doc = "Not inverted."]
    NOT_INVERTED,
    #[doc = "Input inverted."]
    INPUT_INVERTED,
}
impl INVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INVW::NOT_INVERTED => false,
            INVW::INPUT_INVERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INVW<'a> {
    w: &'a mut W,
}
impl<'a> _INVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not inverted."]
    #[inline]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(INVW::NOT_INVERTED)
    }
    #[doc = "Input inverted."]
    #[inline]
    pub fn input_inverted(self) -> &'a mut W {
        self.variant(INVW::INPUT_INVERTED)
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
#[doc = "Values that can be written to the field `EDGE`"]
pub enum EDGEW {
    #[doc = "No edge detection."]
    NO_EDGE_DETECTION,
    #[doc = "Rising edge detection enabled."]
    RISING_EDGE_DETECTIO,
}
impl EDGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDGEW::NO_EDGE_DETECTION => false,
            EDGEW::RISING_EDGE_DETECTIO => true,
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
    #[doc = "No edge detection."]
    #[inline]
    pub fn no_edge_detection(self) -> &'a mut W {
        self.variant(EDGEW::NO_EDGE_DETECTION)
    }
    #[doc = "Rising edge detection enabled."]
    #[inline]
    pub fn rising_edge_detectio(self) -> &'a mut W {
        self.variant(EDGEW::RISING_EDGE_DETECTIO)
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
#[doc = "Values that can be written to the field `SYNCH`"]
pub enum SYNCHW {
    #[doc = "Disable   synchronization."]
    DISABLE__SYNCHRONIZ,
    #[doc = "Enable   synchronization."]
    ENABLE__SYNCHRONIZA,
}
impl SYNCHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYNCHW::DISABLE__SYNCHRONIZ => false,
            SYNCHW::ENABLE__SYNCHRONIZA => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCHW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable synchronization."]
    #[inline]
    pub fn disable__synchroniz(self) -> &'a mut W {
        self.variant(SYNCHW::DISABLE__SYNCHRONIZ)
    }
    #[doc = "Enable synchronization."]
    #[inline]
    pub fn enable__synchroniza(self) -> &'a mut W {
        self.variant(SYNCHW::ENABLE__SYNCHRONIZA)
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
#[doc = "Values that can be written to the field `PULSE`"]
pub enum PULSEW {
    #[doc = "Disable single pulse generation."]
    DISABLE_SINGLE_PULSE,
    #[doc = "Enable single pulse generation."]
    ENABLE_SINGLE_PULSE,
}
impl PULSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PULSEW::DISABLE_SINGLE_PULSE => false,
            PULSEW::ENABLE_SINGLE_PULSE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PULSEW<'a> {
    w: &'a mut W,
}
impl<'a> _PULSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PULSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable single pulse generation."]
    #[inline]
    pub fn disable_single_pulse(self) -> &'a mut W {
        self.variant(PULSEW::DISABLE_SINGLE_PULSE)
    }
    #[doc = "Enable single pulse generation."]
    #[inline]
    pub fn enable_single_pulse(self) -> &'a mut W {
        self.variant(PULSEW::ENABLE_SINGLE_PULSE)
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
#[doc = "Values that can be written to the field `SELECT`"]
pub enum SELECTW {
    #[doc = "CTOUT_15 or T3_MAT3"]
    CTOUT_15_OR_T3_MAT3,
    #[doc = "T0_MAT0"]
    T0_MAT0,
}
impl SELECTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SELECTW::CTOUT_15_OR_T3_MAT3 => 0,
            SELECTW::T0_MAT0 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _SELECTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELECTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CTOUT_15 or T3_MAT3"]
    #[inline]
    pub fn ctout_15_or_t3_mat3(self) -> &'a mut W {
        self.variant(SELECTW::CTOUT_15_OR_T3_MAT3)
    }
    #[doc = "T0_MAT0"]
    #[inline]
    pub fn t0_mat0(self) -> &'a mut W {
        self.variant(SELECTW::T0_MAT0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bit 0 - Invert input"]
    #[inline]
    pub fn inv(&self) -> INVR {
        INVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enable rising edge detection"]
    #[inline]
    pub fn edge(&self) -> EDGER {
        EDGER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enable synchronization"]
    #[inline]
    pub fn synch(&self) -> SYNCHR {
        SYNCHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enable single pulse generation."]
    #[inline]
    pub fn pulse(&self) -> PULSER {
        PULSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:7 - Select input. Values 0x2 to 0xF are reserved."]
    #[inline]
    pub fn select(&self) -> SELECTR {
        SELECTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - Invert input"]
    #[inline]
    pub fn inv(&mut self) -> _INVW {
        _INVW { w: self }
    }
    #[doc = "Bit 1 - Enable rising edge detection"]
    #[inline]
    pub fn edge(&mut self) -> _EDGEW {
        _EDGEW { w: self }
    }
    #[doc = "Bit 2 - Enable synchronization"]
    #[inline]
    pub fn synch(&mut self) -> _SYNCHW {
        _SYNCHW { w: self }
    }
    #[doc = "Bit 3 - Enable single pulse generation."]
    #[inline]
    pub fn pulse(&mut self) -> _PULSEW {
        _PULSEW { w: self }
    }
    #[doc = "Bits 4:7 - Select input. Values 0x2 to 0xF are reserved."]
    #[inline]
    pub fn select(&mut self) -> _SELECTW {
        _SELECTW { w: self }
    }
}
