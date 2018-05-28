#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CREG5 {
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
#[doc = "Possible values of the field `M0SUBTAPSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M0SUBTAPSELR {
    #[doc = "No effect."]
    NO_EFFECT,
    #[doc = "Disable JTAG debug. Once JTAG is disabled, JTAG access remains disabled until the chip is reset by any source."]
    DISABLE_JTAG_DEBUG,
}
impl M0SUBTAPSELR {
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
            M0SUBTAPSELR::NO_EFFECT => false,
            M0SUBTAPSELR::DISABLE_JTAG_DEBUG => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M0SUBTAPSELR {
        match value {
            false => M0SUBTAPSELR::NO_EFFECT,
            true => M0SUBTAPSELR::DISABLE_JTAG_DEBUG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline]
    pub fn is_no_effect(&self) -> bool {
        *self == M0SUBTAPSELR::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `DISABLE_JTAG_DEBUG`"]
    #[inline]
    pub fn is_disable_jtag_debug(&self) -> bool {
        *self == M0SUBTAPSELR::DISABLE_JTAG_DEBUG
    }
}
#[doc = "Possible values of the field `M4TAPSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M4TAPSELR {
    #[doc = "No effect."]
    NO_EFFECT,
    #[doc = "Disable JTAG debug. Once JTAG is disabled, JTAG access remains disabled until the chip is reset by any source."]
    DISABLE_JTAG_DEBUG,
}
impl M4TAPSELR {
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
            M4TAPSELR::NO_EFFECT => false,
            M4TAPSELR::DISABLE_JTAG_DEBUG => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M4TAPSELR {
        match value {
            false => M4TAPSELR::NO_EFFECT,
            true => M4TAPSELR::DISABLE_JTAG_DEBUG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline]
    pub fn is_no_effect(&self) -> bool {
        *self == M4TAPSELR::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `DISABLE_JTAG_DEBUG`"]
    #[inline]
    pub fn is_disable_jtag_debug(&self) -> bool {
        *self == M4TAPSELR::DISABLE_JTAG_DEBUG
    }
}
#[doc = "Possible values of the field `M0APPTAPSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M0APPTAPSELR {
    #[doc = "No effect."]
    NO_EFFECT,
    #[doc = "Disable JTAG debug. Once JTAG is disabled, JTAG access remains disabled until the chip is reset by any source."]
    DISABLE_JTAG_DEBUG,
}
impl M0APPTAPSELR {
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
            M0APPTAPSELR::NO_EFFECT => false,
            M0APPTAPSELR::DISABLE_JTAG_DEBUG => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M0APPTAPSELR {
        match value {
            false => M0APPTAPSELR::NO_EFFECT,
            true => M0APPTAPSELR::DISABLE_JTAG_DEBUG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline]
    pub fn is_no_effect(&self) -> bool {
        *self == M0APPTAPSELR::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `DISABLE_JTAG_DEBUG`"]
    #[inline]
    pub fn is_disable_jtag_debug(&self) -> bool {
        *self == M0APPTAPSELR::DISABLE_JTAG_DEBUG
    }
}
#[doc = "Values that can be written to the field `M0SUBTAPSEL`"]
pub enum M0SUBTAPSELW {
    #[doc = "No effect."]
    NO_EFFECT,
    #[doc = "Disable JTAG debug. Once JTAG is disabled, JTAG access remains disabled until the chip is reset by any source."]
    DISABLE_JTAG_DEBUG,
}
impl M0SUBTAPSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M0SUBTAPSELW::NO_EFFECT => false,
            M0SUBTAPSELW::DISABLE_JTAG_DEBUG => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M0SUBTAPSELW<'a> {
    w: &'a mut W,
}
impl<'a> _M0SUBTAPSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M0SUBTAPSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(M0SUBTAPSELW::NO_EFFECT)
    }
    #[doc = "Disable JTAG debug. Once JTAG is disabled, JTAG access remains disabled until the chip is reset by any source."]
    #[inline]
    pub fn disable_jtag_debug(self) -> &'a mut W {
        self.variant(M0SUBTAPSELW::DISABLE_JTAG_DEBUG)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `M4TAPSEL`"]
pub enum M4TAPSELW {
    #[doc = "No effect."]
    NO_EFFECT,
    #[doc = "Disable JTAG debug. Once JTAG is disabled, JTAG access remains disabled until the chip is reset by any source."]
    DISABLE_JTAG_DEBUG,
}
impl M4TAPSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M4TAPSELW::NO_EFFECT => false,
            M4TAPSELW::DISABLE_JTAG_DEBUG => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M4TAPSELW<'a> {
    w: &'a mut W,
}
impl<'a> _M4TAPSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M4TAPSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(M4TAPSELW::NO_EFFECT)
    }
    #[doc = "Disable JTAG debug. Once JTAG is disabled, JTAG access remains disabled until the chip is reset by any source."]
    #[inline]
    pub fn disable_jtag_debug(self) -> &'a mut W {
        self.variant(M4TAPSELW::DISABLE_JTAG_DEBUG)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `M0APPTAPSEL`"]
