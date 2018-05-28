#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RESET_CTRL1 {
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
pub struct _TIMER0_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER0_RSTW<'a> {
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
pub struct _TIMER1_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER1_RSTW<'a> {
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
pub struct _TIMER2_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER2_RSTW<'a> {
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
pub struct _TIMER3_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER3_RSTW<'a> {
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
pub struct _RITIMER_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _RITIMER_RSTW<'a> {
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
pub struct _SCT_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SCT_RSTW<'a> {
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
pub struct _MOTOCONPWM_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _MOTOCONPWM_RSTW<'a> {
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
pub struct _QEI_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _QEI_RSTW<'a> {
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
pub struct _ADC0_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC0_RSTW<'a> {
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
pub struct _ADC1_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC1_RSTW<'a> {
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
pub struct _DAC_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _DAC_RSTW<'a> {
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
pub struct _UART0_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _UART0_RSTW<'a> {
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
pub struct _UART1_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _UART1_RSTW<'a> {
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
pub struct _UART2_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _UART2_RSTW<'a> {
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
pub struct _UART3_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _UART3_RSTW<'a> {
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
pub struct _I2C0_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C0_RSTW<'a> {
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
pub struct _I2C1_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C1_RSTW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SSP0_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SSP0_RSTW<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SSP1_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SSP1_RSTW<'a> {
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
pub struct _I2S_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _I2S_RSTW<'a> {
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
pub struct _SPIFI_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SPIFI_RSTW<'a> {
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
#[doc = r" Proxy"]
pub struct _CAN1_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN1_RSTW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CAN0_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN0_RSTW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _M0APP_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _M0APP_RSTW<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SGPIO_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SGPIO_RSTW<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SPI_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI_RSTW<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADCHS_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCHS_RSTW<'a> {
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
        const OFFSET: u8 = 28;
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
    #[doc = "Bit 0 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn timer0_rst(&mut self) -> _TIMER0_RSTW {
        _TIMER0_RSTW { w: self }
    }
    #[doc = "Bit 1 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn timer1_rst(&mut self) -> _TIMER1_RSTW {
        _TIMER1_RSTW { w: self }
    }
    #[doc = "Bit 2 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn timer2_rst(&mut self) -> _TIMER2_RSTW {
        _TIMER2_RSTW { w: self }
    }
    #[doc = "Bit 3 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn timer3_rst(&mut self) -> _TIMER3_RSTW {
        _TIMER3_RSTW { w: self }
    }
    #[doc = "Bit 4 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn ritimer_rst(&mut self) -> _RITIMER_RSTW {
        _RITIMER_RSTW { w: self }
    }
    #[doc = "Bit 5 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn sct_rst(&mut self) -> _SCT_RSTW {
        _SCT_RSTW { w: self }
    }
    #[doc = "Bit 6 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn motoconpwm_rst(&mut self) -> _MOTOCONPWM_RSTW {
        _MOTOCONPWM_RSTW { w: self }
    }
    #[doc = "Bit 7 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn qei_rst(&mut self) -> _QEI_RSTW {
        _QEI_RSTW { w: self }
    }
    #[doc = "Bit 8 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn adc0_rst(&mut self) -> _ADC0_RSTW {
        _ADC0_RSTW { w: self }
    }
    #[doc = "Bit 9 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn adc1_rst(&mut self) -> _ADC1_RSTW {
        _ADC1_RSTW { w: self }
    }
    #[doc = "Bit 10 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn dac_rst(&mut self) -> _DAC_RSTW {
        _DAC_RSTW { w: self }
    }
    #[doc = "Bit 12 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn uart0_rst(&mut self) -> _UART0_RSTW {
        _UART0_RSTW { w: self }
    }
    #[doc = "Bit 13 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn uart1_rst(&mut self) -> _UART1_RSTW {
        _UART1_RSTW { w: self }
    }
    #[doc = "Bit 14 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn uart2_rst(&mut self) -> _UART2_RSTW {
        _UART2_RSTW { w: self }
    }
    #[doc = "Bit 15 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn uart3_rst(&mut self) -> _UART3_RSTW {
        _UART3_RSTW { w: self }
    }
    #[doc = "Bit 16 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn i2c0_rst(&mut self) -> _I2C0_RSTW {
        _I2C0_RSTW { w: self }
    }
    #[doc = "Bit 17 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn i2c1_rst(&mut self) -> _I2C1_RSTW {
        _I2C1_RSTW { w: self }
    }
    #[doc = "Bit 18 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn ssp0_rst(&mut self) -> _SSP0_RSTW {
        _SSP0_RSTW { w: self }
    }
    #[doc = "Bit 19 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn ssp1_rst(&mut self) -> _SSP1_RSTW {
        _SSP1_RSTW { w: self }
    }
    #[doc = "Bit 20 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn i2s_rst(&mut self) -> _I2S_RSTW {
        _I2S_RSTW { w: self }
    }
    #[doc = "Bit 21 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn spifi_rst(&mut self) -> _SPIFI_RSTW {
        _SPIFI_RSTW { w: self }
    }
    #[doc = "Bit 22 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn can1_rst(&mut self) -> _CAN1_RSTW {
        _CAN1_RSTW { w: self }
    }
    #[doc = "Bit 23 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn can0_rst(&mut self) -> _CAN0_RSTW {
        _CAN0_RSTW { w: self }
    }
    #[doc = "Bit 24 - Writing a one activates the reset. Writing a 0 clears the reset. This bit must be cleared by software."]
    #[inline]
    pub fn m0app_rst(&mut self) -> _M0APP_RSTW {
        _M0APP_RSTW { w: self }
    }
    #[doc = "Bit 25 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn sgpio_rst(&mut self) -> _SGPIO_RSTW {
        _SGPIO_RSTW { w: self }
    }
    #[doc = "Bit 26 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn spi_rst(&mut self) -> _SPI_RSTW {
        _SPI_RSTW { w: self }
    }
    #[doc = "Bit 28 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn adchs_rst(&mut self) -> _ADCHS_RSTW {
        _ADCHS_RSTW { w: self }
    }
}
