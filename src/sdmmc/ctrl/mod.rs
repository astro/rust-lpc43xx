#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
#[doc = "Possible values of the field `CONTROLLER_RESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONTROLLER_RESETR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Reset. Reset SD/MMC controller"]
    RESET,
}
impl CONTROLLER_RESETR {
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
            CONTROLLER_RESETR::NO_CHANGE => false,
            CONTROLLER_RESETR::RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CONTROLLER_RESETR {
        match value {
            false => CONTROLLER_RESETR::NO_CHANGE,
            true => CONTROLLER_RESETR::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == CONTROLLER_RESETR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == CONTROLLER_RESETR::RESET
    }
}
#[doc = "Possible values of the field `FIFO_RESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_RESETR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Reset. Reset to data FIFO To reset FIFO pointers"]
    RESET,
}
impl FIFO_RESETR {
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
            FIFO_RESETR::NO_CHANGE => false,
            FIFO_RESETR::RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FIFO_RESETR {
        match value {
            false => FIFO_RESETR::NO_CHANGE,
            true => FIFO_RESETR::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == FIFO_RESETR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == FIFO_RESETR::RESET
    }
}
#[doc = "Possible values of the field `DMA_RESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_RESETR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Reset. Reset internal DMA interface control logic"]
    RESET,
}
impl DMA_RESETR {
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
            DMA_RESETR::NO_CHANGE => false,
            DMA_RESETR::RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA_RESETR {
        match value {
            false => DMA_RESETR::NO_CHANGE,
            true => DMA_RESETR::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == DMA_RESETR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == DMA_RESETR::RESET
    }
}
#[doc = "Possible values of the field `INT_ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_ENABLER {
    #[doc = "Disable interrupts"]
    DISABLE_INTERRUPTS,
    #[doc = "Enable interrupts"]
    ENABLE_INTERRUPTS,
}
impl INT_ENABLER {
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
            INT_ENABLER::DISABLE_INTERRUPTS => false,
            INT_ENABLER::ENABLE_INTERRUPTS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT_ENABLER {
        match value {
            false => INT_ENABLER::DISABLE_INTERRUPTS,
            true => INT_ENABLER::ENABLE_INTERRUPTS,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_INTERRUPTS`"]
    #[inline]
    pub fn is_disable_interrupts(&self) -> bool {
        *self == INT_ENABLER::DISABLE_INTERRUPTS
    }
    #[doc = "Checks if the value of the field is `ENABLE_INTERRUPTS`"]
    #[inline]
    pub fn is_enable_interrupts(&self) -> bool {
        *self == INT_ENABLER::ENABLE_INTERRUPTS
    }
}
#[doc = "Possible values of the field `READ_WAIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READ_WAITR {
    #[doc = "Clear read wait"]
    CLEAR_READ_WAIT,
    #[doc = "Assert read wait"]
    ASSERT_READ_WAIT,
}
impl READ_WAITR {
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
            READ_WAITR::CLEAR_READ_WAIT => false,
            READ_WAITR::ASSERT_READ_WAIT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> READ_WAITR {
        match value {
            false => READ_WAITR::CLEAR_READ_WAIT,
            true => READ_WAITR::ASSERT_READ_WAIT,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR_READ_WAIT`"]
    #[inline]
    pub fn is_clear_read_wait(&self) -> bool {
        *self == READ_WAITR::CLEAR_READ_WAIT
    }
    #[doc = "Checks if the value of the field is `ASSERT_READ_WAIT`"]
    #[inline]
    pub fn is_assert_read_wait(&self) -> bool {
        *self == READ_WAITR::ASSERT_READ_WAIT
    }
}
#[doc = "Possible values of the field `SEND_IRQ_RESPONSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEND_IRQ_RESPONSER {
    #[doc = "No change"]
    NO_CHANGE,
    #[doc = "Send auto IRQ response"]
    SEND_AUTO_IRQ_RESPON,
}
impl SEND_IRQ_RESPONSER {
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
            SEND_IRQ_RESPONSER::NO_CHANGE => false,
            SEND_IRQ_RESPONSER::SEND_AUTO_IRQ_RESPON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEND_IRQ_RESPONSER {
        match value {
            false => SEND_IRQ_RESPONSER::NO_CHANGE,
            true => SEND_IRQ_RESPONSER::SEND_AUTO_IRQ_RESPON,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == SEND_IRQ_RESPONSER::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SEND_AUTO_IRQ_RESPON`"]
    #[inline]
    pub fn is_send_auto_irq_respon(&self) -> bool {
        *self == SEND_IRQ_RESPONSER::SEND_AUTO_IRQ_RESPON
    }
}
#[doc = "Possible values of the field `ABORT_READ_DATA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABORT_READ_DATAR {
    #[doc = "No change"]
    NO_CHANGE,
    #[doc = "Abort. After suspend command is issued during read-transfer, software polls card to find when suspend happened. Once suspend occurs, software sets bit to reset data state-machine, which is waiting for next block of data. This bit automatically clears once data state machine resets to idle. Used in SDIO card suspend sequence."]
    ABORT,
}
impl ABORT_READ_DATAR {
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
            ABORT_READ_DATAR::NO_CHANGE => false,
            ABORT_READ_DATAR::ABORT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ABORT_READ_DATAR {
        match value {
            false => ABORT_READ_DATAR::NO_CHANGE,
            true => ABORT_READ_DATAR::ABORT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == ABORT_READ_DATAR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `ABORT`"]
    #[inline]
    pub fn is_abort(&self) -> bool {
        *self == ABORT_READ_DATAR::ABORT
    }
}
#[doc = "Possible values of the field `SEND_CCSD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEND_CCSDR {
    #[doc = "Clear bit if the SD/MMC controller does not reset the bit."]
    CLEAR_BIT,
    #[doc = "Send Command Completion Signal Disable (CCSD) to CE-ATA device"]
    SEND_COMMAND_COMPLET,
}
impl SEND_CCSDR {
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
            SEND_CCSDR::CLEAR_BIT => false,
            SEND_CCSDR::SEND_COMMAND_COMPLET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEND_CCSDR {
        match value {
            false => SEND_CCSDR::CLEAR_BIT,
            true => SEND_CCSDR::SEND_COMMAND_COMPLET,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR_BIT`"]
    #[inline]
    pub fn is_clear_bit_(&self) -> bool {
        *self == SEND_CCSDR::CLEAR_BIT
    }
    #[doc = "Checks if the value of the field is `SEND_COMMAND_COMPLET`"]
    #[inline]
    pub fn is_send_command_complet(&self) -> bool {
        *self == SEND_CCSDR::SEND_COMMAND_COMPLET
    }
}
#[doc = "Possible values of the field `SEND_AUTO_STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEND_AUTO_STOPR {
    #[doc = "Clear this bit if the SD/MMC controller does not reset the bit."]
    CLEAR_THIS_BIT_IF_TH,
    #[doc = "Send internally generated STOP after sending CCSD to CE-ATA device."]
    SEND_INTERNALLY_GENE,
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
            SEND_AUTO_STOPR::CLEAR_THIS_BIT_IF_TH => false,
            SEND_AUTO_STOPR::SEND_INTERNALLY_GENE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEND_AUTO_STOPR {
        match value {
            false => SEND_AUTO_STOPR::CLEAR_THIS_BIT_IF_TH,
            true => SEND_AUTO_STOPR::SEND_INTERNALLY_GENE,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR_THIS_BIT_IF_TH`"]
    #[inline]
    pub fn is_clear_this_bit_if_th(&self) -> bool {
        *self == SEND_AUTO_STOPR::CLEAR_THIS_BIT_IF_TH
    }
    #[doc = "Checks if the value of the field is `SEND_INTERNALLY_GENE`"]
    #[inline]
    pub fn is_send_internally_gene(&self) -> bool {
        *self == SEND_AUTO_STOPR::SEND_INTERNALLY_GENE
    }
}
#[doc = "Possible values of the field `CEATA_DEVICE_INTERRUPT_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEATA_DEVICE_INTERRUPT_STATUSR {
    #[doc = "Disabled. Interrupts not enabled in CE-ATA device (nIEN = 1 in ATA control register)"]
    DISABLED,
    #[doc = "Enabled. Interrupts are enabled in CE-ATA device (nIEN = 0 in ATA control register)"]
    ENABLED,
}
impl CEATA_DEVICE_INTERRUPT_STATUSR {
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
            CEATA_DEVICE_INTERRUPT_STATUSR::DISABLED => false,
            CEATA_DEVICE_INTERRUPT_STATUSR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CEATA_DEVICE_INTERRUPT_STATUSR {
        match value {
            false => CEATA_DEVICE_INTERRUPT_STATUSR::DISABLED,
            true => CEATA_DEVICE_INTERRUPT_STATUSR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CEATA_DEVICE_INTERRUPT_STATUSR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CEATA_DEVICE_INTERRUPT_STATUSR::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct CARD_VOLTAGE_A0R {
    bits: bool,
}
impl CARD_VOLTAGE_A0R {
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
pub struct CARD_VOLTAGE_A1R {
    bits: bool,
}
impl CARD_VOLTAGE_A1R {
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
pub struct CARD_VOLTAGE_A2R {
    bits: bool,
}
impl CARD_VOLTAGE_A2R {
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
#[doc = "Possible values of the field `USE_INTERNAL_DMAC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USE_INTERNAL_DMACR {
    #[doc = "Host. The host performs data transfers through the slave interface"]
    HOST,
    #[doc = "DMA. Internal DMA used for data transfer"]
    DMA,
}
impl USE_INTERNAL_DMACR {
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
            USE_INTERNAL_DMACR::HOST => false,
            USE_INTERNAL_DMACR::DMA => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USE_INTERNAL_DMACR {
        match value {
            false => USE_INTERNAL_DMACR::HOST,
            true => USE_INTERNAL_DMACR::DMA,
        }
    }
    #[doc = "Checks if the value of the field is `HOST`"]
    #[inline]
    pub fn is_host(&self) -> bool {
        *self == USE_INTERNAL_DMACR::HOST
    }
    #[doc = "Checks if the value of the field is `DMA`"]
    #[inline]
    pub fn is_dma(&self) -> bool {
        *self == USE_INTERNAL_DMACR::DMA
    }
}
#[doc = "Values that can be written to the field `CONTROLLER_RESET`"]
pub enum CONTROLLER_RESETW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Reset. Reset SD/MMC controller"]
    RESET,
}
impl CONTROLLER_RESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CONTROLLER_RESETW::NO_CHANGE => false,
            CONTROLLER_RESETW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CONTROLLER_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _CONTROLLER_RESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CONTROLLER_RESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline]
    pub fn no_change(self) -> &'a mut W {
        self.variant(CONTROLLER_RESETW::NO_CHANGE)
    }
    #[doc = "Reset. Reset SD/MMC controller"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(CONTROLLER_RESETW::RESET)
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
#[doc = "Values that can be written to the field `FIFO_RESET`"]
pub enum FIFO_RESETW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Reset. Reset to data FIFO To reset FIFO pointers"]
    RESET,
}
impl FIFO_RESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FIFO_RESETW::NO_CHANGE => false,
            FIFO_RESETW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FIFO_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _FIFO_RESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FIFO_RESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline]
    pub fn no_change(self) -> &'a mut W {
        self.variant(FIFO_RESETW::NO_CHANGE)
    }
    #[doc = "Reset. Reset to data FIFO To reset FIFO pointers"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(FIFO_RESETW::RESET)
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
#[doc = "Values that can be written to the field `DMA_RESET`"]
pub enum DMA_RESETW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Reset. Reset internal DMA interface control logic"]
    RESET,
}
impl DMA_RESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMA_RESETW::NO_CHANGE => false,
            DMA_RESETW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_RESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_RESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline]
    pub fn no_change(self) -> &'a mut W {
        self.variant(DMA_RESETW::NO_CHANGE)
    }
    #[doc = "Reset. Reset internal DMA interface control logic"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(DMA_RESETW::RESET)
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
#[doc = "Values that can be written to the field `INT_ENABLE`"]
pub enum INT_ENABLEW {
    #[doc = "Disable interrupts"]
    DISABLE_INTERRUPTS,
    #[doc = "Enable interrupts"]
    ENABLE_INTERRUPTS,
}
impl INT_ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT_ENABLEW::DISABLE_INTERRUPTS => false,
            INT_ENABLEW::ENABLE_INTERRUPTS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _INT_ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT_ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable interrupts"]
    #[inline]
    pub fn disable_interrupts(self) -> &'a mut W {
        self.variant(INT_ENABLEW::DISABLE_INTERRUPTS)
    }
    #[doc = "Enable interrupts"]
    #[inline]
    pub fn enable_interrupts(self) -> &'a mut W {
        self.variant(INT_ENABLEW::ENABLE_INTERRUPTS)
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
#[doc = "Values that can be written to the field `READ_WAIT`"]
pub enum READ_WAITW {
    #[doc = "Clear read wait"]
    CLEAR_READ_WAIT,
    #[doc = "Assert read wait"]
    ASSERT_READ_WAIT,
}
impl READ_WAITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            READ_WAITW::CLEAR_READ_WAIT => false,
            READ_WAITW::ASSERT_READ_WAIT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _READ_WAITW<'a> {
    w: &'a mut W,
}
impl<'a> _READ_WAITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: READ_WAITW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear read wait"]
    #[inline]
    pub fn clear_read_wait(self) -> &'a mut W {
        self.variant(READ_WAITW::CLEAR_READ_WAIT)
    }
    #[doc = "Assert read wait"]
    #[inline]
    pub fn assert_read_wait(self) -> &'a mut W {
        self.variant(READ_WAITW::ASSERT_READ_WAIT)
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
#[doc = "Values that can be written to the field `SEND_IRQ_RESPONSE`"]
pub enum SEND_IRQ_RESPONSEW {
    #[doc = "No change"]
    NO_CHANGE,
    #[doc = "Send auto IRQ response"]
    SEND_AUTO_IRQ_RESPON,
}
impl SEND_IRQ_RESPONSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEND_IRQ_RESPONSEW::NO_CHANGE => false,
            SEND_IRQ_RESPONSEW::SEND_AUTO_IRQ_RESPON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEND_IRQ_RESPONSEW<'a> {
    w: &'a mut W,
}
impl<'a> _SEND_IRQ_RESPONSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEND_IRQ_RESPONSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No change"]
    #[inline]
    pub fn no_change(self) -> &'a mut W {
        self.variant(SEND_IRQ_RESPONSEW::NO_CHANGE)
    }
    #[doc = "Send auto IRQ response"]
    #[inline]
    pub fn send_auto_irq_respon(self) -> &'a mut W {
        self.variant(SEND_IRQ_RESPONSEW::SEND_AUTO_IRQ_RESPON)
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
#[doc = "Values that can be written to the field `ABORT_READ_DATA`"]
pub enum ABORT_READ_DATAW {
    #[doc = "No change"]
    NO_CHANGE,
    #[doc = "Abort. After suspend command is issued during read-transfer, software polls card to find when suspend happened. Once suspend occurs, software sets bit to reset data state-machine, which is waiting for next block of data. This bit automatically clears once data state machine resets to idle. Used in SDIO card suspend sequence."]
    ABORT,
}
impl ABORT_READ_DATAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ABORT_READ_DATAW::NO_CHANGE => false,
            ABORT_READ_DATAW::ABORT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ABORT_READ_DATAW<'a> {
    w: &'a mut W,
}
impl<'a> _ABORT_READ_DATAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ABORT_READ_DATAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No change"]
    #[inline]
    pub fn no_change(self) -> &'a mut W {
        self.variant(ABORT_READ_DATAW::NO_CHANGE)
    }
    #[doc = "Abort. After suspend command is issued during read-transfer, software polls card to find when suspend happened. Once suspend occurs, software sets bit to reset data state-machine, which is waiting for next block of data. This bit automatically clears once data state machine resets to idle. Used in SDIO card suspend sequence."]
    #[inline]
    pub fn abort(self) -> &'a mut W {
        self.variant(ABORT_READ_DATAW::ABORT)
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
#[doc = "Values that can be written to the field `SEND_CCSD`"]
pub enum SEND_CCSDW {
    #[doc = "Clear bit if the SD/MMC controller does not reset the bit."]
    CLEAR_BIT,
    #[doc = "Send Command Completion Signal Disable (CCSD) to CE-ATA device"]
    SEND_COMMAND_COMPLET,
}
impl SEND_CCSDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEND_CCSDW::CLEAR_BIT => false,
            SEND_CCSDW::SEND_COMMAND_COMPLET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEND_CCSDW<'a> {
    w: &'a mut W,
}
impl<'a> _SEND_CCSDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEND_CCSDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear bit if the SD/MMC controller does not reset the bit."]
    #[inline]
    pub fn clear_bit_(self) -> &'a mut W {
        self.variant(SEND_CCSDW::CLEAR_BIT)
    }
    #[doc = "Send Command Completion Signal Disable (CCSD) to CE-ATA device"]
    #[inline]
    pub fn send_command_complet(self) -> &'a mut W {
        self.variant(SEND_CCSDW::SEND_COMMAND_COMPLET)
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
#[doc = "Values that can be written to the field `SEND_AUTO_STOP`"]
pub enum SEND_AUTO_STOPW {
    #[doc = "Clear this bit if the SD/MMC controller does not reset the bit."]
    CLEAR_THIS_BIT_IF_TH,
    #[doc = "Send internally generated STOP after sending CCSD to CE-ATA device."]
    SEND_INTERNALLY_GENE,
}
impl SEND_AUTO_STOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEND_AUTO_STOPW::CLEAR_THIS_BIT_IF_TH => false,
            SEND_AUTO_STOPW::SEND_INTERNALLY_GENE => true,
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
    #[doc = "Clear this bit if the SD/MMC controller does not reset the bit."]
    #[inline]
    pub fn clear_this_bit_if_th(self) -> &'a mut W {
        self.variant(SEND_AUTO_STOPW::CLEAR_THIS_BIT_IF_TH)
    }
    #[doc = "Send internally generated STOP after sending CCSD to CE-ATA device."]
    #[inline]
    pub fn send_internally_gene(self) -> &'a mut W {
        self.variant(SEND_AUTO_STOPW::SEND_INTERNALLY_GENE)
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
#[doc = "Values that can be written to the field `CEATA_DEVICE_INTERRUPT_STATUS`"]
pub enum CEATA_DEVICE_INTERRUPT_STATUSW {
    #[doc = "Disabled. Interrupts not enabled in CE-ATA device (nIEN = 1 in ATA control register)"]
    DISABLED,
    #[doc = "Enabled. Interrupts are enabled in CE-ATA device (nIEN = 0 in ATA control register)"]
    ENABLED,
}
impl CEATA_DEVICE_INTERRUPT_STATUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CEATA_DEVICE_INTERRUPT_STATUSW::DISABLED => false,
            CEATA_DEVICE_INTERRUPT_STATUSW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEATA_DEVICE_INTERRUPT_STATUSW<'a> {
    w: &'a mut W,
}
impl<'a> _CEATA_DEVICE_INTERRUPT_STATUSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEATA_DEVICE_INTERRUPT_STATUSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Interrupts not enabled in CE-ATA device (nIEN = 1 in ATA control register)"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CEATA_DEVICE_INTERRUPT_STATUSW::DISABLED)
    }
    #[doc = "Enabled. Interrupts are enabled in CE-ATA device (nIEN = 0 in ATA control register)"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CEATA_DEVICE_INTERRUPT_STATUSW::ENABLED)
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
#[doc = r" Proxy"]
pub struct _CARD_VOLTAGE_A0W<'a> {
    w: &'a mut W,
}
impl<'a> _CARD_VOLTAGE_A0W<'a> {
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
pub struct _CARD_VOLTAGE_A1W<'a> {
    w: &'a mut W,
}
impl<'a> _CARD_VOLTAGE_A1W<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CARD_VOLTAGE_A2W<'a> {
    w: &'a mut W,
}
impl<'a> _CARD_VOLTAGE_A2W<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USE_INTERNAL_DMAC`"]
pub enum USE_INTERNAL_DMACW {
    #[doc = "Host. The host performs data transfers through the slave interface"]
    HOST,
    #[doc = "DMA. Internal DMA used for data transfer"]
    DMA,
}
impl USE_INTERNAL_DMACW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USE_INTERNAL_DMACW::HOST => false,
            USE_INTERNAL_DMACW::DMA => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USE_INTERNAL_DMACW<'a> {
    w: &'a mut W,
}
impl<'a> _USE_INTERNAL_DMACW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USE_INTERNAL_DMACW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Host. The host performs data transfers through the slave interface"]
    #[inline]
    pub fn host(self) -> &'a mut W {
        self.variant(USE_INTERNAL_DMACW::HOST)
    }
    #[doc = "DMA. Internal DMA used for data transfer"]
    #[inline]
    pub fn dma(self) -> &'a mut W {
        self.variant(USE_INTERNAL_DMACW::DMA)
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
        const OFFSET: u8 = 25;
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
    #[doc = "Bit 0 - Controller reset. To reset controller, software should set bit to 1. This bit is auto-cleared after two AHB and two cclk_in clock cycles. This resets: - BIU/CIU interface - CIU and state machines - abort_read_data, send_irq_response, and read_wait bits of Control register - start_cmd bit of Command register Does not affect any registers or DMA interface, or FIFO. or host interrupts."]
    #[inline]
    pub fn controller_reset(&self) -> CONTROLLER_RESETR {
        CONTROLLER_RESETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Fifo reset. To reset FIFO, software should set bit to 1. This bit is auto-cleared after completion of reset operation. auto-cleared after two AHB clocks."]
    #[inline]
    pub fn fifo_reset(&self) -> FIFO_RESETR {
        FIFO_RESETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Dma reset. To reset DMA interface, software should set bit to 1. This bit is auto-cleared after two AHB clocks."]
    #[inline]
    pub fn dma_reset(&self) -> DMA_RESETR {
        DMA_RESETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Global interrupt enable/disable bit. The int port is 1 only when this bit is 1 and one or more unmasked interrupts are set."]
    #[inline]
    pub fn int_enable(&self) -> INT_ENABLER {
        INT_ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Read/wait. For sending read-wait to SDIO cards."]
    #[inline]
    pub fn read_wait(&self) -> READ_WAITR {
        READ_WAITR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Send irq response. This bit automatically clears once response is sent. To wait for MMC card interrupts, the host issues CMD40, and the SD/MMC controller waits for an interrupt response from the MMC card. In the meantime, if the host wants the SD/MMC interface to exit waiting for interrupt state, it can set this bit, at which time the SD/MMC interface command state-machine sends a CMD40 response on the bus and returns to idle state."]
    #[inline]
    pub fn send_irq_response(&self) -> SEND_IRQ_RESPONSER {
        SEND_IRQ_RESPONSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Abort read data. Used in SDIO card suspend sequence."]
    #[inline]
    pub fn abort_read_data(&self) -> ABORT_READ_DATAR {
        ABORT_READ_DATAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Send ccsd. When set, the SD/MMC controller sends CCSD to the CE-ATA device. Software sets this bit only if current command is expecting CCS (that is, RW_BLK) and interrupts are enabled in CE-ATA device. Once the CCSD pattern is sent to device, the SD/MMC interface automatically clears send_ccsd bit. It also sets Command Done (CD) bit in RINTSTS register and generates interrupt to host if Command Done interrupt is not masked. NOTE: Once send_ccsd bit is set, it takes two card clock cycles to drive the CCSD on the CMD line. Due to this, during the boundary conditions it may happen that CCSD is sent to the CE-ATA device, even if the device signalled CCS."]
    #[inline]
    pub fn send_ccsd(&self) -> SEND_CCSDR {
        SEND_CCSDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Send auto stop ccsd. NOTE: Always set send_auto_stop_ccsd and send_ccsd bits together; send_auto_stop_ccsd should not be set independent of send_ccsd. When set, the SD/MMC interface automatically sends internallygenerated STOP command (CMD12) to CE-ATA device. After sending internally-generated STOP command, Auto Command Done (ACD) bit in RINTSTS is set and generates interrupt to host if Auto Command Done interrupt is not masked. After sending the CCSD, the SD/MMC interface automatically clears send_auto_stop_ccsd bit."]
    #[inline]
    pub fn send_auto_stop(&self) -> SEND_AUTO_STOPR {
        SEND_AUTO_STOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - CEATA device interrupt status. Software should appropriately write to this bit after power-on reset or any other reset to CE-ATA device. After reset, usually CE-ATA device interrupt is disabled (nIEN = 1). If the host enables CE-ATA device interrupt, then software should set this bit."]
    #[inline]
    pub fn ceata_device_interrupt_status(&self) -> CEATA_DEVICE_INTERRUPT_STATUSR {
        CEATA_DEVICE_INTERRUPT_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Controls the state of the SD_VOLT0 pin. SD/MMC card voltage control is not implemented."]
    #[inline]
    pub fn card_voltage_a0(&self) -> CARD_VOLTAGE_A0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CARD_VOLTAGE_A0R { bits }
    }
    #[doc = "Bit 17 - Controls the state of the SD_VOLT1 pin. SD/MMC card voltage control is not implemented."]
    #[inline]
    pub fn card_voltage_a1(&self) -> CARD_VOLTAGE_A1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CARD_VOLTAGE_A1R { bits }
    }
    #[doc = "Bit 18 - Controls the state of the SD_VOLT2 pin. SD/MMC card voltage control is not implemented."]
    #[inline]
    pub fn card_voltage_a2(&self) -> CARD_VOLTAGE_A2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CARD_VOLTAGE_A2R { bits }
    }
    #[doc = "Bit 25 - SD/MMC DMA use."]
    #[inline]
    pub fn use_internal_dmac(&self) -> USE_INTERNAL_DMACR {
        USE_INTERNAL_DMACR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
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
    #[doc = "Bit 0 - Controller reset. To reset controller, software should set bit to 1. This bit is auto-cleared after two AHB and two cclk_in clock cycles. This resets: - BIU/CIU interface - CIU and state machines - abort_read_data, send_irq_response, and read_wait bits of Control register - start_cmd bit of Command register Does not affect any registers or DMA interface, or FIFO. or host interrupts."]
    #[inline]
    pub fn controller_reset(&mut self) -> _CONTROLLER_RESETW {
        _CONTROLLER_RESETW { w: self }
    }
    #[doc = "Bit 1 - Fifo reset. To reset FIFO, software should set bit to 1. This bit is auto-cleared after completion of reset operation. auto-cleared after two AHB clocks."]
    #[inline]
    pub fn fifo_reset(&mut self) -> _FIFO_RESETW {
        _FIFO_RESETW { w: self }
    }
    #[doc = "Bit 2 - Dma reset. To reset DMA interface, software should set bit to 1. This bit is auto-cleared after two AHB clocks."]
    #[inline]
    pub fn dma_reset(&mut self) -> _DMA_RESETW {
        _DMA_RESETW { w: self }
    }
    #[doc = "Bit 4 - Global interrupt enable/disable bit. The int port is 1 only when this bit is 1 and one or more unmasked interrupts are set."]
    #[inline]
    pub fn int_enable(&mut self) -> _INT_ENABLEW {
        _INT_ENABLEW { w: self }
    }
    #[doc = "Bit 6 - Read/wait. For sending read-wait to SDIO cards."]
    #[inline]
    pub fn read_wait(&mut self) -> _READ_WAITW {
        _READ_WAITW { w: self }
    }
    #[doc = "Bit 7 - Send irq response. This bit automatically clears once response is sent. To wait for MMC card interrupts, the host issues CMD40, and the SD/MMC controller waits for an interrupt response from the MMC card. In the meantime, if the host wants the SD/MMC interface to exit waiting for interrupt state, it can set this bit, at which time the SD/MMC interface command state-machine sends a CMD40 response on the bus and returns to idle state."]
    #[inline]
    pub fn send_irq_response(&mut self) -> _SEND_IRQ_RESPONSEW {
        _SEND_IRQ_RESPONSEW { w: self }
    }
    #[doc = "Bit 8 - Abort read data. Used in SDIO card suspend sequence."]
    #[inline]
    pub fn abort_read_data(&mut self) -> _ABORT_READ_DATAW {
        _ABORT_READ_DATAW { w: self }
    }
    #[doc = "Bit 9 - Send ccsd. When set, the SD/MMC controller sends CCSD to the CE-ATA device. Software sets this bit only if current command is expecting CCS (that is, RW_BLK) and interrupts are enabled in CE-ATA device. Once the CCSD pattern is sent to device, the SD/MMC interface automatically clears send_ccsd bit. It also sets Command Done (CD) bit in RINTSTS register and generates interrupt to host if Command Done interrupt is not masked. NOTE: Once send_ccsd bit is set, it takes two card clock cycles to drive the CCSD on the CMD line. Due to this, during the boundary conditions it may happen that CCSD is sent to the CE-ATA device, even if the device signalled CCS."]
    #[inline]
    pub fn send_ccsd(&mut self) -> _SEND_CCSDW {
        _SEND_CCSDW { w: self }
    }
    #[doc = "Bit 10 - Send auto stop ccsd. NOTE: Always set send_auto_stop_ccsd and send_ccsd bits together; send_auto_stop_ccsd should not be set independent of send_ccsd. When set, the SD/MMC interface automatically sends internallygenerated STOP command (CMD12) to CE-ATA device. After sending internally-generated STOP command, Auto Command Done (ACD) bit in RINTSTS is set and generates interrupt to host if Auto Command Done interrupt is not masked. After sending the CCSD, the SD/MMC interface automatically clears send_auto_stop_ccsd bit."]
    #[inline]
    pub fn send_auto_stop(&mut self) -> _SEND_AUTO_STOPW {
        _SEND_AUTO_STOPW { w: self }
    }
    #[doc = "Bit 11 - CEATA device interrupt status. Software should appropriately write to this bit after power-on reset or any other reset to CE-ATA device. After reset, usually CE-ATA device interrupt is disabled (nIEN = 1). If the host enables CE-ATA device interrupt, then software should set this bit."]
    #[inline]
    pub fn ceata_device_interrupt_status(&mut self) -> _CEATA_DEVICE_INTERRUPT_STATUSW {
        _CEATA_DEVICE_INTERRUPT_STATUSW { w: self }
    }
    #[doc = "Bit 16 - Controls the state of the SD_VOLT0 pin. SD/MMC card voltage control is not implemented."]
    #[inline]
    pub fn card_voltage_a0(&mut self) -> _CARD_VOLTAGE_A0W {
        _CARD_VOLTAGE_A0W { w: self }
    }
    #[doc = "Bit 17 - Controls the state of the SD_VOLT1 pin. SD/MMC card voltage control is not implemented."]
    #[inline]
    pub fn card_voltage_a1(&mut self) -> _CARD_VOLTAGE_A1W {
        _CARD_VOLTAGE_A1W { w: self }
    }
    #[doc = "Bit 18 - Controls the state of the SD_VOLT2 pin. SD/MMC card voltage control is not implemented."]
    #[inline]
    pub fn card_voltage_a2(&mut self) -> _CARD_VOLTAGE_A2W {
        _CARD_VOLTAGE_A2W { w: self }
    }
    #[doc = "Bit 25 - SD/MMC DMA use."]
    #[inline]
    pub fn use_internal_dmac(&mut self) -> _USE_INTERNAL_DMACW {
        _USE_INTERNAL_DMACW { w: self }
    }
}
