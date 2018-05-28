#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RESET_CTRL0 {
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
pub struct _CORE_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CORE_RSTW<'a> {
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
pub struct _PERIPH_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _PERIPH_RSTW<'a> {
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
pub struct _MASTER_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _MASTER_RSTW<'a> {
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
pub struct _WWDT_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _WWDT_RSTW<'a> {
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
pub struct _CREG_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CREG_RSTW<'a> {
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
pub struct _BUS_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _BUS_RSTW<'a> {
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
pub struct _SCU_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SCU_RSTW<'a> {
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
pub struct _M0_SUB_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _M0_SUB_RSTW<'a> {
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
pub struct _M4_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _M4_RSTW<'a> {
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
pub struct _LCD_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _LCD_RSTW<'a> {
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
pub struct _USB0_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB0_RSTW<'a> {
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
pub struct _USB1_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB1_RSTW<'a> {
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
pub struct _DMA_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_RSTW<'a> {
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
pub struct _SDIO_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SDIO_RSTW<'a> {
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
pub struct _EMC_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _EMC_RSTW<'a> {
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
pub struct _ETHERNET_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _ETHERNET_RSTW<'a> {
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
pub struct _FLASHA_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASHA_RSTW<'a> {
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
pub struct _EEPROM_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _EEPROM_RSTW<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_RSTW<'a> {
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
#[doc = r" Proxy"]
pub struct _FLASHB_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASHB_RSTW<'a> {
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
        const OFFSET: u8 = 29;
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
    pub fn core_rst(&mut self) -> _CORE_RSTW {
        _CORE_RSTW { w: self }
    }
    #[doc = "Bit 1 - Writing a one activates the reset. This bit is automatically cleared to 0 after three clock cycles."]
    #[inline]
    pub fn periph_rst(&mut self) -> _PERIPH_RSTW {
        _PERIPH_RSTW { w: self }
    }
    #[doc = "Bit 2 - Writing a one activates the reset. This bit is automatically cleared to 0 after three clock cycles."]
    #[inline]
    pub fn master_rst(&mut self) -> _MASTER_RSTW {
        _MASTER_RSTW { w: self }
    }
    #[doc = "Bit 4 - Writing a one to this bit has no effect."]
    #[inline]
    pub fn wwdt_rst(&mut self) -> _WWDT_RSTW {
        _WWDT_RSTW { w: self }
    }
    #[doc = "Bit 5 - Writing a one to this bit has no effect."]
    #[inline]
    pub fn creg_rst(&mut self) -> _CREG_RSTW {
        _CREG_RSTW { w: self }
    }
    #[doc = "Bit 8 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle. Do not use during normal operation"]
    #[inline]
    pub fn bus_rst(&mut self) -> _BUS_RSTW {
        _BUS_RSTW { w: self }
    }
    #[doc = "Bit 9 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn scu_rst(&mut self) -> _SCU_RSTW {
        _SCU_RSTW { w: self }
    }
    #[doc = "Bit 12 - Writing a one activates the reset. Writing a 0 clears the reset. This bit must be cleared by software."]
    #[inline]
    pub fn m0_sub_rst(&mut self) -> _M0_SUB_RSTW {
        _M0_SUB_RSTW { w: self }
    }
    #[doc = "Bit 13 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn m4_rst(&mut self) -> _M4_RSTW {
        _M4_RSTW { w: self }
    }
    #[doc = "Bit 16 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn lcd_rst(&mut self) -> _LCD_RSTW {
        _LCD_RSTW { w: self }
    }
    #[doc = "Bit 17 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn usb0_rst(&mut self) -> _USB0_RSTW {
        _USB0_RSTW { w: self }
    }
    #[doc = "Bit 18 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn usb1_rst(&mut self) -> _USB1_RSTW {
        _USB1_RSTW { w: self }
    }
    #[doc = "Bit 19 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn dma_rst(&mut self) -> _DMA_RSTW {
        _DMA_RSTW { w: self }
    }
    #[doc = "Bit 20 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn sdio_rst(&mut self) -> _SDIO_RSTW {
        _SDIO_RSTW { w: self }
    }
    #[doc = "Bit 21 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn emc_rst(&mut self) -> _EMC_RSTW {
        _EMC_RSTW { w: self }
    }
    #[doc = "Bit 22 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn ethernet_rst(&mut self) -> _ETHERNET_RSTW {
        _ETHERNET_RSTW { w: self }
    }
    #[doc = "Bit 25 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn flasha_rst(&mut self) -> _FLASHA_RSTW {
        _FLASHA_RSTW { w: self }
    }
    #[doc = "Bit 27 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn eeprom_rst(&mut self) -> _EEPROM_RSTW {
        _EEPROM_RSTW { w: self }
    }
    #[doc = "Bit 28 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn gpio_rst(&mut self) -> _GPIO_RSTW {
        _GPIO_RSTW { w: self }
    }
    #[doc = "Bit 29 - Writing a one activates the reset. This bit is automatically cleared to 0 after one clock cycle."]
    #[inline]
    pub fn flashb_rst(&mut self) -> _FLASHB_RSTW {
        _FLASHB_RSTW { w: self }
    }
}
