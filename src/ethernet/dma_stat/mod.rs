#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMA_STAT {
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
pub struct TIR {
    bits: bool,
}
impl TIR {
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
pub struct TPSR {
    bits: bool,
}
impl TPSR {
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
pub struct TUR {
    bits: bool,
}
impl TUR {
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
pub struct TJTR {
    bits: bool,
}
impl TJTR {
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
pub struct OVFR {
    bits: bool,
}
impl OVFR {
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
pub struct UNFR {
    bits: bool,
}
impl UNFR {
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
pub struct RIR {
    bits: bool,
}
impl RIR {
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
pub struct RUR {
    bits: bool,
}
impl RUR {
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
pub struct RPSR {
    bits: bool,
}
impl RPSR {
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
pub struct RWTR {
    bits: bool,
}
impl RWTR {
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
pub struct ETIR {
    bits: bool,
}
impl ETIR {
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
pub struct FBIR {
    bits: bool,
}
impl FBIR {
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
pub struct ERIR {
    bits: bool,
}
impl ERIR {
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
pub struct NISR {
    bits: bool,
}
impl NISR {
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
pub struct RSR {
    bits: u8,
}
impl RSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TSR {
    bits: u8,
}
impl TSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EB1R {
    bits: bool,
}
impl EB1R {
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
pub struct EB2R {
    bits: bool,
}
impl EB2R {
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
pub struct EB3R {
    bits: bool,
}
impl EB3R {
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
pub struct _TIW<'a> {
    w: &'a mut W,
}
impl<'a> _TIW<'a> {
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
pub struct _TPSW<'a> {
    w: &'a mut W,
}
impl<'a> _TPSW<'a> {
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
pub struct _TUW<'a> {
    w: &'a mut W,
}
impl<'a> _TUW<'a> {
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
pub struct _TJTW<'a> {
    w: &'a mut W,
}
impl<'a> _TJTW<'a> {
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
pub struct _OVFW<'a> {
    w: &'a mut W,
}
impl<'a> _OVFW<'a> {
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
pub struct _UNFW<'a> {
    w: &'a mut W,
}
impl<'a> _UNFW<'a> {
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
pub struct _RIW<'a> {
    w: &'a mut W,
}
impl<'a> _RIW<'a> {
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
pub struct _RUW<'a> {
    w: &'a mut W,
}
impl<'a> _RUW<'a> {
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
pub struct _RPSW<'a> {
    w: &'a mut W,
}
impl<'a> _RPSW<'a> {
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
pub struct _RWTW<'a> {
    w: &'a mut W,
}
impl<'a> _RWTW<'a> {
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
pub struct _ETIW<'a> {
    w: &'a mut W,
}
impl<'a> _ETIW<'a> {
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
pub struct _FBIW<'a> {
    w: &'a mut W,
}
impl<'a> _FBIW<'a> {
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
pub struct _ERIW<'a> {
    w: &'a mut W,
}
impl<'a> _ERIW<'a> {
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
pub struct _NISW<'a> {
    w: &'a mut W,
}
impl<'a> _NISW<'a> {
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
pub struct _RSW<'a> {
    w: &'a mut W,
}
impl<'a> _RSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TSW<'a> {
    w: &'a mut W,
}
impl<'a> _TSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EB1W<'a> {
    w: &'a mut W,
}
impl<'a> _EB1W<'a> {
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
pub struct _EB2W<'a> {
    w: &'a mut W,
}
impl<'a> _EB2W<'a> {
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
pub struct _EB3W<'a> {
    w: &'a mut W,
}
impl<'a> _EB3W<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Transmit interrupt This bit indicates that frame transmission is finished and TDES1[31] is set in the First Descriptor."]
    #[inline]
    pub fn ti(&self) -> TIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TIR { bits }
    }
    #[doc = "Bit 1 - Transmit process stopped This bit is set when the transmission is stopped."]
    #[inline]
    pub fn tps(&self) -> TPSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TPSR { bits }
    }
    #[doc = "Bit 2 - Transmit buffer unavailable This bit indicates that the Next Descriptor in the Transmit List is owned by the host and cannot be acquired by the DMA. Transmission is suspended. Bits[22:20] explain the Transmit Process state transitions. To resume processing transmit descriptors, the host should change the ownership of the bit of the descriptor and then issue a Transmit Poll Demand command."]
    #[inline]
    pub fn tu(&self) -> TUR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TUR { bits }
    }
    #[doc = "Bit 3 - Transmit jabber timeout This bit indicates that the Transmit Jabber Timer expired, meaning that the transmitter had been excessively active. The transmission process is aborted and placed in the Stopped state. This causes the Transmit Jabber Timeout TDES0[14] flag to assert."]
    #[inline]
    pub fn tjt(&self) -> TJTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TJTR { bits }
    }
    #[doc = "Bit 4 - Receive overflow This bit indicates that the Receive Buffer had an Overflow during frame reception. If the partial frame is transferred to application, the overflow status is set in RDES0[11]."]
    #[inline]
    pub fn ovf(&self) -> OVFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OVFR { bits }
    }
    #[doc = "Bit 5 - Transmit underflow This bit indicates that the Transmit Buffer had an Underflow during frame transmission. Transmission is suspended and an Underflow Error TDES0[1] is set."]
    #[inline]
    pub fn unf(&self) -> UNFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UNFR { bits }
    }
    #[doc = "Bit 6 - Receive interrupt This bit indicates the completion of frame reception. Specific frame status information has been posted in the descriptor. Reception remains in the Running state."]
    #[inline]
    pub fn ri(&self) -> RIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RIR { bits }
    }
    #[doc = "Bit 7 - Receive buffer unavailable This bit indicates that the Next Descriptor in the Receive List is owned by the host and cannot be acquired by the DMA. Receive Process is suspended. To resume processing Receive descriptors, the host should change the ownership of the descriptor and issue a Receive Poll Demand command. If no Receive Poll Demand is issued, Receive Process resumes when the next recognized incoming frame is received. This bit is set only when the previous Receive Descriptor was owned by the DMA."]
    #[inline]
    pub fn ru(&self) -> RUR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RUR { bits }
    }
    #[doc = "Bit 8 - Received process stopped This bit is asserted when the Receive Process enters the Stopped state."]
    #[inline]
    pub fn rps(&self) -> RPSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RPSR { bits }
    }
    #[doc = "Bit 9 - Receive watchdog timeout This bit is asserted when a frame with a length greater than 2,048 bytes is received (10,240 when Jumbo Frame mode is enabled)."]
    #[inline]
    pub fn rwt(&self) -> RWTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RWTR { bits }
    }
    #[doc = "Bit 10 - Early transmit interrupt This bit indicates that the frame to be transmitted was fully transferred to the MTL Transmit FIFO."]
    #[inline]
    pub fn eti(&self) -> ETIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ETIR { bits }
    }
    #[doc = "Bit 13 - Fatal bus error interrupt This bit indicates that a bus error occurred, as detailed in bits [25:23]. When this bit is set, the corresponding DMA engine disables all its bus accesses."]
    #[inline]
    pub fn fbi(&self) -> FBIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FBIR { bits }
    }
    #[doc = "Bit 14 - Early receive interrupt This bit indicates that the DMA had filled the first data buffer of the packet. Receive Interrupt bit 6 in this register automatically clears this bit."]
    #[inline]
    pub fn eri(&self) -> ERIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ERIR { bits }
    }
    #[doc = "Bit 15 - Abnormal interrupt summary Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in the DMA_INT_EN register: DMA_STAT register, bit 1: Transmit process stopped DMA_STAT register, bit 3: Transmit jabber timeout DMA_STAT register, bit 4: Receive overflow DMA_STAT register, bit 5: Transmit underflow DMA_STAT register, bit 7: Receiver buffer unavailable DMA_STAT register, bit 8: Receive process stopped DMA_STAT register, bit 9: Receive watchdog timeout DMA_STAT register, bit 10: Early transmit interrupt DMA_STAT register, bit 13: Fatal bus error Only unmasked bits affect the Abnormal Interrupt Summary bit. This is a sticky bit and must be cleared each time a corresponding bit that causes AIS to be set is cleared."]
    #[inline]
    pub fn aie(&self) -> AIER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AIER { bits }
    }
    #[doc = "Bit 16 - Normal interrupt summary Normal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in the DMA_INT_EN register: DMA_STAT register, bit 0: Transmit interrupt DMA_STAT register, bit 2: Transmit buffer unavailable DMA_STAT register, bit 6: Receive interrupt DMA_STAT register, bit 14: Early receive interrupt Only unmasked bits affect the Normal Interrupt Summary bit. This is a sticky bit and must be cleared (by writing a 1 to this bit) each time a corresponding bit that causes NIS to be set is cleared."]
    #[inline]
    pub fn nis(&self) -> NISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NISR { bits }
    }
    #[doc = "Bits 17:19 - Receive Process State These bits indicate the receive DMA state machine state. This field does not generate an interrupt. 000 = Stopped: Reset or Stop Receive Command issued. 001 = Running: Fetching Receive Transfer Descriptor. 010 = Reserved. 011 = Running: Waiting for receive packet. 100 = Suspended: Receive Descriptor Unavailable. 101 = Running: Closing Receive Descriptor. 110 = TIME_STAMP write state. 111 = Running: Transferring the receive packet data from receive buffer to host memory."]
    #[inline]
    pub fn rs(&self) -> RSR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSR { bits }
    }
    #[doc = "Bits 20:22 - Transmit Process State These bits indicate the transmit DMA state machine state. This field does not generate an interrupt. 000 = Stopped; Reset or Stop Transmit Command issued. 001 = Running; Fetching Transmit Transfer Descriptor. 010 = Running; Waiting for status. 011 = Running; Reading Data from host memory buffer and queuing it to transmit buffer (Tx FIFO). 100 = TIME_STAMP write state. 101 = Reserved. 110 = Suspended; Transmit Descriptor Unavailable or Transmit Buffer Underflow. 111 = Running; Closing Transmit Descriptor."]
    #[inline]
    pub fn ts(&self) -> TSR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TSR { bits }
    }
    #[doc = "Bit 23 - Error bit 1 This bit indicates the type of error that caused a Bus Error (e.g., error response on the AHB interface). This bits is valid only when bit 13 in this register is set. This field does not generate an interrupt. 1 = Error during data transfer by TxDMA. 0 = Error during data transfer by RxDMA."]
    #[inline]
    pub fn eb1(&self) -> EB1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EB1R { bits }
    }
    #[doc = "Bit 24 - Error bit 2 This bit indicates the type of error that caused a Bus Error (e.g., error response on the AHB interface). This bits is valid only when bit 13 in this register is set. This field does not generate an interrupt. 1 = Error during read transfer. 0 = Error during write transfer."]
    #[inline]
    pub fn eb2(&self) -> EB2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EB2R { bits }
    }
    #[doc = "Bit 25 - Error bit 3 This bit indicates the type of error that caused a Bus Error (e.g., error response on the AHB interface). This bits is valid only when bit 13 in this register is set. This field does not generate an interrupt. 1 = Error during descriptor access. 0 = Error during data buffer access."]
    #[inline]
    pub fn eb3(&self) -> EB3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EB3R { bits }
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
    #[doc = "Bit 0 - Transmit interrupt This bit indicates that frame transmission is finished and TDES1[31] is set in the First Descriptor."]
    #[inline]
    pub fn ti(&mut self) -> _TIW {
        _TIW { w: self }
    }
    #[doc = "Bit 1 - Transmit process stopped This bit is set when the transmission is stopped."]
    #[inline]
    pub fn tps(&mut self) -> _TPSW {
        _TPSW { w: self }
    }
    #[doc = "Bit 2 - Transmit buffer unavailable This bit indicates that the Next Descriptor in the Transmit List is owned by the host and cannot be acquired by the DMA. Transmission is suspended. Bits[22:20] explain the Transmit Process state transitions. To resume processing transmit descriptors, the host should change the ownership of the bit of the descriptor and then issue a Transmit Poll Demand command."]
    #[inline]
    pub fn tu(&mut self) -> _TUW {
        _TUW { w: self }
    }
    #[doc = "Bit 3 - Transmit jabber timeout This bit indicates that the Transmit Jabber Timer expired, meaning that the transmitter had been excessively active. The transmission process is aborted and placed in the Stopped state. This causes the Transmit Jabber Timeout TDES0[14] flag to assert."]
    #[inline]
    pub fn tjt(&mut self) -> _TJTW {
        _TJTW { w: self }
    }
    #[doc = "Bit 4 - Receive overflow This bit indicates that the Receive Buffer had an Overflow during frame reception. If the partial frame is transferred to application, the overflow status is set in RDES0[11]."]
    #[inline]
    pub fn ovf(&mut self) -> _OVFW {
        _OVFW { w: self }
    }
    #[doc = "Bit 5 - Transmit underflow This bit indicates that the Transmit Buffer had an Underflow during frame transmission. Transmission is suspended and an Underflow Error TDES0[1] is set."]
    #[inline]
    pub fn unf(&mut self) -> _UNFW {
        _UNFW { w: self }
    }
    #[doc = "Bit 6 - Receive interrupt This bit indicates the completion of frame reception. Specific frame status information has been posted in the descriptor. Reception remains in the Running state."]
    #[inline]
    pub fn ri(&mut self) -> _RIW {
        _RIW { w: self }
    }
    #[doc = "Bit 7 - Receive buffer unavailable This bit indicates that the Next Descriptor in the Receive List is owned by the host and cannot be acquired by the DMA. Receive Process is suspended. To resume processing Receive descriptors, the host should change the ownership of the descriptor and issue a Receive Poll Demand command. If no Receive Poll Demand is issued, Receive Process resumes when the next recognized incoming frame is received. This bit is set only when the previous Receive Descriptor was owned by the DMA."]
    #[inline]
    pub fn ru(&mut self) -> _RUW {
        _RUW { w: self }
    }
    #[doc = "Bit 8 - Received process stopped This bit is asserted when the Receive Process enters the Stopped state."]
    #[inline]
    pub fn rps(&mut self) -> _RPSW {
        _RPSW { w: self }
    }
    #[doc = "Bit 9 - Receive watchdog timeout This bit is asserted when a frame with a length greater than 2,048 bytes is received (10,240 when Jumbo Frame mode is enabled)."]
    #[inline]
    pub fn rwt(&mut self) -> _RWTW {
        _RWTW { w: self }
    }
    #[doc = "Bit 10 - Early transmit interrupt This bit indicates that the frame to be transmitted was fully transferred to the MTL Transmit FIFO."]
    #[inline]
    pub fn eti(&mut self) -> _ETIW {
        _ETIW { w: self }
    }
    #[doc = "Bit 13 - Fatal bus error interrupt This bit indicates that a bus error occurred, as detailed in bits [25:23]. When this bit is set, the corresponding DMA engine disables all its bus accesses."]
    #[inline]
    pub fn fbi(&mut self) -> _FBIW {
        _FBIW { w: self }
    }
    #[doc = "Bit 14 - Early receive interrupt This bit indicates that the DMA had filled the first data buffer of the packet. Receive Interrupt bit 6 in this register automatically clears this bit."]
    #[inline]
    pub fn eri(&mut self) -> _ERIW {
        _ERIW { w: self }
    }
    #[doc = "Bit 15 - Abnormal interrupt summary Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in the DMA_INT_EN register: DMA_STAT register, bit 1: Transmit process stopped DMA_STAT register, bit 3: Transmit jabber timeout DMA_STAT register, bit 4: Receive overflow DMA_STAT register, bit 5: Transmit underflow DMA_STAT register, bit 7: Receiver buffer unavailable DMA_STAT register, bit 8: Receive process stopped DMA_STAT register, bit 9: Receive watchdog timeout DMA_STAT register, bit 10: Early transmit interrupt DMA_STAT register, bit 13: Fatal bus error Only unmasked bits affect the Abnormal Interrupt Summary bit. This is a sticky bit and must be cleared each time a corresponding bit that causes AIS to be set is cleared."]
    #[inline]
    pub fn aie(&mut self) -> _AIEW {
        _AIEW { w: self }
    }
    #[doc = "Bit 16 - Normal interrupt summary Normal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in the DMA_INT_EN register: DMA_STAT register, bit 0: Transmit interrupt DMA_STAT register, bit 2: Transmit buffer unavailable DMA_STAT register, bit 6: Receive interrupt DMA_STAT register, bit 14: Early receive interrupt Only unmasked bits affect the Normal Interrupt Summary bit. This is a sticky bit and must be cleared (by writing a 1 to this bit) each time a corresponding bit that causes NIS to be set is cleared."]
    #[inline]
    pub fn nis(&mut self) -> _NISW {
        _NISW { w: self }
    }
    #[doc = "Bits 17:19 - Receive Process State These bits indicate the receive DMA state machine state. This field does not generate an interrupt. 000 = Stopped: Reset or Stop Receive Command issued. 001 = Running: Fetching Receive Transfer Descriptor. 010 = Reserved. 011 = Running: Waiting for receive packet. 100 = Suspended: Receive Descriptor Unavailable. 101 = Running: Closing Receive Descriptor. 110 = TIME_STAMP write state. 111 = Running: Transferring the receive packet data from receive buffer to host memory."]
    #[inline]
    pub fn rs(&mut self) -> _RSW {
        _RSW { w: self }
    }
    #[doc = "Bits 20:22 - Transmit Process State These bits indicate the transmit DMA state machine state. This field does not generate an interrupt. 000 = Stopped; Reset or Stop Transmit Command issued. 001 = Running; Fetching Transmit Transfer Descriptor. 010 = Running; Waiting for status. 011 = Running; Reading Data from host memory buffer and queuing it to transmit buffer (Tx FIFO). 100 = TIME_STAMP write state. 101 = Reserved. 110 = Suspended; Transmit Descriptor Unavailable or Transmit Buffer Underflow. 111 = Running; Closing Transmit Descriptor."]
    #[inline]
    pub fn ts(&mut self) -> _TSW {
        _TSW { w: self }
    }
    #[doc = "Bit 23 - Error bit 1 This bit indicates the type of error that caused a Bus Error (e.g., error response on the AHB interface). This bits is valid only when bit 13 in this register is set. This field does not generate an interrupt. 1 = Error during data transfer by TxDMA. 0 = Error during data transfer by RxDMA."]
    #[inline]
    pub fn eb1(&mut self) -> _EB1W {
        _EB1W { w: self }
    }
    #[doc = "Bit 24 - Error bit 2 This bit indicates the type of error that caused a Bus Error (e.g., error response on the AHB interface). This bits is valid only when bit 13 in this register is set. This field does not generate an interrupt. 1 = Error during read transfer. 0 = Error during write transfer."]
    #[inline]
    pub fn eb2(&mut self) -> _EB2W {
        _EB2W { w: self }
    }
    #[doc = "Bit 25 - Error bit 3 This bit indicates the type of error that caused a Bus Error (e.g., error response on the AHB interface). This bits is valid only when bit 13 in this register is set. This field does not generate an interrupt. 1 = Error during descriptor access. 0 = Error during data buffer access."]
    #[inline]
    pub fn eb3(&mut self) -> _EB3W {
        _EB3W { w: self }
    }
}
