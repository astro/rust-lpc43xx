#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DYNAMICRASCAS {
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
#[doc = "Possible values of the field `RAS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RASR {
    #[doc = "One EMC_CCLK cycle."]
    ONE_EMC_CCLK_CYCLE,
    #[doc = "Two EMC_CCLK cycles."]
    TWO_EMC_CCLK_CYCLES,
    #[doc = "Three EMC_CCLK cycles (POR reset value)."]
    THREE_EMC_CCLK_CYCLE,
}
impl RASR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RASR::ONE_EMC_CCLK_CYCLE => 1,
            RASR::TWO_EMC_CCLK_CYCLES => 2,
            RASR::THREE_EMC_CCLK_CYCLE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RASR {
        match value {
            1 => RASR::ONE_EMC_CCLK_CYCLE,
            2 => RASR::TWO_EMC_CCLK_CYCLES,
            3 => RASR::THREE_EMC_CCLK_CYCLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONE_EMC_CCLK_CYCLE`"]
    #[inline]
    pub fn is_one_emc_cclk_cycle(&self) -> bool {
        *self == RASR::ONE_EMC_CCLK_CYCLE
    }
    #[doc = "Checks if the value of the field is `TWO_EMC_CCLK_CYCLES`"]
    #[inline]
    pub fn is_two_emc_cclk_cycles(&self) -> bool {
        *self == RASR::TWO_EMC_CCLK_CYCLES
    }
    #[doc = "Checks if the value of the field is `THREE_EMC_CCLK_CYCLE`"]
    #[inline]
    pub fn is_three_emc_cclk_cycle(&self) -> bool {
        *self == RASR::THREE_EMC_CCLK_CYCLE
    }
}
#[doc = "Possible values of the field `CAS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CASR {
    #[doc = "One EMC_CCLK cycle."]
    ONE_EMC_CCLK_CYCLE,
    #[doc = "Two EMC_CCLK cycles."]
    TWO_EMC_CCLK_CYCLES,
    #[doc = "Three EMC_CCLK cycles (POR reset value)."]
    THREE_EMC_CCLK_CYCLE,
}
impl CASR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CASR::ONE_EMC_CCLK_CYCLE => 1,
            CASR::TWO_EMC_CCLK_CYCLES => 2,
            CASR::THREE_EMC_CCLK_CYCLE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CASR {
        match value {
            1 => CASR::ONE_EMC_CCLK_CYCLE,
            2 => CASR::TWO_EMC_CCLK_CYCLES,
            3 => CASR::THREE_EMC_CCLK_CYCLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONE_EMC_CCLK_CYCLE`"]
    #[inline]
    pub fn is_one_emc_cclk_cycle(&self) -> bool {
        *self == CASR::ONE_EMC_CCLK_CYCLE
    }
    #[doc = "Checks if the value of the field is `TWO_EMC_CCLK_CYCLES`"]
    #[inline]
    pub fn is_two_emc_cclk_cycles(&self) -> bool {
        *self == CASR::TWO_EMC_CCLK_CYCLES
    }
    #[doc = "Checks if the value of the field is `THREE_EMC_CCLK_CYCLE`"]
    #[inline]
    pub fn is_three_emc_cclk_cycle(&self) -> bool {
        *self == CASR::THREE_EMC_CCLK_CYCLE
    }
}
#[doc = "Values that can be written to the field `RAS`"]
pub enum RASW {
    #[doc = "One EMC_CCLK cycle."]
    ONE_EMC_CCLK_CYCLE,
    #[doc = "Two EMC_CCLK cycles."]
    TWO_EMC_CCLK_CYCLES,
    #[doc = "Three EMC_CCLK cycles (POR reset value)."]
    THREE_EMC_CCLK_CYCLE,
}
impl RASW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RASW::ONE_EMC_CCLK_CYCLE => 1,
            RASW::TWO_EMC_CCLK_CYCLES => 2,
            RASW::THREE_EMC_CCLK_CYCLE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RASW<'a> {
    w: &'a mut W,
}
impl<'a> _RASW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RASW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "One EMC_CCLK cycle."]
    #[inline]
    pub fn one_emc_cclk_cycle(self) -> &'a mut W {
        self.variant(RASW::ONE_EMC_CCLK_CYCLE)
    }
    #[doc = "Two EMC_CCLK cycles."]
    #[inline]
    pub fn two_emc_cclk_cycles(self) -> &'a mut W {
        self.variant(RASW::TWO_EMC_CCLK_CYCLES)
    }
    #[doc = "Three EMC_CCLK cycles (POR reset value)."]
    #[inline]
    pub fn three_emc_cclk_cycle(self) -> &'a mut W {
        self.variant(RASW::THREE_EMC_CCLK_CYCLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CAS`"]
pub enum CASW {
    #[doc = "One EMC_CCLK cycle."]
    ONE_EMC_CCLK_CYCLE,
    #[doc = "Two EMC_CCLK cycles."]
    TWO_EMC_CCLK_CYCLES,
    #[doc = "Three EMC_CCLK cycles (POR reset value)."]
    THREE_EMC_CCLK_CYCLE,
}
impl CASW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CASW::ONE_EMC_CCLK_CYCLE => 1,
            CASW::TWO_EMC_CCLK_CYCLES => 2,
            CASW::THREE_EMC_CCLK_CYCLE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CASW<'a> {
    w: &'a mut W,
}
impl<'a> _CASW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CASW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "One EMC_CCLK cycle."]
    #[inline]
    pub fn one_emc_cclk_cycle(self) -> &'a mut W {
        self.variant(CASW::ONE_EMC_CCLK_CYCLE)
    }
    #[doc = "Two EMC_CCLK cycles."]
    #[inline]
    pub fn two_emc_cclk_cycles(self) -> &'a mut W {
        self.variant(CASW::TWO_EMC_CCLK_CYCLES)
    }
    #[doc = "Three EMC_CCLK cycles (POR reset value)."]
    #[inline]
    pub fn three_emc_cclk_cycle(self) -> &'a mut W {
        self.variant(CASW::THREE_EMC_CCLK_CYCLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:1 - RAS latency (active to read/write delay)."]
    #[inline]
    pub fn ras(&self) -> RASR {
        RASR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - CAS latency."]
    #[inline]
    pub fn cas(&self) -> CASR {
        CASR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 771 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - RAS latency (active to read/write delay)."]
    #[inline]
    pub fn ras(&mut self) -> _RASW {
        _RASW { w: self }
    }
    #[doc = "Bits 8:9 - CAS latency."]
    #[inline]
    pub fn cas(&mut self) -> _CASW {
        _CASW { w: self }
    }
}
