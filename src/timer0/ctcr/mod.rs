#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTCR {
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
#[doc = "Possible values of the field `CTMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTMODER {
    #[doc = "Timer Mode. Counts every rising PCLK edge"]
    TIMER_MODE,
    #[doc = "Counter Mode rising edge. TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    COUNTER_MODE_RISING,
    #[doc = "Counter Mode falling edge. TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    COUNTER_MODE_FALLING,
    #[doc = "Counter Mode edges. TC is incremented on both edges on the CAP input selected by bits 3:2."]
    COUNTER_MODE_EDGES,
}
impl CTMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CTMODER::TIMER_MODE => 0,
            CTMODER::COUNTER_MODE_RISING => 1,
            CTMODER::COUNTER_MODE_FALLING => 2,
            CTMODER::COUNTER_MODE_EDGES => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CTMODER {
        match value {
            0 => CTMODER::TIMER_MODE,
            1 => CTMODER::COUNTER_MODE_RISING,
            2 => CTMODER::COUNTER_MODE_FALLING,
            3 => CTMODER::COUNTER_MODE_EDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_MODE`"]
    #[inline]
    pub fn is_timer_mode(&self) -> bool {
        *self == CTMODER::TIMER_MODE
    }
    #[doc = "Checks if the value of the field is `COUNTER_MODE_RISING`"]
    #[inline]
    pub fn is_counter_mode_rising(&self) -> bool {
        *self == CTMODER::COUNTER_MODE_RISING
    }
    #[doc = "Checks if the value of the field is `COUNTER_MODE_FALLING`"]
    #[inline]
    pub fn is_counter_mode_falling(&self) -> bool {
        *self == CTMODER::COUNTER_MODE_FALLING
    }
    #[doc = "Checks if the value of the field is `COUNTER_MODE_EDGES`"]
    #[inline]
    pub fn is_counter_mode_edges(&self) -> bool {
        *self == CTMODER::COUNTER_MODE_EDGES
    }
}
#[doc = "Possible values of the field `CINSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINSELR {
    #[doc = "CAP0. CAPn.0 for TIMERn"]
    CAP0,
    #[doc = "CAP1. CAPn.1 for TIMERn"]
    CAP1,
    #[doc = "CAP2. CAPn.2 for TIMERn"]
    CAP2,
    #[doc = "CAP3. CAPn.3 for TIMERn"]
    CAP3,
}
impl CINSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CINSELR::CAP0 => 0,
            CINSELR::CAP1 => 1,
            CINSELR::CAP2 => 2,
            CINSELR::CAP3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CINSELR {
        match value {
            0 => CINSELR::CAP0,
            1 => CINSELR::CAP1,
            2 => CINSELR::CAP2,
            3 => CINSELR::CAP3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CAP0`"]
    #[inline]
    pub fn is_cap0(&self) -> bool {
        *self == CINSELR::CAP0
    }
    #[doc = "Checks if the value of the field is `CAP1`"]
    #[inline]
    pub fn is_cap1(&self) -> bool {
        *self == CINSELR::CAP1
    }
    #[doc = "Checks if the value of the field is `CAP2`"]
    #[inline]
    pub fn is_cap2(&self) -> bool {
        *self == CINSELR::CAP2
    }
    #[doc = "Checks if the value of the field is `CAP3`"]
    #[inline]
    pub fn is_cap3(&self) -> bool {
        *self == CINSELR::CAP3
    }
}
#[doc = "Values that can be written to the field `CTMODE`"]
pub enum CTMODEW {
    #[doc = "Timer Mode. Counts every rising PCLK edge"]
    TIMER_MODE,
    #[doc = "Counter Mode rising edge. TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    COUNTER_MODE_RISING,
    #[doc = "Counter Mode falling edge. TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    COUNTER_MODE_FALLING,
    #[doc = "Counter Mode edges. TC is incremented on both edges on the CAP input selected by bits 3:2."]
    COUNTER_MODE_EDGES,
}
impl CTMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CTMODEW::TIMER_MODE => 0,
            CTMODEW::COUNTER_MODE_RISING => 1,
            CTMODEW::COUNTER_MODE_FALLING => 2,
            CTMODEW::COUNTER_MODE_EDGES => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timer Mode. Counts every rising PCLK edge"]
    #[inline]
    pub fn timer_mode(self) -> &'a mut W {
        self.variant(CTMODEW::TIMER_MODE)
    }
    #[doc = "Counter Mode rising edge. TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    #[inline]
    pub fn counter_mode_rising(self) -> &'a mut W {
        self.variant(CTMODEW::COUNTER_MODE_RISING)
    }
    #[doc = "Counter Mode falling edge. TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    #[inline]
    pub fn counter_mode_falling(self) -> &'a mut W {
        self.variant(CTMODEW::COUNTER_MODE_FALLING)
    }
    #[doc = "Counter Mode edges. TC is incremented on both edges on the CAP input selected by bits 3:2."]
    #[inline]
    pub fn counter_mode_edges(self) -> &'a mut W {
        self.variant(CTMODEW::COUNTER_MODE_EDGES)
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
#[doc = "Values that can be written to the field `CINSEL`"]
pub enum CINSELW {
    #[doc = "CAP0. CAPn.0 for TIMERn"]
    CAP0,
    #[doc = "CAP1. CAPn.1 for TIMERn"]
    CAP1,
    #[doc = "CAP2. CAPn.2 for TIMERn"]
    CAP2,
    #[doc = "CAP3. CAPn.3 for TIMERn"]
    CAP3,
}
impl CINSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CINSELW::CAP0 => 0,
            CINSELW::CAP1 => 1,
            CINSELW::CAP2 => 2,
            CINSELW::CAP3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CINSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CINSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CINSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CAP0. CAPn.0 for TIMERn"]
    #[inline]
    pub fn cap0(self) -> &'a mut W {
        self.variant(CINSELW::CAP0)
    }
    #[doc = "CAP1. CAPn.1 for TIMERn"]
    #[inline]
    pub fn cap1(self) -> &'a mut W {
        self.variant(CINSELW::CAP1)
    }
    #[doc = "CAP2. CAPn.2 for TIMERn"]
    #[inline]
    pub fn cap2(self) -> &'a mut W {
        self.variant(CINSELW::CAP2)
    }
    #[doc = "CAP3. CAPn.3 for TIMERn"]
    #[inline]
    pub fn cap3(self) -> &'a mut W {
        self.variant(CINSELW::CAP3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
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
    #[doc = "Bits 0:1 - Counter/Timer Mode This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
    #[inline]
    pub fn ctmode(&self) -> CTMODER {
        CTMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the TnCTCR, the 3 bits for that input in the Capture Control Register (TnCCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
    #[inline]
    pub fn cinsel(&self) -> CINSELR {
        CINSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
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
    #[doc = "Bits 0:1 - Counter/Timer Mode This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
    #[inline]
    pub fn ctmode(&mut self) -> _CTMODEW {
        _CTMODEW { w: self }
    }
    #[doc = "Bits 2:3 - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the TnCTCR, the 3 bits for that input in the Capture Control Register (TnCCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
    #[inline]
    pub fn cinsel(&mut self) -> _CINSELW {
        _CINSELW { w: self }
    }
}
