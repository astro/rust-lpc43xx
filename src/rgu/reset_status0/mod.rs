#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RESET_STATUS0 {
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
pub struct PERIPH_RSTR {
    bits: u8,
}
impl PERIPH_RSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MASTER_RSTR {
    bits: u8,
}
impl MASTER_RSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WWDT_RSTR {
    bits: u8,
}
impl WWDT_RSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CREG_RSTR {
    bits: u8,
}
impl CREG_RSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BUS_RSTR {
    bits: u8,
}
impl BUS_RSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SCU_RSTR {
    bits: u8,
}
impl SCU_RSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct M0SUB_RSTR {
    bits: u8,
}
impl M0SUB_RSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct M4_RSTR {
    bits: u8,
}
impl M4_RSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PERIPH_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _PERIPH_RSTW<'a> {
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
pub struct _MASTER_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _MASTER_RSTW<'a> {
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
pub struct _WWDT_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _WWDT_RSTW<'a> {
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
pub struct _CREG_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CREG_RSTW<'a> {
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
pub struct _BUS_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _BUS_RSTW<'a> {
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
pub struct _SCU_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SCU_RSTW<'a> {
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
pub struct _M0SUB_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _M0SUB_RSTW<'a> {
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
pub struct _M4_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _M4_RSTW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 2:3 - Status of the PERIPH_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator - this reset is self-clearing 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn periph_rst(&self) -> PERIPH_RSTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PERIPH_RSTR { bits }
    }
    #[doc = "Bits 4:5 - Status of the MASTER_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator - this reset is self-clearing 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn master_rst(&self) -> MASTER_RSTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MASTER_RSTR { bits }
    }
    #[doc = "Bits 8:9 - Status of the WWDT_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reserved"]
    #[inline]
    pub fn wwdt_rst(&self) -> WWDT_RSTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WWDT_RSTR { bits }
    }
    #[doc = "Bits 10:11 - Status of the CREG_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reserved"]
    #[inline]
    pub fn creg_rst(&self) -> CREG_RSTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CREG_RSTR { bits }
    }
    #[doc = "Bits 16:17 - Status of the BUS_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn bus_rst(&self) -> BUS_RSTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BUS_RSTR { bits }
    }
    #[doc = "Bits 18:19 - Status of the SCU_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn scu_rst(&self) -> SCU_RSTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SCU_RSTR { bits }
    }
    #[doc = "Bits 24:25 - Status of the M0SUB_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn m0sub_rst(&self) -> M0SUB_RSTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        M0SUB_RSTR { bits }
    }
    #[doc = "Bits 26:27 - Status of the M4_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn m4_rst(&self) -> M4_RSTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        M4_RSTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1431634000 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 2:3 - Status of the PERIPH_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator - this reset is self-clearing 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn periph_rst(&mut self) -> _PERIPH_RSTW {
        _PERIPH_RSTW { w: self }
    }
    #[doc = "Bits 4:5 - Status of the MASTER_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator - this reset is self-clearing 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn master_rst(&mut self) -> _MASTER_RSTW {
        _MASTER_RSTW { w: self }
    }
    #[doc = "Bits 8:9 - Status of the WWDT_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reserved"]
    #[inline]
    pub fn wwdt_rst(&mut self) -> _WWDT_RSTW {
        _WWDT_RSTW { w: self }
    }
    #[doc = "Bits 10:11 - Status of the CREG_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reserved"]
    #[inline]
    pub fn creg_rst(&mut self) -> _CREG_RSTW {
        _CREG_RSTW { w: self }
    }
    #[doc = "Bits 16:17 - Status of the BUS_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn bus_rst(&mut self) -> _BUS_RSTW {
        _BUS_RSTW { w: self }
    }
    #[doc = "Bits 18:19 - Status of the SCU_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn scu_rst(&mut self) -> _SCU_RSTW {
        _SCU_RSTW { w: self }
    }
    #[doc = "Bits 24:25 - Status of the M0SUB_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn m0sub_rst(&mut self) -> _M0SUB_RSTW {
        _M0SUB_RSTW { w: self }
    }
    #[doc = "Bits 26:27 - Status of the M4_RST reset generator output 00 = No reset activated 01 = Reset output activated by input to the reset generator 10 = Reserved 11 = Reset output activated by software write to RESET_CTRL register"]
    #[inline]
    pub fn m4_rst(&mut self) -> _M4_RSTW {
        _M4_RSTW { w: self }
    }
}
