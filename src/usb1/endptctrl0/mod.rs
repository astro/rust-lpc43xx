#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ENDPTCTRL0 {
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
    #[doc = "Endpoint ok."]
    ENDPOINT_OK,
    #[doc = "Endpoint stalled Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. It will continue returning STALL until the bit is cleared by software, or it will automatically be cleared upon receipt of a new SETUP request. After receiving a SETUP request, this bit will continue to be cleared by hardware until the associated ENDSETUPSTAT bit is cleared.[1]"]
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
            RXSR::ENDPOINT_OK => false,
            RXSR::ENDPOINT_STALLED_SOF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXSR {
        match value {
            false => RXSR::ENDPOINT_OK,
            true => RXSR::ENDPOINT_STALLED_SOF,
        }
    }
    #[doc = "Checks if the value of the field is `ENDPOINT_OK`"]
    #[inline]
    pub fn is_endpoint_ok(&self) -> bool {
        *self == RXSR::ENDPOINT_OK
    }
    #[doc = "Checks if the value of the field is `ENDPOINT_STALLED_SOF`"]
    #[inline]
    pub fn is_endpoint_stalled_sof(&self) -> bool {
        *self == RXSR::ENDPOINT_STALLED_SOF
    }
}
#[doc = r" Value of the field"]
pub struct RXTR {
    bits: u8,
}
impl RXTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RXER {
    bits: bool,
}
impl RXER {
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
#[doc = "Possible values of the field `TXS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSR {
    #[doc = "Endpoint ok."]
    ENDPOINT_OK,
    #[doc = "Endpoint stalled Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. It will continue returning STALL until the bit is cleared by software, or it will automatically be cleared upon receipt of a new SETUP request. After receiving a SETUP request, this bit will continue to be cleared by hardware until the associated ENDSETUPSTAT bit is cleared.[1]"]
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
            TXSR::ENDPOINT_OK => false,
            TXSR::ENDPOINT_STALLED_SOF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXSR {
        match value {
            false => TXSR::ENDPOINT_OK,
            true => TXSR::ENDPOINT_STALLED_SOF,
        }
    }
    #[doc = "Checks if the value of the field is `ENDPOINT_OK`"]
    #[inline]
    pub fn is_endpoint_ok(&self) -> bool {
        *self == TXSR::ENDPOINT_OK
    }
    #[doc = "Checks if the value of the field is `ENDPOINT_STALLED_SOF`"]
    #[inline]
    pub fn is_endpoint_stalled_sof(&self) -> bool {
        *self == TXSR::ENDPOINT_STALLED_SOF
    }
}
#[doc = r" Value of the field"]
pub struct TXTR {
    bits: u8,
}
impl TXTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TXER {
    bits: bool,
}
impl TXER {
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
#[doc = "Values that can be written to the field `RXS`"]
pub enum RXSW {
    #[doc = "Endpoint ok."]
    ENDPOINT_OK,
    #[doc = "Endpoint stalled Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. It will continue returning STALL until the bit is cleared by software, or it will automatically be cleared upon receipt of a new SETUP request. After receiving a SETUP request, this bit will continue to be cleared by hardware until the associated ENDSETUPSTAT bit is cleared.[1]"]
    ENDPOINT_STALLED_SOF,
}
impl RXSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXSW::ENDPOINT_OK => false,
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
    #[doc = "Endpoint ok."]
    #[inline]
    pub fn endpoint_ok(self) -> &'a mut W {
        self.variant(RXSW::ENDPOINT_OK)
    }
    #[doc = "Endpoint stalled Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. It will continue returning STALL until the bit is cleared by software, or it will automatically be cleared upon receipt of a new SETUP request. After receiving a SETUP request, this bit will continue to be cleared by hardware until the associated ENDSETUPSTAT bit is cleared.[1]"]
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
#[doc = r" Proxy"]
pub struct _RXTW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTW<'a> {
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
#[doc = r" Proxy"]
pub struct _RXEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXEW<'a> {
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
    #[doc = "Endpoint ok."]
    ENDPOINT_OK,
    #[doc = "Endpoint stalled Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. It will continue returning STALL until the bit is cleared by software, or it will automatically be cleared upon receipt of a new SETUP request. After receiving a SETUP request, this bit will continue to be cleared by hardware until the associated ENDSETUPSTAT bit is cleared.[1]"]
    ENDPOINT_STALLED_SOF,
}
impl TXSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXSW::ENDPOINT_OK => false,
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
    #[doc = "Endpoint ok."]
    #[inline]
    pub fn endpoint_ok(self) -> &'a mut W {
        self.variant(TXSW::ENDPOINT_OK)
    }
    #[doc = "Endpoint stalled Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. It will continue returning STALL until the bit is cleared by software, or it will automatically be cleared upon receipt of a new SETUP request. After receiving a SETUP request, this bit will continue to be cleared by hardware until the associated ENDSETUPSTAT bit is cleared.[1]"]
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
#[doc = r" Proxy"]
pub struct _TXTW<'a> {
    w: &'a mut W,
}
impl<'a> _TXTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXEW<'a> {
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
    #[doc = "Bits 2:3 - Endpoint type Endpoint 0 is always a control endpoint."]
    #[inline]
    pub fn rxt(&self) -> RXTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RXTR { bits }
    }
    #[doc = "Bit 7 - Rx endpoint enable Endpoint enabled. Control endpoint 0 is always enabled. This bit is always 1."]
    #[inline]
    pub fn rxe(&self) -> RXER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXER { bits }
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
    #[doc = "Bits 18:19 - Endpoint type Endpoint 0 is always a control endpoint."]
    #[inline]
    pub fn txt(&self) -> TXTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXTR { bits }
    }
    #[doc = "Bit 23 - Tx endpoint enable Endpoint enabled. Control endpoint 0 is always enabled. This bit is always 1."]
    #[inline]
    pub fn txe(&self) -> TXER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXER { bits }
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
    #[doc = "Bits 2:3 - Endpoint type Endpoint 0 is always a control endpoint."]
    #[inline]
    pub fn rxt(&mut self) -> _RXTW {
        _RXTW { w: self }
    }
    #[doc = "Bit 7 - Rx endpoint enable Endpoint enabled. Control endpoint 0 is always enabled. This bit is always 1."]
    #[inline]
    pub fn rxe(&mut self) -> _RXEW {
        _RXEW { w: self }
    }
    #[doc = "Bit 16 - Tx endpoint stall"]
    #[inline]
    pub fn txs(&mut self) -> _TXSW {
        _TXSW { w: self }
    }
    #[doc = "Bits 18:19 - Endpoint type Endpoint 0 is always a control endpoint."]
    #[inline]
    pub fn txt(&mut self) -> _TXTW {
        _TXTW { w: self }
    }
    #[doc = "Bit 23 - Tx endpoint enable Endpoint enabled. Control endpoint 0 is always enabled. This bit is always 1."]
    #[inline]
    pub fn txe(&mut self) -> _TXEW {
        _TXEW { w: self }
    }
}
