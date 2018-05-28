#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::INTEN {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct EE_PROG_DONER {
    bits: bool,
}
impl EE_PROG_DONER {
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
    #[doc = "Bit 2 - EEPROM program operation finished interrupt enable bit. Bit is: - set when one is written in the corresponding bit of the INTENSET register. - cleared when one is written to the corresponding bit of the INTENCLR register."]
    #[inline]
    pub fn ee_prog_done(&self) -> EE_PROG_DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EE_PROG_DONER { bits }
    }
}
