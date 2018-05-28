#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CMD {
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
pub struct DATALENR {
    bits: u16,
}
impl DATALENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct POLLR {
    bits: bool,
}
impl POLLR {
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
#[doc = "Possible values of the field `DOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOUTR {
    #[doc = "Input from serial flash."]
    INPUT_FROM_SERIAL_FL,
    #[doc = "Output to serial flash."]
    OUTPUT_TO_SERIAL_FLA,
}
impl DOUTR {
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
            DOUTR::INPUT_FROM_SERIAL_FL => false,
            DOUTR::OUTPUT_TO_SERIAL_FLA => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOUTR {
        match value {
            false => DOUTR::INPUT_FROM_SERIAL_FL,
            true => DOUTR::OUTPUT_TO_SERIAL_FLA,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT_FROM_SERIAL_FL`"]
    #[inline]
    pub fn is_input_from_serial_fl(&self) -> bool {
        *self == DOUTR::INPUT_FROM_SERIAL_FL
    }
    #[doc = "Checks if the value of the field is `OUTPUT_TO_SERIAL_FLA`"]
    #[inline]
    pub fn is_output_to_serial_fla(&self) -> bool {
        *self == DOUTR::OUTPUT_TO_SERIAL_FLA
    }
}
#[doc = r" Value of the field"]
pub struct INTLENR {
    bits: u8,
}
impl INTLENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `FIELDFORM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIELDFORMR {
    #[doc = "All serial. All fields of the command are serial."]
    ALL_SERIAL,
    #[doc = "Quad/dual data. Data field is quad/dual, other fields are serial."]
    QUADDUAL_DATA,
    #[doc = "Serial opcode. Opcode field is serial. Other fields are quad/dual."]
    SERIAL_OPCODE,
    #[doc = "All quad/dual. All fields of the command are in quad/dual format."]
    ALL_QUADDUAL,
}
impl FIELDFORMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FIELDFORMR::ALL_SERIAL => 0,
            FIELDFORMR::QUADDUAL_DATA => 1,
            FIELDFORMR::SERIAL_OPCODE => 2,
            FIELDFORMR::ALL_QUADDUAL => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FIELDFORMR {
        match value {
            0 => FIELDFORMR::ALL_SERIAL,
            1 => FIELDFORMR::QUADDUAL_DATA,
            2 => FIELDFORMR::SERIAL_OPCODE,
            3 => FIELDFORMR::ALL_QUADDUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ALL_SERIAL`"]
    #[inline]
    pub fn is_all_serial(&self) -> bool {
        *self == FIELDFORMR::ALL_SERIAL
    }
    #[doc = "Checks if the value of the field is `QUADDUAL_DATA`"]
    #[inline]
    pub fn is_quaddual_data(&self) -> bool {
        *self == FIELDFORMR::QUADDUAL_DATA
    }
    #[doc = "Checks if the value of the field is `SERIAL_OPCODE`"]
    #[inline]
    pub fn is_serial_opcode(&self) -> bool {
        *self == FIELDFORMR::SERIAL_OPCODE
    }
    #[doc = "Checks if the value of the field is `ALL_QUADDUAL`"]
    #[inline]
    pub fn is_all_quaddual(&self) -> bool {
        *self == FIELDFORMR::ALL_QUADDUAL
    }
}
#[doc = "Possible values of the field `FRAMEFORM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAMEFORMR {
    #[doc = "Opcode. Opcode only, no address."]
    OPCODE,
    #[doc = "Opcode one byte. Opcode, least significant byte of address."]
    OPCODE_ONE_BYTE,
    #[doc = "Opcode two bytes. Opcode, two least significant bytes of address."]
    OPCODE_TWO_BYTES,
    #[doc = "Opcode three bytes. Opcode, three least significant bytes of address."]
    OPCODE_THREE_BYTES,
    #[doc = "Opcode four bytes. Opcode, 4 bytes of address."]
    OPCODE_FOUR_BYTES,
    #[doc = "No opcode three bytes. No opcode, 3 least significant bytes of address."]
    NO_OPCODE_THREE_BYTE,
    #[doc = "No opcode four bytes. No opcode, 4 bytes of address."]
    NO_OPCODE_FOUR_BYTES,
}
impl FRAMEFORMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FRAMEFORMR::OPCODE => 1,
            FRAMEFORMR::OPCODE_ONE_BYTE => 2,
            FRAMEFORMR::OPCODE_TWO_BYTES => 3,
            FRAMEFORMR::OPCODE_THREE_BYTES => 4,
            FRAMEFORMR::OPCODE_FOUR_BYTES => 5,
            FRAMEFORMR::NO_OPCODE_THREE_BYTE => 6,
            FRAMEFORMR::NO_OPCODE_FOUR_BYTES => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FRAMEFORMR {
        match value {
            1 => FRAMEFORMR::OPCODE,
            2 => FRAMEFORMR::OPCODE_ONE_BYTE,
            3 => FRAMEFORMR::OPCODE_TWO_BYTES,
            4 => FRAMEFORMR::OPCODE_THREE_BYTES,
            5 => FRAMEFORMR::OPCODE_FOUR_BYTES,
            6 => FRAMEFORMR::NO_OPCODE_THREE_BYTE,
            7 => FRAMEFORMR::NO_OPCODE_FOUR_BYTES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPCODE`"]
    #[inline]
    pub fn is_opcode(&self) -> bool {
        *self == FRAMEFORMR::OPCODE
    }
    #[doc = "Checks if the value of the field is `OPCODE_ONE_BYTE`"]
    #[inline]
    pub fn is_opcode_one_byte(&self) -> bool {
        *self == FRAMEFORMR::OPCODE_ONE_BYTE
    }
    #[doc = "Checks if the value of the field is `OPCODE_TWO_BYTES`"]
    #[inline]
    pub fn is_opcode_two_bytes(&self) -> bool {
        *self == FRAMEFORMR::OPCODE_TWO_BYTES
    }
    #[doc = "Checks if the value of the field is `OPCODE_THREE_BYTES`"]
    #[inline]
    pub fn is_opcode_three_bytes(&self) -> bool {
        *self == FRAMEFORMR::OPCODE_THREE_BYTES
    }
    #[doc = "Checks if the value of the field is `OPCODE_FOUR_BYTES`"]
    #[inline]
    pub fn is_opcode_four_bytes(&self) -> bool {
        *self == FRAMEFORMR::OPCODE_FOUR_BYTES
    }
    #[doc = "Checks if the value of the field is `NO_OPCODE_THREE_BYTE`"]
    #[inline]
    pub fn is_no_opcode_three_byte(&self) -> bool {
        *self == FRAMEFORMR::NO_OPCODE_THREE_BYTE
    }
    #[doc = "Checks if the value of the field is `NO_OPCODE_FOUR_BYTES`"]
    #[inline]
    pub fn is_no_opcode_four_bytes(&self) -> bool {
        *self == FRAMEFORMR::NO_OPCODE_FOUR_BYTES
    }
}
#[doc = r" Value of the field"]
pub struct OPCODER {
    bits: u8,
}
impl OPCODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _DATALENW<'a> {
    w: &'a mut W,
}
impl<'a> _DATALENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 16383;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _POLLW<'a> {
    w: &'a mut W,
}
impl<'a> _POLLW<'a> {
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
#[doc = "Values that can be written to the field `DOUT`"]
pub enum DOUTW {
    #[doc = "Input from serial flash."]
    INPUT_FROM_SERIAL_FL,
    #[doc = "Output to serial flash."]
    OUTPUT_TO_SERIAL_FLA,
}
impl DOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DOUTW::INPUT_FROM_SERIAL_FL => false,
            DOUTW::OUTPUT_TO_SERIAL_FLA => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _DOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOUTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input from serial flash."]
    #[inline]
    pub fn input_from_serial_fl(self) -> &'a mut W {
        self.variant(DOUTW::INPUT_FROM_SERIAL_FL)
    }
    #[doc = "Output to serial flash."]
    #[inline]
    pub fn output_to_serial_fla(self) -> &'a mut W {
        self.variant(DOUTW::OUTPUT_TO_SERIAL_FLA)
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
#[doc = r" Proxy"]
pub struct _INTLENW<'a> {
    w: &'a mut W,
}
impl<'a> _INTLENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FIELDFORM`"]
pub enum FIELDFORMW {
    #[doc = "All serial. All fields of the command are serial."]
    ALL_SERIAL,
    #[doc = "Quad/dual data. Data field is quad/dual, other fields are serial."]
    QUADDUAL_DATA,
    #[doc = "Serial opcode. Opcode field is serial. Other fields are quad/dual."]
    SERIAL_OPCODE,
    #[doc = "All quad/dual. All fields of the command are in quad/dual format."]
    ALL_QUADDUAL,
}
impl FIELDFORMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FIELDFORMW::ALL_SERIAL => 0,
            FIELDFORMW::QUADDUAL_DATA => 1,
            FIELDFORMW::SERIAL_OPCODE => 2,
            FIELDFORMW::ALL_QUADDUAL => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FIELDFORMW<'a> {
    w: &'a mut W,
}
impl<'a> _FIELDFORMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FIELDFORMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "All serial. All fields of the command are serial."]
    #[inline]
    pub fn all_serial(self) -> &'a mut W {
        self.variant(FIELDFORMW::ALL_SERIAL)
    }
    #[doc = "Quad/dual data. Data field is quad/dual, other fields are serial."]
    #[inline]
    pub fn quaddual_data(self) -> &'a mut W {
        self.variant(FIELDFORMW::QUADDUAL_DATA)
    }
    #[doc = "Serial opcode. Opcode field is serial. Other fields are quad/dual."]
    #[inline]
    pub fn serial_opcode(self) -> &'a mut W {
        self.variant(FIELDFORMW::SERIAL_OPCODE)
    }
    #[doc = "All quad/dual. All fields of the command are in quad/dual format."]
    #[inline]
    pub fn all_quaddual(self) -> &'a mut W {
        self.variant(FIELDFORMW::ALL_QUADDUAL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FRAMEFORM`"]
pub enum FRAMEFORMW {
    #[doc = "Opcode. Opcode only, no address."]
    OPCODE,
    #[doc = "Opcode one byte. Opcode, least significant byte of address."]
    OPCODE_ONE_BYTE,
    #[doc = "Opcode two bytes. Opcode, two least significant bytes of address."]
    OPCODE_TWO_BYTES,
    #[doc = "Opcode three bytes. Opcode, three least significant bytes of address."]
    OPCODE_THREE_BYTES,
    #[doc = "Opcode four bytes. Opcode, 4 bytes of address."]
    OPCODE_FOUR_BYTES,
    #[doc = "No opcode three bytes. No opcode, 3 least significant bytes of address."]
    NO_OPCODE_THREE_BYTE,
    #[doc = "No opcode four bytes. No opcode, 4 bytes of address."]
    NO_OPCODE_FOUR_BYTES,
}
impl FRAMEFORMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FRAMEFORMW::OPCODE => 1,
            FRAMEFORMW::OPCODE_ONE_BYTE => 2,
            FRAMEFORMW::OPCODE_TWO_BYTES => 3,
            FRAMEFORMW::OPCODE_THREE_BYTES => 4,
            FRAMEFORMW::OPCODE_FOUR_BYTES => 5,
            FRAMEFORMW::NO_OPCODE_THREE_BYTE => 6,
            FRAMEFORMW::NO_OPCODE_FOUR_BYTES => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRAMEFORMW<'a> {
    w: &'a mut W,
}
impl<'a> _FRAMEFORMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRAMEFORMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Opcode. Opcode only, no address."]
    #[inline]
    pub fn opcode(self) -> &'a mut W {
        self.variant(FRAMEFORMW::OPCODE)
    }
    #[doc = "Opcode one byte. Opcode, least significant byte of address."]
    #[inline]
    pub fn opcode_one_byte(self) -> &'a mut W {
        self.variant(FRAMEFORMW::OPCODE_ONE_BYTE)
    }
    #[doc = "Opcode two bytes. Opcode, two least significant bytes of address."]
    #[inline]
    pub fn opcode_two_bytes(self) -> &'a mut W {
        self.variant(FRAMEFORMW::OPCODE_TWO_BYTES)
    }
    #[doc = "Opcode three bytes. Opcode, three least significant bytes of address."]
    #[inline]
    pub fn opcode_three_bytes(self) -> &'a mut W {
        self.variant(FRAMEFORMW::OPCODE_THREE_BYTES)
    }
    #[doc = "Opcode four bytes. Opcode, 4 bytes of address."]
    #[inline]
    pub fn opcode_four_bytes(self) -> &'a mut W {
        self.variant(FRAMEFORMW::OPCODE_FOUR_BYTES)
    }
    #[doc = "No opcode three bytes. No opcode, 3 least significant bytes of address."]
    #[inline]
    pub fn no_opcode_three_byte(self) -> &'a mut W {
        self.variant(FRAMEFORMW::NO_OPCODE_THREE_BYTE)
    }
    #[doc = "No opcode four bytes. No opcode, 4 bytes of address."]
    #[inline]
    pub fn no_opcode_four_bytes(self) -> &'a mut W {
        self.variant(FRAMEFORMW::NO_OPCODE_FOUR_BYTES)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OPCODEW<'a> {
    w: &'a mut W,
}
impl<'a> _OPCODEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:13 - Except when the POLL bit in this register is 1, this field controls how many data bytes are in the command. 0 indicates that the command does not contain a data field."]
    #[inline]
    pub fn datalen(&self) -> DATALENR {
        let bits = {
            const MASK: u16 = 16383;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DATALENR { bits }
    }
    #[doc = "Bit 14 - This bit should be written as 1 only with an opcode that a) contains an input data field, and b) causes the serial flash device to return byte status repetitively (e.g., a Read Status command). When this bit is 1, the SPIFI hardware continues to read bytes until the test specified by the dataLen field is met. The hardware tests the bit in each status byte selected by DATALEN bits 2:0, until a bit is found that is equal to DATALEN bit 3. When the test succeeds, the SPIFI captures the byte that meets this test so that it can be read from the Data Register, and terminates the command by raising CS. The end-of-command interrupt can be enabled to inform software when this occurs"]
    #[inline]
    pub fn poll(&self) -> POLLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        POLLR { bits }
    }
    #[doc = "Bit 15 - If the DATALEN field is not zero, this bit controls the direction of the data:"]
    #[inline]
    pub fn dout(&self) -> DOUTR {
        DOUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:18 - This field controls how many intermediate bytes precede the data. (Each such byte may require 8 or 2 SCK cycles, depending on whether the intermediate field is in serial, 2-bit, or 4-bit format.) Intermediate bytes are output by the SPIFI, and include post-address control information, dummy and delay bytes. See the description of the Intermediate Data register for the contents of such bytes."]
    #[inline]
    pub fn intlen(&self) -> INTLENR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INTLENR { bits }
    }
    #[doc = "Bits 19:20 - This field controls how the fields of the command are sent."]
    #[inline]
    pub fn fieldform(&self) -> FIELDFORMR {
        FIELDFORMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 21:23 - This field controls the opcode and address fields."]
    #[inline]
    pub fn frameform(&self) -> FRAMEFORMR {
        FRAMEFORMR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:31 - The opcode of the command (not used for some FRAMEFORM values)."]
    #[inline]
    pub fn opcode(&self) -> OPCODER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OPCODER { bits }
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
    #[doc = "Bits 0:13 - Except when the POLL bit in this register is 1, this field controls how many data bytes are in the command. 0 indicates that the command does not contain a data field."]
    #[inline]
    pub fn datalen(&mut self) -> _DATALENW {
        _DATALENW { w: self }
    }
    #[doc = "Bit 14 - This bit should be written as 1 only with an opcode that a) contains an input data field, and b) causes the serial flash device to return byte status repetitively (e.g., a Read Status command). When this bit is 1, the SPIFI hardware continues to read bytes until the test specified by the dataLen field is met. The hardware tests the bit in each status byte selected by DATALEN bits 2:0, until a bit is found that is equal to DATALEN bit 3. When the test succeeds, the SPIFI captures the byte that meets this test so that it can be read from the Data Register, and terminates the command by raising CS. The end-of-command interrupt can be enabled to inform software when this occurs"]
    #[inline]
    pub fn poll(&mut self) -> _POLLW {
        _POLLW { w: self }
    }
    #[doc = "Bit 15 - If the DATALEN field is not zero, this bit controls the direction of the data:"]
    #[inline]
    pub fn dout(&mut self) -> _DOUTW {
        _DOUTW { w: self }
    }
    #[doc = "Bits 16:18 - This field controls how many intermediate bytes precede the data. (Each such byte may require 8 or 2 SCK cycles, depending on whether the intermediate field is in serial, 2-bit, or 4-bit format.) Intermediate bytes are output by the SPIFI, and include post-address control information, dummy and delay bytes. See the description of the Intermediate Data register for the contents of such bytes."]
    #[inline]
    pub fn intlen(&mut self) -> _INTLENW {
        _INTLENW { w: self }
    }
    #[doc = "Bits 19:20 - This field controls how the fields of the command are sent."]
    #[inline]
    pub fn fieldform(&mut self) -> _FIELDFORMW {
        _FIELDFORMW { w: self }
    }
    #[doc = "Bits 21:23 - This field controls the opcode and address fields."]
    #[inline]
    pub fn frameform(&mut self) -> _FRAMEFORMW {
        _FRAMEFORMW { w: self }
    }
    #[doc = "Bits 24:31 - The opcode of the command (not used for some FRAMEFORM values)."]
    #[inline]
    pub fn opcode(&mut self) -> _OPCODEW {
        _OPCODEW { w: self }
    }
}
