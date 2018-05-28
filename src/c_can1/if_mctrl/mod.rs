#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IF_MCTRL {
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
pub struct DLC3_0R {
    bits: u8,
}
impl DLC3_0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `EOB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOBR {
    #[doc = "Single message object or last message object of a FIFO buffer."]
    SINGLE_MESSAGE_OBJEC,
    #[doc = "Message object belongs to a FIFO buffer and is not the last message object of that FIFO buffer."]
    MESSAGE_OBJECT_BELON,
}
impl EOBR {
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
            EOBR::SINGLE_MESSAGE_OBJEC => true,
            EOBR::MESSAGE_OBJECT_BELON => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EOBR {
        match value {
            true => EOBR::SINGLE_MESSAGE_OBJEC,
            false => EOBR::MESSAGE_OBJECT_BELON,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_MESSAGE_OBJEC`"]
    #[inline]
    pub fn is_single_message_objec(&self) -> bool {
        *self == EOBR::SINGLE_MESSAGE_OBJEC
    }
    #[doc = "Checks if the value of the field is `MESSAGE_OBJECT_BELON`"]
    #[inline]
    pub fn is_message_object_belon(&self) -> bool {
        *self == EOBR::MESSAGE_OBJECT_BELON
    }
}
#[doc = "Possible values of the field `TXRQST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRQSTR {
    #[doc = "The transmission of this message object is requested and is not yet done"]
    REQUEST,
    #[doc = "This message object is not waiting for transmission."]
    WAIT,
}
impl TXRQSTR {
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
            TXRQSTR::REQUEST => true,
            TXRQSTR::WAIT => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXRQSTR {
        match value {
            true => TXRQSTR::REQUEST,
            false => TXRQSTR::WAIT,
        }
    }
    #[doc = "Checks if the value of the field is `REQUEST`"]
    #[inline]
    pub fn is_request(&self) -> bool {
        *self == TXRQSTR::REQUEST
    }
    #[doc = "Checks if the value of the field is `WAIT`"]
    #[inline]
    pub fn is_wait(&self) -> bool {
        *self == TXRQSTR::WAIT
    }
}
#[doc = "Possible values of the field `RMTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMTENR {
    #[doc = "At the reception of a remote frame, TXRQST is set."]
    TXRQSTSET,
    #[doc = "At the reception of a remote frame, TXRQST is left unchanged."]
    UNCHANGED,
}
impl RMTENR {
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
            RMTENR::TXRQSTSET => true,
            RMTENR::UNCHANGED => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RMTENR {
        match value {
            true => RMTENR::TXRQSTSET,
            false => RMTENR::UNCHANGED,
        }
    }
    #[doc = "Checks if the value of the field is `TXRQSTSET`"]
    #[inline]
    pub fn is_txrqstset(&self) -> bool {
        *self == RMTENR::TXRQSTSET
    }
    #[doc = "Checks if the value of the field is `UNCHANGED`"]
    #[inline]
    pub fn is_unchanged(&self) -> bool {
        *self == RMTENR::UNCHANGED
    }
}
#[doc = "Possible values of the field `RXIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIER {
    #[doc = "INTPND will be set after successful reception of a frame."]
    INTPNDSET,
    #[doc = "INTPND will be left unchanged after successful reception of a frame."]
    UNCHANGED,
}
impl RXIER {
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
            RXIER::INTPNDSET => true,
            RXIER::UNCHANGED => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXIER {
        match value {
            true => RXIER::INTPNDSET,
            false => RXIER::UNCHANGED,
        }
    }
    #[doc = "Checks if the value of the field is `INTPNDSET`"]
    #[inline]
    pub fn is_intpndset(&self) -> bool {
        *self == RXIER::INTPNDSET
    }
    #[doc = "Checks if the value of the field is `UNCHANGED`"]
    #[inline]
    pub fn is_unchanged(&self) -> bool {
        *self == RXIER::UNCHANGED
    }
}
#[doc = "Possible values of the field `TXIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXIER {
    #[doc = "INTPND will be set after a successful reception of a frame."]
    INTPNDSET,
    #[doc = "The INTPND bit will be left unchanged after a successful reception of a frame."]
    UNCHANGED,
}
impl TXIER {
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
            TXIER::INTPNDSET => true,
            TXIER::UNCHANGED => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXIER {
        match value {
            true => TXIER::INTPNDSET,
            false => TXIER::UNCHANGED,
        }
    }
    #[doc = "Checks if the value of the field is `INTPNDSET`"]
    #[inline]
    pub fn is_intpndset(&self) -> bool {
        *self == TXIER::INTPNDSET
    }
    #[doc = "Checks if the value of the field is `UNCHANGED`"]
    #[inline]
    pub fn is_unchanged(&self) -> bool {
        *self == TXIER::UNCHANGED
    }
}
#[doc = "Possible values of the field `UMASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UMASKR {
    #[doc = "Use mask (MSK[28:0], MXTD, and MDIR) for acceptance filtering."]
    USE_MASK,
    #[doc = "Mask ignored."]
    MASK_IGNORED,
}
impl UMASKR {
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
            UMASKR::USE_MASK => true,
            UMASKR::MASK_IGNORED => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UMASKR {
        match value {
            true => UMASKR::USE_MASK,
            false => UMASKR::MASK_IGNORED,
        }
    }
    #[doc = "Checks if the value of the field is `USE_MASK`"]
    #[inline]
    pub fn is_use_mask(&self) -> bool {
        *self == UMASKR::USE_MASK
    }
    #[doc = "Checks if the value of the field is `MASK_IGNORED`"]
    #[inline]
    pub fn is_mask_ignored(&self) -> bool {
        *self == UMASKR::MASK_IGNORED
    }
}
#[doc = "Possible values of the field `INTPND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTPNDR {
    #[doc = "This message object is the source of an interrupt. The Interrupt Identifier in the Interrupt Register will point to this message object if there is no other interrupt source with higher priority."]
    INTSOURCE,
    #[doc = "This message object is not the source of an interrupt."]
    NOINTSOURCE,
}
impl INTPNDR {
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
            INTPNDR::INTSOURCE => true,
            INTPNDR::NOINTSOURCE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INTPNDR {
        match value {
            true => INTPNDR::INTSOURCE,
            false => INTPNDR::NOINTSOURCE,
        }
    }
    #[doc = "Checks if the value of the field is `INTSOURCE`"]
    #[inline]
    pub fn is_intsource(&self) -> bool {
        *self == INTPNDR::INTSOURCE
    }
    #[doc = "Checks if the value of the field is `NOINTSOURCE`"]
    #[inline]
    pub fn is_nointsource(&self) -> bool {
        *self == INTPNDR::NOINTSOURCE
    }
}
#[doc = "Possible values of the field `MSGLST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSGLSTR {
    #[doc = "The Message Handler stored a new message into this object when NEWDAT was still set, the CPU has lost a message."]
    THE_MESSAGE_HANDLER,
    #[doc = "No message lost since this bit was reset last by the CPU."]
    NO_MESSAGE_LOST_SINC,
}
impl MSGLSTR {
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
            MSGLSTR::THE_MESSAGE_HANDLER => true,
            MSGLSTR::NO_MESSAGE_LOST_SINC => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSGLSTR {
        match value {
            true => MSGLSTR::THE_MESSAGE_HANDLER,
            false => MSGLSTR::NO_MESSAGE_LOST_SINC,
        }
    }
    #[doc = "Checks if the value of the field is `THE_MESSAGE_HANDLER`"]
    #[inline]
    pub fn is_the_message_handler(&self) -> bool {
        *self == MSGLSTR::THE_MESSAGE_HANDLER
    }
    #[doc = "Checks if the value of the field is `NO_MESSAGE_LOST_SINC`"]
    #[inline]
    pub fn is_no_message_lost_sinc(&self) -> bool {
        *self == MSGLSTR::NO_MESSAGE_LOST_SINC
    }
}
#[doc = "Possible values of the field `NEWDAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEWDATR {
    #[doc = "The message handler or the CPU has written new data into the data portion of this message object."]
    THE_MESSAGE_HANDLER,
    #[doc = "No new data has been written into the data portion of this message object by the message handler since this flag was cleared last by the CPU."]
    NO_NEW_DATA_HAS_BEEN,
}
impl NEWDATR {
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
            NEWDATR::THE_MESSAGE_HANDLER => true,
            NEWDATR::NO_NEW_DATA_HAS_BEEN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NEWDATR {
        match value {
            true => NEWDATR::THE_MESSAGE_HANDLER,
            false => NEWDATR::NO_NEW_DATA_HAS_BEEN,
        }
    }
    #[doc = "Checks if the value of the field is `THE_MESSAGE_HANDLER`"]
    #[inline]
    pub fn is_the_message_handler(&self) -> bool {
        *self == NEWDATR::THE_MESSAGE_HANDLER
    }
    #[doc = "Checks if the value of the field is `NO_NEW_DATA_HAS_BEEN`"]
    #[inline]
    pub fn is_no_new_data_has_been(&self) -> bool {
        *self == NEWDATR::NO_NEW_DATA_HAS_BEEN
    }
}
#[doc = r" Proxy"]
pub struct _DLC3_0W<'a> {
    w: &'a mut W,
}
impl<'a> _DLC3_0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EOB`"]
pub enum EOBW {
    #[doc = "Single message object or last message object of a FIFO buffer."]
    SINGLE_MESSAGE_OBJEC,
    #[doc = "Message object belongs to a FIFO buffer and is not the last message object of that FIFO buffer."]
    MESSAGE_OBJECT_BELON,
}
impl EOBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EOBW::SINGLE_MESSAGE_OBJEC => true,
            EOBW::MESSAGE_OBJECT_BELON => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EOBW<'a> {
    w: &'a mut W,
}
impl<'a> _EOBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EOBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Single message object or last message object of a FIFO buffer."]
    #[inline]
    pub fn single_message_objec(self) -> &'a mut W {
        self.variant(EOBW::SINGLE_MESSAGE_OBJEC)
    }
    #[doc = "Message object belongs to a FIFO buffer and is not the last message object of that FIFO buffer."]
    #[inline]
    pub fn message_object_belon(self) -> &'a mut W {
        self.variant(EOBW::MESSAGE_OBJECT_BELON)
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
#[doc = "Values that can be written to the field `TXRQST`"]
pub enum TXRQSTW {
    #[doc = "The transmission of this message object is requested and is not yet done"]
    REQUEST,
    #[doc = "This message object is not waiting for transmission."]
    WAIT,
}
impl TXRQSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXRQSTW::REQUEST => true,
            TXRQSTW::WAIT => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXRQSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TXRQSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXRQSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The transmission of this message object is requested and is not yet done"]
    #[inline]
    pub fn request(self) -> &'a mut W {
        self.variant(TXRQSTW::REQUEST)
    }
    #[doc = "This message object is not waiting for transmission."]
    #[inline]
    pub fn wait(self) -> &'a mut W {
        self.variant(TXRQSTW::WAIT)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RMTEN`"]
pub enum RMTENW {
    #[doc = "At the reception of a remote frame, TXRQST is set."]
    TXRQSTSET,
    #[doc = "At the reception of a remote frame, TXRQST is left unchanged."]
    UNCHANGED,
}
impl RMTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RMTENW::TXRQSTSET => true,
            RMTENW::UNCHANGED => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RMTENW<'a> {
    w: &'a mut W,
}
impl<'a> _RMTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RMTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "At the reception of a remote frame, TXRQST is set."]
    #[inline]
    pub fn txrqstset(self) -> &'a mut W {
        self.variant(RMTENW::TXRQSTSET)
    }
    #[doc = "At the reception of a remote frame, TXRQST is left unchanged."]
    #[inline]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(RMTENW::UNCHANGED)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXIE`"]
