#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STAT {
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
#[doc = "Possible values of the field `LEC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LECR {
    #[doc = "No error."]
    NO_ERROR_,
    #[doc = "Stuff error: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed."]
    STUFF_ERROR_MORE_TH,
    #[doc = "Form error: A fixed format part of a received frame has the wrong format."]
    FORM_ERROR_A_FIXED_,
    #[doc = "AckError: The message this CAN core transmitted was not acknowledged."]
    ACKERROR_THE_MESSAG,
    #[doc = "Bit1Error: During the transmission of a message (with the exception of the arbitration field), the device wanted to send a HIGH/recessive level (bit of logical value 1), but the monitored bus value was LOW/dominant."]
    BIT1ERROR_DURING_TH,
    #[doc = "Bit0Error: During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a LOW/dominant level (data or identifier bit logical value 0), but the monitored Bus value was HIGH/recessive. During busoff recovery this status is set each time a sequence of 11 HIGH/recessive bits has been monitored. This enables the CPU to monitor the proceeding of the busoff recovery sequence (indicating the bus is not stuck at LOW/dominant or continuously disturbed)."]
    BIT0ERROR_DURING_TH,
    #[doc = "CRCError: The CRC checksum was incorrect in the message received."]
    CRCERROR_THE_CRC_CH,
    #[doc = "Unused: No CAN bus event was detected (written by the CPU)."]
    UNUSED_NO_CAN_BUS_E,
}
impl LECR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LECR::NO_ERROR_ => 0,
            LECR::STUFF_ERROR_MORE_TH => 1,
            LECR::FORM_ERROR_A_FIXED_ => 2,
            LECR::ACKERROR_THE_MESSAG => 3,
            LECR::BIT1ERROR_DURING_TH => 4,
            LECR::BIT0ERROR_DURING_TH => 5,
            LECR::CRCERROR_THE_CRC_CH => 6,
            LECR::UNUSED_NO_CAN_BUS_E => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LECR {
        match value {
            0 => LECR::NO_ERROR_,
            1 => LECR::STUFF_ERROR_MORE_TH,
            2 => LECR::FORM_ERROR_A_FIXED_,
            3 => LECR::ACKERROR_THE_MESSAG,
            4 => LECR::BIT1ERROR_DURING_TH,
            5 => LECR::BIT0ERROR_DURING_TH,
            6 => LECR::CRCERROR_THE_CRC_CH,
            7 => LECR::UNUSED_NO_CAN_BUS_E,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR_`"]
    #[inline]
    pub fn is_no_error_(&self) -> bool {
        *self == LECR::NO_ERROR_
    }
    #[doc = "Checks if the value of the field is `STUFF_ERROR_MORE_TH`"]
    #[inline]
    pub fn is_stuff_error_more_th(&self) -> bool {
        *self == LECR::STUFF_ERROR_MORE_TH
    }
    #[doc = "Checks if the value of the field is `FORM_ERROR_A_FIXED_`"]
    #[inline]
    pub fn is_form_error_a_fixed_(&self) -> bool {
        *self == LECR::FORM_ERROR_A_FIXED_
    }
    #[doc = "Checks if the value of the field is `ACKERROR_THE_MESSAG`"]
    #[inline]
    pub fn is_ackerror_the_messag(&self) -> bool {
        *self == LECR::ACKERROR_THE_MESSAG
    }
    #[doc = "Checks if the value of the field is `BIT1ERROR_DURING_TH`"]
    #[inline]
    pub fn is_bit1error_during_th(&self) -> bool {
        *self == LECR::BIT1ERROR_DURING_TH
    }
    #[doc = "Checks if the value of the field is `BIT0ERROR_DURING_TH`"]
    #[inline]
    pub fn is_bit0error_during_th(&self) -> bool {
        *self == LECR::BIT0ERROR_DURING_TH
    }
    #[doc = "Checks if the value of the field is `CRCERROR_THE_CRC_CH`"]
    #[inline]
    pub fn is_crcerror_the_crc_ch(&self) -> bool {
        *self == LECR::CRCERROR_THE_CRC_CH
    }
    #[doc = "Checks if the value of the field is `UNUSED_NO_CAN_BUS_E`"]
    #[inline]
    pub fn is_unused_no_can_bus_e(&self) -> bool {
        *self == LECR::UNUSED_NO_CAN_BUS_E
    }
}
#[doc = "Possible values of the field `TXOK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXOKR {
    #[doc = "Since this bit was last reset by the CPU, a message has been successfully transmitted (error free and acknowledged by at least one other node)."]
    MSGTRANSFER,
    #[doc = "Since this bit was reset by the CPU, no message has been successfully transmitted."]
    NOMSGTRANSFER,
}
impl TXOKR {
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
            TXOKR::MSGTRANSFER => true,
            TXOKR::NOMSGTRANSFER => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXOKR {
        match value {
            true => TXOKR::MSGTRANSFER,
            false => TXOKR::NOMSGTRANSFER,
        }
    }
    #[doc = "Checks if the value of the field is `MSGTRANSFER`"]
    #[inline]
    pub fn is_msgtransfer(&self) -> bool {
        *self == TXOKR::MSGTRANSFER
    }
    #[doc = "Checks if the value of the field is `NOMSGTRANSFER`"]
    #[inline]
    pub fn is_nomsgtransfer(&self) -> bool {
        *self == TXOKR::NOMSGTRANSFER
    }
}
#[doc = "Possible values of the field `RXOK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOKR {
    #[doc = "Since this bit was last set to zero by the CPU, a message has been successfully received independent of the result of acceptance filtering."]
    MSGTRANSFER,
    #[doc = "Since this bit was last reset by the CPU, no message has been successfully transmitted."]
    NOMSGTRANSFER,
}
impl RXOKR {
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
            RXOKR::MSGTRANSFER => true,
            RXOKR::NOMSGTRANSFER => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXOKR {
        match value {
            true => RXOKR::MSGTRANSFER,
            false => RXOKR::NOMSGTRANSFER,
        }
    }
    #[doc = "Checks if the value of the field is `MSGTRANSFER`"]
    #[inline]
    pub fn is_msgtransfer(&self) -> bool {
        *self == RXOKR::MSGTRANSFER
    }
    #[doc = "Checks if the value of the field is `NOMSGTRANSFER`"]
    #[inline]
    pub fn is_nomsgtransfer(&self) -> bool {
        *self == RXOKR::NOMSGTRANSFER
    }
}
#[doc = "Possible values of the field `EPASS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPASSR {
    #[doc = "The CAN controller is in the error passive state as defined in the  CAN 2.0 specification."]
    PASSIVE,
    #[doc = "The CAN controller is in the error active state."]
    ACTIVE,
}
impl EPASSR {
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
            EPASSR::PASSIVE => true,
            EPASSR::ACTIVE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPASSR {
        match value {
            true => EPASSR::PASSIVE,
            false => EPASSR::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `PASSIVE`"]
    #[inline]
    pub fn is_passive(&self) -> bool {
        *self == EPASSR::PASSIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline]
    pub fn is_active(&self) -> bool {
        *self == EPASSR::ACTIVE
    }
}
#[doc = "Possible values of the field `EWARN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWARNR {
    #[doc = "At least one of the error counters in the EML has reached the error warning limit of 96."]
    AT_LEAST_ONE_OF_THE_,
    #[doc = "Both error counters are below the error warning limit of 96."]
    BOTH_ERROR_COUNTERS_,
}
impl EWARNR {
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
            EWARNR::AT_LEAST_ONE_OF_THE_ => true,
            EWARNR::BOTH_ERROR_COUNTERS_ => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EWARNR {
        match value {
            true => EWARNR::AT_LEAST_ONE_OF_THE_,
            false => EWARNR::BOTH_ERROR_COUNTERS_,
        }
    }
    #[doc = "Checks if the value of the field is `AT_LEAST_ONE_OF_THE_`"]
    #[inline]
    pub fn is_at_least_one_of_the_(&self) -> bool {
        *self == EWARNR::AT_LEAST_ONE_OF_THE_
    }
    #[doc = "Checks if the value of the field is `BOTH_ERROR_COUNTERS_`"]
    #[inline]
    pub fn is_both_error_counters_(&self) -> bool {
        *self == EWARNR::BOTH_ERROR_COUNTERS_
    }
}
#[doc = "Possible values of the field `BOFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOFFR {
    #[doc = "The CAN controller is in busoff state."]
    BUSOFF,
    #[doc = "The CAN module is not in busoff."]
    NOBUSOFF,
}
impl BOFFR {
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
            BOFFR::BUSOFF => true,
            BOFFR::NOBUSOFF => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOFFR {
        match value {
            true => BOFFR::BUSOFF,
            false => BOFFR::NOBUSOFF,
        }
    }
    #[doc = "Checks if the value of the field is `BUSOFF`"]
    #[inline]
    pub fn is_busoff(&self) -> bool {
        *self == BOFFR::BUSOFF
    }
    #[doc = "Checks if the value of the field is `NOBUSOFF`"]
    #[inline]
    pub fn is_nobusoff(&self) -> bool {
        *self == BOFFR::NOBUSOFF
    }
}
#[doc = "Values that can be written to the field `LEC`"]
pub enum LECW {
    #[doc = "No error."]
    NO_ERROR_,
    #[doc = "Stuff error: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed."]
    STUFF_ERROR_MORE_TH,
    #[doc = "Form error: A fixed format part of a received frame has the wrong format."]
    FORM_ERROR_A_FIXED_,
    #[doc = "AckError: The message this CAN core transmitted was not acknowledged."]
    ACKERROR_THE_MESSAG,
    #[doc = "Bit1Error: During the transmission of a message (with the exception of the arbitration field), the device wanted to send a HIGH/recessive level (bit of logical value 1), but the monitored bus value was LOW/dominant."]
    BIT1ERROR_DURING_TH,
    #[doc = "Bit0Error: During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a LOW/dominant level (data or identifier bit logical value 0), but the monitored Bus value was HIGH/recessive. During busoff recovery this status is set each time a sequence of 11 HIGH/recessive bits has been monitored. This enables the CPU to monitor the proceeding of the busoff recovery sequence (indicating the bus is not stuck at LOW/dominant or continuously disturbed)."]
    BIT0ERROR_DURING_TH,
    #[doc = "CRCError: The CRC checksum was incorrect in the message received."]
    CRCERROR_THE_CRC_CH,
    #[doc = "Unused: No CAN bus event was detected (written by the CPU)."]
    UNUSED_NO_CAN_BUS_E,
}
impl LECW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LECW::NO_ERROR_ => 0,
            LECW::STUFF_ERROR_MORE_TH => 1,
            LECW::FORM_ERROR_A_FIXED_ => 2,
            LECW::ACKERROR_THE_MESSAG => 3,
            LECW::BIT1ERROR_DURING_TH => 4,
            LECW::BIT0ERROR_DURING_TH => 5,
            LECW::CRCERROR_THE_CRC_CH => 6,
            LECW::UNUSED_NO_CAN_BUS_E => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LECW<'a> {
    w: &'a mut W,
}
impl<'a> _LECW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LECW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No error."]
    #[inline]
    pub fn no_error_(self) -> &'a mut W {
        self.variant(LECW::NO_ERROR_)
    }
    #[doc = "Stuff error: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed."]
    #[inline]
    pub fn stuff_error_more_th(self) -> &'a mut W {
        self.variant(LECW::STUFF_ERROR_MORE_TH)
    }
    #[doc = "Form error: A fixed format part of a received frame has the wrong format."]
    #[inline]
    pub fn form_error_a_fixed_(self) -> &'a mut W {
        self.variant(LECW::FORM_ERROR_A_FIXED_)
    }
    #[doc = "AckError: The message this CAN core transmitted was not acknowledged."]
    #[inline]
    pub fn ackerror_the_messag(self) -> &'a mut W {
        self.variant(LECW::ACKERROR_THE_MESSAG)
    }
    #[doc = "Bit1Error: During the transmission of a message (with the exception of the arbitration field), the device wanted to send a HIGH/recessive level (bit of logical value 1), but the monitored bus value was LOW/dominant."]
    #[inline]
    pub fn bit1error_during_th(self) -> &'a mut W {
        self.variant(LECW::BIT1ERROR_DURING_TH)
    }
    #[doc = "Bit0Error: During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a LOW/dominant level (data or identifier bit logical value 0), but the monitored Bus value was HIGH/recessive. During busoff recovery this status is set each time a sequence of 11 HIGH/recessive bits has been monitored. This enables the CPU to monitor the proceeding of the busoff recovery sequence (indicating the bus is not stuck at LOW/dominant or continuously disturbed)."]
    #[inline]
    pub fn bit0error_during_th(self) -> &'a mut W {
        self.variant(LECW::BIT0ERROR_DURING_TH)
    }
    #[doc = "CRCError: The CRC checksum was incorrect in the message received."]
    #[inline]
    pub fn crcerror_the_crc_ch(self) -> &'a mut W {
        self.variant(LECW::CRCERROR_THE_CRC_CH)
    }
    #[doc = "Unused: No CAN bus event was detected (written by the CPU)."]
    #[inline]
    pub fn unused_no_can_bus_e(self) -> &'a mut W {
        self.variant(LECW::UNUSED_NO_CAN_BUS_E)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXOK`"]
pub enum TXOKW {
    #[doc = "Since this bit was last reset by the CPU, a message has been successfully transmitted (error free and acknowledged by at least one other node)."]
    MSGTRANSFER,
    #[doc = "Since this bit was reset by the CPU, no message has been successfully transmitted."]
    NOMSGTRANSFER,
}
impl TXOKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXOKW::MSGTRANSFER => true,
            TXOKW::NOMSGTRANSFER => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXOKW<'a> {
    w: &'a mut W,
}
impl<'a> _TXOKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXOKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Since this bit was last reset by the CPU, a message has been successfully transmitted (error free and acknowledged by at least one other node)."]
    #[inline]
    pub fn msgtransfer(self) -> &'a mut W {
        self.variant(TXOKW::MSGTRANSFER)
    }
    #[doc = "Since this bit was reset by the CPU, no message has been successfully transmitted."]
    #[inline]
    pub fn nomsgtransfer(self) -> &'a mut W {
        self.variant(TXOKW::NOMSGTRANSFER)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXOK`"]
