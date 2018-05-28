#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SET_STAT {
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
pub struct _WAKEUP0_SETSTW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP0_SETSTW<'a> {
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
pub struct _WAKEUP1_SETSTW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP1_SETSTW<'a> {
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
pub struct _WAKEUP2_SETSTW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP2_SETSTW<'a> {
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
pub struct _WAKEUP3_SETSTW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP3_SETSTW<'a> {
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
pub struct _ATIMER_SETSTW<'a> {
    w: &'a mut W,
}
impl<'a> _ATIMER_SETSTW<'a> {
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
pub struct _RTC_SETSTW<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_SETSTW<'a> {
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
pub struct _BOD_SETSTW<'a> {
    w: &'a mut W,
}
impl<'a> _BOD_SETSTW<'a> {
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
pub struct _WWDT_SETSTW<'a> {
    w: &'a mut W,
}
impl<'a> _WWDT_SETSTW<'a> {
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
pub struct _ETH_SETSTW<'a> {
    w: &'a mut W,
}
impl<'a> _ETH_SETSTW<'a> {
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
pub struct _USB0_SETSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB0_SETSTW<'a> {
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
pub struct _USB1_SETSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB1_SETSTW<'a> {
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
pub struct _SDMMC_SETSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SDMMC_SETSTW<'a> {
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
pub struct _CAN_SETSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_SETSTW<'a> {
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
pub struct _TIM2_SETSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM2_SETSTW<'a> {
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
pub struct _TIM6_SETSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM6_SETSTW<'a> {
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
pub struct _QEI_SETSTW<'a> {
    w: &'a mut W,
}
impl<'a> _QEI_SETSTW<'a> {
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
pub struct _TIM14_SETSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM14_SETSTW<'a> {
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
pub struct _RESET_SETSTW<'a> {
    w: &'a mut W,
}
impl<'a> _RESET_SETSTW<'a> {
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
pub struct _BODRESET_SETSTW<'a> {
    w: &'a mut W,
}
impl<'a> _BODRESET_SETSTW<'a> {
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
pub struct _DPDRESET_SETSTW<'a> {
    w: &'a mut W,
}
impl<'a> _DPDRESET_SETSTW<'a> {
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
    #[doc = "Bit 0 - Writing a 1 to this bit sets the STATUS event bit 0 in the STATUS register."]
    #[inline]
    pub fn wakeup0_setst(&mut self) -> _WAKEUP0_SETSTW {
        _WAKEUP0_SETSTW { w: self }
    }
    #[doc = "Bit 1 - Writing a 1 to this bit sets the STATUS event bit 1 in the STATUS register."]
    #[inline]
    pub fn wakeup1_setst(&mut self) -> _WAKEUP1_SETSTW {
        _WAKEUP1_SETSTW { w: self }
    }
    #[doc = "Bit 2 - Writing a 1 to this bit sets the STATUS event bit 2 in the STATUS register."]
    #[inline]
    pub fn wakeup2_setst(&mut self) -> _WAKEUP2_SETSTW {
        _WAKEUP2_SETSTW { w: self }
    }
    #[doc = "Bit 3 - Writing a 1 to this bit sets the STATUS event bit 3 in the STATUS register."]
    #[inline]
    pub fn wakeup3_setst(&mut self) -> _WAKEUP3_SETSTW {
        _WAKEUP3_SETSTW { w: self }
    }
    #[doc = "Bit 4 - Writing a 1 to this bit sets the STATUS event bit 4 in the STATUS register."]
    #[inline]
    pub fn atimer_setst(&mut self) -> _ATIMER_SETSTW {
        _ATIMER_SETSTW { w: self }
    }
    #[doc = "Bit 5 - Writing a 1 to this bit sets the STATUS event bit 5 in the STATUS register."]
    #[inline]
    pub fn rtc_setst(&mut self) -> _RTC_SETSTW {
        _RTC_SETSTW { w: self }
    }
    #[doc = "Bit 6 - Writing a 1 to this bit sets the STATUS event bit 6 in the STATUS register."]
    #[inline]
    pub fn bod_setst(&mut self) -> _BOD_SETSTW {
        _BOD_SETSTW { w: self }
    }
    #[doc = "Bit 7 - Writing a 1 to this bit sets the STATUS event bit 7 in the STATUS register."]
    #[inline]
    pub fn wwdt_setst(&mut self) -> _WWDT_SETSTW {
        _WWDT_SETSTW { w: self }
    }
    #[doc = "Bit 8 - Writing a 1 to this bit sets the STATUS event bit 8 in the STATUS register."]
    #[inline]
    pub fn eth_setst(&mut self) -> _ETH_SETSTW {
        _ETH_SETSTW { w: self }
    }
    #[doc = "Bit 9 - Writing a 1 to this bit sets the STATUS event bit 9 in the STATUS register."]
    #[inline]
    pub fn usb0_setst(&mut self) -> _USB0_SETSTW {
        _USB0_SETSTW { w: self }
    }
    #[doc = "Bit 10 - Writing a 1 to this bit sets the STATUS event bit 10 in the STATUS register."]
    #[inline]
    pub fn usb1_setst(&mut self) -> _USB1_SETSTW {
        _USB1_SETSTW { w: self }
    }
    #[doc = "Bit 11 - Writing a 1 to this bit sets the STATUS event bit 11 in the STATUS register."]
    #[inline]
    pub fn sdmmc_setst(&mut self) -> _SDMMC_SETSTW {
        _SDMMC_SETSTW { w: self }
    }
    #[doc = "Bit 12 - Writing a 1 to this bit sets the STATUS event bit 12 in the STATUS register."]
    #[inline]
    pub fn can_setst(&mut self) -> _CAN_SETSTW {
        _CAN_SETSTW { w: self }
    }
    #[doc = "Bit 13 - Writing a 1 to this bit sets the STATUS event bit 13 in the STATUS register."]
    #[inline]
    pub fn tim2_setst(&mut self) -> _TIM2_SETSTW {
        _TIM2_SETSTW { w: self }
    }
    #[doc = "Bit 14 - Writing a 1 to this bit sets the STATUS event bit 14 in the STATUS register."]
    #[inline]
    pub fn tim6_setst(&mut self) -> _TIM6_SETSTW {
        _TIM6_SETSTW { w: self }
    }
    #[doc = "Bit 15 - Writing a 1 to this bit sets the STATUS event bit 15 in the STATUS register."]
    #[inline]
    pub fn qei_setst(&mut self) -> _QEI_SETSTW {
        _QEI_SETSTW { w: self }
    }
    #[doc = "Bit 16 - Writing a 1 to this bit sets the STATUS event bit 16 in the STATUS register."]
    #[inline]
    pub fn tim14_setst(&mut self) -> _TIM14_SETSTW {
        _TIM14_SETSTW { w: self }
    }
    #[doc = "Bit 19 - Writing a 1 to this bit sets the STATUS event bit 19 in the STATUS register."]
    #[inline]
    pub fn reset_setst(&mut self) -> _RESET_SETSTW {
        _RESET_SETSTW { w: self }
    }
    #[doc = "Bit 20 - Writing a 1 to this bit sets the STATUS event bit 20 in the STATUS register."]
    #[inline]
    pub fn bodreset_setst(&mut self) -> _BODRESET_SETSTW {
        _BODRESET_SETSTW { w: self }
    }
    #[doc = "Bit 21 - Writing a 1 to this bit sets the STATUS event bit 21 in the STATUS register."]
    #[inline]
    pub fn dpdreset_setst(&mut self) -> _DPDRESET_SETSTW {
        _DPDRESET_SETSTW { w: self }
    }
}
