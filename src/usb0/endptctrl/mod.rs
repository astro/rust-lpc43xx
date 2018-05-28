#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ENDPTCTRL {
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
#[doc = "Possible values of the field `RXS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXSR {
    #[doc = "Endpoint ok. This bit will be cleared automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpoint and this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared."]
    ENDPOINT_OK_THIS_BI,
    #[doc = "Endpoint stalled Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. It will continue returning STALL until the bit is cleared by software, or it will automatically be cleared upon receipt of a new SETUP request."]
    ENDPOINT_STALLED_SOF,
}
impl RXSR {
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
            RXSR::ENDPOINT_OK_THIS_BI => false,
            RXSR::ENDPOINT_STALLED_SOF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXSR {
        match value {
            false => RXSR::ENDPOINT_OK_THIS_BI,
            true => RXSR::ENDPOINT_STALLED_SOF,
        }
    }
    #[doc = "Checks if the value of the field is `ENDPOINT_OK_THIS_BI`"]
    #[inline]
    pub fn is_endpoint_ok_this_bi(&self) -> bool {
        *self == RXSR::ENDPOINT_OK_THIS_BI
    }
    #[doc = "Checks if the value of the field is `ENDPOINT_STALLED_SOF`"]
    #[inline]
    pub fn is_endpoint_stalled_sof(&self) -> bool {
        *self == RXSR::ENDPOINT_STALLED_SOF
    }
}
#[doc = "Possible values of the field `RXT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTR {
    #[doc = "Control"]
    CONTROL,
    #[doc = "Isochronous"]
    ISOCHRONOUS,
    #[doc = "Bulk"]
    BULK,
}
impl RXTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXTR::CONTROL => 0,
            RXTR::ISOCHRONOUS => 1,
            RXTR::BULK => 2,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXTR {
        match value {
            0 => RXTR::CONTROL,
            1 => RXTR::ISOCHRONOUS,
            2 => RXTR::BULK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONTROL`"]
    #[inline]
    pub fn is_control(&self) -> bool {
        *self == RXTR::CONTROL
    }
    #[doc = "Checks if the value of the field is `ISOCHRONOUS`"]
    #[inline]
    pub fn is_isochronous(&self) -> bool {
        *self == RXTR::ISOCHRONOUS
    }
    #[doc = "Checks if the value of the field is `BULK`"]
    #[inline]
    pub fn is_bulk(&self) -> bool {
        *self == RXTR::BULK
    }
}
#[doc = "Possible values of the field `RXI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl RXIR {
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
            RXIR::DISABLED => false,
            RXIR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXIR {
        match value {
            false => RXIR::DISABLED,
            true => RXIR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RXIR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RXIR::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct RXRR {
    bits: bool,
}
impl RXRR {
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
#[doc = "Possible values of the field `RXE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXER {
    #[doc = "Endpoint disabled."]
    ENDPOINT_DISABLED,
    #[doc = "Endpoint enabled."]
    ENDPOINT_ENABLED,
}
impl RXER {
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
            RXER::ENDPOINT_DISABLED => false,
            RXER::ENDPOINT_ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXER {
        match value {
            false => RXER::ENDPOINT_DISABLED,
            true => RXER::ENDPOINT_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENDPOINT_DISABLED`"]
    #[inline]
    pub fn is_endpoint_disabled(&self) -> bool {
        *self == RXER::ENDPOINT_DISABLED
    }
    #[doc = "Checks if the value of the field is `ENDPOINT_ENABLED`"]
    #[inline]
    pub fn is_endpoint_enabled(&self) -> bool {
        *self == RXER::ENDPOINT_ENABLED
    }
}
#[doc = "Possible values of the field `TXS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSR {
    #[doc = "Endpoint ok. This bit will be cleared automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpoint, and this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared."]
    ENDPOINT_OK_THIS_BI,
    #[doc = "Endpoint stalled Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. It will continue returning STALL until the bit is cleared by software, or it will automatically be cleared upon receipt of a new SETUP request."]
    ENDPOINT_STALLED_SOF,
}
impl TXSR {
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
            TXSR::ENDPOINT_OK_THIS_BI => false,
            TXSR::ENDPOINT_STALLED_SOF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXSR {
        match value {
            false => TXSR::ENDPOINT_OK_THIS_BI,
            true => TXSR::ENDPOINT_STALLED_SOF,
        }
    }
    #[doc = "Checks if the value of the field is `ENDPOINT_OK_THIS_BI`"]
    #[inline]
    pub fn is_endpoint_ok_this_bi(&self) -> bool {
        *self == TXSR::ENDPOINT_OK_THIS_BI
    }
    #[doc = "Checks if the value of the field is `ENDPOINT_STALLED_SOF`"]
    #[inline]
    pub fn is_endpoint_stalled_sof(&self) -> bool {
        *self == TXSR::ENDPOINT_STALLED_SOF
    }
}
#[doc = "Possible values of the field `TXT1_0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXT1_0R {
    #[doc = "Control"]
    CONTROL,
    #[doc = "Isochronous"]
    ISOCHRONOUS,
    #[doc = "Bulk"]
    BULK,
    #[doc = "Interrupt"]
    INTERRUPT,
}
impl TXT1_0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXT1_0R::CONTROL => 0,
            TXT1_0R::ISOCHRONOUS => 1,
            TXT1_0R::BULK => 2,
            TXT1_0R::INTERRUPT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXT1_0R {
        match value {
            0 => TXT1_0R::CONTROL,
            1 => TXT1_0R::ISOCHRONOUS,
            2 => TXT1_0R::BULK,
            3 => TXT1_0R::INTERRUPT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONTROL`"]
    #[inline]
    pub fn is_control(&self) -> bool {
        *self == TXT1_0R::CONTROL
    }
    #[doc = "Checks if the value of the field is `ISOCHRONOUS`"]
    #[inline]
    pub fn is_isochronous(&self) -> bool {
        *self == TXT1_0R::ISOCHRONOUS
    }
    #[doc = "Checks if the value of the field is `BULK`"]
    #[inline]
    pub fn is_bulk(&self) -> bool {
        *self == TXT1_0R::BULK
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline]
    pub fn is_interrupt(&self) -> bool {
        *self == TXT1_0R::INTERRUPT
    }
}
#[doc = "Possible values of the field `TXI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXIR {
    #[doc = "Enabled"]
    ENABLED,
    #[doc = "Disabled"]
    DISABLED,
}
impl TXIR {
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
            TXIR::ENABLED => false,
            TXIR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXIR {
        match value {
            false => TXIR::ENABLED,
            true => TXIR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TXIR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TXIR::DISABLED
    }
}
#[doc = r" Value of the field"]
pub struct TXRR {
    bits: bool,
}
impl TXRR {
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
#[doc = "Possible values of the field `TXE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXER {
    #[doc = "Endpoint disabled."]
    ENDPOINT_DISABLED,
    #[doc = "Endpoint enabled."]
    ENDPOINT_ENABLED,
}
impl TXER {
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
            TXER::ENDPOINT_DISABLED => false,
            TXER::ENDPOINT_ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXER {
        match value {
            false => TXER::ENDPOINT_DISABLED,
            true => TXER::ENDPOINT_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENDPOINT_DISABLED`"]
    #[inline]
    pub fn is_endpoint_disabled(&self) -> bool {
        *self == TXER::ENDPOINT_DISABLED
    }
    #[doc = "Checks if the value of the field is `ENDPOINT_ENABLED`"]
    #[inline]
    pub fn is_endpoint_enabled(&self) -> bool {
        *self == TXER::ENDPOINT_ENABLED
    }
}
#[doc = "Values that can be written to the field `RXS`"]
pub enum RXSW {
    #[doc = "Endpoint ok. This bit will be cleared automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpoint and this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared."]
    ENDPOINT_OK_THIS_BI,
    #[doc = "Endpoint stalled Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. It will continue returning STALL until the bit is cleared by software, or it will automatically be cleared upon receipt of a new SETUP request."]
    ENDPOINT_STALLED_SOF,
}
impl RXSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXSW::ENDPOINT_OK_THIS_BI => false,
            RXSW::ENDPOINT_STALLED_SOF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXSW<'a> {
    w: &'a mut W,
}
impl<'a> _RXSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Endpoint ok. This bit will be cleared automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpoint and this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared."]
    #[inline]
    pub fn endpoint_ok_this_bi(self) -> &'a mut W {
        self.variant(RXSW::ENDPOINT_OK_THIS_BI)
    }
    #[doc = "Endpoint stalled Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. It will continue returning STALL until the bit is cleared by software, or it will automatically be cleared upon receipt of a new SETUP request."]
    #[inline]
    pub fn endpoint_stalled_sof(self) -> &'a mut W {
        self.variant(RXSW::ENDPOINT_STALLED_SOF)
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
#[doc = "Values that can be written to the field `RXT`"]
pub enum RXTW {
    #[doc = "Control"]
    CONTROL,
    #[doc = "Isochronous"]
    ISOCHRONOUS,
    #[doc = "Bulk"]
    BULK,
}
impl RXTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXTW::CONTROL => 0,
            RXTW::ISOCHRONOUS => 1,
            RXTW::BULK => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXTW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Control"]
    #[inline]
    pub fn control(self) -> &'a mut W {
        self.variant(RXTW::CONTROL)
    }
    #[doc = "Isochronous"]
    #[inline]
    pub fn isochronous(self) -> &'a mut W {
        self.variant(RXTW::ISOCHRONOUS)
    }
    #[doc = "Bulk"]
    #[inline]
    pub fn bulk(self) -> &'a mut W {
        self.variant(RXTW::BULK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXI`"]
pub enum RXIW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl RXIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXIW::DISABLED => false,
            RXIW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXIW<'a> {
    w: &'a mut W,
}
impl<'a> _RXIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXIW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXIW::ENABLED)
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
#[doc = r" Proxy"]
pub struct _RXRW<'a> {
    w: &'a mut W,
}
impl<'a> _RXRW<'a> {
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
#[doc = "Values that can be written to the field `RXE`"]
pub enum RXEW {
    #[doc = "Endpoint disabled."]
    ENDPOINT_DISABLED,
    #[doc = "Endpoint enabled."]
    ENDPOINT_ENABLED,
}
impl RXEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXEW::ENDPOINT_DISABLED => false,
            RXEW::ENDPOINT_ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Endpoint disabled."]
    #[inline]
    pub fn endpoint_disabled(self) -> &'a mut W {
        self.variant(RXEW::ENDPOINT_DISABLED)
    }
    #[doc = "Endpoint enabled."]
    #[inline]
    pub fn endpoint_enabled(self) -> &'a mut W {
        self.variant(RXEW::ENDPOINT_ENABLED)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXS`"]
pub enum TXSW {
    #[doc = "Endpoint ok. This bit will be cleared automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpoint, and this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared."]
    ENDPOINT_OK_THIS_BI,
    #[doc = "Endpoint stalled Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. It will continue returning STALL until the bit is cleared by software, or it will automatically be cleared upon receipt of a new SETUP request."]
    ENDPOINT_STALLED_SOF,
}
impl TXSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXSW::ENDPOINT_OK_THIS_BI => false,
            TXSW::ENDPOINT_STALLED_SOF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXSW<'a> {
    w: &'a mut W,
}
impl<'a> _TXSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Endpoint ok. This bit will be cleared automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpoint, and this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared."]
    #[inline]
    pub fn endpoint_ok_this_bi(self) -> &'a mut W {
        self.variant(TXSW::ENDPOINT_OK_THIS_BI)
    }
    #[doc = "Endpoint stalled Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. It will continue returning STALL until the bit is cleared by software, or it will automatically be cleared upon receipt of a new SETUP request."]
    #[inline]
    pub fn endpoint_stalled_sof(self) -> &'a mut W {
        self.variant(TXSW::ENDPOINT_STALLED_SOF)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXT1_0`"]
pub enum TXT1_0W {
    #[doc = "Control"]
    CONTROL,
    #[doc = "Isochronous"]
    ISOCHRONOUS,
    #[doc = "Bulk"]
    BULK,
    #[doc = "Interrupt"]
    INTERRUPT,
}
impl TXT1_0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TXT1_0W::CONTROL => 0,
            TXT1_0W::ISOCHRONOUS => 1,
            TXT1_0W::BULK => 2,
            TXT1_0W::INTERRUPT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXT1_0W<'a> {
    w: &'a mut W,
}
impl<'a> _TXT1_0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXT1_0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Control"]
    #[inline]
    pub fn control(self) -> &'a mut W {
        self.variant(TXT1_0W::CONTROL)
    }
    #[doc = "Isochronous"]
    #[inline]
    pub fn isochronous(self) -> &'a mut W {
        self.variant(TXT1_0W::ISOCHRONOUS)
    }
    #[doc = "Bulk"]
    #[inline]
    pub fn bulk(self) -> &'a mut W {
        self.variant(TXT1_0W::BULK)
    }
    #[doc = "Interrupt"]
    #[inline]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(TXT1_0W::INTERRUPT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXI`"]
pub enum TXIW {
    #[doc = "Enabled"]
    ENABLED,
    #[doc = "Disabled"]
    DISABLED,
}
impl TXIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXIW::ENABLED => false,
            TXIW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXIW<'a> {
    w: &'a mut W,
}
impl<'a> _TXIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXIW::ENABLED)
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXIW::DISABLED)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXRW<'a> {
    w: &'a mut W,
}
impl<'a> _TXRW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXE`"]
pub enum TXEW {
    #[doc = "Endpoint disabled."]
    ENDPOINT_DISABLED,
    #[doc = "Endpoint enabled."]
    ENDPOINT_ENABLED,
}
impl TXEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXEW::ENDPOINT_DISABLED => false,
            TXEW::ENDPOINT_ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Endpoint disabled."]
    #[inline]
    pub fn endpoint_disabled(self) -> &'a mut W {
        self.variant(TXEW::ENDPOINT_DISABLED)
    }
    #[doc = "Endpoint enabled."]
    #[inline]
    pub fn endpoint_enabled(self) -> &'a mut W {
        self.variant(TXEW::ENDPOINT_ENABLED)
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
        const OFFSET: u8 = 23;
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
    #[doc = "Bit 0 - Rx endpoint stall"]
    #[inline]
    pub fn rxs(&self) -> RXSR {
        RXSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:3 - Endpoint type"]
    #[inline]
    pub fn rxt(&self) -> RXTR {
        RXTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - Rx data toggle inhibit This bit is only used for test and should always be written as zero. Writing a one to this bit will cause this endpoint to ignore the data toggle sequence and always accept data packets regardless of their data PID."]
    #[inline]
    pub fn rxi(&self) -> RXIR {
        RXIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Rx data toggle reset Write 1 to reset the PID sequence. Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PIDs between the host and device."]
    #[inline]
    pub fn rxr(&self) -> RXRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXRR { bits }
    }
    #[doc = "Bit 7 - Rx endpoint enable An endpoint should be enabled only after it has been configured."]
    #[inline]
    pub fn rxe(&self) -> RXER {
        RXER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Tx endpoint stall"]
    #[inline]
    pub fn txs(&self) -> TXSR {
        TXSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 18:19 - Tx endpoint type"]
    #[inline]
    pub fn txt1_0(&self) -> TXT1_0R {
        TXT1_0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 21 - Tx data toggle inhibit This bit is only used for test and should always be written as zero. Writing a one to this bit will cause this endpoint to ignore the data toggle sequence and always accept data packets regardless of their data PID."]
    #[inline]
    pub fn txi(&self) -> TXIR {
        TXIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Tx data toggle reset Write 1 to reset the PID sequence. Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PIDs between the host and device."]
    #[inline]
    pub fn txr(&self) -> TXRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXRR { bits }
    }
    #[doc = "Bit 23 - Tx endpoint enable An endpoint should be enabled only after it has been configured"]
    #[inline]
    pub fn txe(&self) -> TXER {
        TXER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
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
    #[doc = "Bit 0 - Rx endpoint stall"]
    #[inline]
    pub fn rxs(&mut self) -> _RXSW {
        _RXSW { w: self }
    }
    #[doc = "Bits 2:3 - Endpoint type"]
    #[inline]
    pub fn rxt(&mut self) -> _RXTW {
        _RXTW { w: self }
    }
    #[doc = "Bit 5 - Rx data toggle inhibit This bit is only used for test and should always be written as zero. Writing a one to this bit will cause this endpoint to ignore the data toggle sequence and always accept data packets regardless of their data PID."]
    #[inline]
    pub fn rxi(&mut self) -> _RXIW {
        _RXIW { w: self }
    }
    #[doc = "Bit 6 - Rx data toggle reset Write 1 to reset the PID sequence. Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PIDs between the host and device."]
    #[inline]
    pub fn rxr(&mut self) -> _RXRW {
        _RXRW { w: self }
    }
    #[doc = "Bit 7 - Rx endpoint enable An endpoint should be enabled only after it has been configured."]
    #[inline]
    pub fn rxe(&mut self) -> _RXEW {
        _RXEW { w: self }
    }
    #[doc = "Bit 16 - Tx endpoint stall"]
    #[inline]
    pub fn txs(&mut self) -> _TXSW {
        _TXSW { w: self }
    }
    #[doc = "Bits 18:19 - Tx endpoint type"]
    #[inline]
    pub fn txt1_0(&mut self) -> _TXT1_0W {
        _TXT1_0W { w: self }
    }
    #[doc = "Bit 21 - Tx data toggle inhibit This bit is only used for test and should always be written as zero. Writing a one to this bit will cause this endpoint to ignore the data toggle sequence and always accept data packets regardless of their data PID."]
    #[inline]
    pub fn txi(&mut self) -> _TXIW {
        _TXIW { w: self }
    }
    #[doc = "Bit 22 - Tx data toggle reset Write 1 to reset the PID sequence. Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PIDs between the host and device."]
    #[inline]
    pub fn txr(&mut self) -> _TXRW {
        _TXRW { w: self }
    }
    #[doc = "Bit 23 - Tx endpoint enable An endpoint should be enabled only after it has been configured"]
    #[inline]
    pub fn txe(&mut self) -> _TXEW {
        _TXEW { w: self }
    }
}
