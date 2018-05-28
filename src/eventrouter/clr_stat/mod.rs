#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLR_STAT {
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
}
#[doc = r" Proxy"]
pub struct _WAKEUP0_CLRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP0_CLRSTW<'a> {
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
#[doc = r" Proxy"]
pub struct _WAKEUP1_CLRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP1_CLRSTW<'a> {
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
#[doc = r" Proxy"]
pub struct _WAKEUP2_CLRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP2_CLRSTW<'a> {
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
#[doc = r" Proxy"]
pub struct _WAKEUP3_CLRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP3_CLRSTW<'a> {
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
#[doc = r" Proxy"]
pub struct _ATIMER_CLRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _ATIMER_CLRSTW<'a> {
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
#[doc = r" Proxy"]
pub struct _RTC_CLRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_CLRSTW<'a> {
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
#[doc = r" Proxy"]
pub struct _BOD_CLRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _BOD_CLRSTW<'a> {
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
#[doc = r" Proxy"]
pub struct _WWDT_CLRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _WWDT_CLRSTW<'a> {
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
pub struct _ETH_CLRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _ETH_CLRSTW<'a> {
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
pub struct _USB0_CLRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB0_CLRSTW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USB1_CLRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB1_CLRSTW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SDMMC_CLRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SDMMC_CLRSTW<'a> {
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
#[doc = r" Proxy"]
pub struct _CAN_CLRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_CLRSTW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TIM2_CLRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM2_CLRSTW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TIM6_CLRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM6_CLRSTW<'a> {
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _QEI_CLRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _QEI_CLRSTW<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TIM14_CLRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM14_CLRSTW<'a> {
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
#[doc = r" Proxy"]
pub struct _RESET_CLRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _RESET_CLRSTW<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BODRESET_CLRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _BODRESET_CLRSTW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DPDRESET_CLRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _DPDRESET_CLRSTW<'a> {
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
    #[doc = "Bit 0 - Writing a 1 to this bit clears the STATUS event bit 0 in the STATUS register."]
    #[inline]
    pub fn wakeup0_clrst(&mut self) -> _WAKEUP0_CLRSTW {
        _WAKEUP0_CLRSTW { w: self }
    }
    #[doc = "Bit 1 - Writing a 1 to this bit clears the STATUS event bit 1 in the STATUS register."]
    #[inline]
    pub fn wakeup1_clrst(&mut self) -> _WAKEUP1_CLRSTW {
        _WAKEUP1_CLRSTW { w: self }
    }
    #[doc = "Bit 2 - Writing a 1 to this bit clears the STATUS event bit 2 in the STATUS register."]
    #[inline]
    pub fn wakeup2_clrst(&mut self) -> _WAKEUP2_CLRSTW {
        _WAKEUP2_CLRSTW { w: self }
    }
    #[doc = "Bit 3 - Writing a 1 to this bit clears the STATUS event bit 3 in the STATUS register."]
    #[inline]
    pub fn wakeup3_clrst(&mut self) -> _WAKEUP3_CLRSTW {
        _WAKEUP3_CLRSTW { w: self }
    }
    #[doc = "Bit 4 - Writing a 1 to this bit clears the STATUS event bit 4 in the STATUS register."]
    #[inline]
    pub fn atimer_clrst(&mut self) -> _ATIMER_CLRSTW {
        _ATIMER_CLRSTW { w: self }
    }
    #[doc = "Bit 5 - Writing a 1 to this bit clears the STATUS event bit 5 in the STATUS register."]
    #[inline]
    pub fn rtc_clrst(&mut self) -> _RTC_CLRSTW {
        _RTC_CLRSTW { w: self }
    }
    #[doc = "Bit 6 - Writing a 1 to this bit clears the STATUS event bit 6 in the STATUS register."]
    #[inline]
    pub fn bod_clrst(&mut self) -> _BOD_CLRSTW {
        _BOD_CLRSTW { w: self }
    }
    #[doc = "Bit 7 - Writing a 1 to this bit clears the STATUS event bit 7 in the STATUS register."]
    #[inline]
    pub fn wwdt_clrst(&mut self) -> _WWDT_CLRSTW {
        _WWDT_CLRSTW { w: self }
    }
    #[doc = "Bit 8 - Writing a 1 to this bit clears the STATUS event bit 8 in the STATUS register."]
    #[inline]
    pub fn eth_clrst(&mut self) -> _ETH_CLRSTW {
        _ETH_CLRSTW { w: self }
    }
    #[doc = "Bit 9 - Writing a 1 to this bit clears the STATUS event bit 9 in the STATUS register."]
    #[inline]
    pub fn usb0_clrst(&mut self) -> _USB0_CLRSTW {
        _USB0_CLRSTW { w: self }
    }
    #[doc = "Bit 10 - Writing a 1 to this bit clears the STATUS event bit 10 in the STATUS register."]
    #[inline]
    pub fn usb1_clrst(&mut self) -> _USB1_CLRSTW {
        _USB1_CLRSTW { w: self }
    }
    #[doc = "Bit 11 - Writing a 1 to this bit clears the STATUS event bit 11 in the STATUS register."]
    #[inline]
    pub fn sdmmc_clrst(&mut self) -> _SDMMC_CLRSTW {
        _SDMMC_CLRSTW { w: self }
    }
    #[doc = "Bit 12 - Writing a 1 to this bit clears the STATUS event bit 12 in the STATUS register."]
    #[inline]
    pub fn can_clrst(&mut self) -> _CAN_CLRSTW {
        _CAN_CLRSTW { w: self }
    }
    #[doc = "Bit 13 - Writing a 1 to this bit clears the STATUS event bit 13 in the STATUS register."]
    #[inline]
    pub fn tim2_clrst(&mut self) -> _TIM2_CLRSTW {
        _TIM2_CLRSTW { w: self }
    }
    #[doc = "Bit 14 - Writing a 1 to this bit clears the STATUS event bit 14 in the STATUS register."]
    #[inline]
    pub fn tim6_clrst(&mut self) -> _TIM6_CLRSTW {
        _TIM6_CLRSTW { w: self }
    }
    #[doc = "Bit 15 - Writing a 1 to this bit clears the STATUS event bit 15 in the STATUS register."]
    #[inline]
    pub fn qei_clrst(&mut self) -> _QEI_CLRSTW {
        _QEI_CLRSTW { w: self }
    }
    #[doc = "Bit 16 - Writing a 1 to this bit clears the STATUS event bit 16 in the STATUS register."]
    #[inline]
    pub fn tim14_clrst(&mut self) -> _TIM14_CLRSTW {
        _TIM14_CLRSTW { w: self }
    }
    #[doc = "Bit 19 - Writing a 1 to this bit clears the STATUS event bit 19 in the STATUS register."]
    #[inline]
    pub fn reset_clrst(&mut self) -> _RESET_CLRSTW {
        _RESET_CLRSTW { w: self }
    }
    #[doc = "Bit 20 - Writing a 1 to this bit clears the STATUS event bit 20 in the STATUS register."]
    #[inline]
    pub fn bodreset_clrst(&mut self) -> _BODRESET_CLRSTW {
        _BODRESET_CLRSTW { w: self }
    }
    #[doc = "Bit 21 - Writing a 1 to this bit clears the STATUS event bit 21 in the STATUS register."]
    #[inline]
    pub fn dpdreset_clrst(&mut self) -> _DPDRESET_CLRSTW {
        _DPDRESET_CLRSTW { w: self }
    }
}