pub enum RXIEW {
    #[doc = "INTPND will be set after successful reception of a frame."]
    INTPNDSET,
    #[doc = "INTPND will be left unchanged after successful reception of a frame."]
    UNCHANGED,
}
impl RXIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXIEW::INTPNDSET => true,
            RXIEW::UNCHANGED => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "INTPND will be set after successful reception of a frame."]
    #[inline]
    pub fn intpndset(self) -> &'a mut W {
        self.variant(RXIEW::INTPNDSET)
    }
    #[doc = "INTPND will be left unchanged after successful reception of a frame."]
    #[inline]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(RXIEW::UNCHANGED)
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
#[doc = "Values that can be written to the field `TXIE`"]
pub enum TXIEW {
    #[doc = "INTPND will be set after a successful reception of a frame."]
    INTPNDSET,
    #[doc = "The INTPND bit will be left unchanged after a successful reception of a frame."]
    UNCHANGED,
}
impl TXIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXIEW::INTPNDSET => true,
            TXIEW::UNCHANGED => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "INTPND will be set after a successful reception of a frame."]
    #[inline]
    pub fn intpndset(self) -> &'a mut W {
        self.variant(TXIEW::INTPNDSET)
    }
    #[doc = "The INTPND bit will be left unchanged after a successful reception of a frame."]
    #[inline]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(TXIEW::UNCHANGED)
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
#[doc = "Values that can be written to the field `UMASK`"]
pub enum UMASKW {
    #[doc = "Use mask (MSK[28:0], MXTD, and MDIR) for acceptance filtering."]
    USE_MASK,
    #[doc = "Mask ignored."]
    MASK_IGNORED,
}
impl UMASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UMASKW::USE_MASK => true,
            UMASKW::MASK_IGNORED => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UMASKW<'a> {
    w: &'a mut W,
}
impl<'a> _UMASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UMASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use mask (MSK[28:0], MXTD, and MDIR) for acceptance filtering."]
    #[inline]
    pub fn use_mask(self) -> &'a mut W {
        self.variant(UMASKW::USE_MASK)
    }
    #[doc = "Mask ignored."]
    #[inline]
    pub fn mask_ignored(self) -> &'a mut W {
        self.variant(UMASKW::MASK_IGNORED)
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
#[doc = "Values that can be written to the field `INTPND`"]
pub enum INTPNDW {
    #[doc = "This message object is the source of an interrupt. The Interrupt Identifier in the Interrupt Register will point to this message object if there is no other interrupt source with higher priority."]
    INTSOURCE,
    #[doc = "This message object is not the source of an interrupt."]
    NOINTSOURCE,
}
impl INTPNDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INTPNDW::INTSOURCE => true,
            INTPNDW::NOINTSOURCE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INTPNDW<'a> {
    w: &'a mut W,
}
impl<'a> _INTPNDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INTPNDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This message object is the source of an interrupt. The Interrupt Identifier in the Interrupt Register will point to this message object if there is no other interrupt source with higher priority."]
    #[inline]
    pub fn intsource(self) -> &'a mut W {
        self.variant(INTPNDW::INTSOURCE)
    }
    #[doc = "This message object is not the source of an interrupt."]
    #[inline]
    pub fn nointsource(self) -> &'a mut W {
        self.variant(INTPNDW::NOINTSOURCE)
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
#[doc = "Values that can be written to the field `MSGLST`"]
pub enum MSGLSTW {
    #[doc = "The Message Handler stored a new message into this object when NEWDAT was still set, the CPU has lost a message."]
    THE_MESSAGE_HANDLER,
    #[doc = "No message lost since this bit was reset last by the CPU."]
    NO_MESSAGE_LOST_SINC,
}
impl MSGLSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSGLSTW::THE_MESSAGE_HANDLER => true,
            MSGLSTW::NO_MESSAGE_LOST_SINC => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSGLSTW<'a> {
    w: &'a mut W,
}
impl<'a> _MSGLSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSGLSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The Message Handler stored a new message into this object when NEWDAT was still set, the CPU has lost a message."]
    #[inline]
    pub fn the_message_handler(self) -> &'a mut W {
        self.variant(MSGLSTW::THE_MESSAGE_HANDLER)
    }
    #[doc = "No message lost since this bit was reset last by the CPU."]
    #[inline]
    pub fn no_message_lost_sinc(self) -> &'a mut W {
        self.variant(MSGLSTW::NO_MESSAGE_LOST_SINC)
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
#[doc = "Values that can be written to the field `NEWDAT`"]
pub enum NEWDATW {
    #[doc = "The message handler or the CPU has written new data into the data portion of this message object."]
    THE_MESSAGE_HANDLER,
    #[doc = "No new data has been written into the data portion of this message object by the message handler since this flag was cleared last by the CPU."]
    NO_NEW_DATA_HAS_BEEN,
}
impl NEWDATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NEWDATW::THE_MESSAGE_HANDLER => true,
            NEWDATW::NO_NEW_DATA_HAS_BEEN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NEWDATW<'a> {
    w: &'a mut W,
}
impl<'a> _NEWDATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NEWDATW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The message handler or the CPU has written new data into the data portion of this message object."]
    #[inline]
    pub fn the_message_handler(self) -> &'a mut W {
        self.variant(NEWDATW::THE_MESSAGE_HANDLER)
    }
    #[doc = "No new data has been written into the data portion of this message object by the message handler since this flag was cleared last by the CPU."]
    #[inline]
    pub fn no_new_data_has_been(self) -> &'a mut W {
        self.variant(NEWDATW::NO_NEW_DATA_HAS_BEEN)
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
    #[doc = "Bits 0:3 - Data length code The Data Length Code of a Message Object must be defined the same as in all the corresponding objects with the same identifier at other nodes. When the Message Handler stores a data frame, it will write the DLC to the value given by the received message. 0000 to 1000 = Data frame has 0 - 8 data bytes. 1001 to 1111 = Data frame has 8 data bytes."]
    #[inline]
    pub fn dlc3_0(&self) -> DLC3_0R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLC3_0R { bits }
    }
    #[doc = "Bit 7 - End of buffer"]
    #[inline]
    pub fn eob(&self) -> EOBR {
        EOBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Transmit request"]
    #[inline]
    pub fn txrqst(&self) -> TXRQSTR {
        TXRQSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Remote enable"]
    #[inline]
    pub fn rmten(&self) -> RMTENR {
        RMTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Receive interrupt enable"]
    #[inline]
    pub fn rxie(&self) -> RXIER {
        RXIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Transmit interrupt enable"]
    #[inline]
    pub fn txie(&self) -> TXIER {
        TXIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Use acceptance mask If UMASK is set to 1, the message object's mask bits have to be programmed during initialization of the message object before MAGVAL is set to 1."]
    #[inline]
    pub fn umask(&self) -> UMASKR {
        UMASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Interrupt pending"]
    #[inline]
    pub fn intpnd(&self) -> INTPNDR {
        INTPNDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Message lost (only valid for message objects in the direction receive)."]
    #[inline]
    pub fn msglst(&self) -> MSGLSTR {
        MSGLSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - New data"]
    #[inline]
    pub fn newdat(&self) -> NEWDATR {
        NEWDATR::_from({
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
    #[doc = "Bits 0:3 - Data length code The Data Length Code of a Message Object must be defined the same as in all the corresponding objects with the same identifier at other nodes. When the Message Handler stores a data frame, it will write the DLC to the value given by the received message. 0000 to 1000 = Data frame has 0 - 8 data bytes. 1001 to 1111 = Data frame has 8 data bytes."]
    #[inline]
    pub fn dlc3_0(&mut self) -> _DLC3_0W {
        _DLC3_0W { w: self }
    }
    #[doc = "Bit 7 - End of buffer"]
    #[inline]
    pub fn eob(&mut self) -> _EOBW {
        _EOBW { w: self }
    }
    #[doc = "Bit 8 - Transmit request"]
    #[inline]
    pub fn txrqst(&mut self) -> _TXRQSTW {
        _TXRQSTW { w: self }
    }
    #[doc = "Bit 9 - Remote enable"]
    #[inline]
    pub fn rmten(&mut self) -> _RMTENW {
        _RMTENW { w: self }
    }
    #[doc = "Bit 10 - Receive interrupt enable"]
    #[inline]
    pub fn rxie(&mut self) -> _RXIEW {
        _RXIEW { w: self }
    }
    #[doc = "Bit 11 - Transmit interrupt enable"]
    #[inline]
    pub fn txie(&mut self) -> _TXIEW {
        _TXIEW { w: self }
    }
    #[doc = "Bit 12 - Use acceptance mask If UMASK is set to 1, the message object's mask bits have to be programmed during initialization of the message object before MAGVAL is set to 1."]
    #[inline]
    pub fn umask(&mut self) -> _UMASKW {
        _UMASKW { w: self }
    }
    #[doc = "Bit 13 - Interrupt pending"]
    #[inline]
    pub fn intpnd(&mut self) -> _INTPNDW {
        _INTPNDW { w: self }
    }
    #[doc = "Bit 14 - Message lost (only valid for message objects in the direction receive)."]
    #[inline]
    pub fn msglst(&mut self) -> _MSGLSTW {
        _MSGLSTW { w: self }
    }
    #[doc = "Bit 15 - New data"]
    #[inline]
    pub fn newdat(&mut self) -> _NEWDATW {
        _NEWDATW { w: self }
    }
}
