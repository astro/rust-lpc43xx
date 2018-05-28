#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SET_EN {
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
pub struct _WAKEUP0_SETENW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP0_SETENW<'a> {
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
pub struct _WAKEUP1_SETENW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP1_SETENW<'a> {
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
pub struct _WAKEUP2_SETENW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP2_SETENW<'a> {
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
pub struct _WAKEUP3_SETENW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP3_SETENW<'a> {
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
pub struct _ATIMER_SETENW<'a> {
    w: &'a mut W,
}
impl<'a> _ATIMER_SETENW<'a> {
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
pub struct _RTC_SETENW<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_SETENW<'a> {
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
pub struct _BOD_SETENW<'a> {
    w: &'a mut W,
}
impl<'a> _BOD_SETENW<'a> {
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
pub struct _WWDT_SETENW<'a> {
    w: &'a mut W,
}
impl<'a> _WWDT_SETENW<'a> {
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
pub struct _ETH_SETENW<'a> {
    w: &'a mut W,
}
impl<'a> _ETH_SETENW<'a> {
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
pub struct _USB0_SETENW<'a> {
    w: &'a mut W,
}
impl<'a> _USB0_SETENW<'a> {
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
pub struct _USB1_SETENW<'a> {
    w: &'a mut W,
}
impl<'a> _USB1_SETENW<'a> {
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
pub struct _SDMMC_SETENW<'a> {
    w: &'a mut W,
}
impl<'a> _SDMMC_SETENW<'a> {
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
pub struct _CAN_SETENW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_SETENW<'a> {
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
pub struct _TIM2_SETENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM2_SETENW<'a> {
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
pub struct _TIM6_SETENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM6_SETENW<'a> {
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
pub struct _QEI_SETENW<'a> {
    w: &'a mut W,
}
impl<'a> _QEI_SETENW<'a> {
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
pub struct _TIM14_SETENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM14_SETENW<'a> {
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
pub struct _RESET_SETENW<'a> {
    w: &'a mut W,
}
impl<'a> _RESET_SETENW<'a> {
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
pub struct _BODRESET_SETENW<'a> {
    w: &'a mut W,
}
impl<'a> _BODRESET_SETENW<'a> {
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
pub struct _DPDRESET_SETENW<'a> {
    w: &'a mut W,
}
impl<'a> _DPDRESET_SETENW<'a> {
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
    #[doc = "Bit 0 - Writing a 1 to this bit sets the event enable bit 0 in the ENABLE register."]
    #[inline]
    pub fn wakeup0_seten(&mut self) -> _WAKEUP0_SETENW {
        _WAKEUP0_SETENW { w: self }
    }
    #[doc = "Bit 1 - Writing a 1 to this bit sets the event enable bit 1 in the ENABLE register."]
    #[inline]
    pub fn wakeup1_seten(&mut self) -> _WAKEUP1_SETENW {
        _WAKEUP1_SETENW { w: self }
    }
    #[doc = "Bit 2 - Writing a 1 to this bit sets the event enable bit 2 in the ENABLE register."]
    #[inline]
    pub fn wakeup2_seten(&mut self) -> _WAKEUP2_SETENW {
        _WAKEUP2_SETENW { w: self }
    }
    #[doc = "Bit 3 - Writing a 1 to this bit sets the event enable bit 3 in the ENABLE register."]
    #[inline]
    pub fn wakeup3_seten(&mut self) -> _WAKEUP3_SETENW {
        _WAKEUP3_SETENW { w: self }
    }
    #[doc = "Bit 4 - Writing a 1 to this bit sets the event enable bit 4 in the ENABLE register."]
    #[inline]
    pub fn atimer_seten(&mut self) -> _ATIMER_SETENW {
        _ATIMER_SETENW { w: self }
    }
    #[doc = "Bit 5 - Writing a 1 to this bit sets the event enable bit 5 in the ENABLE register."]
    #[inline]
    pub fn rtc_seten(&mut self) -> _RTC_SETENW {
        _RTC_SETENW { w: self }
    }
    #[doc = "Bit 6 - Writing a 1 to this bit sets the event enable bit 6 in the ENABLE register."]
    #[inline]
    pub fn bod_seten(&mut self) -> _BOD_SETENW {
        _BOD_SETENW { w: self }
    }
    #[doc = "Bit 7 - Writing a 1 to this bit sets the event enable bit 7 in the ENABLE register."]
    #[inline]
    pub fn wwdt_seten(&mut self) -> _WWDT_SETENW {
        _WWDT_SETENW { w: self }
    }
    #[doc = "Bit 8 - Writing a 1 to this bit sets the event enable bit 8 in the ENABLE register."]
    #[inline]
    pub fn eth_seten(&mut self) -> _ETH_SETENW {
        _ETH_SETENW { w: self }
    }
    #[doc = "Bit 9 - Writing a 1 to this bit sets the event enable bit 9 in the ENABLE register."]
    #[inline]
    pub fn usb0_seten(&mut self) -> _USB0_SETENW {
        _USB0_SETENW { w: self }
    }
    #[doc = "Bit 10 - Writing a 1 to this bit sets the event enable bit 10 in the ENABLE register."]
    #[inline]
    pub fn usb1_seten(&mut self) -> _USB1_SETENW {
        _USB1_SETENW { w: self }
    }
    #[doc = "Bit 11 - Writing a 1 to this bit sets the event enable bit 11 in the ENABLE register."]
    #[inline]
    pub fn sdmmc_seten(&mut self) -> _SDMMC_SETENW {
        _SDMMC_SETENW { w: self }
    }
    #[doc = "Bit 12 - Writing a 1 to this bit sets the event enable bit 12 in the ENABLE register."]
    #[inline]
    pub fn can_seten(&mut self) -> _CAN_SETENW {
        _CAN_SETENW { w: self }
    }
    #[doc = "Bit 13 - Writing a 1 to this bit sets the event enable bit 13 in the ENABLE register."]
    #[inline]
    pub fn tim2_seten(&mut self) -> _TIM2_SETENW {
        _TIM2_SETENW { w: self }
    }
    #[doc = "Bit 14 - Writing a 1 to this bit sets the event enable bit 14 in the ENABLE register."]
    #[inline]
    pub fn tim6_seten(&mut self) -> _TIM6_SETENW {
        _TIM6_SETENW { w: self }
    }
    #[doc = "Bit 15 - Writing a 1 to this bit sets the event enable bit 15 in the ENABLE register."]
    #[inline]
    pub fn qei_seten(&mut self) -> _QEI_SETENW {
        _QEI_SETENW { w: self }
    }
    #[doc = "Bit 16 - Writing a 1 to this bit sets the event enable bit 16 in the ENABLE register."]
    #[inline]
    pub fn tim14_seten(&mut self) -> _TIM14_SETENW {
        _TIM14_SETENW { w: self }
    }
    #[doc = "Bit 19 - Writing a 1 to this bit sets the event enable bit 19 in the ENABLE register."]
    #[inline]
    pub fn reset_seten(&mut self) -> _RESET_SETENW {
        _RESET_SETENW { w: self }
    }
    #[doc = "Bit 20 - Writing a 1 to this bit sets the event enable bit 20 in the ENABLE register."]
    #[inline]
    pub fn bodreset_seten(&mut self) -> _BODRESET_SETENW {
        _BODRESET_SETENW { w: self }
    }
    #[doc = "Bit 21 - Writing a 1 to this bit sets the event enable bit 21 in the ENABLE register."]
    #[inline]
    pub fn dpdreset_seten(&mut self) -> _DPDRESET_SETENW {
        _DPDRESET_SETENW { w: self }
    }
}
