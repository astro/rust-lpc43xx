#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DMA_MFRM_BUFOF {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct FMCR {
    bits: u16,
}
impl FMCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OCR {
    bits: bool,
}
impl OCR {
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
pub struct FMAR {
    bits: u16,
}
impl FMAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OFR {
    bits: bool,
}
impl OFR {
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
    #[doc = "Bits 0:15 - Number of frames missed This register field can be read by the application (Read), can be set to 1 by the Ethernet core on a certain internal event (Self Set), and is automatically cleared to 0 on a register read. A register write of 0 has no effect on this field. Indicates the number of frames missed by the controller due to the Host Receive Buffer being unavailable. This counter is incremented each time the DMA discards an incoming frame. The counter is cleared when this register is read with."]
    #[inline]
    pub fn fmc(&self) -> FMCR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        FMCR { bits }
    }
    #[doc = "Bit 16 - Overflow bit for missed frame counter This register field can be read by the application (Read), can be set to 1 by the Ethernet core on a certain internal event (Self Set), and is automatically cleared to 0 on a register read. A register write of 0 has no effect on this field."]
    #[inline]
    pub fn oc(&self) -> OCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OCR { bits }
    }
    #[doc = "Bits 17:27 - Number of frames missed by the application This register field can be read by the application (Read), can be set to 1 by the Ethernet core on a certain internal event (Self Set), and is automatically cleared to 0 on a register read. A register write of 0 has no effect on this field. Indicates the number of frames missed by the application. This counter is incremented each time the MTL asserts the sideband signal. The counter is cleared when this register is read with ."]
    #[inline]
    pub fn fma(&self) -> FMAR {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        FMAR { bits }
    }
    #[doc = "Bit 28 - Overflow bit for FIFO overflow counter This register field can be read by the application (Read), can be set to 1 by the Ethernet core on a certain internal event (Self Set), and is automatically cleared to 0 on a register read. A register write of 0 has no effect on this field."]
    #[inline]
    pub fn of(&self) -> OFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OFR { bits }
    }
}
