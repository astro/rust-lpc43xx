#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MAC_DEBUG {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct RXIDLESTATR {
    bits: bool,
}
impl RXIDLESTATR {
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
pub struct FIFOSTAT0R {
    bits: u8,
}
impl FIFOSTAT0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RXFIFOSTAT1R {
    bits: bool,
}
impl RXFIFOSTAT1R {
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
pub struct RXFIFOSTATR {
    bits: u8,
}
impl RXFIFOSTATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RXFIFOLVLR {
    bits: u8,
}
impl RXFIFOLVLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TXIDLESTATR {
    bits: bool,
}
impl TXIDLESTATR {
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
pub struct TXSTATR {
    bits: u8,
}
impl TXSTATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PAUSER {
    bits: bool,
}
impl PAUSER {
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
pub struct TXFIFOSTATR {
    bits: u8,
}
impl TXFIFOSTATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TXFIFOSTAT1R {
    bits: bool,
}
impl TXFIFOSTAT1R {
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
pub struct TXFIFOLVLR {
    bits: bool,
}
impl TXFIFOLVLR {
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
pub struct TXFIFOFULLR {
    bits: bool,
}
impl TXFIFOFULLR {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - When high, it indicates that the MAC MII receive protocol engine is actively receiving data and not in IDLE state."]
    #[inline]
    pub fn rxidlestat(&self) -> RXIDLESTATR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXIDLESTATR { bits }
    }
    #[doc = "Bits 1:2 - When high, it indicates the active state of the small FIFO Read and Write controllers respectively of the MAC receive Frame Controller module."]
    #[inline]
    pub fn fifostat0(&self) -> FIFOSTAT0R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FIFOSTAT0R { bits }
    }
    #[doc = "Bit 4 - When high, it indicates that the MTL RxFIFO Write Controller is active and transferring a received frame to the FIFO."]
    #[inline]
    pub fn rxfifostat1(&self) -> RXFIFOSTAT1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXFIFOSTAT1R { bits }
    }
    #[doc = "Bits 5:6 - State of the RxFIFO read Controller: 00 = idle state 01 = reading frame data 10 = reading frame status (or Time stamp) 11 = flushing the frame data and status"]
    #[inline]
    pub fn rxfifostat(&self) -> RXFIFOSTATR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RXFIFOSTATR { bits }
    }
    #[doc = "Bits 8:9 - Status of the RxFIFO Fill-level 00 = RxFIFO Empty 01 = RxFIFO fill-level below flow-control de-activate threshold 10 = RxFIFO fill-level above flow-control activate threshold 11 = RxFIFO Full"]
    #[inline]
    pub fn rxfifolvl(&self) -> RXFIFOLVLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RXFIFOLVLR { bits }
    }
    #[doc = "Bit 16 - When high, it indicates that the MAC MII transmit protocol engine is actively transmitting data and not in IDLE state."]
    #[inline]
    pub fn txidlestat(&self) -> TXIDLESTATR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXIDLESTATR { bits }
    }
    #[doc = "Bits 17:18 - State of the MAC Transmit Frame Controller module: 00 = idle 01 = Waiting for Status of previous frame or IFG/backoff period to be over 10 = Generating and transmitting a PAUSE control frame (in full duplex mode) 11 = Transferring input frame for transmission"]
    #[inline]
    pub fn txstat(&self) -> TXSTATR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXSTATR { bits }
    }
    #[doc = "Bit 19 - When high, it indicates that the MAC transmitter is in PAUSE condition (in full-duplex only) and hence will not schedule any frame for transmission."]
    #[inline]
    pub fn pause(&self) -> PAUSER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PAUSER { bits }
    }
    #[doc = "Bits 20:21 - State of the TxFIFO read Controller 00 = idle state 01 = READ state (transferring data to MAC transmitter) 10 = Waiting for TxStatus from MAC transmitter 11 = Writing the received TxStatus or flushing the TxFIFO"]
    #[inline]
    pub fn txfifostat(&self) -> TXFIFOSTATR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXFIFOSTATR { bits }
    }
    #[doc = "Bit 22 - When high, it indicates that the TxFIFO Write Controller is active and transferring data to the TxFIFO."]
    #[inline]
    pub fn txfifostat1(&self) -> TXFIFOSTAT1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXFIFOSTAT1R { bits }
    }
    #[doc = "Bit 24 - When high, it indicates that the TxFIFO is not empty and has some data left for transmission."]
    #[inline]
    pub fn txfifolvl(&self) -> TXFIFOLVLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXFIFOLVLR { bits }
    }
    #[doc = "Bit 25 - When high, it indicates that the TxStatus FIFO is full and hence the controller will not be accepting any more frames for transmission."]
    #[inline]
    pub fn txfifofull(&self) -> TXFIFOFULLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXFIFOFULLR { bits }
    }
}
