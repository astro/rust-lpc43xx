#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HCSPARAMS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct N_PORTSR {
    bits: u8,
}
impl N_PORTSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PPCR {
    bits: bool,
}
impl PPCR {
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
pub struct N_PCCR {
    bits: u8,
}
impl N_PCCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct N_CCR {
    bits: u8,
}
impl N_CCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PIR {
    bits: bool,
}
impl PIR {
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
pub struct N_PTTR {
    bits: u8,
}
impl N_PTTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct N_TTR {
    bits: u8,
}
impl N_TTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Number of downstream ports. This field specifies the number of physical downstream ports implemented on this host controller."]
    #[inline]
    pub fn n_ports(&self) -> N_PORTSR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        N_PORTSR { bits }
    }
    #[doc = "Bit 4 - Port Power Control. This field indicates whether the host controller implementation includes port power control."]
    #[inline]
    pub fn ppc(&self) -> PPCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PPCR { bits }
    }
    #[doc = "Bits 8:11 - Number of Ports per Companion Controller. This field indicates the number of ports supported per internal Companion Controller."]
    #[inline]
    pub fn n_pcc(&self) -> N_PCCR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        N_PCCR { bits }
    }
    #[doc = "Bits 12:15 - Number of Companion Controller. This field indicates the number of companion controllers associated with this USB2.0 host controller."]
    #[inline]
    pub fn n_cc(&self) -> N_CCR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        N_CCR { bits }
    }
    #[doc = "Bit 16 - Port indicators. This bit indicates whether the ports support port indicator control."]
    #[inline]
    pub fn pi(&self) -> PIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIR { bits }
    }
    #[doc = "Bits 20:23 - Number of Ports per Transaction Translator. This field indicates the number of ports assigned to each transaction translator within the USB2.0 host controller."]
    #[inline]
    pub fn n_ptt(&self) -> N_PTTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        N_PTTR { bits }
    }
    #[doc = "Bits 24:27 - Number of Transaction Translators. This field indicates the number of embedded transaction translators associated with the USB2.0 host controller."]
    #[inline]
    pub fn n_tt(&self) -> N_TTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        N_TTR { bits }
    }
}
