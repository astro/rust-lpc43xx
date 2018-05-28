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
#[doc = "Possible values of the field `E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ER {
    #[doc = "Disabled (default). Disabling the DMA Controller reduces power consumption."]
    DISABLED_DEFAULT,
    #[doc = "Enabled"]
    ENABLED,
}
impl ER {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ER::DISABLED_DEFAULT => false,
            ER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ER {
        match value {
            false => ER::DISABLED_DEFAULT,
            true => ER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_DEFAULT`"]
    #[inline]
    pub fn is_disabled_default(&self) -> bool {
        *self == ER::DISABLED_DEFAULT
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ER::ENABLED
    }
}
#[doc = "Possible values of the field `M0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M0R {
    #[doc = "Little-endian mode (default)."]
    LITTLE_ENDIAN_MODE,
    #[doc = "Big-endian mode."]
    BIG_ENDIAN_MODE,
}
impl M0R {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            M0R::LITTLE_ENDIAN_MODE => false,
            M0R::BIG_ENDIAN_MODE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M0R {
        match value {
            false => M0R::LITTLE_ENDIAN_MODE,
            true => M0R::BIG_ENDIAN_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `LITTLE_ENDIAN_MODE`"]
    #[inline]
    pub fn is_little_endian_mode(&self) -> bool {
        *self == M0R::LITTLE_ENDIAN_MODE
    }
    #[doc = "Checks if the value of the field is `BIG_ENDIAN_MODE`"]
    #[inline]
    pub fn is_big_endian_mode(&self) -> bool {
        *self == M0R::BIG_ENDIAN_MODE
    }
}
#[doc = "Possible values of the field `M1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M1R {
    #[doc = "Little-endian mode (default)."]
    LITTLE_ENDIAN_MODE,
    #[doc = "Big-endian mode."]
    BIG_ENDIAN_MODE,
}
impl M1R {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            M1R::LITTLE_ENDIAN_MODE => false,
            M1R::BIG_ENDIAN_MODE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M1R {
        match value {
            false => M1R::LITTLE_ENDIAN_MODE,
            true => M1R::BIG_ENDIAN_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `LITTLE_ENDIAN_MODE`"]
    #[inline]
    pub fn is_little_endian_mode(&self) -> bool {
        *self == M1R::LITTLE_ENDIAN_MODE
    }
    #[doc = "Checks if the value of the field is `BIG_ENDIAN_MODE`"]
    #[inline]
    pub fn is_big_endian_mode(&self) -> bool {
        *self == M1R::BIG_ENDIAN_MODE
    }
}
#[doc = "Values that can be written to the field `E`"]
pub enum EW {
    #[doc = "Disabled (default). Disabling the DMA Controller reduces power consumption."]
    DISABLED_DEFAULT,
    #[doc = "Enabled"]
    ENABLED,
}
impl EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EW::DISABLED_DEFAULT => false,
            EW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EW<'a> {
    w: &'a mut W,
}
impl<'a> _EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled (default). Disabling the DMA Controller reduces power consumption."]
    #[inline]
    pub fn disabled_default(self) -> &'a mut W {
        self.variant(EW::DISABLED_DEFAULT)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EW::ENABLED)
    }
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
#[doc = "Values that can be written to the field `M0`"]
pub enum M0W {
    #[doc = "Little-endian mode (default)."]
    LITTLE_ENDIAN_MODE,
    #[doc = "Big-endian mode."]
    BIG_ENDIAN_MODE,
}
impl M0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M0W::LITTLE_ENDIAN_MODE => false,
            M0W::BIG_ENDIAN_MODE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M0W<'a> {
    w: &'a mut W,
}
impl<'a> _M0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Little-endian mode (default)."]
    #[inline]
    pub fn little_endian_mode(self) -> &'a mut W {
        self.variant(M0W::LITTLE_ENDIAN_MODE)
    }
    #[doc = "Big-endian mode."]
    #[inline]
    pub fn big_endian_mode(self) -> &'a mut W {
        self.variant(M0W::BIG_ENDIAN_MODE)
    }
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
#[doc = "Values that can be written to the field `M1`"]
pub enum M1W {
    #[doc = "Little-endian mode (default)."]
    LITTLE_ENDIAN_MODE,
    #[doc = "Big-endian mode."]
    BIG_ENDIAN_MODE,
}
impl M1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M1W::LITTLE_ENDIAN_MODE => false,
            M1W::BIG_ENDIAN_MODE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M1W<'a> {
    w: &'a mut W,
}
impl<'a> _M1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Little-endian mode (default)."]
    #[inline]
    pub fn little_endian_mode(self) -> &'a mut W {
        self.variant(M1W::LITTLE_ENDIAN_MODE)
    }
    #[doc = "Big-endian mode."]
    #[inline]
    pub fn big_endian_mode(self) -> &'a mut W {
        self.variant(M1W::BIG_ENDIAN_MODE)
    }
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - DMA Controller enable:"]
    #[inline]
    pub fn e(&self) -> ER {
        ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - AHB Master 0 endianness configuration:"]
    #[inline]
    pub fn m0(&self) -> M0R {
        M0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - AHB Master 1 endianness configuration:"]
    #[inline]
    pub fn m1(&self) -> M1R {
        M1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    #[doc = "Bit 0 - DMA Controller enable:"]
    #[inline]
    pub fn e(&mut self) -> _EW {
        _EW { w: self }
    }
    #[doc = "Bit 1 - AHB Master 0 endianness configuration:"]
    #[inline]
    pub fn m0(&mut self) -> _M0W {
        _M0W { w: self }
    }
    #[doc = "Bit 2 - AHB Master 1 endianness configuration:"]
    #[inline]
    pub fn m1(&mut self) -> _M1W {
        _M1W { w: self }
    }
}
