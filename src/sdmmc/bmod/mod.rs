#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BMOD {
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
pub struct SWRR {
    bits: bool,
}
impl SWRR {
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
pub struct FBR {
    bits: bool,
}
impl FBR {
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
pub struct DSLR {
    bits: u8,
}
impl DSLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DER {
    bits: bool,
}
impl DER {
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
#[doc = "Possible values of the field `PBL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBLR {
    #[doc = "1 transfer"]
    _1_TRANSFER,
    #[doc = "4 transfers"]
    _4_TRANSFERS,
    #[doc = "8 transfers"]
    _8_TRANSFERS,
    #[doc = "16 transfers"]
    _16_TRANSFERS,
    #[doc = "32 transfers"]
    _32_TRANSFERS,
    #[doc = "64 transfers"]
    _64_TRANSFERS,
    #[doc = "128 transfers"]
    _128_TRANSFERS,
    #[doc = "256 transfers"]
    _256_TRANSFERS,
}
impl PBLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PBLR::_1_TRANSFER => 0,
            PBLR::_4_TRANSFERS => 1,
            PBLR::_8_TRANSFERS => 2,
            PBLR::_16_TRANSFERS => 3,
            PBLR::_32_TRANSFERS => 4,
            PBLR::_64_TRANSFERS => 5,
            PBLR::_128_TRANSFERS => 6,
            PBLR::_256_TRANSFERS => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PBLR {
        match value {
            0 => PBLR::_1_TRANSFER,
            1 => PBLR::_4_TRANSFERS,
            2 => PBLR::_8_TRANSFERS,
            3 => PBLR::_16_TRANSFERS,
            4 => PBLR::_32_TRANSFERS,
            5 => PBLR::_64_TRANSFERS,
            6 => PBLR::_128_TRANSFERS,
            7 => PBLR::_256_TRANSFERS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1_TRANSFER`"]
    #[inline]
    pub fn is_1_transfer(&self) -> bool {
        *self == PBLR::_1_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_4_TRANSFERS`"]
    #[inline]
    pub fn is_4_transfers(&self) -> bool {
        *self == PBLR::_4_TRANSFERS
    }
    #[doc = "Checks if the value of the field is `_8_TRANSFERS`"]
    #[inline]
    pub fn is_8_transfers(&self) -> bool {
        *self == PBLR::_8_TRANSFERS
    }
    #[doc = "Checks if the value of the field is `_16_TRANSFERS`"]
    #[inline]
    pub fn is_16_transfers(&self) -> bool {
        *self == PBLR::_16_TRANSFERS
    }
    #[doc = "Checks if the value of the field is `_32_TRANSFERS`"]
    #[inline]
    pub fn is_32_transfers(&self) -> bool {
        *self == PBLR::_32_TRANSFERS
    }
    #[doc = "Checks if the value of the field is `_64_TRANSFERS`"]
    #[inline]
    pub fn is_64_transfers(&self) -> bool {
        *self == PBLR::_64_TRANSFERS
    }
    #[doc = "Checks if the value of the field is `_128_TRANSFERS`"]
    #[inline]
    pub fn is_128_transfers(&self) -> bool {
        *self == PBLR::_128_TRANSFERS
    }
    #[doc = "Checks if the value of the field is `_256_TRANSFERS`"]
    #[inline]
    pub fn is_256_transfers(&self) -> bool {
        *self == PBLR::_256_TRANSFERS
    }
}
#[doc = r" Proxy"]
pub struct _SWRW<'a> {
    w: &'a mut W,
}
impl<'a> _SWRW<'a> {
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
pub struct _FBW<'a> {
    w: &'a mut W,
}
impl<'a> _FBW<'a> {
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
pub struct _DSLW<'a> {
    w: &'a mut W,
}
impl<'a> _DSLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DEW<'a> {
    w: &'a mut W,
}
impl<'a> _DEW<'a> {
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
#[doc = "Values that can be written to the field `PBL`"]
pub enum PBLW {
    #[doc = "1 transfer"]
    _1_TRANSFER,
    #[doc = "4 transfers"]
    _4_TRANSFERS,
    #[doc = "8 transfers"]
    _8_TRANSFERS,
    #[doc = "16 transfers"]
    _16_TRANSFERS,
    #[doc = "32 transfers"]
    _32_TRANSFERS,
    #[doc = "64 transfers"]
    _64_TRANSFERS,
    #[doc = "128 transfers"]
    _128_TRANSFERS,
    #[doc = "256 transfers"]
    _256_TRANSFERS,
}
impl PBLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PBLW::_1_TRANSFER => 0,
            PBLW::_4_TRANSFERS => 1,
            PBLW::_8_TRANSFERS => 2,
            PBLW::_16_TRANSFERS => 3,
            PBLW::_32_TRANSFERS => 4,
            PBLW::_64_TRANSFERS => 5,
            PBLW::_128_TRANSFERS => 6,
            PBLW::_256_TRANSFERS => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PBLW<'a> {
    w: &'a mut W,
}
impl<'a> _PBLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PBLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1 transfer"]
    #[inline]
    pub fn _1_transfer(self) -> &'a mut W {
        self.variant(PBLW::_1_TRANSFER)
    }
    #[doc = "4 transfers"]
    #[inline]
    pub fn _4_transfers(self) -> &'a mut W {
        self.variant(PBLW::_4_TRANSFERS)
    }
    #[doc = "8 transfers"]
    #[inline]
    pub fn _8_transfers(self) -> &'a mut W {
        self.variant(PBLW::_8_TRANSFERS)
    }
    #[doc = "16 transfers"]
    #[inline]
    pub fn _16_transfers(self) -> &'a mut W {
        self.variant(PBLW::_16_TRANSFERS)
    }
    #[doc = "32 transfers"]
    #[inline]
    pub fn _32_transfers(self) -> &'a mut W {
        self.variant(PBLW::_32_TRANSFERS)
    }
    #[doc = "64 transfers"]
    #[inline]
    pub fn _64_transfers(self) -> &'a mut W {
        self.variant(PBLW::_64_TRANSFERS)
    }
    #[doc = "128 transfers"]
    #[inline]
    pub fn _128_transfers(self) -> &'a mut W {
        self.variant(PBLW::_128_TRANSFERS)
    }
    #[doc = "256 transfers"]
    #[inline]
    pub fn _256_transfers(self) -> &'a mut W {
        self.variant(PBLW::_256_TRANSFERS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - Software Reset. When set, the DMA Controller resets all its internal registers. SWR is read/write. It is automatically cleared after 1 clock cycle."]
    #[inline]
    pub fn swr(&self) -> SWRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWRR { bits }
    }
    #[doc = "Bit 1 - Fixed Burst. Controls whether the AHB Master interface performs fixed burst transfers or not. When set, the AHB will use only SINGLE, INCR4, INCR8 or INCR16 during start of normal burst transfers. When reset, the AHB will use SINGLE and INCR burst transfer operations. FB is read/write."]
    #[inline]
    pub fn fb(&self) -> FBR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FBR { bits }
    }
    #[doc = "Bits 2:6 - Descriptor Skip Length. Specifies the number of HWord/Word/Dword to skip between two unchained descriptors. This is applicable only for dual buffer structure. DSL is read/write."]
    #[inline]
    pub fn dsl(&self) -> DSLR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DSLR { bits }
    }
    #[doc = "Bit 7 - SD/MMC DMA Enable. When set, the SD/MMC DMA is enabled. DE is read/write."]
    #[inline]
    pub fn de(&self) -> DER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DER { bits }
    }
    #[doc = "Bits 8:10 - Programmable Burst Length. These bits indicate the maximum number of beats to be performed in one SD/MMC DMA transaction. The SD/MMC DMA will always attempt to burst as specified in PBL each time it starts a Burst transfer on the host bus. The permissible values are 1, 4, 8, 16, 32, 64, 128 and 256. This value is the mirror of MSIZE of FIFOTH register. In order to change this value, write the required value to FIFOTH register. This is an encode value as follows.Transfer unit is 32 bit. PBL is a read-only value."]
    #[inline]
    pub fn pbl(&self) -> PBLR {
        PBLR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bit 0 - Software Reset. When set, the DMA Controller resets all its internal registers. SWR is read/write. It is automatically cleared after 1 clock cycle."]
    #[inline]
    pub fn swr(&mut self) -> _SWRW {
        _SWRW { w: self }
    }
    #[doc = "Bit 1 - Fixed Burst. Controls whether the AHB Master interface performs fixed burst transfers or not. When set, the AHB will use only SINGLE, INCR4, INCR8 or INCR16 during start of normal burst transfers. When reset, the AHB will use SINGLE and INCR burst transfer operations. FB is read/write."]
    #[inline]
    pub fn fb(&mut self) -> _FBW {
        _FBW { w: self }
    }
    #[doc = "Bits 2:6 - Descriptor Skip Length. Specifies the number of HWord/Word/Dword to skip between two unchained descriptors. This is applicable only for dual buffer structure. DSL is read/write."]
    #[inline]
    pub fn dsl(&mut self) -> _DSLW {
        _DSLW { w: self }
    }
    #[doc = "Bit 7 - SD/MMC DMA Enable. When set, the SD/MMC DMA is enabled. DE is read/write."]
    #[inline]
    pub fn de(&mut self) -> _DEW {
        _DEW { w: self }
    }
    #[doc = "Bits 8:10 - Programmable Burst Length. These bits indicate the maximum number of beats to be performed in one SD/MMC DMA transaction. The SD/MMC DMA will always attempt to burst as specified in PBL each time it starts a Burst transfer on the host bus. The permissible values are 1, 4, 8, 16, 32, 64, 128 and 256. This value is the mirror of MSIZE of FIFOTH register. In order to change this value, write the required value to FIFOTH register. This is an encode value as follows.Transfer unit is 32 bit. PBL is a read-only value."]
    #[inline]
    pub fn pbl(&mut self) -> _PBLW {
        _PBLW { w: self }
    }
}
