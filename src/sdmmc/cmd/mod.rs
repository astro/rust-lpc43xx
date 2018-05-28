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
pub struct CMD_INDEXR {
    bits: u8,
}
impl CMD_INDEXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `RESPONSE_EXPECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESPONSE_EXPECTR {
    #[doc = "None. No response expected from card"]
    NONE,
    #[doc = "Expected. Response expected from card"]
    EXPECTED,
}
impl RESPONSE_EXPECTR {
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
            RESPONSE_EXPECTR::NONE => false,
            RESPONSE_EXPECTR::EXPECTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESPONSE_EXPECTR {
        match value {
            false => RESPONSE_EXPECTR::NONE,
            true => RESPONSE_EXPECTR::EXPECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == RESPONSE_EXPECTR::NONE
    }
    #[doc = "Checks if the value of the field is `EXPECTED`"]
    #[inline]
    pub fn is_expected(&self) -> bool {
        *self == RESPONSE_EXPECTR::EXPECTED
    }
}
#[doc = "Possible values of the field `RESPONSE_LENGTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESPONSE_LENGTHR {
    #[doc = "Short. Short response expected from card"]
    SHORT,
    #[doc = "Long. Long response expected from card"]
    LONG,
}
impl RESPONSE_LENGTHR {
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
            RESPONSE_LENGTHR::SHORT => false,
            RESPONSE_LENGTHR::LONG => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESPONSE_LENGTHR {
        match value {
            false => RESPONSE_LENGTHR::SHORT,
            true => RESPONSE_LENGTHR::LONG,
        }
    }
    #[doc = "Checks if the value of the field is `SHORT`"]
    #[inline]
    pub fn is_short(&self) -> bool {
        *self == RESPONSE_LENGTHR::SHORT
    }
    #[doc = "Checks if the value of the field is `LONG`"]
    #[inline]
    pub fn is_long(&self) -> bool {
        *self == RESPONSE_LENGTHR::LONG
    }
}
#[doc = "Possible values of the field `CHECK_RESPONSE_CRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHECK_RESPONSE_CRCR {
    #[doc = "Do not check response CRC"]
    DO_NOT_CHECK_RESPONS,
    #[doc = "Check response CRC"]
    CHECK_RESPONSE_CRC,
}
impl CHECK_RESPONSE_CRCR {
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
            CHECK_RESPONSE_CRCR::DO_NOT_CHECK_RESPONS => false,
            CHECK_RESPONSE_CRCR::CHECK_RESPONSE_CRC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHECK_RESPONSE_CRCR {
        match value {
            false => CHECK_RESPONSE_CRCR::DO_NOT_CHECK_RESPONS,
            true => CHECK_RESPONSE_CRCR::CHECK_RESPONSE_CRC,
        }
    }
    #[doc = "Checks if the value of the field is `DO_NOT_CHECK_RESPONS`"]
    #[inline]
    pub fn is_do_not_check_respons(&self) -> bool {
        *self == CHECK_RESPONSE_CRCR::DO_NOT_CHECK_RESPONS
    }
    #[doc = "Checks if the value of the field is `CHECK_RESPONSE_CRC`"]
    #[inline]
    pub fn is_check_response_crc(&self) -> bool {
        *self == CHECK_RESPONSE_CRCR::CHECK_RESPONSE_CRC
    }
}
#[doc = "Possible values of the field `DATA_EXPECTED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_EXPECTEDR {
    #[doc = "None. No data transfer expected (read/write)"]
    NONE,
    #[doc = "Data. Data transfer expected (read/write)"]
    DATA,
}
impl DATA_EXPECTEDR {
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
            DATA_EXPECTEDR::NONE => false,
            DATA_EXPECTEDR::DATA => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATA_EXPECTEDR {
        match value {
            false => DATA_EXPECTEDR::NONE,
            true => DATA_EXPECTEDR::DATA,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == DATA_EXPECTEDR::NONE
    }
    #[doc = "Checks if the value of the field is `DATA`"]
    #[inline]
    pub fn is_data(&self) -> bool {
        *self == DATA_EXPECTEDR::DATA
    }
}
#[doc = "Possible values of the field `READ_WRITE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READ_WRITER {
    #[doc = "Read from card"]
    READ_FROM_CARD,
    #[doc = "Write to card"]
    WRITE_TO_CARD,
}
impl READ_WRITER {
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
            READ_WRITER::READ_FROM_CARD => false,
            READ_WRITER::WRITE_TO_CARD => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> READ_WRITER {
        match value {
            false => READ_WRITER::READ_FROM_CARD,
            true => READ_WRITER::WRITE_TO_CARD,
        }
    }
    #[doc = "Checks if the value of the field is `READ_FROM_CARD`"]
    #[inline]
    pub fn is_read_from_card(&self) -> bool {
        *self == READ_WRITER::READ_FROM_CARD
    }
    #[doc = "Checks if the value of the field is `WRITE_TO_CARD`"]
    #[inline]
    pub fn is_write_to_card(&self) -> bool {
        *self == READ_WRITER::WRITE_TO_CARD
    }
}
#[doc = "Possible values of the field `TRANSFER_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRANSFER_MODER {
    #[doc = "Block data transfer command"]
    BLOCK_DATA_TRANSFER,
    #[doc = "Stream data transfer command"]
    STREAM_DATA_TRANSFER,
}
impl TRANSFER_MODER {
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
            TRANSFER_MODER::BLOCK_DATA_TRANSFER => false,
            TRANSFER_MODER::STREAM_DATA_TRANSFER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRANSFER_MODER {
        match value {
            false => TRANSFER_MODER::BLOCK_DATA_TRANSFER,
            true => TRANSFER_MODER::STREAM_DATA_TRANSFER,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCK_DATA_TRANSFER`"]
    #[inline]
    pub fn is_block_data_transfer(&self) -> bool {
        *self == TRANSFER_MODER::BLOCK_DATA_TRANSFER
    }
    #[doc = "Checks if the value of the field is `STREAM_DATA_TRANSFER`"]
    #[inline]
    pub fn is_stream_data_transfer(&self) -> bool {
        *self == TRANSFER_MODER::STREAM_DATA_TRANSFER
    }
}
#[doc = "Possible values of the field `SEND_AUTO_STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEND_AUTO_STOPR {
    #[doc = "No stop command sent at end of data transfer"]
    NO_STOP_COMMAND_SENT,
    #[doc = "Send stop command at end of data transfer"]
    SEND_STOP_COMMAND_AT,
}
impl SEND_AUTO_STOPR {
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
            SEND_AUTO_STOPR::NO_STOP_COMMAND_SENT => false,
            SEND_AUTO_STOPR::SEND_STOP_COMMAND_AT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEND_AUTO_STOPR {
        match value {
            false => SEND_AUTO_STOPR::NO_STOP_COMMAND_SENT,
            true => SEND_AUTO_STOPR::SEND_STOP_COMMAND_AT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_STOP_COMMAND_SENT`"]
    #[inline]
    pub fn is_no_stop_command_sent(&self) -> bool {
        *self == SEND_AUTO_STOPR::NO_STOP_COMMAND_SENT
    }
    #[doc = "Checks if the value of the field is `SEND_STOP_COMMAND_AT`"]
    #[inline]
    pub fn is_send_stop_command_at(&self) -> bool {
        *self == SEND_AUTO_STOPR::SEND_STOP_COMMAND_AT
    }
}
#[doc = "Possible values of the field `WAIT_PRVDATA_COMPLETE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAIT_PRVDATA_COMPLETER {
    #[doc = "Send. Send command at once, even if previous data transfer has not completed."]
    SEND,
    #[doc = "Wait. Wait for previous data transfer completion before sending command."]
    WAIT,
}
impl WAIT_PRVDATA_COMPLETER {
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
            WAIT_PRVDATA_COMPLETER::SEND => false,
            WAIT_PRVDATA_COMPLETER::WAIT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAIT_PRVDATA_COMPLETER {
        match value {
            false => WAIT_PRVDATA_COMPLETER::SEND,
            true => WAIT_PRVDATA_COMPLETER::WAIT,
        }
    }
    #[doc = "Checks if the value of the field is `SEND`"]
    #[inline]
    pub fn is_send(&self) -> bool {
        *self == WAIT_PRVDATA_COMPLETER::SEND
    }
    #[doc = "Checks if the value of the field is `WAIT`"]
    #[inline]
    pub fn is_wait(&self) -> bool {
        *self == WAIT_PRVDATA_COMPLETER::WAIT
    }
}
#[doc = "Possible values of the field `STOP_ABORT_CMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOP_ABORT_CMDR {
    #[doc = "Disabled. Neither stop nor abort command to stop current data transfer in progress. If abort is sent to function-number currently selected or not in data-transfer mode, then bit should be set to 0."]
    DISABLED,
    #[doc = "Enabled. Stop or abort command intended to stop current data transfer in progress."]
    ENABLED,
}
impl STOP_ABORT_CMDR {
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
            STOP_ABORT_CMDR::DISABLED => false,
            STOP_ABORT_CMDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STOP_ABORT_CMDR {
        match value {
            false => STOP_ABORT_CMDR::DISABLED,
            true => STOP_ABORT_CMDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == STOP_ABORT_CMDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == STOP_ABORT_CMDR::ENABLED
    }
}
#[doc = "Possible values of the field `SEND_INITIALIZATION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEND_INITIALIZATIONR {
    #[doc = "No. Do not send initialization sequence (80 clocks of 1) before sending this command."]
    NO,
    #[doc = "Send. Send initialization sequence before sending this command."]
    SEND,
}
impl SEND_INITIALIZATIONR {
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
            SEND_INITIALIZATIONR::NO => false,
            SEND_INITIALIZATIONR::SEND => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEND_INITIALIZATIONR {
        match value {
            false => SEND_INITIALIZATIONR::NO,
            true => SEND_INITIALIZATIONR::SEND,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == SEND_INITIALIZATIONR::NO
    }
    #[doc = "Checks if the value of the field is `SEND`"]
    #[inline]
    pub fn is_send(&self) -> bool {
        *self == SEND_INITIALIZATIONR::SEND
    }
}
#[doc = "Possible values of the field `UPDATE_CLOCK_REGISTERS_ONLY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPDATE_CLOCK_REGISTERS_ONLYR {
    #[doc = "Normal. Normal command sequence"]
    NORMAL,
    #[doc = "No. Do not send commands, just update clock register value into card clock domain"]
    NO,
}
impl UPDATE_CLOCK_REGISTERS_ONLYR {
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
            UPDATE_CLOCK_REGISTERS_ONLYR::NORMAL => false,
            UPDATE_CLOCK_REGISTERS_ONLYR::NO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UPDATE_CLOCK_REGISTERS_ONLYR {
        match value {
            false => UPDATE_CLOCK_REGISTERS_ONLYR::NORMAL,
            true => UPDATE_CLOCK_REGISTERS_ONLYR::NO,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == UPDATE_CLOCK_REGISTERS_ONLYR::NORMAL
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == UPDATE_CLOCK_REGISTERS_ONLYR::NO
    }
}
#[doc = "Possible values of the field `READ_CEATA_DEVICE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READ_CEATA_DEVICER {
    #[doc = "No read. Host is not performing read access (RW_REG or RW_BLK) towards CE-ATA device."]
    NO_READ,
    #[doc = "Read. Host is performing read access (RW_REG or RW_BLK) towards CE-ATA device."]
    READ,
}
impl READ_CEATA_DEVICER {
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
            READ_CEATA_DEVICER::NO_READ => false,
            READ_CEATA_DEVICER::READ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> READ_CEATA_DEVICER {
        match value {
            false => READ_CEATA_DEVICER::NO_READ,
            true => READ_CEATA_DEVICER::READ,
        }
    }
    #[doc = "Checks if the value of the field is `NO_READ`"]
    #[inline]
    pub fn is_no_read(&self) -> bool {
        *self == READ_CEATA_DEVICER::NO_READ
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == READ_CEATA_DEVICER::READ
    }
}
#[doc = "Possible values of the field `CCS_EXPECTED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCS_EXPECTEDR {
    #[doc = "Disabled. Interrupts are not enabled in CE-ATA device (nIEN = 1 in ATA control register), or command does not expect CCS from device."]
    DISABLED,
    #[doc = "Enabled. Interrupts are enabled in CE-ATA device (nIEN = 0), and RW_BLK command expects command completion signal from CE-ATA device."]
    ENABLED,
}
impl CCS_EXPECTEDR {
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
            CCS_EXPECTEDR::DISABLED => false,
            CCS_EXPECTEDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCS_EXPECTEDR {
        match value {
            false => CCS_EXPECTEDR::DISABLED,
            true => CCS_EXPECTEDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CCS_EXPECTEDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CCS_EXPECTEDR::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct ENABLE_BOOTR {
    bits: bool,
}
impl ENABLE_BOOTR {
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
#[doc = r" Value of the field"]
pub struct EXPECT_BOOT_ACKR {
    bits: bool,
}
impl EXPECT_BOOT_ACKR {
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
#[doc = r" Value of the field"]
pub struct DISABLE_BOOTR {
    bits: bool,
}
impl DISABLE_BOOTR {
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
#[doc = "Possible values of the field `BOOT_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOOT_MODER {
    #[doc = "Mandatory Boot operation"]
    MANDATORY_BOOT_OPERA,
    #[doc = "Alternate Boot operation"]
    ALTERNATE_BOOT_OPERA,
}
impl BOOT_MODER {
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
            BOOT_MODER::MANDATORY_BOOT_OPERA => false,
            BOOT_MODER::ALTERNATE_BOOT_OPERA => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOOT_MODER {
        match value {
            false => BOOT_MODER::MANDATORY_BOOT_OPERA,
            true => BOOT_MODER::ALTERNATE_BOOT_OPERA,
        }
    }
    #[doc = "Checks if the value of the field is `MANDATORY_BOOT_OPERA`"]
    #[inline]
    pub fn is_mandatory_boot_opera(&self) -> bool {
        *self == BOOT_MODER::MANDATORY_BOOT_OPERA
    }
    #[doc = "Checks if the value of the field is `ALTERNATE_BOOT_OPERA`"]
    #[inline]
    pub fn is_alternate_boot_opera(&self) -> bool {
        *self == BOOT_MODER::ALTERNATE_BOOT_OPERA
    }
}
#[doc = "Possible values of the field `VOLT_SWITCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VOLT_SWITCHR {
    #[doc = "Disabled. No voltage switching"]
    DISABLED,
    #[doc = "Enabled. Voltage switching enabled; must be set for CMD11 only"]
    ENABLED,
}
impl VOLT_SWITCHR {
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
            VOLT_SWITCHR::DISABLED => false,
            VOLT_SWITCHR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VOLT_SWITCHR {
        match value {
            false => VOLT_SWITCHR::DISABLED,
            true => VOLT_SWITCHR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == VOLT_SWITCHR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == VOLT_SWITCHR::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct START_CMDR {
    bits: bool,
}
impl START_CMDR {
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
#[doc = r" Proxy"]
pub struct _CMD_INDEXW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD_INDEXW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RESPONSE_EXPECT`"]
pub enum RESPONSE_EXPECTW {
    #[doc = "None. No response expected from card"]
    NONE,
    #[doc = "Expected. Response expected from card"]
    EXPECTED,
}
impl RESPONSE_EXPECTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESPONSE_EXPECTW::NONE => false,
            RESPONSE_EXPECTW::EXPECTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESPONSE_EXPECTW<'a> {
    w: &'a mut W,
}
impl<'a> _RESPONSE_EXPECTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESPONSE_EXPECTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "None. No response expected from card"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(RESPONSE_EXPECTW::NONE)
    }
    #[doc = "Expected. Response expected from card"]
    #[inline]
    pub fn expected(self) -> &'a mut W {
        self.variant(RESPONSE_EXPECTW::EXPECTED)
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
#[doc = "Values that can be written to the field `RESPONSE_LENGTH`"]
pub enum RESPONSE_LENGTHW {
    #[doc = "Short. Short response expected from card"]
    SHORT,
    #[doc = "Long. Long response expected from card"]
    LONG,
}
impl RESPONSE_LENGTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESPONSE_LENGTHW::SHORT => false,
            RESPONSE_LENGTHW::LONG => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESPONSE_LENGTHW<'a> {
    w: &'a mut W,
}
impl<'a> _RESPONSE_LENGTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESPONSE_LENGTHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Short. Short response expected from card"]
    #[inline]
    pub fn short(self) -> &'a mut W {
        self.variant(RESPONSE_LENGTHW::SHORT)
    }
    #[doc = "Long. Long response expected from card"]
    #[inline]
    pub fn long(self) -> &'a mut W {
        self.variant(RESPONSE_LENGTHW::LONG)
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
#[doc = "Values that can be written to the field `CHECK_RESPONSE_CRC`"]
pub enum CHECK_RESPONSE_CRCW {
    #[doc = "Do not check response CRC"]
    DO_NOT_CHECK_RESPONS,
    #[doc = "Check response CRC"]
    CHECK_RESPONSE_CRC,
}
impl CHECK_RESPONSE_CRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHECK_RESPONSE_CRCW::DO_NOT_CHECK_RESPONS => false,
            CHECK_RESPONSE_CRCW::CHECK_RESPONSE_CRC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHECK_RESPONSE_CRCW<'a> {
    w: &'a mut W,
}
impl<'a> _CHECK_RESPONSE_CRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHECK_RESPONSE_CRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not check response CRC"]
    #[inline]
    pub fn do_not_check_respons(self) -> &'a mut W {
        self.variant(CHECK_RESPONSE_CRCW::DO_NOT_CHECK_RESPONS)
    }
    #[doc = "Check response CRC"]
    #[inline]
    pub fn check_response_crc(self) -> &'a mut W {
        self.variant(CHECK_RESPONSE_CRCW::CHECK_RESPONSE_CRC)
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
#[doc = "Values that can be written to the field `DATA_EXPECTED`"]
pub enum DATA_EXPECTEDW {
    #[doc = "None. No data transfer expected (read/write)"]
    NONE,
    #[doc = "Data. Data transfer expected (read/write)"]
    DATA,
}
impl DATA_EXPECTEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATA_EXPECTEDW::NONE => false,
            DATA_EXPECTEDW::DATA => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA_EXPECTEDW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_EXPECTEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA_EXPECTEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "None. No data transfer expected (read/write)"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(DATA_EXPECTEDW::NONE)
    }
    #[doc = "Data. Data transfer expected (read/write)"]
    #[inline]
    pub fn data(self) -> &'a mut W {
        self.variant(DATA_EXPECTEDW::DATA)
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
#[doc = "Values that can be written to the field `READ_WRITE`"]
pub enum READ_WRITEW {
    #[doc = "Read from card"]
    READ_FROM_CARD,
    #[doc = "Write to card"]
    WRITE_TO_CARD,
}
impl READ_WRITEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            READ_WRITEW::READ_FROM_CARD => false,
            READ_WRITEW::WRITE_TO_CARD => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _READ_WRITEW<'a> {
    w: &'a mut W,
}
impl<'a> _READ_WRITEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: READ_WRITEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read from card"]
    #[inline]
    pub fn read_from_card(self) -> &'a mut W {
        self.variant(READ_WRITEW::READ_FROM_CARD)
    }
    #[doc = "Write to card"]
    #[inline]
    pub fn write_to_card(self) -> &'a mut W {
        self.variant(READ_WRITEW::WRITE_TO_CARD)
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
#[doc = "Values that can be written to the field `TRANSFER_MODE`"]
pub enum TRANSFER_MODEW {
    #[doc = "Block data transfer command"]
    BLOCK_DATA_TRANSFER,
    #[doc = "Stream data transfer command"]
    STREAM_DATA_TRANSFER,
}
impl TRANSFER_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRANSFER_MODEW::BLOCK_DATA_TRANSFER => false,
            TRANSFER_MODEW::STREAM_DATA_TRANSFER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRANSFER_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _TRANSFER_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRANSFER_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Block data transfer command"]
    #[inline]
    pub fn block_data_transfer(self) -> &'a mut W {
        self.variant(TRANSFER_MODEW::BLOCK_DATA_TRANSFER)
    }
    #[doc = "Stream data transfer command"]
    #[inline]
    pub fn stream_data_transfer(self) -> &'a mut W {
        self.variant(TRANSFER_MODEW::STREAM_DATA_TRANSFER)
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
#[doc = "Values that can be written to the field `SEND_AUTO_STOP`"]
pub enum SEND_AUTO_STOPW {
    #[doc = "No stop command sent at end of data transfer"]
    NO_STOP_COMMAND_SENT,
    #[doc = "Send stop command at end of data transfer"]
    SEND_STOP_COMMAND_AT,
}
impl SEND_AUTO_STOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEND_AUTO_STOPW::NO_STOP_COMMAND_SENT => false,
            SEND_AUTO_STOPW::SEND_STOP_COMMAND_AT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEND_AUTO_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _SEND_AUTO_STOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEND_AUTO_STOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No stop command sent at end of data transfer"]
    #[inline]
    pub fn no_stop_command_sent(self) -> &'a mut W {
        self.variant(SEND_AUTO_STOPW::NO_STOP_COMMAND_SENT)
    }
    #[doc = "Send stop command at end of data transfer"]
    #[inline]
    pub fn send_stop_command_at(self) -> &'a mut W {
        self.variant(SEND_AUTO_STOPW::SEND_STOP_COMMAND_AT)
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
#[doc = "Values that can be written to the field `WAIT_PRVDATA_COMPLETE`"]
pub enum WAIT_PRVDATA_COMPLETEW {
    #[doc = "Send. Send command at once, even if previous data transfer has not completed."]
    SEND,
    #[doc = "Wait. Wait for previous data transfer completion before sending command."]
    WAIT,
}
impl WAIT_PRVDATA_COMPLETEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAIT_PRVDATA_COMPLETEW::SEND => false,
            WAIT_PRVDATA_COMPLETEW::WAIT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAIT_PRVDATA_COMPLETEW<'a> {
    w: &'a mut W,
}
impl<'a> _WAIT_PRVDATA_COMPLETEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAIT_PRVDATA_COMPLETEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Send. Send command at once, even if previous data transfer has not completed."]
    #[inline]
    pub fn send(self) -> &'a mut W {
        self.variant(WAIT_PRVDATA_COMPLETEW::SEND)
    }
    #[doc = "Wait. Wait for previous data transfer completion before sending command."]
    #[inline]
    pub fn wait(self) -> &'a mut W {
        self.variant(WAIT_PRVDATA_COMPLETEW::WAIT)
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
#[doc = "Values that can be written to the field `STOP_ABORT_CMD`"]
pub enum STOP_ABORT_CMDW {
    #[doc = "Disabled. Neither stop nor abort command to stop current data transfer in progress. If abort is sent to function-number currently selected or not in data-transfer mode, then bit should be set to 0."]
    DISABLED,
    #[doc = "Enabled. Stop or abort command intended to stop current data transfer in progress."]
    ENABLED,
}
impl STOP_ABORT_CMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STOP_ABORT_CMDW::DISABLED => false,
            STOP_ABORT_CMDW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STOP_ABORT_CMDW<'a> {
    w: &'a mut W,
}
impl<'a> _STOP_ABORT_CMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STOP_ABORT_CMDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Neither stop nor abort command to stop current data transfer in progress. If abort is sent to function-number currently selected or not in data-transfer mode, then bit should be set to 0."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(STOP_ABORT_CMDW::DISABLED)
    }
    #[doc = "Enabled. Stop or abort command intended to stop current data transfer in progress."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(STOP_ABORT_CMDW::ENABLED)
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
#[doc = "Values that can be written to the field `SEND_INITIALIZATION`"]
pub enum SEND_INITIALIZATIONW {
    #[doc = "No. Do not send initialization sequence (80 clocks of 1) before sending this command."]
    NO,
    #[doc = "Send. Send initialization sequence before sending this command."]
    SEND,
}
impl SEND_INITIALIZATIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEND_INITIALIZATIONW::NO => false,
            SEND_INITIALIZATIONW::SEND => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEND_INITIALIZATIONW<'a> {
    w: &'a mut W,
}
impl<'a> _SEND_INITIALIZATIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEND_INITIALIZATIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No. Do not send initialization sequence (80 clocks of 1) before sending this command."]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(SEND_INITIALIZATIONW::NO)
    }
    #[doc = "Send. Send initialization sequence before sending this command."]
    #[inline]
    pub fn send(self) -> &'a mut W {
        self.variant(SEND_INITIALIZATIONW::SEND)
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
#[doc = "Values that can be written to the field `UPDATE_CLOCK_REGISTERS_ONLY`"]
pub enum UPDATE_CLOCK_REGISTERS_ONLYW {
    #[doc = "Normal. Normal command sequence"]
    NORMAL,
    #[doc = "No. Do not send commands, just update clock register value into card clock domain"]
    NO,
}
impl UPDATE_CLOCK_REGISTERS_ONLYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UPDATE_CLOCK_REGISTERS_ONLYW::NORMAL => false,
            UPDATE_CLOCK_REGISTERS_ONLYW::NO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UPDATE_CLOCK_REGISTERS_ONLYW<'a> {
    w: &'a mut W,
}
impl<'a> _UPDATE_CLOCK_REGISTERS_ONLYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UPDATE_CLOCK_REGISTERS_ONLYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. Normal command sequence"]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(UPDATE_CLOCK_REGISTERS_ONLYW::NORMAL)
    }
    #[doc = "No. Do not send commands, just update clock register value into card clock domain"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(UPDATE_CLOCK_REGISTERS_ONLYW::NO)
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
#[doc = "Values that can be written to the field `READ_CEATA_DEVICE`"]
pub enum READ_CEATA_DEVICEW {
    #[doc = "No read. Host is not performing read access (RW_REG or RW_BLK) towards CE-ATA device."]
    NO_READ,
    #[doc = "Read. Host is performing read access (RW_REG or RW_BLK) towards CE-ATA device."]
    READ,
}
impl READ_CEATA_DEVICEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            READ_CEATA_DEVICEW::NO_READ => false,
            READ_CEATA_DEVICEW::READ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _READ_CEATA_DEVICEW<'a> {
    w: &'a mut W,
}
impl<'a> _READ_CEATA_DEVICEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: READ_CEATA_DEVICEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No read. Host is not performing read access (RW_REG or RW_BLK) towards CE-ATA device."]
    #[inline]
    pub fn no_read(self) -> &'a mut W {
        self.variant(READ_CEATA_DEVICEW::NO_READ)
    }
    #[doc = "Read. Host is performing read access (RW_REG or RW_BLK) towards CE-ATA device."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(READ_CEATA_DEVICEW::READ)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CCS_EXPECTED`"]
pub enum CCS_EXPECTEDW {
    #[doc = "Disabled. Interrupts are not enabled in CE-ATA device (nIEN = 1 in ATA control register), or command does not expect CCS from device."]
    DISABLED,
    #[doc = "Enabled. Interrupts are enabled in CE-ATA device (nIEN = 0), and RW_BLK command expects command completion signal from CE-ATA device."]
    ENABLED,
}
impl CCS_EXPECTEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCS_EXPECTEDW::DISABLED => false,
            CCS_EXPECTEDW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCS_EXPECTEDW<'a> {
    w: &'a mut W,
}
impl<'a> _CCS_EXPECTEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCS_EXPECTEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Interrupts are not enabled in CE-ATA device (nIEN = 1 in ATA control register), or command does not expect CCS from device."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CCS_EXPECTEDW::DISABLED)
    }
    #[doc = "Enabled. Interrupts are enabled in CE-ATA device (nIEN = 0), and RW_BLK command expects command completion signal from CE-ATA device."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CCS_EXPECTEDW::ENABLED)
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
#[doc = r" Proxy"]
pub struct _ENABLE_BOOTW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_BOOTW<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EXPECT_BOOT_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _EXPECT_BOOT_ACKW<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DISABLE_BOOTW<'a> {
    w: &'a mut W,
}
impl<'a> _DISABLE_BOOTW<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BOOT_MODE`"]
pub enum BOOT_MODEW {
    #[doc = "Mandatory Boot operation"]
    MANDATORY_BOOT_OPERA,
    #[doc = "Alternate Boot operation"]
    ALTERNATE_BOOT_OPERA,
}
impl BOOT_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BOOT_MODEW::MANDATORY_BOOT_OPERA => false,
            BOOT_MODEW::ALTERNATE_BOOT_OPERA => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOOT_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _BOOT_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOOT_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Mandatory Boot operation"]
    #[inline]
    pub fn mandatory_boot_opera(self) -> &'a mut W {
        self.variant(BOOT_MODEW::MANDATORY_BOOT_OPERA)
    }
    #[doc = "Alternate Boot operation"]
    #[inline]
    pub fn alternate_boot_opera(self) -> &'a mut W {
        self.variant(BOOT_MODEW::ALTERNATE_BOOT_OPERA)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `VOLT_SWITCH`"]
pub enum VOLT_SWITCHW {
    #[doc = "Disabled. No voltage switching"]
    DISABLED,
    #[doc = "Enabled. Voltage switching enabled; must be set for CMD11 only"]
    ENABLED,
}
impl VOLT_SWITCHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VOLT_SWITCHW::DISABLED => false,
            VOLT_SWITCHW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VOLT_SWITCHW<'a> {
    w: &'a mut W,
}
impl<'a> _VOLT_SWITCHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VOLT_SWITCHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. No voltage switching"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VOLT_SWITCHW::DISABLED)
    }
    #[doc = "Enabled. Voltage switching enabled; must be set for CMD11 only"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VOLT_SWITCHW::ENABLED)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _START_CMDW<'a> {
    w: &'a mut W,
}
impl<'a> _START_CMDW<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:5 - Command index"]
    #[inline]
    pub fn cmd_index(&self) -> CMD_INDEXR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMD_INDEXR { bits }
    }
    #[doc = "Bit 6 - Response expect"]
    #[inline]
    pub fn response_expect(&self) -> RESPONSE_EXPECTR {
        RESPONSE_EXPECTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Response length"]
    #[inline]
    pub fn response_length(&self) -> RESPONSE_LENGTHR {
        RESPONSE_LENGTHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Check response crc. Some of command responses do not return valid CRC bits. Software should disable CRC checks for those commands in order to disable CRC checking by controller."]
    #[inline]
    pub fn check_response_crc(&self) -> CHECK_RESPONSE_CRCR {
        CHECK_RESPONSE_CRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Data expected"]
    #[inline]
    pub fn data_expected(&self) -> DATA_EXPECTEDR {
        DATA_EXPECTEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - read/write. Don't care if no data expected from card."]
    #[inline]
    pub fn read_write(&self) -> READ_WRITER {
        READ_WRITER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Transfer mode. Don't care if no data expected."]
    #[inline]
    pub fn transfer_mode(&self) -> TRANSFER_MODER {
        TRANSFER_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Send auto stop. When set, the SD/MMC interface sends stop command to SD_MMC_CEATA cards at end of data transfer. Refer to Table 339 to determine: - when send_auto_stop bit should be set, since some data transfers do not need explicit stop commands - open-ended transfers that software should explicitly send to stop command Additionally, when resume is sent to resume - suspended memory access of SD-Combo card - bit should be set correctly if suspended data transfer needs send_auto_stop. Don't care if no data expected from card."]
    #[inline]
    pub fn send_auto_stop(&self) -> SEND_AUTO_STOPR {
        SEND_AUTO_STOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Wait prvdata complete. The wait_prvdata_complete = 0 option typically used to query status of card during data transfer or to stop current data transfer; card_number should be same as in previous command."]
    #[inline]
    pub fn wait_prvdata_complete(&self) -> WAIT_PRVDATA_COMPLETER {
        WAIT_PRVDATA_COMPLETER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Stop abort command. When open-ended or predefined data transfer is in progress, and host issues stop or abort command to stop data transfer, bit should be set so that command/data state-machines of CIU can return correctly to idle state. This is also applicable for Boot mode transfers. To Abort boot mode, this bit should be set along with CMD[26] = disable_boot."]
    #[inline]
    pub fn stop_abort_cmd(&self) -> STOP_ABORT_CMDR {
        STOP_ABORT_CMDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Send initialization. After power on, 80 clocks must be sent to card for initialization before sending any commands to card. Bit should be set while sending first command to card so that controller will initialize clocks before sending command to card. This bit should not be set for either of the boot modes (alternate or mandatory)."]
    #[inline]
    pub fn send_initialization(&self) -> SEND_INITIALIZATIONR {
        SEND_INITIALIZATIONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Update clock registers only. Following register values transferred into card clock domain: CLKDIV, CLRSRC, CLKENA. Changes card clocks (change frequency, truncate off or on, and set low-frequency mode); provided in order to change clock frequency or stop clock without having to send command to cards. During normal command sequence, when update_clock_registers_only = 0, following control registers are transferred from BIU to CIU: CMD, CMDARG, TMOUT, CTYPE, BLKSIZ, BYTCNT. CIU uses new register values for new command sequence to card(s). When bit is set, there are no Command Done interrupts because no command is sent to SD_MMC_CEATA cards."]
    #[inline]
    pub fn update_clock_registers_only(&self) -> UPDATE_CLOCK_REGISTERS_ONLYR {
        UPDATE_CLOCK_REGISTERS_ONLYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Read ceata device. Software should set this bit to indicate that CE-ATA device is being accessed for read transfer. This bit is used to disable read data time-out indication while performing CE-ATA read transfers. Maximum value of I/O transmission delay can be no less than 10 seconds.The SD/MMC interface should not indicate read data time-out while waiting for data from CE-ATA device."]
    #[inline]
    pub fn read_ceata_device(&self) -> READ_CEATA_DEVICER {
        READ_CEATA_DEVICER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - CCS expected. If the command expects Command Completion Signal (CCS) from the CE-ATA device, the software should set this control bit. The SD/MMC controller sets the Data Transfer Over (DTO) bit in the RINTSTS register and generates an interrupt to the host if the Data Transfer Over interrupt is not masked."]
    #[inline]
    pub fn ccs_expected(&self) -> CCS_EXPECTEDR {
        CCS_EXPECTEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Enable Boot - this bit should be set only for mandatory boot mode. When Software sets this bit along with start_cmd, CIU starts the boot sequence for the corresponding card by asserting the CMD line low. Do NOT set disable_boot and enable_boot together."]
    #[inline]
    pub fn enable_boot(&self) -> ENABLE_BOOTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_BOOTR { bits }
    }
    #[doc = "Bit 25 - Expect Boot Acknowledge. When Software sets this bit along with enable_boot, CIU expects a boot acknowledge start pattern of 0-1-0 from the selected card."]
    #[inline]
    pub fn expect_boot_ack(&self) -> EXPECT_BOOT_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EXPECT_BOOT_ACKR { bits }
    }
    #[doc = "Bit 26 - Disable Boot. When software sets this bit along with start_cmd, CIU terminates the boot operation. Do NOT set disable_boot and enable_boot together."]
    #[inline]
    pub fn disable_boot(&self) -> DISABLE_BOOTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DISABLE_BOOTR { bits }
    }
    #[doc = "Bit 27 - Boot Mode"]
    #[inline]
    pub fn boot_mode(&self) -> BOOT_MODER {
        BOOT_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Voltage switch bit"]
    #[inline]
    pub fn volt_switch(&self) -> VOLT_SWITCHR {
        VOLT_SWITCHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Start command. Once command is taken by CIU, this bit is cleared. When bit is set, host should not attempt to write to any command registers. If write is attempted, hardware lock error is set in raw interrupt register. Once command is sent and response is received from SD_MMC_CEATA cards, Command Done bit is set in the raw interrupt register."]
    #[inline]
    pub fn start_cmd(&self) -> START_CMDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        START_CMDR { bits }
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
    #[doc = "Bits 0:5 - Command index"]
    #[inline]
    pub fn cmd_index(&mut self) -> _CMD_INDEXW {
        _CMD_INDEXW { w: self }
    }
    #[doc = "Bit 6 - Response expect"]
    #[inline]
    pub fn response_expect(&mut self) -> _RESPONSE_EXPECTW {
        _RESPONSE_EXPECTW { w: self }
    }
    #[doc = "Bit 7 - Response length"]
    #[inline]
    pub fn response_length(&mut self) -> _RESPONSE_LENGTHW {
        _RESPONSE_LENGTHW { w: self }
    }
    #[doc = "Bit 8 - Check response crc. Some of command responses do not return valid CRC bits. Software should disable CRC checks for those commands in order to disable CRC checking by controller."]
    #[inline]
    pub fn check_response_crc(&mut self) -> _CHECK_RESPONSE_CRCW {
        _CHECK_RESPONSE_CRCW { w: self }
    }
    #[doc = "Bit 9 - Data expected"]
    #[inline]
    pub fn data_expected(&mut self) -> _DATA_EXPECTEDW {
        _DATA_EXPECTEDW { w: self }
    }
    #[doc = "Bit 10 - read/write. Don't care if no data expected from card."]
    #[inline]
    pub fn read_write(&mut self) -> _READ_WRITEW {
        _READ_WRITEW { w: self }
    }
    #[doc = "Bit 11 - Transfer mode. Don't care if no data expected."]
    #[inline]
    pub fn transfer_mode(&mut self) -> _TRANSFER_MODEW {
        _TRANSFER_MODEW { w: self }
    }
    #[doc = "Bit 12 - Send auto stop. When set, the SD/MMC interface sends stop command to SD_MMC_CEATA cards at end of data transfer. Refer to Table 339 to determine: - when send_auto_stop bit should be set, since some data transfers do not need explicit stop commands - open-ended transfers that software should explicitly send to stop command Additionally, when resume is sent to resume - suspended memory access of SD-Combo card - bit should be set correctly if suspended data transfer needs send_auto_stop. Don't care if no data expected from card."]
    #[inline]
    pub fn send_auto_stop(&mut self) -> _SEND_AUTO_STOPW {
        _SEND_AUTO_STOPW { w: self }
    }
    #[doc = "Bit 13 - Wait prvdata complete. The wait_prvdata_complete = 0 option typically used to query status of card during data transfer or to stop current data transfer; card_number should be same as in previous command."]
    #[inline]
    pub fn wait_prvdata_complete(&mut self) -> _WAIT_PRVDATA_COMPLETEW {
        _WAIT_PRVDATA_COMPLETEW { w: self }
    }
    #[doc = "Bit 14 - Stop abort command. When open-ended or predefined data transfer is in progress, and host issues stop or abort command to stop data transfer, bit should be set so that command/data state-machines of CIU can return correctly to idle state. This is also applicable for Boot mode transfers. To Abort boot mode, this bit should be set along with CMD[26] = disable_boot."]
    #[inline]
    pub fn stop_abort_cmd(&mut self) -> _STOP_ABORT_CMDW {
        _STOP_ABORT_CMDW { w: self }
    }
    #[doc = "Bit 15 - Send initialization. After power on, 80 clocks must be sent to card for initialization before sending any commands to card. Bit should be set while sending first command to card so that controller will initialize clocks before sending command to card. This bit should not be set for either of the boot modes (alternate or mandatory)."]
    #[inline]
    pub fn send_initialization(&mut self) -> _SEND_INITIALIZATIONW {
        _SEND_INITIALIZATIONW { w: self }
    }
    #[doc = "Bit 21 - Update clock registers only. Following register values transferred into card clock domain: CLKDIV, CLRSRC, CLKENA. Changes card clocks (change frequency, truncate off or on, and set low-frequency mode); provided in order to change clock frequency or stop clock without having to send command to cards. During normal command sequence, when update_clock_registers_only = 0, following control registers are transferred from BIU to CIU: CMD, CMDARG, TMOUT, CTYPE, BLKSIZ, BYTCNT. CIU uses new register values for new command sequence to card(s). When bit is set, there are no Command Done interrupts because no command is sent to SD_MMC_CEATA cards."]
    #[inline]
    pub fn update_clock_registers_only(&mut self) -> _UPDATE_CLOCK_REGISTERS_ONLYW {
        _UPDATE_CLOCK_REGISTERS_ONLYW { w: self }
    }
    #[doc = "Bit 22 - Read ceata device. Software should set this bit to indicate that CE-ATA device is being accessed for read transfer. This bit is used to disable read data time-out indication while performing CE-ATA read transfers. Maximum value of I/O transmission delay can be no less than 10 seconds.The SD/MMC interface should not indicate read data time-out while waiting for data from CE-ATA device."]
    #[inline]
    pub fn read_ceata_device(&mut self) -> _READ_CEATA_DEVICEW {
        _READ_CEATA_DEVICEW { w: self }
    }
    #[doc = "Bit 23 - CCS expected. If the command expects Command Completion Signal (CCS) from the CE-ATA device, the software should set this control bit. The SD/MMC controller sets the Data Transfer Over (DTO) bit in the RINTSTS register and generates an interrupt to the host if the Data Transfer Over interrupt is not masked."]
    #[inline]
    pub fn ccs_expected(&mut self) -> _CCS_EXPECTEDW {
        _CCS_EXPECTEDW { w: self }
    }
    #[doc = "Bit 24 - Enable Boot - this bit should be set only for mandatory boot mode. When Software sets this bit along with start_cmd, CIU starts the boot sequence for the corresponding card by asserting the CMD line low. Do NOT set disable_boot and enable_boot together."]
    #[inline]
    pub fn enable_boot(&mut self) -> _ENABLE_BOOTW {
        _ENABLE_BOOTW { w: self }
    }
    #[doc = "Bit 25 - Expect Boot Acknowledge. When Software sets this bit along with enable_boot, CIU expects a boot acknowledge start pattern of 0-1-0 from the selected card."]
    #[inline]
    pub fn expect_boot_ack(&mut self) -> _EXPECT_BOOT_ACKW {
        _EXPECT_BOOT_ACKW { w: self }
    }
    #[doc = "Bit 26 - Disable Boot. When software sets this bit along with start_cmd, CIU terminates the boot operation. Do NOT set disable_boot and enable_boot together."]
    #[inline]
    pub fn disable_boot(&mut self) -> _DISABLE_BOOTW {
        _DISABLE_BOOTW { w: self }
    }
    #[doc = "Bit 27 - Boot Mode"]
    #[inline]
    pub fn boot_mode(&mut self) -> _BOOT_MODEW {
        _BOOT_MODEW { w: self }
    }
    #[doc = "Bit 28 - Voltage switch bit"]
    #[inline]
    pub fn volt_switch(&mut self) -> _VOLT_SWITCHW {
        _VOLT_SWITCHW { w: self }
    }
    #[doc = "Bit 31 - Start command. Once command is taken by CIU, this bit is cleared. When bit is set, host should not attempt to write to any command registers. If write is attempted, hardware lock error is set in raw interrupt register. Once command is sent and response is received from SD_MMC_CEATA cards, Command Done bit is set in the raw interrupt register."]
    #[inline]
    pub fn start_cmd(&mut self) -> _START_CMDW {
        _START_CMDW { w: self }
    }
}
