#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCICTRL {
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
#[doc = "Possible values of the field `SCIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCIENR {
    #[doc = "Disabled. Smart card interface disabled."]
    DISABLED,
    #[doc = "Enabled. synchronous half duplex smart card interface is enabled."]
    ENABLED,
}
impl SCIENR {
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
            SCIENR::DISABLED => false,
            SCIENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCIENR {
        match value {
            false => SCIENR::DISABLED,
            true => SCIENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SCIENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SCIENR::ENABLED
    }
}
#[doc = "Possible values of the field `NACKDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACKDISR {
    #[doc = "Enabled. A NACK response is enabled."]
    ENABLED,
    #[doc = "Disabled. A NACK response is inhibited."]
    DISABLED,
}
impl NACKDISR {
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
            NACKDISR::ENABLED => false,
            NACKDISR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NACKDISR {
        match value {
            false => NACKDISR::ENABLED,
            true => NACKDISR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == NACKDISR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == NACKDISR::DISABLED
    }
}
#[doc = "Possible values of the field `PROTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROTSELR {
    #[doc = "T = 0"]
    T_EQ_0,
    #[doc = "T = 1"]
    T_EQ_1,
}
impl PROTSELR {
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
            PROTSELR::T_EQ_0 => false,
            PROTSELR::T_EQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PROTSELR {
        match value {
            false => PROTSELR::T_EQ_0,
            true => PROTSELR::T_EQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `T_EQ_0`"]
    #[inline]
    pub fn is_t_eq_0(&self) -> bool {
        *self == PROTSELR::T_EQ_0
    }
    #[doc = "Checks if the value of the field is `T_EQ_1`"]
    #[inline]
    pub fn is_t_eq_1(&self) -> bool {
        *self == PROTSELR::T_EQ_1
    }
}
#[doc = r" Value of the field"]
pub struct TXRETRYR {
    bits: u8,
}
impl TXRETRYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GUARDTIMER {
    bits: u8,
}
impl GUARDTIMER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `SCIEN`"]
pub enum SCIENW {
    #[doc = "Disabled. Smart card interface disabled."]
    DISABLED,
    #[doc = "Enabled. synchronous half duplex smart card interface is enabled."]
    ENABLED,
}
impl SCIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCIENW::DISABLED => false,
            SCIENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCIENW<'a> {
    w: &'a mut W,
}
impl<'a> _SCIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Smart card interface disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SCIENW::DISABLED)
    }
    #[doc = "Enabled. synchronous half duplex smart card interface is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SCIENW::ENABLED)
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
#[doc = "Values that can be written to the field `NACKDIS`"]
pub enum NACKDISW {
    #[doc = "Enabled. A NACK response is enabled."]
    ENABLED,
    #[doc = "Disabled. A NACK response is inhibited."]
    DISABLED,
}
impl NACKDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NACKDISW::ENABLED => false,
            NACKDISW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NACKDISW<'a> {
    w: &'a mut W,
}
impl<'a> _NACKDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NACKDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled. A NACK response is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NACKDISW::ENABLED)
    }
    #[doc = "Disabled. A NACK response is inhibited."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NACKDISW::DISABLED)
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
#[doc = "Values that can be written to the field `PROTSEL`"]
pub enum PROTSELW {
    #[doc = "T = 0"]
    T_EQ_0,
    #[doc = "T = 1"]
    T_EQ_1,
}
impl PROTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PROTSELW::T_EQ_0 => false,
            PROTSELW::T_EQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PROTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PROTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PROTSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "T = 0"]
    #[inline]
    pub fn t_eq_0(self) -> &'a mut W {
        self.variant(PROTSELW::T_EQ_0)
    }
    #[doc = "T = 1"]
    #[inline]
    pub fn t_eq_1(self) -> &'a mut W {
        self.variant(PROTSELW::T_EQ_1)
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
#[doc = r" Proxy"]
pub struct _TXRETRYW<'a> {
    w: &'a mut W,
}
impl<'a> _TXRETRYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GUARDTIMEW<'a> {
    w: &'a mut W,
}
impl<'a> _GUARDTIMEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bit 0 - Smart Card Interface Enable."]
    #[inline]
    pub fn scien(&self) -> SCIENR {
        SCIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - NACK response disable. Only applicable in T=0."]
    #[inline]
    pub fn nackdis(&self) -> NACKDISR {
        NACKDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Protocol selection as defined in the ISO7816-3 standard."]
    #[inline]
    pub fn protsel(&self) -> PROTSELR {
        PROTSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 5:7 - Maximum number of retransmissions in case of a negative acknowledge (protocol T=0). When the retry counter is exceeded, the USART will be locked until the FIFO is cleared. A TX error interrupt is generated when enabled."]
    #[inline]
    pub fn txretry(&self) -> TXRETRYR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXRETRYR { bits }
    }
    #[doc = "Bits 8:15 - Extra guard time. No extra guard time (0x0) results in a standard guard time as defined in ISO 7816-3, depending on the protocol type. A guard time of 0xFF indicates a minimal guard time as defined for the selected protocol."]
    #[inline]
    pub fn guardtime(&self) -> GUARDTIMER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GUARDTIMER { bits }
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
    #[doc = "Bit 0 - Smart Card Interface Enable."]
    #[inline]
    pub fn scien(&mut self) -> _SCIENW {
        _SCIENW { w: self }
    }
    #[doc = "Bit 1 - NACK response disable. Only applicable in T=0."]
    #[inline]
    pub fn nackdis(&mut self) -> _NACKDISW {
        _NACKDISW { w: self }
    }
    #[doc = "Bit 2 - Protocol selection as defined in the ISO7816-3 standard."]
    #[inline]
    pub fn protsel(&mut self) -> _PROTSELW {
        _PROTSELW { w: self }
    }
    #[doc = "Bits 5:7 - Maximum number of retransmissions in case of a negative acknowledge (protocol T=0). When the retry counter is exceeded, the USART will be locked until the FIFO is cleared. A TX error interrupt is generated when enabled."]
    #[inline]
    pub fn txretry(&mut self) -> _TXRETRYW {
        _TXRETRYW { w: self }
    }
    #[doc = "Bits 8:15 - Extra guard time. No extra guard time (0x0) results in a standard guard time as defined in ISO 7816-3, depending on the protocol type. A guard time of 0xFF indicates a minimal guard time as defined for the selected protocol."]
    #[inline]
    pub fn guardtime(&mut self) -> _GUARDTIMEW {
        _GUARDTIMEW { w: self }
    }
}
