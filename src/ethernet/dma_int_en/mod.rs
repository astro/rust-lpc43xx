#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMA_INT_EN {
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
pub struct TIER {
    bits: bool,
}
impl TIER {
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
pub struct TSER {
    bits: bool,
}
impl TSER {
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
pub struct TUER {
    bits: bool,
}
impl TUER {
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
pub struct TJER {
    bits: bool,
}
impl TJER {
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
pub struct OVER {
    bits: bool,
}
impl OVER {
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
pub struct UNER {
    bits: bool,
}
impl UNER {
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
pub struct RIER {
    bits: bool,
}
impl RIER {
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
pub struct RUER {
    bits: bool,
}
impl RUER {
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
pub struct RSER {
    bits: bool,
}
impl RSER {
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
pub struct RWER {
    bits: bool,
}
impl RWER {
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
pub struct ETER {
    bits: bool,
}
impl ETER {
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
pub struct FBER {
    bits: bool,
}
impl FBER {
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
pub struct ERER {
    bits: bool,
}
impl ERER {
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
pub struct AIER {
    bits: bool,
}
impl AIER {
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
pub struct NIER {
    bits: bool,
}
impl NIER {
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
#[doc = r" Proxy"]
pub struct _TIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TIEW<'a> {
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
pub struct _TSEW<'a> {
    w: &'a mut W,
}
impl<'a> _TSEW<'a> {
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
pub struct _TUEW<'a> {
    w: &'a mut W,
}
impl<'a> _TUEW<'a> {
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
pub struct _TJEW<'a> {
    w: &'a mut W,
}
impl<'a> _TJEW<'a> {
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
pub struct _OVEW<'a> {
    w: &'a mut W,
}
impl<'a> _OVEW<'a> {
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
pub struct _UNEW<'a> {
    w: &'a mut W,
}
impl<'a> _UNEW<'a> {
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
pub struct _RIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RIEW<'a> {
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
pub struct _RUEW<'a> {
    w: &'a mut W,
}
impl<'a> _RUEW<'a> {
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
pub struct _RSEW<'a> {
    w: &'a mut W,
}
impl<'a> _RSEW<'a> {
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
pub struct _RWEW<'a> {
    w: &'a mut W,
}
impl<'a> _RWEW<'a> {
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
pub struct _ETEW<'a> {
    w: &'a mut W,
}
impl<'a> _ETEW<'a> {
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
pub struct _FBEW<'a> {
    w: &'a mut W,
}
impl<'a> _FBEW<'a> {
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
pub struct _EREW<'a> {
    w: &'a mut W,
}
impl<'a> _EREW<'a> {
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
pub struct _AIEW<'a> {
    w: &'a mut W,
}
impl<'a> _AIEW<'a> {
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
pub struct _NIEW<'a> {
    w: &'a mut W,
}
impl<'a> _NIEW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Transmit interrupt enable When this bit is set with Normal Interrupt Summary Enable (bit 16 in this register), Transmit Interrupt is enabled. When this bit is reset, Transmit Interrupt is disabled."]
    #[inline]
    pub fn tie(&self) -> TIER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TIER { bits }
    }
    #[doc = "Bit 1 - Transmit stopped enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), Transmission Stopped Interrupt is enabled. When this bit is reset, Transmission Stopped Interrupt is disabled."]
    #[inline]
    pub fn tse(&self) -> TSER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSER { bits }
    }
    #[doc = "Bit 2 - Transmit buffer unavailable enable When this bit is set with Normal Interrupt Summary Enable (bit 16 in this register), Transmit Buffer Unavailable Interrupt is enabled. When this bit is reset, Transmit Buffer Unavailable Interrupt is disabled."]
    #[inline]
    pub fn tue(&self) -> TUER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TUER { bits }
    }
    #[doc = "Bit 3 - Transmit jabber timeout enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), Transmit Jabber Timeout Interrupt is enabled. When this bit is reset, Transmit Jabber Timeout Interrupt is disabled."]
    #[inline]
    pub fn tje(&self) -> TJER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TJER { bits }
    }
    #[doc = "Bit 4 - Overflow interrupt enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), Receive Overflow Interrupt is enabled. When this bit is reset, Overflow Interrupt is disabled."]
    #[inline]
    pub fn ove(&self) -> OVER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OVER { bits }
    }
    #[doc = "Bit 5 - Underflow interrupt enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), Transmit Underflow Interrupt is enabled. When this bit is reset, Underflow Interrupt is disabled."]
    #[inline]
    pub fn une(&self) -> UNER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UNER { bits }
    }
    #[doc = "Bit 6 - Receive interrupt enable When this bit is set with Normal Interrupt Summary Enable (bit 16 in this register), Receive Interrupt is enabled. When this bit is reset, Receive Interrupt is disabled."]
    #[inline]
    pub fn rie(&self) -> RIER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RIER { bits }
    }
    #[doc = "Bit 7 - Receive buffer unavailable enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), Receive Buffer Unavailable Interrupt is enabled. When this bit is reset, the Receive Buffer Unavailable Interrupt is disabled."]
    #[inline]
    pub fn rue(&self) -> RUER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RUER { bits }
    }
    #[doc = "Bit 8 - Received stopped enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), Receive Stopped Interrupt is enabled. When this bit is reset, Receive Stopped Interrupt is disabled."]
    #[inline]
    pub fn rse(&self) -> RSER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RSER { bits }
    }
    #[doc = "Bit 9 - Receive watchdog timeout enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), the Receive Watchdog Timeout Interrupt is enabled. When this bit is reset, Receive Watchdog Timeout Interrupt is disabled."]
    #[inline]
    pub fn rwe(&self) -> RWER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RWER { bits }
    }
    #[doc = "Bit 10 - Early transmit interrupt enable When this bit is set with an Abnormal Interrupt Summary Enable (bit 15 in this register), Early Transmit Interrupt is enabled. When this bit is reset, Early Transmit Interrupt is disabled."]
    #[inline]
    pub fn ete(&self) -> ETER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ETER { bits }
    }
    #[doc = "Bit 13 - Fatal bus error enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), the Fatal Bus Error Interrupt is enabled. When this bit is reset, Fatal Bus Error Enable Interrupt is disabled."]
    #[inline]
    pub fn fbe(&self) -> FBER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FBER { bits }
    }
    #[doc = "Bit 14 - Early receive interrupt enable When this bit is set with Normal Interrupt Summary Enable (bit 16 in this register), Early Receive Interrupt is enabled. When this bit is reset, Early Receive Interrupt is disabled."]
    #[inline]
    pub fn ere(&self) -> ERER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ERER { bits }
    }
    #[doc = "Bit 15 - Abnormal interrupt summary enable When this bit is set, an Abnormal Interrupt is enabled. When this bit is reset, an Abnormal Interrupt is disabled. This bit enables the following bits DMA_STAT register, bit 1: Transmit process stopped DMA_STAT register, bit 3: Transmit jabber timeout DMA_STAT register, bit 4: Receive overflow DMA_STAT register, bit 5: Transmit underflow DMA_STAT register, bit 7: Receiver buffer unavailable DMA_STAT register, bit 8: Receive process stopped DMA_STAT register, bit 9: Receive watchdog timeout DMA_STAT register, bit 10: Early transmit interrupt DMA_STAT register, bit 13: Fatal bus error"]
    #[inline]
    pub fn aie(&self) -> AIER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AIER { bits }
    }
    #[doc = "Bit 16 - Normal interrupt summary enable When this bit is set, a normal interrupt is enabled. When this bit is reset, a normal interrupt is disabled. This bit enables the following bits: DMA_STAT register, bit 0: Transmit interrupt DMA_STAT register, bit 2: Transmit buffer unavailable DMA_STAT register, bit 6: Receive interrupt DMA_STAT register, bit 14: Early receive interrupt"]
    #[inline]
    pub fn nie(&self) -> NIER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NIER { bits }
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
    #[doc = "Bit 0 - Transmit interrupt enable When this bit is set with Normal Interrupt Summary Enable (bit 16 in this register), Transmit Interrupt is enabled. When this bit is reset, Transmit Interrupt is disabled."]
    #[inline]
    pub fn tie(&mut self) -> _TIEW {
        _TIEW { w: self }
    }
    #[doc = "Bit 1 - Transmit stopped enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), Transmission Stopped Interrupt is enabled. When this bit is reset, Transmission Stopped Interrupt is disabled."]
    #[inline]
    pub fn tse(&mut self) -> _TSEW {
        _TSEW { w: self }
    }
    #[doc = "Bit 2 - Transmit buffer unavailable enable When this bit is set with Normal Interrupt Summary Enable (bit 16 in this register), Transmit Buffer Unavailable Interrupt is enabled. When this bit is reset, Transmit Buffer Unavailable Interrupt is disabled."]
    #[inline]
    pub fn tue(&mut self) -> _TUEW {
        _TUEW { w: self }
    }
    #[doc = "Bit 3 - Transmit jabber timeout enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), Transmit Jabber Timeout Interrupt is enabled. When this bit is reset, Transmit Jabber Timeout Interrupt is disabled."]
    #[inline]
    pub fn tje(&mut self) -> _TJEW {
        _TJEW { w: self }
    }
    #[doc = "Bit 4 - Overflow interrupt enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), Receive Overflow Interrupt is enabled. When this bit is reset, Overflow Interrupt is disabled."]
    #[inline]
    pub fn ove(&mut self) -> _OVEW {
        _OVEW { w: self }
    }
    #[doc = "Bit 5 - Underflow interrupt enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), Transmit Underflow Interrupt is enabled. When this bit is reset, Underflow Interrupt is disabled."]
    #[inline]
    pub fn une(&mut self) -> _UNEW {
        _UNEW { w: self }
    }
    #[doc = "Bit 6 - Receive interrupt enable When this bit is set with Normal Interrupt Summary Enable (bit 16 in this register), Receive Interrupt is enabled. When this bit is reset, Receive Interrupt is disabled."]
    #[inline]
    pub fn rie(&mut self) -> _RIEW {
        _RIEW { w: self }
    }
    #[doc = "Bit 7 - Receive buffer unavailable enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), Receive Buffer Unavailable Interrupt is enabled. When this bit is reset, the Receive Buffer Unavailable Interrupt is disabled."]
    #[inline]
    pub fn rue(&mut self) -> _RUEW {
        _RUEW { w: self }
    }
    #[doc = "Bit 8 - Received stopped enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), Receive Stopped Interrupt is enabled. When this bit is reset, Receive Stopped Interrupt is disabled."]
    #[inline]
    pub fn rse(&mut self) -> _RSEW {
        _RSEW { w: self }
    }
    #[doc = "Bit 9 - Receive watchdog timeout enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), the Receive Watchdog Timeout Interrupt is enabled. When this bit is reset, Receive Watchdog Timeout Interrupt is disabled."]
    #[inline]
    pub fn rwe(&mut self) -> _RWEW {
        _RWEW { w: self }
    }
    #[doc = "Bit 10 - Early transmit interrupt enable When this bit is set with an Abnormal Interrupt Summary Enable (bit 15 in this register), Early Transmit Interrupt is enabled. When this bit is reset, Early Transmit Interrupt is disabled."]
    #[inline]
    pub fn ete(&mut self) -> _ETEW {
        _ETEW { w: self }
    }
    #[doc = "Bit 13 - Fatal bus error enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), the Fatal Bus Error Interrupt is enabled. When this bit is reset, Fatal Bus Error Enable Interrupt is disabled."]
    #[inline]
    pub fn fbe(&mut self) -> _FBEW {
        _FBEW { w: self }
    }
    #[doc = "Bit 14 - Early receive interrupt enable When this bit is set with Normal Interrupt Summary Enable (bit 16 in this register), Early Receive Interrupt is enabled. When this bit is reset, Early Receive Interrupt is disabled."]
    #[inline]
    pub fn ere(&mut self) -> _EREW {
        _EREW { w: self }
    }
    #[doc = "Bit 15 - Abnormal interrupt summary enable When this bit is set, an Abnormal Interrupt is enabled. When this bit is reset, an Abnormal Interrupt is disabled. This bit enables the following bits DMA_STAT register, bit 1: Transmit process stopped DMA_STAT register, bit 3: Transmit jabber timeout DMA_STAT register, bit 4: Receive overflow DMA_STAT register, bit 5: Transmit underflow DMA_STAT register, bit 7: Receiver buffer unavailable DMA_STAT register, bit 8: Receive process stopped DMA_STAT register, bit 9: Receive watchdog timeout DMA_STAT register, bit 10: Early transmit interrupt DMA_STAT register, bit 13: Fatal bus error"]
    #[inline]
    pub fn aie(&mut self) -> _AIEW {
        _AIEW { w: self }
    }
    #[doc = "Bit 16 - Normal interrupt summary enable When this bit is set, a normal interrupt is enabled. When this bit is reset, a normal interrupt is disabled. This bit enables the following bits: DMA_STAT register, bit 0: Transmit interrupt DMA_STAT register, bit 2: Transmit buffer unavailable DMA_STAT register, bit 6: Receive interrupt DMA_STAT register, bit 14: Early receive interrupt"]
    #[inline]
    pub fn nie(&mut self) -> _NIEW {
        _NIEW { w: self }
    }
}
