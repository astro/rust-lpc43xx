#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IF_ARB2 {
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
pub struct ID28_16R {
    bits: u16,
}
impl ID28_16R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `DIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRR {
    #[doc = "Direction = transmit. On TXRQST, the respective Message Object is transmitted as a Data Frame. On reception of a Remote Frame with matching identifier, the TXRQST bit of this Message Object is set (if RMTEN = one)."]
    DIRECTION_EQ_TRANSMIT,
    #[doc = "Direction = receive. On TXRQST, a Remote Frame with the identifier of this Message Object is transmitted. On reception of a Data Frame with matching identifier, that message is stored in this Message Object."]
    DIRECTION_EQ_RECEIVE_,
}
impl DIRR {
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
            DIRR::DIRECTION_EQ_TRANSMIT => true,
            DIRR::DIRECTION_EQ_RECEIVE_ => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIRR {
        match value {
            true => DIRR::DIRECTION_EQ_TRANSMIT,
            false => DIRR::DIRECTION_EQ_RECEIVE_,
        }
    }
    #[doc = "Checks if the value of the field is `DIRECTION_EQ_TRANSMIT`"]
    #[inline]
    pub fn is_direction_eq_transmit(&self) -> bool {
        *self == DIRR::DIRECTION_EQ_TRANSMIT
    }
    #[doc = "Checks if the value of the field is `DIRECTION_EQ_RECEIVE_`"]
    #[inline]
    pub fn is_direction_eq_receive_(&self) -> bool {
        *self == DIRR::DIRECTION_EQ_RECEIVE_
    }
}
#[doc = "Possible values of the field `XTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTDR {
    #[doc = "The 29-bit extended identifier will be used for this message object."]
    THE_29_BIT_EXTENDED_,
    #[doc = "The 11-bit standard identifier will be used for this message object."]
    THE_11_BIT_STANDARD_,
}
impl XTDR {
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
            XTDR::THE_29_BIT_EXTENDED_ => true,
            XTDR::THE_11_BIT_STANDARD_ => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XTDR {
        match value {
            true => XTDR::THE_29_BIT_EXTENDED_,
            false => XTDR::THE_11_BIT_STANDARD_,
        }
    }
    #[doc = "Checks if the value of the field is `THE_29_BIT_EXTENDED_`"]
    #[inline]
    pub fn is_the_29_bit_extended_(&self) -> bool {
        *self == XTDR::THE_29_BIT_EXTENDED_
    }
    #[doc = "Checks if the value of the field is `THE_11_BIT_STANDARD_`"]
    #[inline]
    pub fn is_the_11_bit_standard_(&self) -> bool {
        *self == XTDR::THE_11_BIT_STANDARD_
    }
}
#[doc = "Possible values of the field `MSGVAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSGVALR {
    #[doc = "The message object is configured and should be considered by the message handler."]
    THE_MESSAGE_OBJECT_I,
    #[doc = "The message object is ignored by the message handler."]
    THE_MESSAGE_OBJECT_I,
}
impl MSGVALR {
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
            MSGVALR::THE_MESSAGE_OBJECT_I => true,
            MSGVALR::THE_MESSAGE_OBJECT_I => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSGVALR {
        match value {
            true => MSGVALR::THE_MESSAGE_OBJECT_I,
            false => MSGVALR::THE_MESSAGE_OBJECT_I,
        }
    }
    #[doc = "Checks if the value of the field is `THE_MESSAGE_OBJECT_I`"]
    #[inline]
    pub fn is_the_message_object_i(&self) -> bool {
        *self == MSGVALR::THE_MESSAGE_OBJECT_I
    }
    #[doc = "Checks if the value of the field is `THE_MESSAGE_OBJECT_I`"]
    #[inline]
    pub fn is_the_message_object_i(&self) -> bool {
        *self == MSGVALR::THE_MESSAGE_OBJECT_I
    }
}
#[doc = r" Proxy"]
pub struct _ID28_16W<'a> {
    w: &'a mut W,
}
impl<'a> _ID28_16W<'a> {
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
#[doc = "Values that can be written to the field `DIR`"]
pub enum DIRW {
    #[doc = "Direction = transmit. On TXRQST, the respective Message Object is transmitted as a Data Frame. On reception of a Remote Frame with matching identifier, the TXRQST bit of this Message Object is set (if RMTEN = one)."]
    DIRECTION_EQ_TRANSMIT,
    #[doc = "Direction = receive. On TXRQST, a Remote Frame with the identifier of this Message Object is transmitted. On reception of a Data Frame with matching identifier, that message is stored in this Message Object."]
    DIRECTION_EQ_RECEIVE_,
}
impl DIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIRW::DIRECTION_EQ_TRANSMIT => true,
            DIRW::DIRECTION_EQ_RECEIVE_ => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIRW<'a> {
    w: &'a mut W,
}
impl<'a> _DIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Direction = transmit. On TXRQST, the respective Message Object is transmitted as a Data Frame. On reception of a Remote Frame with matching identifier, the TXRQST bit of this Message Object is set (if RMTEN = one)."]
    #[inline]
    pub fn direction_eq_transmit(self) -> &'a mut W {
        self.variant(DIRW::DIRECTION_EQ_TRANSMIT)
    }
    #[doc = "Direction = receive. On TXRQST, a Remote Frame with the identifier of this Message Object is transmitted. On reception of a Data Frame with matching identifier, that message is stored in this Message Object."]
    #[inline]
    pub fn direction_eq_receive_(self) -> &'a mut W {
        self.variant(DIRW::DIRECTION_EQ_RECEIVE_)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `XTD`"]
