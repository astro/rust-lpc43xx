#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power mode register"]
    pub pm: PM,
    #[doc = "0x04 - CCU base clocks status register"]
    pub base_stat: BASE_STAT,
    _reserved0: [u8; 248usize],
    #[doc = "0x100 - CLK_AUDIO clock configuration register"]
    pub clk_audio_cfg: CLK_AUDIO_CFG,
    #[doc = "0x104 - CLK_AUDIO clock status register"]
    pub clk_audio_stat: CLK_AUDIO_STAT,
    _reserved1: [u8; 248usize],
    #[doc = "0x200 - CLK_APB2_USART3 clock configuration register"]
    pub clk_apb2_usart3_cfg: CLK_APB2_USART3_CFG,
    #[doc = "0x204 - CLK_APB2_USART3 clock status register"]
    pub clk_apb2_usart3_stat: CLK_APB2_USART3_STAT,
    _reserved2: [u8; 248usize],
    #[doc = "0x300 - CLK_APB2_USART2 clock configuration register"]
    pub clk_apb2_usart2_cfg: CLK_APB2_USART2_CFG,
    #[doc = "0x304 - CLK_APB2_USART clock status register"]
    pub clk_apb2_usart2_stat: CLK_APB2_USART2_STAT,
    _reserved3: [u8; 248usize],
    #[doc = "0x400 - CLK_APB2_UART1 clock configuration register"]
    pub clk_apb0_uart1_bus_cfg: CLK_APB0_UART1_BUS_CFG,
    #[doc = "0x404 - CLK_APB0_UART1 clock status register"]
    pub clk_apb0_uart1_stat: CLK_APB0_UART1_STAT,
    _reserved4: [u8; 248usize],
    #[doc = "0x500 - CLK_APB2_USART0 clock configuration register"]
    pub clk_apb0_usart0_cfg: CLK_APB0_USART0_CFG,
    #[doc = "0x504 - CLK_APB0_USART0 clock status register"]
    pub clk_apb0_usart0_stat: CLK_APB0_USART0_STAT,
    _reserved5: [u8; 248usize],
    #[doc = "0x600 - CLK_APB2_SSP1 clock configuration register"]
    pub clk_apb2_ssp1_cfg: CLK_APB2_SSP1_CFG,
    #[doc = "0x604 - CLK_APB2_SSP1 clock status register"]
    pub clk_apb2_ssp1_stat: CLK_APB2_SSP1_STAT,
    _reserved6: [u8; 248usize],
    #[doc = "0x700 - CLK_APB0_SSP0 clock configuration register"]
    pub clk_apb0_ssp0_cfg: CLK_APB0_SSP0_CFG,
    #[doc = "0x704 - CLK_APB0_SSP0 clock status register"]
    pub clk_apb0_ssp0_stat: CLK_APB0_SSP0_STAT,
    _reserved7: [u8; 248usize],
    #[doc = "0x800 - CLK_SDIO clock configuration register"]
    pub clk_sdio_cfg: CLK_SDIO_CFG,
    #[doc = "0x804 - CLK_SDIO clock status register"]
    pub clk_sdio_stat: CLK_SDIO_STAT,
}
#[doc = "Power mode register"]
pub struct PM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power mode register"]
pub mod pm;
#[doc = "CCU base clocks status register"]
pub struct BASE_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCU base clocks status register"]
pub mod base_stat;
#[doc = "CLK_AUDIO clock configuration register"]
pub struct CLK_AUDIO_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_AUDIO clock configuration register"]
pub mod clk_audio_cfg;
#[doc = "CLK_APB2_USART3 clock configuration register"]
pub struct CLK_APB2_USART3_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB2_USART3 clock configuration register"]
pub mod clk_apb2_usart3_cfg;
#[doc = "CLK_APB2_USART2 clock configuration register"]
pub struct CLK_APB2_USART2_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB2_USART2 clock configuration register"]
pub mod clk_apb2_usart2_cfg;
#[doc = "CLK_APB2_UART1 clock configuration register"]
pub struct CLK_APB0_UART1_BUS_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB2_UART1 clock configuration register"]
pub mod clk_apb0_uart1_bus_cfg;
#[doc = "CLK_APB2_USART0 clock configuration register"]
pub struct CLK_APB0_USART0_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB2_USART0 clock configuration register"]
pub mod clk_apb0_usart0_cfg;
#[doc = "CLK_APB0_SSP0 clock configuration register"]
pub struct CLK_APB0_SSP0_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB0_SSP0 clock configuration register"]
pub mod clk_apb0_ssp0_cfg;
#[doc = "CLK_APB2_SSP1 clock configuration register"]
pub struct CLK_APB2_SSP1_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB2_SSP1 clock configuration register"]
pub mod clk_apb2_ssp1_cfg;
#[doc = "CLK_SDIO clock configuration register"]
pub struct CLK_SDIO_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_SDIO clock configuration register"]
pub mod clk_sdio_cfg;
#[doc = "CLK_AUDIO clock status register"]
pub struct CLK_AUDIO_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_AUDIO clock status register"]
pub mod clk_audio_stat;
#[doc = "CLK_APB2_USART3 clock status register"]
pub struct CLK_APB2_USART3_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB2_USART3 clock status register"]
pub mod clk_apb2_usart3_stat;
#[doc = "CLK_APB2_USART clock status register"]
pub struct CLK_APB2_USART2_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB2_USART clock status register"]
pub mod clk_apb2_usart2_stat;
#[doc = "CLK_APB0_UART1 clock status register"]
pub struct CLK_APB0_UART1_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB0_UART1 clock status register"]
pub mod clk_apb0_uart1_stat;
#[doc = "CLK_APB0_USART0 clock status register"]
pub struct CLK_APB0_USART0_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB0_USART0 clock status register"]
pub mod clk_apb0_usart0_stat;
#[doc = "CLK_APB2_SSP1 clock status register"]
pub struct CLK_APB2_SSP1_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB2_SSP1 clock status register"]
pub mod clk_apb2_ssp1_stat;
#[doc = "CLK_APB0_SSP0 clock status register"]
pub struct CLK_APB0_SSP0_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB0_SSP0 clock status register"]
pub mod clk_apb0_ssp0_stat;
#[doc = "CLK_SDIO clock status register"]
pub struct CLK_SDIO_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_SDIO clock status register"]
pub mod clk_sdio_stat;
