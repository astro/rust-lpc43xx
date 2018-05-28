#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONFIG {
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
pub struct TRIGGER__MASKR {
    bits: u8,
}
impl TRIGGER__MASKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRIGGER_MODER {
    bits: u8,
}
impl TRIGGER_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRIGGER_SYNCR {
    bits: bool,
}
impl TRIGGER_SYNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct CHANNEL_ID_ENR {
    bits: bool,
}
impl CHANNEL_ID_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct RECOVERY_TIMER {
    bits: u8,
}
impl RECOVERY_TIMER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _TRIGGER__MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGGER__MASKW<'a> {
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
pub struct _TRIGGER_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGGER_MODEW<'a> {
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
pub struct _TRIGGER_SYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGGER_SYNCW<'a> {
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
pub struct _CHANNEL_ID_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CHANNEL_ID_ENW<'a> {
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
pub struct _RECOVERY_TIMEW<'a> {
    w: &'a mut W,
}
impl<'a> _RECOVERY_TIMEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 6;
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
    #[doc = "Bits 0:1 - 00 = triggers off 01 = software trigger only 10 = external trigger only 11 = both triggers allowed"]
    #[inline]
    pub fn trigger__mask(&self) -> TRIGGER__MASKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIGGER__MASKR { bits }
    }
    #[doc = "Bits 2:3 - 00 = rising external trigger 01 = falling external trigger 10 = low external trigger 11 = high external trigger"]
    #[inline]
    pub fn trigger_mode(&self) -> TRIGGER_MODER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIGGER_MODER { bits }
    }
    #[doc = "Bit 4 - 0 = do not synchronize external trigger input 1 = synchronize external trigger input"]
    #[inline]
    pub fn trigger_sync(&self) -> TRIGGER_SYNCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TRIGGER_SYNCR { bits }
    }
    #[doc = "Bit 5 - 0 = do not add channel ID to FIFO output data 1 = add channel ID to FIFO output data"]
    #[inline]
    pub fn channel_id_en(&self) -> CHANNEL_ID_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CHANNEL_ID_ENR { bits }
    }
    #[doc = "Bits 6:13 - ADC recovery time from power down"]
    #[inline]
    pub fn recovery_time(&self) -> RECOVERY_TIMER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RECOVERY_TIMER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 9216 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - 00 = triggers off 01 = software trigger only 10 = external trigger only 11 = both triggers allowed"]
    #[inline]
    pub fn trigger__mask(&mut self) -> _TRIGGER__MASKW {
        _TRIGGER__MASKW { w: self }
    }
    #[doc = "Bits 2:3 - 00 = rising external trigger 01 = falling external trigger 10 = low external trigger 11 = high external trigger"]
    #[inline]
    pub fn trigger_mode(&mut self) -> _TRIGGER_MODEW {
        _TRIGGER_MODEW { w: self }
    }
    #[doc = "Bit 4 - 0 = do not synchronize external trigger input 1 = synchronize external trigger input"]
    #[inline]
    pub fn trigger_sync(&mut self) -> _TRIGGER_SYNCW {
        _TRIGGER_SYNCW { w: self }
    }
    #[doc = "Bit 5 - 0 = do not add channel ID to FIFO output data 1 = add channel ID to FIFO output data"]
    #[inline]
    pub fn channel_id_en(&mut self) -> _CHANNEL_ID_ENW {
        _CHANNEL_ID_ENW { w: self }
    }
    #[doc = "Bits 6:13 - ADC recovery time from power down"]
    #[inline]
    pub fn recovery_time(&mut self) -> _RECOVERY_TIMEW {
        _RECOVERY_TIMEW { w: self }
    }
}