pub enum RXOKW {
    #[doc = "Since this bit was last set to zero by the CPU, a message has been successfully received independent of the result of acceptance filtering."]
    MSGTRANSFER,
    #[doc = "Since this bit was last reset by the CPU, no message has been successfully transmitted."]
    NOMSGTRANSFER,
}
impl RXOKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXOKW::MSGTRANSFER => true,
            RXOKW::NOMSGTRANSFER => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXOKW<'a> {
    w: &'a mut W,
}
impl<'a> _RXOKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXOKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Since this bit was last set to zero by the CPU, a message has been successfully received independent of the result of acceptance filtering."]
    #[inline]
    pub fn msgtransfer(self) -> &'a mut W {
        self.variant(RXOKW::MSGTRANSFER)
    }
    #[doc = "Since this bit was last reset by the CPU, no message has been successfully transmitted."]
    #[inline]
    pub fn nomsgtransfer(self) -> &'a mut W {
        self.variant(RXOKW::NOMSGTRANSFER)
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
#[doc = "Values that can be written to the field `EPASS`"]
pub enum EPASSW {
    #[doc = "The CAN controller is in the error passive state as defined in the  CAN 2.0 specification."]
    PASSIVE,
    #[doc = "The CAN controller is in the error active state."]
    ACTIVE,
}
impl EPASSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPASSW::PASSIVE => true,
            EPASSW::ACTIVE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _EPASSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPASSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The CAN controller is in the error passive state as defined in the CAN 2.0 specification."]
    #[inline]
    pub fn passive(self) -> &'a mut W {
        self.variant(EPASSW::PASSIVE)
    }
    #[doc = "The CAN controller is in the error active state."]
    #[inline]
    pub fn active(self) -> &'a mut W {
        self.variant(EPASSW::ACTIVE)
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
#[doc = "Values that can be written to the field `EWARN`"]
pub enum EWARNW {
    #[doc = "At least one of the error counters in the EML has reached the error warning limit of 96."]
    AT_LEAST_ONE_OF_THE_,
    #[doc = "Both error counters are below the error warning limit of 96."]
    BOTH_ERROR_COUNTERS_,
}
impl EWARNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EWARNW::AT_LEAST_ONE_OF_THE_ => true,
            EWARNW::BOTH_ERROR_COUNTERS_ => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EWARNW<'a> {
    w: &'a mut W,
}
impl<'a> _EWARNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EWARNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "At least one of the error counters in the EML has reached the error warning limit of 96."]
    #[inline]
    pub fn at_least_one_of_the_(self) -> &'a mut W {
        self.variant(EWARNW::AT_LEAST_ONE_OF_THE_)
    }
    #[doc = "Both error counters are below the error warning limit of 96."]
    #[inline]
    pub fn both_error_counters_(self) -> &'a mut W {
        self.variant(EWARNW::BOTH_ERROR_COUNTERS_)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BOFF`"]
pub enum BOFFW {
    #[doc = "The CAN controller is in busoff state."]
    BUSOFF,
    #[doc = "The CAN module is not in busoff."]
    NOBUSOFF,
}
impl BOFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BOFFW::BUSOFF => true,
            BOFFW::NOBUSOFF => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOFFW<'a> {
    w: &'a mut W,
}
impl<'a> _BOFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The CAN controller is in busoff state."]
    #[inline]
    pub fn busoff(self) -> &'a mut W {
        self.variant(BOFFW::BUSOFF)
    }
    #[doc = "The CAN module is not in busoff."]
    #[inline]
    pub fn nobusoff(self) -> &'a mut W {
        self.variant(BOFFW::NOBUSOFF)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Last error code Type of the last error to occur on the CAN bus.The LEC field holds a code which indicates the type of the last error to occur on the CAN bus. This field will be cleared to 0 when a message has been transferred (reception or transmission) without error. The unused code 111 may be written by the CPU to check for updates."]
    #[inline]
    pub fn lec(&self) -> LECR {
        LECR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Transmitted a message successfully This bit is reset by the CPU. It is never reset by the CAN controller."]
    #[inline]
    pub fn txok(&self) -> TXOKR {
        TXOKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Received a message successfully This bit is reset by the CPU. It is never reset by the CAN controller."]
    #[inline]
    pub fn rxok(&self) -> RXOKR {
        RXOKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Error passive"]
    #[inline]
    pub fn epass(&self) -> EPASSR {
        EPASSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Warning status"]
    #[inline]
    pub fn ewarn(&self) -> EWARNR {
        EWARNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Busoff status"]
    #[inline]
    pub fn boff(&self) -> BOFFR {
        BOFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
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
    #[doc = "Bits 0:2 - Last error code Type of the last error to occur on the CAN bus.The LEC field holds a code which indicates the type of the last error to occur on the CAN bus. This field will be cleared to 0 when a message has been transferred (reception or transmission) without error. The unused code 111 may be written by the CPU to check for updates."]
    #[inline]
    pub fn lec(&mut self) -> _LECW {
        _LECW { w: self }
    }
    #[doc = "Bit 3 - Transmitted a message successfully This bit is reset by the CPU. It is never reset by the CAN controller."]
    #[inline]
    pub fn txok(&mut self) -> _TXOKW {
        _TXOKW { w: self }
    }
    #[doc = "Bit 4 - Received a message successfully This bit is reset by the CPU. It is never reset by the CAN controller."]
    #[inline]
    pub fn rxok(&mut self) -> _RXOKW {
        _RXOKW { w: self }
    }
    #[doc = "Bit 5 - Error passive"]
    #[inline]
    pub fn epass(&mut self) -> _EPASSW {
        _EPASSW { w: self }
    }
    #[doc = "Bit 6 - Warning status"]
    #[inline]
    pub fn ewarn(&mut self) -> _EWARNW {
        _EWARNW { w: self }
    }
    #[doc = "Bit 7 - Busoff status"]
    #[inline]
    pub fn boff(&mut self) -> _BOFFW {
        _BOFFW { w: self }
    }
}
