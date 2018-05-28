#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::BASE_STAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct BASE_APB3_CLK_INDR {
    bits: bool,
}
impl BASE_APB3_CLK_INDR {
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
pub struct BASE_APB1_CLK_INDR {
    bits: bool,
}
impl BASE_APB1_CLK_INDR {
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
pub struct BASE_SPIFI_CLK_INDR {
    bits: bool,
}
impl BASE_SPIFI_CLK_INDR {
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
pub struct BASE_M3_CLK_INDR {
    bits: bool,
}
impl BASE_M3_CLK_INDR {
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
pub struct BASE_USB0_CLK_INDR {
    bits: bool,
}
impl BASE_USB0_CLK_INDR {
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
pub struct BASE_USB1_CLK_INDR {
    bits: bool,
}
impl BASE_USB1_CLK_INDR {
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
    #[doc = "Bit 0 - Base clock indicator for BASE_APB3_CLK 0 = All branch clocks switched off. 1 = At least one branch clock running."]
    #[inline]
    pub fn base_apb3_clk_ind(&self) -> BASE_APB3_CLK_INDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BASE_APB3_CLK_INDR { bits }
    }
    #[doc = "Bit 1 - Base clock indicator for BASE_APB1_CLK 0 = All branch clocks switched off. 1 = At least one branch clock running."]
    #[inline]
    pub fn base_apb1_clk_ind(&self) -> BASE_APB1_CLK_INDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BASE_APB1_CLK_INDR { bits }
    }
    #[doc = "Bit 2 - Base clock indicator for BASE_SPIFI_CLK 0 = All branch clocks switched off. 1 = At least one branch clock running."]
    #[inline]
    pub fn base_spifi_clk_ind(&self) -> BASE_SPIFI_CLK_INDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BASE_SPIFI_CLK_INDR { bits }
    }
    #[doc = "Bit 3 - Base clock indicator for BASE_M3_CLK 0 = All branch clocks switched off. 1 = At least one branch clock running."]
    #[inline]
    pub fn base_m3_clk_ind(&self) -> BASE_M3_CLK_INDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BASE_M3_CLK_INDR { bits }
    }
    #[doc = "Bit 7 - Base clock indicator for BASE_USB0_CLK 0 = All branch clocks switched off. 1 = At least one branch clock running."]
    #[inline]
    pub fn base_usb0_clk_ind(&self) -> BASE_USB0_CLK_INDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BASE_USB0_CLK_INDR { bits }
    }
    #[doc = "Bit 8 - Base clock indicator for BASE_USB1_CLK 0 = All branch clocks switched off. 1 = at least one branch clock running."]
    #[inline]
    pub fn base_usb1_clk_ind(&self) -> BASE_USB1_CLK_INDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BASE_USB1_CLK_INDR { bits }
    }
}
