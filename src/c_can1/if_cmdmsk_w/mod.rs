#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IF_CMDMSK_W {
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
#[doc = "Possible values of the field `DATA_B`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_BR {
    #[doc = "Transfer data bytes 4-7 to message object."]
    TRANSFER_DATA_BYTES,
    #[doc = "data bytes 4-7 unchanged."]
    DATA_BYTES_4_7_UNCHA,
}
impl DATA_BR {
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
            DATA_BR::TRANSFER_DATA_BYTES => true,
            DATA_BR::DATA_BYTES_4_7_UNCHA => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATA_BR {
        match value {
            true => DATA_BR::TRANSFER_DATA_BYTES,
            false => DATA_BR::DATA_BYTES_4_7_UNCHA,
        }
    }
    #[doc = "Checks if the value of the field is `TRANSFER_DATA_BYTES`"]
    #[inline]
    pub fn is_transfer_data_bytes(&self) -> bool {
        *self == DATA_BR::TRANSFER_DATA_BYTES
    }
    #[doc = "Checks if the value of the field is `DATA_BYTES_4_7_UNCHA`"]
    #[inline]
    pub fn is_data_bytes_4_7_uncha(&self) -> bool {
        *self == DATA_BR::DATA_BYTES_4_7_UNCHA
    }
}
#[doc = "Possible values of the field `DATA_A`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_AR {
    #[doc = "Transfer data bytes 0-3 to message object."]
    TRANSFER_DATA_BYTES,
    #[doc = "data bytes 0-3 unchanged."]
    DATA_BYTES_0_3_UNCHA,
}
impl DATA_AR {
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
            DATA_AR::TRANSFER_DATA_BYTES => true,
            DATA_AR::DATA_BYTES_0_3_UNCHA => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATA_AR {
        match value {
            true => DATA_AR::TRANSFER_DATA_BYTES,
            false => DATA_AR::DATA_BYTES_0_3_UNCHA,
        }
    }
    #[doc = "Checks if the value of the field is `TRANSFER_DATA_BYTES`"]
    #[inline]
    pub fn is_transfer_data_bytes(&self) -> bool {
        *self == DATA_AR::TRANSFER_DATA_BYTES
    }
    #[doc = "Checks if the value of the field is `DATA_BYTES_0_3_UNCHA`"]
    #[inline]
    pub fn is_data_bytes_0_3_uncha(&self) -> bool {
        *self == DATA_AR::DATA_BYTES_0_3_UNCHA
    }
}
#[doc = "Possible values of the field `TXRQST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRQSTR {
    #[doc = "Request a transmission. Set the TXRQST bit IF1/2_MCTRL."]
    REQUEST_A_TRANSMISSI,
    #[doc = "No transmission request. TXRQSRT bit unchanged in IF1/2_MCTRL. If a transmission is requested by programming this bit, the TXRQST bit in the CANIFn_MCTRL register is ignored."]
    NO_TRANSMISSION_REQU,
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
            TXRQSTR::REQUEST_A_TRANSMISSI => true,
            TXRQSTR::NO_TRANSMISSION_REQU => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXRQSTR {
        match value {
            true => TXRQSTR::REQUEST_A_TRANSMISSI,
            false => TXRQSTR::NO_TRANSMISSION_REQU,
        }
    }
    #[doc = "Checks if the value of the field is `REQUEST_A_TRANSMISSI`"]
    #[inline]
    pub fn is_request_a_transmissi(&self) -> bool {
        *self == TXRQSTR::REQUEST_A_TRANSMISSI
    }
    #[doc = "Checks if the value of the field is `NO_TRANSMISSION_REQU`"]
    #[inline]
    pub fn is_no_transmission_requ(&self) -> bool {
        *self == TXRQSTR::NO_TRANSMISSION_REQU
    }
}
#[doc = r" Value of the field"]
pub struct CLRINTPNDR {
    bits: bool,
}
impl CLRINTPNDR {
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
#[doc = "Possible values of the field `CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRLR {
    #[doc = "Transfer control bits to message object"]
    TRANSFER_CONTROL_BIT,
    #[doc = "Control bits unchanged."]
    CONTROL_BITS_UNCHANG,
}
impl CTRLR {
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
            CTRLR::TRANSFER_CONTROL_BIT => true,
            CTRLR::CONTROL_BITS_UNCHANG => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTRLR {
        match value {
            true => CTRLR::TRANSFER_CONTROL_BIT,
            false => CTRLR::CONTROL_BITS_UNCHANG,
        }
    }
    #[doc = "Checks if the value of the field is `TRANSFER_CONTROL_BIT`"]
    #[inline]
    pub fn is_transfer_control_bit(&self) -> bool {
        *self == CTRLR::TRANSFER_CONTROL_BIT
    }
    #[doc = "Checks if the value of the field is `CONTROL_BITS_UNCHANG`"]
    #[inline]
    pub fn is_control_bits_unchang(&self) -> bool {
        *self == CTRLR::CONTROL_BITS_UNCHANG
    }
}
#[doc = "Possible values of the field `ARB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARBR {
    #[doc = "Transfer Identifier, DIR, XTD, and MSGVAL bits to message object."]
    TRANSFER_IDENTIFIER,
    #[doc = "Arbitration bits unchanged."]
    ARBITRATION_BITS_UNC,
}
impl ARBR {
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
            ARBR::TRANSFER_IDENTIFIER => true,
            ARBR::ARBITRATION_BITS_UNC => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ARBR {
        match value {
            true => ARBR::TRANSFER_IDENTIFIER,
            false => ARBR::ARBITRATION_BITS_UNC,
        }
    }
    #[doc = "Checks if the value of the field is `TRANSFER_IDENTIFIER`"]
    #[inline]
    pub fn is_transfer_identifier(&self) -> bool {
        *self == ARBR::TRANSFER_IDENTIFIER
    }
    #[doc = "Checks if the value of the field is `ARBITRATION_BITS_UNC`"]
    #[inline]
    pub fn is_arbitration_bits_unc(&self) -> bool {
        *self == ARBR::ARBITRATION_BITS_UNC
    }
}
#[doc = "Possible values of the field `MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASKR {
    #[doc = "Transfer Identifier MASK + MDIR + MXTD to message object."]
    TRANSFER_IDENTIFIER,
    #[doc = "Mask bits unchanged."]
    MASK_BITS_UNCHANGED,
}
impl MASKR {
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
            MASKR::TRANSFER_IDENTIFIER => true,
            MASKR::MASK_BITS_UNCHANGED => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASKR {
        match value {
            true => MASKR::TRANSFER_IDENTIFIER,
            false => MASKR::MASK_BITS_UNCHANGED,
        }
    }
    #[doc = "Checks if the value of the field is `TRANSFER_IDENTIFIER`"]
    #[inline]
    pub fn is_transfer_identifier(&self) -> bool {
        *self == MASKR::TRANSFER_IDENTIFIER
    }
    #[doc = "Checks if the value of the field is `MASK_BITS_UNCHANGED`"]
    #[inline]
    pub fn is_mask_bits_unchanged(&self) -> bool {
        *self == MASKR::MASK_BITS_UNCHANGED
    }
}
#[doc = r" Value of the field"]
pub struct WR_RDR {
    bits: bool,
}
impl WR_RDR {
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
#[doc = "Values that can be written to the field `DATA_B`"]
pub enum DATA_BW {
    #[doc = "Transfer data bytes 4-7 to message object."]
    TRANSFER_DATA_BYTES,
    #[doc = "data bytes 4-7 unchanged."]
    DATA_BYTES_4_7_UNCHA,
}
impl DATA_BW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATA_BW::TRANSFER_DATA_BYTES => true,
            DATA_BW::DATA_BYTES_4_7_UNCHA => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA_BW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_BW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA_BW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transfer data bytes 4-7 to message object."]
    #[inline]
    pub fn transfer_data_bytes(self) -> &'a mut W {
        self.variant(DATA_BW::TRANSFER_DATA_BYTES)
    }
    #[doc = "data bytes 4-7 unchanged."]
    #[inline]
    pub fn data_bytes_4_7_uncha(self) -> &'a mut W {
        self.variant(DATA_BW::DATA_BYTES_4_7_UNCHA)
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
#[doc = "Values that can be written to the field `DATA_A`"]
pub enum DATA_AW {
    #[doc = "Transfer data bytes 0-3 to message object."]
    TRANSFER_DATA_BYTES,
    #[doc = "data bytes 0-3 unchanged."]
    DATA_BYTES_0_3_UNCHA,
}
impl DATA_AW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATA_AW::TRANSFER_DATA_BYTES => true,
            DATA_AW::DATA_BYTES_0_3_UNCHA => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA_AW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_AW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA_AW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transfer data bytes 0-3 to message object."]
    #[inline]
    pub fn transfer_data_bytes(self) -> &'a mut W {
        self.variant(DATA_AW::TRANSFER_DATA_BYTES)
    }
    #[doc = "data bytes 0-3 unchanged."]
    #[inline]
    pub fn data_bytes_0_3_uncha(self) -> &'a mut W {
        self.variant(DATA_AW::DATA_BYTES_0_3_UNCHA)
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
#[doc = "Values that can be written to the field `TXRQST`"]
pub enum TXRQSTW {
    #[doc = "Request a transmission. Set the TXRQST bit IF1/2_MCTRL."]
    REQUEST_A_TRANSMISSI,
    #[doc = "No transmission request. TXRQSRT bit unchanged in IF1/2_MCTRL. If a transmission is requested by programming this bit, the TXRQST bit in the CANIFn_MCTRL register is ignored."]
    NO_TRANSMISSION_REQU,
}
impl TXRQSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXRQSTW::REQUEST_A_TRANSMISSI => true,
            TXRQSTW::NO_TRANSMISSION_REQU => false,
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
    #[doc = "Request a transmission. Set the TXRQST bit IF1/2_MCTRL."]
    #[inline]
    pub fn request_a_transmissi(self) -> &'a mut W {
        self.variant(TXRQSTW::REQUEST_A_TRANSMISSI)
    }
    #[doc = "No transmission request. TXRQSRT bit unchanged in IF1/2_MCTRL. If a transmission is requested by programming this bit, the TXRQST bit in the CANIFn_MCTRL register is ignored."]
    #[inline]
    pub fn no_transmission_requ(self) -> &'a mut W {
        self.variant(TXRQSTW::NO_TRANSMISSION_REQU)
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
pub struct _CLRINTPNDW<'a> {
    w: &'a mut W,
}
impl<'a> _CLRINTPNDW<'a> {
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
#[doc = "Values that can be written to the field `CTRL`"]
pub enum CTRLW {
    #[doc = "Transfer control bits to message object"]
    TRANSFER_CONTROL_BIT,
    #[doc = "Control bits unchanged."]
    CONTROL_BITS_UNCHANG,
}
impl CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTRLW::TRANSFER_CONTROL_BIT => true,
            CTRLW::CONTROL_BITS_UNCHANG => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transfer control bits to message object"]
    #[inline]
    pub fn transfer_control_bit(self) -> &'a mut W {
        self.variant(CTRLW::TRANSFER_CONTROL_BIT)
    }
    #[doc = "Control bits unchanged."]
    #[inline]
    pub fn control_bits_unchang(self) -> &'a mut W {
        self.variant(CTRLW::CONTROL_BITS_UNCHANG)
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
#[doc = "Values that can be written to the field `ARB`"]
pub enum ARBW {
    #[doc = "Transfer Identifier, DIR, XTD, and MSGVAL bits to message object."]
    TRANSFER_IDENTIFIER,
    #[doc = "Arbitration bits unchanged."]
    ARBITRATION_BITS_UNC,
}
impl ARBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ARBW::TRANSFER_IDENTIFIER => true,
            ARBW::ARBITRATION_BITS_UNC => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ARBW<'a> {
    w: &'a mut W,
}
impl<'a> _ARBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transfer Identifier, DIR, XTD, and MSGVAL bits to message object."]
    #[inline]
    pub fn transfer_identifier(self) -> &'a mut W {
        self.variant(ARBW::TRANSFER_IDENTIFIER)
    }
    #[doc = "Arbitration bits unchanged."]
    #[inline]
    pub fn arbitration_bits_unc(self) -> &'a mut W {
        self.variant(ARBW::ARBITRATION_BITS_UNC)
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
#[doc = "Values that can be written to the field `MASK`"]
pub enum MASKW {
    #[doc = "Transfer Identifier MASK + MDIR + MXTD to message object."]
    TRANSFER_IDENTIFIER,
    #[doc = "Mask bits unchanged."]
    MASK_BITS_UNCHANGED,
}
impl MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MASKW::TRANSFER_IDENTIFIER => true,
            MASKW::MASK_BITS_UNCHANGED => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transfer Identifier MASK + MDIR + MXTD to message object."]
    #[inline]
    pub fn transfer_identifier(self) -> &'a mut W {
        self.variant(MASKW::TRANSFER_IDENTIFIER)
    }
    #[doc = "Mask bits unchanged."]
    #[inline]
    pub fn mask_bits_unchanged(self) -> &'a mut W {
        self.variant(MASKW::MASK_BITS_UNCHANGED)
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
#[doc = r" Proxy"]
pub struct _WR_RDW<'a> {
    w: &'a mut W,
}
impl<'a> _WR_RDW<'a> {
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
    #[doc = "Bit 0 - Access data bytes 4-7"]
    #[inline]
    pub fn data_b(&self) -> DATA_BR {
        DATA_BR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Access data bytes 0-3"]
    #[inline]
    pub fn data_a(&self) -> DATA_AR {
        DATA_AR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Access transmission request bit"]
    #[inline]
    pub fn txrqst(&self) -> TXRQSTR {
        TXRQSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - This bit is ignored in the write direction."]
    #[inline]
    pub fn clrintpnd(&self) -> CLRINTPNDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRINTPNDR { bits }
    }
    #[doc = "Bit 4 - Access control bits"]
    #[inline]
    pub fn ctrl(&self) -> CTRLR {
        CTRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Access arbitration bits"]
    #[inline]
    pub fn arb(&self) -> ARBR {
        ARBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Access mask bits"]
    #[inline]
    pub fn mask(&self) -> MASKR {
        MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Write transfer Transfer data from the selected message buffer registers to the message object addressed by the command request register CANIFn_CMDREQ."]
    #[inline]
    pub fn wr_rd(&self) -> WR_RDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WR_RDR { bits }
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
    #[doc = "Bit 0 - Access data bytes 4-7"]
    #[inline]
    pub fn data_b(&mut self) -> _DATA_BW {
        _DATA_BW { w: self }
    }
    #[doc = "Bit 1 - Access data bytes 0-3"]
    #[inline]
    pub fn data_a(&mut self) -> _DATA_AW {
        _DATA_AW { w: self }
    }
    #[doc = "Bit 2 - Access transmission request bit"]
    #[inline]
    pub fn txrqst(&mut self) -> _TXRQSTW {
        _TXRQSTW { w: self }
    }
    #[doc = "Bit 3 - This bit is ignored in the write direction."]
    #[inline]
    pub fn clrintpnd(&mut self) -> _CLRINTPNDW {
        _CLRINTPNDW { w: self }
    }
    #[doc = "Bit 4 - Access control bits"]
    #[inline]
    pub fn ctrl(&mut self) -> _CTRLW {
        _CTRLW { w: self }
    }
    #[doc = "Bit 5 - Access arbitration bits"]
    #[inline]
    pub fn arb(&mut self) -> _ARBW {
        _ARBW { w: self }
    }
    #[doc = "Bit 6 - Access mask bits"]
    #[inline]
    pub fn mask(&mut self) -> _MASKW {
        _MASKW { w: self }
    }
    #[doc = "Bit 7 - Write transfer Transfer data from the selected message buffer registers to the message object addressed by the command request register CANIFn_CMDREQ."]
    #[inline]
    pub fn wr_rd(&mut self) -> _WR_RDW {
        _WR_RDW { w: self }
    }
}
