#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RS485CTRL {
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
#[doc = "Possible values of the field `NMMEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMMENR {
    #[doc = "Disabled. RS-485/EIA-485 Normal Multidrop Mode (NMM) is disabled."]
    DISABLED,
    #[doc = "Enabled. RS-485/EIA-485 Normal Multidrop Mode (NMM) is enabled. In this mode, an address is detected when a received byte causes the USART to set the parity error and generate an interrupt."]
    ENABLED,
}
impl NMMENR {
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
            NMMENR::DISABLED => false,
            NMMENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NMMENR {
        match value {
            false => NMMENR::DISABLED,
            true => NMMENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == NMMENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == NMMENR::ENABLED
    }
}
#[doc = "Possible values of the field `RXDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDISR {
    #[doc = "Enabled. The receiver is enabled."]
    ENABLED,
    #[doc = "Disabled.The receiver is disabled."]
    DISABLED,
}
impl RXDISR {
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
            RXDISR::ENABLED => false,
            RXDISR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXDISR {
        match value {
            false => RXDISR::ENABLED,
            true => RXDISR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RXDISR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RXDISR::DISABLED
    }
}
#[doc = "Possible values of the field `AADEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AADENR {
    #[doc = "Disabled. Auto Address Detect (AAD) is disabled."]
    DISABLED,
    #[doc = "Enabled. Auto Address Detect (AAD) is enabled."]
    ENABLED,
}
impl AADENR {
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
            AADENR::DISABLED => false,
            AADENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AADENR {
        match value {
            false => AADENR::DISABLED,
            true => AADENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == AADENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == AADENR::ENABLED
    }
}
#[doc = "Possible values of the field `DCTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCTRLR {
    #[doc = "Disabled. Disable Auto Direction Control."]
    DISABLED,
    #[doc = "Enabled. Enable Auto Direction Control."]
    ENABLED,
}
impl DCTRLR {
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
            DCTRLR::DISABLED => false,
            DCTRLR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCTRLR {
        match value {
            false => DCTRLR::DISABLED,
            true => DCTRLR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DCTRLR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DCTRLR::ENABLED
    }
}
#[doc = "Possible values of the field `OINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OINVR {
    #[doc = "Low. The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted."]
    LOW,
    #[doc = "High. The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted."]
    HIGH,
}
impl OINVR {
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
            OINVR::LOW => false,
            OINVR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OINVR {
        match value {
            false => OINVR::LOW,
            true => OINVR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == OINVR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == OINVR::HIGH
    }
}
#[doc = "Values that can be written to the field `NMMEN`"]
pub enum NMMENW {
    #[doc = "Disabled. RS-485/EIA-485 Normal Multidrop Mode (NMM) is disabled."]
    DISABLED,
    #[doc = "Enabled. RS-485/EIA-485 Normal Multidrop Mode (NMM) is enabled. In this mode, an address is detected when a received byte causes the USART to set the parity error and generate an interrupt."]
    ENABLED,
}
impl NMMENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NMMENW::DISABLED => false,
            NMMENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NMMENW<'a> {
    w: &'a mut W,
}
impl<'a> _NMMENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NMMENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. RS-485/EIA-485 Normal Multidrop Mode (NMM) is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NMMENW::DISABLED)
    }
    #[doc = "Enabled. RS-485/EIA-485 Normal Multidrop Mode (NMM) is enabled. In this mode, an address is detected when a received byte causes the USART to set the parity error and generate an interrupt."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NMMENW::ENABLED)
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
#[doc = "Values that can be written to the field `RXDIS`"]
pub enum RXDISW {
    #[doc = "Enabled. The receiver is enabled."]
    ENABLED,
    #[doc = "Disabled.The receiver is disabled."]
    DISABLED,
}
impl RXDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXDISW::ENABLED => false,
            RXDISW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXDISW<'a> {
    w: &'a mut W,
}
impl<'a> _RXDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled. The receiver is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXDISW::ENABLED)
    }
    #[doc = "Disabled.The receiver is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXDISW::DISABLED)
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
#[doc = "Values that can be written to the field `AADEN`"]
pub enum AADENW {
    #[doc = "Disabled. Auto Address Detect (AAD) is disabled."]
    DISABLED,
    #[doc = "Enabled. Auto Address Detect (AAD) is enabled."]
    ENABLED,
}
impl AADENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AADENW::DISABLED => false,
            AADENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AADENW<'a> {
    w: &'a mut W,
}
impl<'a> _AADENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AADENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Auto Address Detect (AAD) is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AADENW::DISABLED)
    }
    #[doc = "Enabled. Auto Address Detect (AAD) is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AADENW::ENABLED)
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
#[doc = "Values that can be written to the field `DCTRL`"]
pub enum DCTRLW {
    #[doc = "Disabled. Disable Auto Direction Control."]
    DISABLED,
    #[doc = "Enabled. Enable Auto Direction Control."]
    ENABLED,
}
impl DCTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DCTRLW::DISABLED => false,
            DCTRLW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _DCTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Disable Auto Direction Control."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DCTRLW::DISABLED)
    }
    #[doc = "Enabled. Enable Auto Direction Control."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DCTRLW::ENABLED)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OINV`"]
pub enum OINVW {
    #[doc = "Low. The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted."]
    LOW,
    #[doc = "High. The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted."]
    HIGH,
}
impl OINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OINVW::LOW => false,
            OINVW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OINVW<'a> {
    w: &'a mut W,
}
impl<'a> _OINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low. The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(OINVW::LOW)
    }
    #[doc = "High. The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(OINVW::HIGH)
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
        const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - NMM enable."]
    #[inline]
    pub fn nmmen(&self) -> NMMENR {
        NMMENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Receiver enable."]
    #[inline]
    pub fn rxdis(&self) -> RXDISR {
        RXDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - AAD enable"]
    #[inline]
    pub fn aaden(&self) -> AADENR {
        AADENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Direction control for DIR pin."]
    #[inline]
    pub fn dctrl(&self) -> DCTRLR {
        DCTRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Direction control pin polarity. This bit reverses the polarity of the direction control signal on the DIR pin."]
    #[inline]
    pub fn oinv(&self) -> OINVR {
        OINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - NMM enable."]
    #[inline]
    pub fn nmmen(&mut self) -> _NMMENW {
        _NMMENW { w: self }
    }
    #[doc = "Bit 1 - Receiver enable."]
    #[inline]
    pub fn rxdis(&mut self) -> _RXDISW {
        _RXDISW { w: self }
    }
    #[doc = "Bit 2 - AAD enable"]
    #[inline]
    pub fn aaden(&mut self) -> _AADENW {
        _AADENW { w: self }
    }
    #[doc = "Bit 4 - Direction control for DIR pin."]
    #[inline]
    pub fn dctrl(&mut self) -> _DCTRLW {
        _DCTRLW { w: self }
    }
    #[doc = "Bit 5 - Direction control pin polarity. This bit reverses the polarity of the direction control signal on the DIR pin."]
    #[inline]
    pub fn oinv(&mut self) -> _OINVW {
        _OINVW { w: self }
    }
}
