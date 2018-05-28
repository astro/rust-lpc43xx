#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ENDPTFLUSH {
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
pub struct FERB0R {
    bits: bool,
}
impl FERB0R {
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
pub struct FERB1R {
    bits: bool,
}
impl FERB1R {
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
pub struct FERB2R {
    bits: bool,
}
impl FERB2R {
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
pub struct FERB3R {
    bits: bool,
}
impl FERB3R {
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
pub struct FETB0R {
    bits: bool,
}
impl FETB0R {
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
pub struct FETB1R {
    bits: bool,
}
impl FETB1R {
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
pub struct FETB2R {
    bits: bool,
}
impl FETB2R {
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
pub struct FETB3R {
    bits: bool,
}
impl FETB3R {
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
pub struct _FERB0W<'a> {
    w: &'a mut W,
}
impl<'a> _FERB0W<'a> {
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
pub struct _FERB1W<'a> {
    w: &'a mut W,
}
impl<'a> _FERB1W<'a> {
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
pub struct _FERB2W<'a> {
    w: &'a mut W,
}
impl<'a> _FERB2W<'a> {
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
pub struct _FERB3W<'a> {
    w: &'a mut W,
}
impl<'a> _FERB3W<'a> {
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
pub struct _FETB0W<'a> {
    w: &'a mut W,
}
impl<'a> _FETB0W<'a> {
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
pub struct _FETB1W<'a> {
    w: &'a mut W,
}
impl<'a> _FETB1W<'a> {
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
pub struct _FETB2W<'a> {
    w: &'a mut W,
}
impl<'a> _FETB2W<'a> {
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
pub struct _FETB3W<'a> {
    w: &'a mut W,
}
impl<'a> _FETB3W<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Flush endpoint receive buffer for physical OUT endpoints. Writing a one to a bit(s) will clear any primed buffers. FERB0 = endpoint 0 ... FERB3 = endpoint 3"]
    #[inline]
    pub fn ferb0(&self) -> FERB0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FERB0R { bits }
    }
    #[doc = "Bit 1 - Flush endpoint receive buffer for physical OUT endpoints. Writing a one to a bit(s) will clear any primed buffers. FERB0 = endpoint 0 ... FERB3 = endpoint 3"]
    #[inline]
    pub fn ferb1(&self) -> FERB1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FERB1R { bits }
    }
    #[doc = "Bit 2 - Flush endpoint receive buffer for physical OUT endpoints. Writing a one to a bit(s) will clear any primed buffers. FERB0 = endpoint 0 ... FERB3 = endpoint 3"]
    #[inline]
    pub fn ferb2(&self) -> FERB2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FERB2R { bits }
    }
    #[doc = "Bit 3 - Flush endpoint receive buffer for physical OUT endpoints. Writing a one to a bit(s) will clear any primed buffers. FERB0 = endpoint 0 ... FERB3 = endpoint 3"]
    #[inline]
    pub fn ferb3(&self) -> FERB3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FERB3R { bits }
    }
    #[doc = "Bit 16 - Flush endpoint transmit buffer for physical IN endpoints. Writing a one to a bit(s) will clear any primed buffers. FETB0 = endpoint 0 ... FETB3 = endpoint 3"]
    #[inline]
    pub fn fetb0(&self) -> FETB0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FETB0R { bits }
    }
    #[doc = "Bit 17 - Flush endpoint transmit buffer for physical IN endpoints. Writing a one to a bit(s) will clear any primed buffers. FETB0 = endpoint 0 ... FETB3 = endpoint 3"]
    #[inline]
    pub fn fetb1(&self) -> FETB1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FETB1R { bits }
    }
    #[doc = "Bit 18 - Flush endpoint transmit buffer for physical IN endpoints. Writing a one to a bit(s) will clear any primed buffers. FETB0 = endpoint 0 ... FETB3 = endpoint 3"]
    #[inline]
    pub fn fetb2(&self) -> FETB2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FETB2R { bits }
    }
    #[doc = "Bit 19 - Flush endpoint transmit buffer for physical IN endpoints. Writing a one to a bit(s) will clear any primed buffers. FETB0 = endpoint 0 ... FETB3 = endpoint 3"]
    #[inline]
    pub fn fetb3(&self) -> FETB3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FETB3R { bits }
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
    #[doc = "Bit 0 - Flush endpoint receive buffer for physical OUT endpoints. Writing a one to a bit(s) will clear any primed buffers. FERB0 = endpoint 0 ... FERB3 = endpoint 3"]
    #[inline]
    pub fn ferb0(&mut self) -> _FERB0W {
        _FERB0W { w: self }
    }
    #[doc = "Bit 1 - Flush endpoint receive buffer for physical OUT endpoints. Writing a one to a bit(s) will clear any primed buffers. FERB0 = endpoint 0 ... FERB3 = endpoint 3"]
    #[inline]
    pub fn ferb1(&mut self) -> _FERB1W {
        _FERB1W { w: self }
    }
    #[doc = "Bit 2 - Flush endpoint receive buffer for physical OUT endpoints. Writing a one to a bit(s) will clear any primed buffers. FERB0 = endpoint 0 ... FERB3 = endpoint 3"]
    #[inline]
    pub fn ferb2(&mut self) -> _FERB2W {
        _FERB2W { w: self }
    }
    #[doc = "Bit 3 - Flush endpoint receive buffer for physical OUT endpoints. Writing a one to a bit(s) will clear any primed buffers. FERB0 = endpoint 0 ... FERB3 = endpoint 3"]
    #[inline]
    pub fn ferb3(&mut self) -> _FERB3W {
        _FERB3W { w: self }
    }
    #[doc = "Bit 16 - Flush endpoint transmit buffer for physical IN endpoints. Writing a one to a bit(s) will clear any primed buffers. FETB0 = endpoint 0 ... FETB3 = endpoint 3"]
    #[inline]
    pub fn fetb0(&mut self) -> _FETB0W {
        _FETB0W { w: self }
    }
    #[doc = "Bit 17 - Flush endpoint transmit buffer for physical IN endpoints. Writing a one to a bit(s) will clear any primed buffers. FETB0 = endpoint 0 ... FETB3 = endpoint 3"]
    #[inline]
    pub fn fetb1(&mut self) -> _FETB1W {
        _FETB1W { w: self }
    }
    #[doc = "Bit 18 - Flush endpoint transmit buffer for physical IN endpoints. Writing a one to a bit(s) will clear any primed buffers. FETB0 = endpoint 0 ... FETB3 = endpoint 3"]
    #[inline]
    pub fn fetb2(&mut self) -> _FETB2W {
        _FETB2W { w: self }
    }
    #[doc = "Bit 19 - Flush endpoint transmit buffer for physical IN endpoints. Writing a one to a bit(s) will clear any primed buffers. FETB0 = endpoint 0 ... FETB3 = endpoint 3"]
    #[inline]
    pub fn fetb3(&mut self) -> _FETB3W {
        _FETB3W { w: self }
    }
}