pub enum M0APPTAPSELW {
    #[doc = "No effect."]
    NO_EFFECT,
    #[doc = "Disable JTAG debug. Once JTAG is disabled, JTAG access remains disabled until the chip is reset by any source."]
    DISABLE_JTAG_DEBUG,
}
impl M0APPTAPSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M0APPTAPSELW::NO_EFFECT => false,
            M0APPTAPSELW::DISABLE_JTAG_DEBUG => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M0APPTAPSELW<'a> {
    w: &'a mut W,
}
impl<'a> _M0APPTAPSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M0APPTAPSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(M0APPTAPSELW::NO_EFFECT)
    }
    #[doc = "Disable JTAG debug. Once JTAG is disabled, JTAG access remains disabled until the chip is reset by any source."]
    #[inline]
    pub fn disable_jtag_debug(self) -> &'a mut W {
        self.variant(M0APPTAPSELW::DISABLE_JTAG_DEBUG)
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
        const OFFSET: u8 = 12;
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
    #[doc = "Bit 10 - JTAG debug disable for M0SUB co-processor. If this bit is set to 1, it can be changed to 0 only through a chip reset."]
    #[inline]
    pub fn m0subtapsel(&self) -> M0SUBTAPSELR {
        M0SUBTAPSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - JTAG debug disable for M4 main processor. If this bit is set to 1, it can be changed to 0 only through a chip reset."]
    #[inline]
    pub fn m4tapsel(&self) -> M4TAPSELR {
        M4TAPSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - JTAG debug disable for M0APPco-processor. If this bit is set to 1, it can be changed to 0 only through a chip reset."]
    #[inline]
    pub fn m0apptapsel(&self) -> M0APPTAPSELR {
        M0APPTAPSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
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
    #[doc = "Bit 10 - JTAG debug disable for M0SUB co-processor. If this bit is set to 1, it can be changed to 0 only through a chip reset."]
    #[inline]
    pub fn m0subtapsel(&mut self) -> _M0SUBTAPSELW {
        _M0SUBTAPSELW { w: self }
    }
    #[doc = "Bit 11 - JTAG debug disable for M4 main processor. If this bit is set to 1, it can be changed to 0 only through a chip reset."]
    #[inline]
    pub fn m4tapsel(&mut self) -> _M4TAPSELW {
        _M4TAPSELW { w: self }
    }
    #[doc = "Bit 12 - JTAG debug disable for M0APPco-processor. If this bit is set to 1, it can be changed to 0 only through a chip reset."]
    #[inline]
    pub fn m0apptapsel(&mut self) -> _M0APPTAPSELW {
        _M0APPTAPSELW { w: self }
    }
}