pub enum XTDW {
    #[doc = "The 29-bit extended identifier will be used for this message object."]
    THE_29_BIT_EXTENDED_,
    #[doc = "The 11-bit standard identifier will be used for this message object."]
    THE_11_BIT_STANDARD_,
}
impl XTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            XTDW::THE_29_BIT_EXTENDED_ => true,
            XTDW::THE_11_BIT_STANDARD_ => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XTDW<'a> {
    w: &'a mut W,
}
impl<'a> _XTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The 29-bit extended identifier will be used for this message object."]
    #[inline]
    pub fn the_29_bit_extended_(self) -> &'a mut W {
        self.variant(XTDW::THE_29_BIT_EXTENDED_)
    }
    #[doc = "The 11-bit standard identifier will be used for this message object."]
    #[inline]
    pub fn the_11_bit_standard_(self) -> &'a mut W {
        self.variant(XTDW::THE_11_BIT_STANDARD_)
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
#[doc = "Values that can be written to the field `MSGVAL`"]
pub enum MSGVALW {
    #[doc = "The message object is configured and should be considered by the message handler."]
    THE_MESSAGE_OBJECT_I,
    #[doc = "The message object is ignored by the message handler."]
    THE_MESSAGE_OBJECT_I,
}
impl MSGVALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSGVALW::THE_MESSAGE_OBJECT_I => true,
            MSGVALW::THE_MESSAGE_OBJECT_I => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSGVALW<'a> {
    w: &'a mut W,
}
impl<'a> _MSGVALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSGVALW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The message object is configured and should be considered by the message handler."]
    #[inline]
    pub fn the_message_object_i(self) -> &'a mut W {
        self.variant(MSGVALW::THE_MESSAGE_OBJECT_I)
    }
    #[doc = "The message object is ignored by the message handler."]
    #[inline]
    pub fn the_message_object_i(self) -> &'a mut W {
        self.variant(MSGVALW::THE_MESSAGE_OBJECT_I)
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
    #[doc = "Bits 0:12 - Message identifier 29-bit identifier (extended frame) 11-bit identifier (standard frame)"]
    #[inline]
    pub fn id28_16(&self) -> ID28_16R {
        let bits = {
            const MASK: u16 = 8191;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        ID28_16R { bits }
    }
    #[doc = "Bit 13 - Message direction"]
    #[inline]
    pub fn dir(&self) -> DIRR {
        DIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Extend identifier"]
    #[inline]
    pub fn xtd(&self) -> XTDR {
        XTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Message valid The CPU must reset the MSGVAL bit of all unused Messages Objects during the initialization before it resets bit INIT in the CAN Control Register. This bit must also be reset before the identifier ID28:0, the control bits XTD, DIR, or the Data Length Code DLC3:0 are modified, or if the Messages Object is no longer required."]
    #[inline]
    pub fn msgval(&self) -> MSGVALR {
        MSGVALR::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:12 - Message identifier 29-bit identifier (extended frame) 11-bit identifier (standard frame)"]
    #[inline]
    pub fn id28_16(&mut self) -> _ID28_16W {
        _ID28_16W { w: self }
    }
    #[doc = "Bit 13 - Message direction"]
    #[inline]
    pub fn dir(&mut self) -> _DIRW {
        _DIRW { w: self }
    }
    #[doc = "Bit 14 - Extend identifier"]
    #[inline]
    pub fn xtd(&mut self) -> _XTDW {
        _XTDW { w: self }
    }
    #[doc = "Bit 15 - Message valid The CPU must reset the MSGVAL bit of all unused Messages Objects during the initialization before it resets bit INIT in the CAN Control Register. This bit must also be reset before the identifier ID28:0, the control bits XTD, DIR, or the Data Length Code DLC3:0 are modified, or if the Messages Object is no longer required."]
    #[inline]
    pub fn msgval(&mut self) -> _MSGVALW {
        _MSGVALW { w: self }
    }
}
