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
pub struct BASE_UART3_CLKR {
    bits: bool,
}
impl BASE_UART3_CLKR {
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
pub struct BASE_UART2_CLKR {
    bits: bool,
}
impl BASE_UART2_CLKR {
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
pub struct BASE_UART1_CLKR {
    bits: bool,
}
impl BASE_UART1_CLKR {
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
pub struct BASE_UART0_CLKR {
    bits: bool,
}
impl BASE_UART0_CLKR {
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
pub struct BASE_SSP1_CLKR {
    bits: bool,
}
impl BASE_SSP1_CLKR {
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
pub struct BASE_SSP0_CLKR {
    bits: bool,
}
impl BASE_SSP0_CLKR {
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
    #[doc = "Bit 1 - Base clock indicator for BASE_UART3_CLK 0 = All branch clocks switched off. 1 = At least one branch clock running."]
    #[inline]
    pub fn base_uart3_clk(&self) -> BASE_UART3_CLKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BASE_UART3_CLKR { bits }
    }
    #[doc = "Bit 2 - Base clock indicator for BASE_UART2_CLK 0 = All branch clocks switched off. 1 = At least one branch clock running."]
    #[inline]
    pub fn base_uart2_clk(&self) -> BASE_UART2_CLKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BASE_UART2_CLKR { bits }
    }
    #[doc = "Bit 3 - Base clock indicator for BASE_UART1_CLK 0 = All branch clocks switched off. 1 = At least one branch clock running."]
    #[inline]
    pub fn base_uart1_clk(&self) -> BASE_UART1_CLKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BASE_UART1_CLKR { bits }
    }
    #[doc = "Bit 4 - Base clock indicator for BASE_UART0_CLK 0 = All branch clocks switched off. 1 = At least one branch clock running."]
    #[inline]
    pub fn base_uart0_clk(&self) -> BASE_UART0_CLKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BASE_UART0_CLKR { bits }
    }
    #[doc = "Bit 5 - Base clock indicator for BASE_SSP1_CLK 0 = All branch clocks switched off. 1 = At least one branch clock running."]
    #[inline]
    pub fn base_ssp1_clk(&self) -> BASE_SSP1_CLKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BASE_SSP1_CLKR { bits }
    }
    #[doc = "Bit 6 - Base clock indicator for BASE_SSP0_CLK 0 = All branch clocks switched off. 1 = At least one branch clock running."]
    #[inline]
    pub fn base_ssp0_clk(&self) -> BASE_SSP0_CLKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BASE_SSP0_CLKR { bits }
    }
}
