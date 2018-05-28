#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IF_MSK2 {
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
pub struct MSK28_16R {
    bits: u16,
}
impl MSK28_16R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `MDIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDIRR {
    #[doc = "The message direction bit (DIR) is used for acceptance filtering."]
    THE_MESSAGE_DIRECTIO,
    #[doc = "The message direction bit (DIR) has no effect on acceptance filtering."]
    THE_MESSAGE_DIRECTIO,
}
impl MDIRR {
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
            MDIRR::THE_MESSAGE_DIRECTIO => true,
            MDIRR::THE_MESSAGE_DIRECTIO => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MDIRR {
        match value {
            true => MDIRR::THE_MESSAGE_DIRECTIO,
            false => MDIRR::THE_MESSAGE_DIRECTIO,
        }
    }
    #[doc = "Checks if the value of the field is `THE_MESSAGE_DIRECTIO`"]
    #[inline]
    pub fn is_the_message_directio(&self) -> bool {
        *self == MDIRR::THE_MESSAGE_DIRECTIO
    }
    #[doc = "Checks if the value of the field is `THE_MESSAGE_DIRECTIO`"]
    #[inline]
    pub fn is_the_message_directio(&self) -> bool {
        *self == MDIRR::THE_MESSAGE_DIRECTIO
    }
}
#[doc = "Possible values of the field `MXTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MXTDR {
    #[doc = "The extended identifier bit (IDE) is used for acceptance filtering."]
    THE_EXTENDED_IDENTIF,
    #[doc = "The extended identifier bit (IDE) has no effect on acceptance filtering."]
    THE_EXTENDED_IDENTIF,
}
impl MXTDR {
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
            MXTDR::THE_EXTENDED_IDENTIF => true,
            MXTDR::THE_EXTENDED_IDENTIF => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MXTDR {
        match value {
            true => MXTDR::THE_EXTENDED_IDENTIF,
            false => MXTDR::THE_EXTENDED_IDENTIF,
        }
    }
    #[doc = "Checks if the value of the field is `THE_EXTENDED_IDENTIF`"]
    #[inline]
    pub fn is_the_extended_identif(&self) -> bool {
        *self == MXTDR::THE_EXTENDED_IDENTIF
    }
    #[doc = "Checks if the value of the field is `THE_EXTENDED_IDENTIF`"]
    #[inline]
    pub fn is_the_extended_identif(&self) -> bool {
        *self == MXTDR::THE_EXTENDED_IDENTIF
    }
}
#[doc = r" Proxy"]
pub struct _MSK28_16W<'a> {
    w: &'a mut W,
}
impl<'a> _MSK28_16W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 8191;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MDIR`"]
pub enum MDIRW {
    #[doc = "The message direction bit (DIR) is used for acceptance filtering."]
    THE_MESSAGE_DIRECTIO,
    #[doc = "The message direction bit (DIR) has no effect on acceptance filtering."]
    THE_MESSAGE_DIRECTIO,
}
impl MDIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MDIRW::THE_MESSAGE_DIRECTIO => true,
            MDIRW::THE_MESSAGE_DIRECTIO => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MDIRW<'a> {
    w: &'a mut W,
}
impl<'a> _MDIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MDIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The message direction bit (DIR) is used for acceptance filtering."]
    #[inline]
    pub fn the_message_directio(self) -> &'a mut W {
        self.variant(MDIRW::THE_MESSAGE_DIRECTIO)
    }
    #[doc = "The message direction bit (DIR) has no effect on acceptance filtering."]
    #[inline]
    pub fn the_message_directio(self) -> &'a mut W {
        self.variant(MDIRW::THE_MESSAGE_DIRECTIO)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MXTD`"]
pub enum MXTDW {
    #[doc = "The extended identifier bit (IDE) is used for acceptance filtering."]
    THE_EXTENDED_IDENTIF,
    #[doc = "The extended identifier bit (IDE) has no effect on acceptance filtering."]
    THE_EXTENDED_IDENTIF,
}
impl MXTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MXTDW::THE_EXTENDED_IDENTIF => true,
            MXTDW::THE_EXTENDED_IDENTIF => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MXTDW<'a> {
    w: &'a mut W,
}
impl<'a> _MXTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MXTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The extended identifier bit (IDE) is used for acceptance filtering."]
    #[inline]
    pub fn the_extended_identif(self) -> &'a mut W {
        self.variant(MXTDW::THE_EXTENDED_IDENTIF)
    }
    #[doc = "The extended identifier bit (IDE) has no effect on acceptance filtering."]
    #[inline]
    pub fn the_extended_identif(self) -> &'a mut W {
        self.variant(MXTDW::THE_EXTENDED_IDENTIF)
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
        const OFFSET: u8 = 15;
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
    #[doc = "Bits 0:12 - Identifier mask 0 = The corresponding bit in the identifier of the message can not inhibit the match in the acceptance filtering. 1 = The corresponding identifier bit is used for acceptance filtering."]
    #[inline]
    pub fn msk28_16(&self) -> MSK28_16R {
        let bits = {
            const MASK: u16 = 8191;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        MSK28_16R { bits }
    }
    #[doc = "Bit 14 - Mask message direction"]
    #[inline]
    pub fn mdir(&self) -> MDIRR {
        MDIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Mask extend identifier"]
    #[inline]
    pub fn mxtd(&self) -> MXTDR {
        MXTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 65535 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:12 - Identifier mask 0 = The corresponding bit in the identifier of the message can not inhibit the match in the acceptance filtering. 1 = The corresponding identifier bit is used for acceptance filtering."]
    #[inline]
    pub fn msk28_16(&mut self) -> _MSK28_16W {
        _MSK28_16W { w: self }
    }
    #[doc = "Bit 14 - Mask message direction"]
    #[inline]
    pub fn mdir(&mut self) -> _MDIRW {
        _MDIRW { w: self }
    }
    #[doc = "Bit 15 - Mask extend identifier"]
    #[inline]
    pub fn mxtd(&mut self) -> _MXTDW {
        _MXTDW { w: self }
    }
}
