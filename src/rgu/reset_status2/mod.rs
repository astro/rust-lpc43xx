#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RESET_STATUS2 {
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
#[doc = r" Value of the field"]
pub struct TIMER0_RSTR {
    bits: u8,
}
impl TIMER0_RSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TIMER1_RSTR {
    bits: u8,
}
impl TIMER1_RSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TIMER2_RSTR {
    bits: u8,
}
impl TIMER2_RSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TIMER3_RSTR {
    bits: u8,
}
impl TIMER3_RSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RITIMER_RSTR {
    bits: u8,
}
impl RITIMER_RSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SCT_RSTR {
    bits: u8,
}
impl SCT_RSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MOTOCONPWM_RSTR {
    bits: u8,
}
impl MOTOCONPWM_RSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct QEI_RSTR {
    bits: u8,
}
impl QEI_RSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ADC0_RSTR {
    bits: u8,
}
impl ADC0_RSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ADC1_RSTR {
    bits: u8,
}
impl ADC1_RSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DAC_RSTR {
    bits: u8,
}
impl DAC_RSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct UART0_RSTR {
    bits: u8,
}
impl UART0_RSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct UART1_RSTR {
    bits: u8,
}
impl UART1_RSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct UART2_RSTR {
    bits: u8,
}
impl UART2_RSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct UART3_RSTR {
    bits: u8,
}
impl UART3_RSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _TIMER0_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER0_RSTW<'a> {
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
#[doc = r" Proxy"]
pub struct _TIMER1_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER1_RSTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
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
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
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
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
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
#[doc = r" Proxy"]
pub struct _SCT_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SCT_RSTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
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
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
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
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
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
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
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
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
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
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
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
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
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
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
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
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
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
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:1 - Status of the TIMER0_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn timer0_rst(&self) -> TIMER0_RSTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TIMER0_RSTR { bits }
    }
    #[doc = "Bits 2:3 - Status of the TIMER1_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn timer1_rst(&self) -> TIMER1_RSTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TIMER1_RSTR { bits }
    }
    #[doc = "Bits 4:5 - Status of the TIMER2_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn timer2_rst(&self) -> TIMER2_RSTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TIMER2_RSTR { bits }
    }
    #[doc = "Bits 6:7 - Status of the TIMER3_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn timer3_rst(&self) -> TIMER3_RSTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TIMER3_RSTR { bits }
    }
    #[doc = "Bits 8:9 - Status of the RITIMER_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn ritimer_rst(&self) -> RITIMER_RSTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RITIMER_RSTR { bits }
    }
    #[doc = "Bits 10:11 - Status of the SCT_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn sct_rst(&self) -> SCT_RSTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SCT_RSTR { bits }
    }
    #[doc = "Bits 12:13 - Status of the MOTOCONPWM_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn motoconpwm_rst(&self) -> MOTOCONPWM_RSTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MOTOCONPWM_RSTR { bits }
    }
    #[doc = "Bits 14:15 - Status of the QEI_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn qei_rst(&self) -> QEI_RSTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        QEI_RSTR { bits }
    }
    #[doc = "Bits 16:17 - Status of the ADC0_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn adc0_rst(&self) -> ADC0_RSTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADC0_RSTR { bits }
    }
    #[doc = "Bits 18:19 - Status of the ADC1_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn adc1_rst(&self) -> ADC1_RSTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADC1_RSTR { bits }
    }
    #[doc = "Bits 20:21 - Status of the DAC_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn dac_rst(&self) -> DAC_RSTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DAC_RSTR { bits }
    }
    #[doc = "Bits 24:25 - Status of the UART0_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn uart0_rst(&self) -> UART0_RSTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        UART0_RSTR { bits }
    }
    #[doc = "Bits 26:27 - Status of the UART1_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn uart1_rst(&self) -> UART1_RSTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        UART1_RSTR { bits }
    }
    #[doc = "Bits 28:29 - Status of the UART2_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn uart2_rst(&self) -> UART2_RSTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        UART2_RSTR { bits }
    }
    #[doc = "Bits 30:31 - Status of the UART3_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn uart3_rst(&self) -> UART3_RSTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        UART3_RSTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1431655765 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Status of the TIMER0_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn timer0_rst(&mut self) -> _TIMER0_RSTW {
        _TIMER0_RSTW { w: self }
    }
    #[doc = "Bits 2:3 - Status of the TIMER1_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn timer1_rst(&mut self) -> _TIMER1_RSTW {
        _TIMER1_RSTW { w: self }
    }
    #[doc = "Bits 4:5 - Status of the TIMER2_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn timer2_rst(&mut self) -> _TIMER2_RSTW {
        _TIMER2_RSTW { w: self }
    }
    #[doc = "Bits 6:7 - Status of the TIMER3_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn timer3_rst(&mut self) -> _TIMER3_RSTW {
        _TIMER3_RSTW { w: self }
    }
    #[doc = "Bits 8:9 - Status of the RITIMER_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn ritimer_rst(&mut self) -> _RITIMER_RSTW {
        _RITIMER_RSTW { w: self }
    }
    #[doc = "Bits 10:11 - Status of the SCT_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn sct_rst(&mut self) -> _SCT_RSTW {
        _SCT_RSTW { w: self }
    }
    #[doc = "Bits 12:13 - Status of the MOTOCONPWM_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn motoconpwm_rst(&mut self) -> _MOTOCONPWM_RSTW {
        _MOTOCONPWM_RSTW { w: self }
    }
    #[doc = "Bits 14:15 - Status of the QEI_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn qei_rst(&mut self) -> _QEI_RSTW {
        _QEI_RSTW { w: self }
    }
    #[doc = "Bits 16:17 - Status of the ADC0_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn adc0_rst(&mut self) -> _ADC0_RSTW {
        _ADC0_RSTW { w: self }
    }
    #[doc = "Bits 18:19 - Status of the ADC1_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn adc1_rst(&mut self) -> _ADC1_RSTW {
        _ADC1_RSTW { w: self }
    }
    #[doc = "Bits 20:21 - Status of the DAC_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn dac_rst(&mut self) -> _DAC_RSTW {
        _DAC_RSTW { w: self }
    }
    #[doc = "Bits 24:25 - Status of the UART0_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn uart0_rst(&mut self) -> _UART0_RSTW {
        _UART0_RSTW { w: self }
    }
    #[doc = "Bits 26:27 - Status of the UART1_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn uart1_rst(&mut self) -> _UART1_RSTW {
        _UART1_RSTW { w: self }
    }
    #[doc = "Bits 28:29 - Status of the UART2_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn uart2_rst(&mut self) -> _UART2_RSTW {
        _UART2_RSTW { w: self }
    }
    #[doc = "Bits 30:31 - Status of the UART3_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn uart3_rst(&mut self) -> _UART3_RSTW {
        _UART3_RSTW { w: self }
    }
}
