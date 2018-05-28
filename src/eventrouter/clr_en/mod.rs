#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLR_EN {
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
pub struct _WAKEUP0_CLRENW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP0_CLRENW<'a> {
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
pub struct _WAKEUP1_CLRENW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP1_CLRENW<'a> {
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
pub struct _WAKEUP2_CLRENW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP2_CLRENW<'a> {
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
pub struct _WAKEUP3_CLRENW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP3_CLRENW<'a> {
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
pub struct _ATIMER_CLRENW<'a> {
    w: &'a mut W,
}
impl<'a> _ATIMER_CLRENW<'a> {
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
pub struct _RTC_CLRENW<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_CLRENW<'a> {
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
pub struct _BOD_CLRENW<'a> {
    w: &'a mut W,
}
impl<'a> _BOD_CLRENW<'a> {
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
pub struct _WWDT_CLRENW<'a> {
    w: &'a mut W,
}
impl<'a> _WWDT_CLRENW<'a> {
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
pub struct _ETH_CLRENW<'a> {
    w: &'a mut W,
}
impl<'a> _ETH_CLRENW<'a> {
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
pub struct _USB0_CLRENW<'a> {
    w: &'a mut W,
}
impl<'a> _USB0_CLRENW<'a> {
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
pub struct _USB1_CLRENW<'a> {
    w: &'a mut W,
}
impl<'a> _USB1_CLRENW<'a> {
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
pub struct _SDMMC_CLRENW<'a> {
    w: &'a mut W,
}
impl<'a> _SDMMC_CLRENW<'a> {
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
pub struct _CAN_CLRENW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_CLRENW<'a> {
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
pub struct _TIM2_CLRENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM2_CLRENW<'a> {
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
pub struct _TIM6_CLRENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM6_CLRENW<'a> {
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
pub struct _QEI_CLRENW<'a> {
    w: &'a mut W,
}
impl<'a> _QEI_CLRENW<'a> {
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
pub struct _TIM14_CLRENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM14_CLRENW<'a> {
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
pub struct _RESET_CLRENW<'a> {
    w: &'a mut W,
}
impl<'a> _RESET_CLRENW<'a> {
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
pub struct _BODRESET_CLRENW<'a> {
    w: &'a mut W,
}
impl<'a> _BODRESET_CLRENW<'a> {
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
pub struct _DPDRESET_CLRENW<'a> {
    w: &'a mut W,
}
impl<'a> _DPDRESET_CLRENW<'a> {
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
    #[doc = "Bit 0 - Writing a 1 to this bit clears the event enable bit 0 in the ENABLE register."]
    #[inline]
    pub fn wakeup0_clren(&mut self) -> _WAKEUP0_CLRENW {
        _WAKEUP0_CLRENW { w: self }
    }
    #[doc = "Bit 1 - Writing a 1 to this bit clears the event enable bit 1 in the ENABLE register."]
    #[inline]
    pub fn wakeup1_clren(&mut self) -> _WAKEUP1_CLRENW {
        _WAKEUP1_CLRENW { w: self }
    }
    #[doc = "Bit 2 - Writing a 1 to this bit clears the event enable bit 2 in the ENABLE register."]
    #[inline]
    pub fn wakeup2_clren(&mut self) -> _WAKEUP2_CLRENW {
        _WAKEUP2_CLRENW { w: self }
    }
    #[doc = "Bit 3 - Writing a 1 to this bit clears the event enable bit 3 in the ENABLE register."]
    #[inline]
    pub fn wakeup3_clren(&mut self) -> _WAKEUP3_CLRENW {
        _WAKEUP3_CLRENW { w: self }
    }
    #[doc = "Bit 4 - Writing a 1 to this bit clears the event enable bit 4 in the ENABLE register."]
    #[inline]
    pub fn atimer_clren(&mut self) -> _ATIMER_CLRENW {
        _ATIMER_CLRENW { w: self }
    }
    #[doc = "Bit 5 - Writing a 1 to this bit clears the event enable bit 5 in the ENABLE register."]
    #[inline]
    pub fn rtc_clren(&mut self) -> _RTC_CLRENW {
        _RTC_CLRENW { w: self }
    }
    #[doc = "Bit 6 - Writing a 1 to this bit clears the event enable bit 6 in the ENABLE register."]
    #[inline]
    pub fn bod_clren(&mut self) -> _BOD_CLRENW {
        _BOD_CLRENW { w: self }
    }
    #[doc = "Bit 7 - Writing a 1 to this bit clears the event enable bit 7 in the ENABLE register."]
    #[inline]
    pub fn wwdt_clren(&mut self) -> _WWDT_CLRENW {
        _WWDT_CLRENW { w: self }
    }
    #[doc = "Bit 8 - Writing a 1 to this bit clears the event enable bit 8 in the ENABLE register."]
    #[inline]
    pub fn eth_clren(&mut self) -> _ETH_CLRENW {
        _ETH_CLRENW { w: self }
    }
    #[doc = "Bit 9 - Writing a 1 to this bit clears the event enable bit 9 in the ENABLE register."]
    #[inline]
    pub fn usb0_clren(&mut self) -> _USB0_CLRENW {
        _USB0_CLRENW { w: self }
    }
    #[doc = "Bit 10 - Writing a 1 to this bit clears the event enable bit 10 in the ENABLE register."]
    #[inline]
    pub fn usb1_clren(&mut self) -> _USB1_CLRENW {
        _USB1_CLRENW { w: self }
    }
    #[doc = "Bit 11 - Writing a 1 to this bit clears the event enable bit 11 in the ENABLE register."]
    #[inline]
    pub fn sdmmc_clren(&mut self) -> _SDMMC_CLRENW {
        _SDMMC_CLRENW { w: self }
    }
    #[doc = "Bit 12 - Writing a 1 to this bit clears the event enable bit 12 in the ENABLE register."]
    #[inline]
    pub fn can_clren(&mut self) -> _CAN_CLRENW {
        _CAN_CLRENW { w: self }
    }
    #[doc = "Bit 13 - Writing a 1 to this bit clears the event enable bit 13 in the ENABLE register."]
    #[inline]
    pub fn tim2_clren(&mut self) -> _TIM2_CLRENW {
        _TIM2_CLRENW { w: self }
    }
    #[doc = "Bit 14 - Writing a 1 to this bit clears the event enable bit 14 in the ENABLE register."]
    #[inline]
    pub fn tim6_clren(&mut self) -> _TIM6_CLRENW {
        _TIM6_CLRENW { w: self }
    }
    #[doc = "Bit 15 - Writing a 1 to this bit clears the event enable bit 15 in the ENABLE register."]
    #[inline]
    pub fn qei_clren(&mut self) -> _QEI_CLRENW {
        _QEI_CLRENW { w: self }
    }
    #[doc = "Bit 16 - Writing a 1 to this bit clears the event enable bit 16 in the ENABLE register."]
    #[inline]
    pub fn tim14_clren(&mut self) -> _TIM14_CLRENW {
        _TIM14_CLRENW { w: self }
    }
    #[doc = "Bit 19 - Writing a 1 to this bit clears the event enable bit 19 in the ENABLE register."]
    #[inline]
    pub fn reset_clren(&mut self) -> _RESET_CLRENW {
        _RESET_CLRENW { w: self }
    }
    #[doc = "Bit 20 - Writing a 1 to this bit clears the event enable bit 20 in the ENABLE register."]
    #[inline]
    pub fn bodreset_clren(&mut self) -> _BODRESET_CLRENW {
        _BODRESET_CLRENW { w: self }
    }
    #[doc = "Bit 21 - Writing a 1 to this bit clears the event enable bit 21 in the ENABLE register."]
    #[inline]
    pub fn dpdreset_clren(&mut self) -> _DPDRESET_CLRENW {
        _DPDRESET_CLRENW { w: self }
    }
}
