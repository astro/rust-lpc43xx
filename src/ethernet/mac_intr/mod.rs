#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MAC_INTR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct PMTR {
    bits: bool,
}
impl PMTR {
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
pub struct TSR {
    bits: bool,
}
impl TSR {
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
    #[doc = "Bit 3 - PMT Interrupt Status This bit is set whenever a Magic packet or Wake-on-LAN frame is received in Power- Down mode (See bits 5 and 6 in Table 560). This bit is cleared when both bits[6:5] are cleared because of a read operation to the PMT Control and Status register."]
    #[inline]
    pub fn pmt(&self) -> PMTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PMTR { bits }
    }
    #[doc = "Bit 9 - Timestamp interrupt status When Advanced Timestamp feature is enabled, this bit is set when any of the following conditions is true: - The system time value equals or exceeds the value specified in the Target Time High and Low registers - There is an overflow in the seconds register This bit is cleared on reading the byte 0 of the Timestamp Status register (Table 576). Otherwise, when default Time stamping is enabled, this bit when set indicates that the system time value equals or exceeds the value specified in the Target Time registers. In this mode, this bit is cleared after the completion of the read of this Interrupt Status Register[9]. In all other modes, this bit is reserved."]
    #[inline]
    pub fn ts(&self) -> TSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSR { bits }
    }
}
