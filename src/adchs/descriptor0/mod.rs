#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DESCRIPTOR0 {
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
pub struct CHANNEL_NRR {
    bits: u8,
}
impl CHANNEL_NRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HALTR {
    bits: bool,
}
impl HALTR {
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
pub struct INTERRUPTR {
    bits: bool,
}
impl INTERRUPTR {
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
pub struct POWER_DOWNR {
    bits: bool,
}
impl POWER_DOWNR {
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
pub struct BRANCHR {
    bits: u8,
}
impl BRANCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MATCH_VALUER {
    bits: u16,
}
impl MATCH_VALUER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct THRESHOLD_SELR {
    bits: u8,
}
impl THRESHOLD_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESET_TIMERR {
    bits: bool,
}
impl RESET_TIMERR {
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
pub struct UPDATE_TABLER {
    bits: bool,
}
impl UPDATE_TABLER {
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
pub struct _CHANNEL_NRW<'a> {
    w: &'a mut W,
}
impl<'a> _CHANNEL_NRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HALTW<'a> {
    w: &'a mut W,
}
impl<'a> _HALTW<'a> {
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
pub struct _INTERRUPTW<'a> {
    w: &'a mut W,
}
impl<'a> _INTERRUPTW<'a> {
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
pub struct _POWER_DOWNW<'a> {
    w: &'a mut W,
}
impl<'a> _POWER_DOWNW<'a> {
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
pub struct _BRANCHW<'a> {
    w: &'a mut W,
}
impl<'a> _BRANCHW<'a> {
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
pub struct _MATCH_VALUEW<'a> {
    w: &'a mut W,
}
impl<'a> _MATCH_VALUEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 16383;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _THRESHOLD_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _THRESHOLD_SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESET_TIMERW<'a> {
    w: &'a mut W,
}
impl<'a> _RESET_TIMERW<'a> {
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
pub struct _UPDATE_TABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _UPDATE_TABLEW<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:2 - 0: convert input 0 1: convert input 1 2: convert input 2 3: convert input 3 4: convert input 4 5: convert input 5 6,7: reserved"]
    #[inline]
    pub fn channel_nr(&self) -> CHANNEL_NRR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CHANNEL_NRR { bits }
    }
    #[doc = "Bit 3 - 0: After this descriptor continue with the next descriptor. 1: halt after this descriptor is processed. Restart at a new trigger."]
    #[inline]
    pub fn halt(&self) -> HALTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HALTR { bits }
    }
    #[doc = "Bit 4 - 1: Raise interrupt when ADC result is available"]
    #[inline]
    pub fn interrupt(&self) -> INTERRUPTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTERRUPTR { bits }
    }
    #[doc = "Bit 5 - 1: Power down after this conversion."]
    #[inline]
    pub fn power_down(&self) -> POWER_DOWNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        POWER_DOWNR { bits }
    }
    #[doc = "Bits 6:7 - 00: Continue with next descriptor (wraps around after top). 01: Branch to the first descriptor in this table. 10: Swap tables and branch to the first descriptor of the new table. 11: reserved (do not store sample). Continue with next descriptor (wraps around after top)."]
    #[inline]
    pub fn branch(&self) -> BRANCHR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BRANCHR { bits }
    }
    #[doc = "Bits 8:21 - Evaluate this descriptor when descriptor timer value is equal to match value."]
    #[inline]
    pub fn match_value(&self) -> MATCH_VALUER {
        let bits = {
            const MASK: u16 = 16383;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        MATCH_VALUER { bits }
    }
    #[doc = "Bits 22:23 - Indicates which threshold comparison level register set is to be used: 00: no comparison, 01: THR_A. 10: THR_B. 11: Reserved"]
    #[inline]
    pub fn threshold_sel(&self) -> THRESHOLD_SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        THRESHOLD_SELR { bits }
    }
    #[doc = "Bit 24 - 1: reset descriptor timer."]
    #[inline]
    pub fn reset_timer(&self) -> RESET_TIMERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESET_TIMERR { bits }
    }
    #[doc = "Bit 31 - 1: Update table with all 8 descriptors of this table. Descriptors of this table that are written without this bit set are not updated until any descriptor of this table is written with this bit set. This field is write only. A read returns 0x0."]
    #[inline]
    pub fn update_table(&self) -> UPDATE_TABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UPDATE_TABLER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 37088 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - 0: convert input 0 1: convert input 1 2: convert input 2 3: convert input 3 4: convert input 4 5: convert input 5 6,7: reserved"]
    #[inline]
    pub fn channel_nr(&mut self) -> _CHANNEL_NRW {
        _CHANNEL_NRW { w: self }
    }
    #[doc = "Bit 3 - 0: After this descriptor continue with the next descriptor. 1: halt after this descriptor is processed. Restart at a new trigger."]
    #[inline]
    pub fn halt(&mut self) -> _HALTW {
        _HALTW { w: self }
    }
    #[doc = "Bit 4 - 1: Raise interrupt when ADC result is available"]
    #[inline]
    pub fn interrupt(&mut self) -> _INTERRUPTW {
        _INTERRUPTW { w: self }
    }
    #[doc = "Bit 5 - 1: Power down after this conversion."]
    #[inline]
    pub fn power_down(&mut self) -> _POWER_DOWNW {
        _POWER_DOWNW { w: self }
    }
    #[doc = "Bits 6:7 - 00: Continue with next descriptor (wraps around after top). 01: Branch to the first descriptor in this table. 10: Swap tables and branch to the first descriptor of the new table. 11: reserved (do not store sample). Continue with next descriptor (wraps around after top)."]
    #[inline]
    pub fn branch(&mut self) -> _BRANCHW {
        _BRANCHW { w: self }
    }
    #[doc = "Bits 8:21 - Evaluate this descriptor when descriptor timer value is equal to match value."]
    #[inline]
    pub fn match_value(&mut self) -> _MATCH_VALUEW {
        _MATCH_VALUEW { w: self }
    }
    #[doc = "Bits 22:23 - Indicates which threshold comparison level register set is to be used: 00: no comparison, 01: THR_A. 10: THR_B. 11: Reserved"]
    #[inline]
    pub fn threshold_sel(&mut self) -> _THRESHOLD_SELW {
        _THRESHOLD_SELW { w: self }
    }
    #[doc = "Bit 24 - 1: reset descriptor timer."]
    #[inline]
    pub fn reset_timer(&mut self) -> _RESET_TIMERW {
        _RESET_TIMERW { w: self }
    }
    #[doc = "Bit 31 - 1: Update table with all 8 descriptors of this table. Descriptors of this table that are written without this bit set are not updated until any descriptor of this table is written with this bit set. This field is write only. A read returns 0x0."]
    #[inline]
    pub fn update_table(&mut self) -> _UPDATE_TABLEW {
        _UPDATE_TABLEW { w: self }
    }
}
