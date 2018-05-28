#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 20usize],
    #[doc = "0x14 - Frequency monitor register"]
    pub freq_mon: FREQ_MON,
    #[doc = "0x18 - Crystal oscillator control register"]
    pub xtal_osc_ctrl: XTAL_OSC_CTRL,
    #[doc = "0x1c - PLL0USB status register"]
    pub pll0usb_stat: PLL0USB_STAT,
    #[doc = "0x20 - PLL0USB control register"]
    pub pll0usb_ctrl: PLL0USB_CTRL,
    #[doc = "0x24 - PLL0USB M-divider register"]
    pub pll0usb_mdiv: PLL0USB_MDIV,
    #[doc = "0x28 - PLL0USB N/P-divider register"]
    pub pll0usb_np_div: PLL0USB_NP_DIV,
    #[doc = "0x2c - PLL0AUDIO status register"]
    pub pll0audio_stat: PLL0AUDIO_STAT,
    #[doc = "0x30 - PLL0AUDIO control register"]
    pub pll0audio_ctrl: PLL0AUDIO_CTRL,
    #[doc = "0x34 - PLL0AUDIO M-divider register"]
    pub pll0audio_mdiv: PLL0AUDIO_MDIV,
    #[doc = "0x38 - PLL0AUDIO N/P-divider register"]
    pub pll0audio_np_div: PLL0AUDIO_NP_DIV,
    #[doc = "0x3c - PLL0AUDIO fractional divider register"]
    pub pll0audio_frac: PLL0AUDIO_FRAC,
    #[doc = "0x40 - PLL1 status register"]
    pub pll1_stat: PLL1_STAT,
    #[doc = "0x44 - PLL1 control register"]
    pub pll1_ctrl: PLL1_CTRL,
    #[doc = "0x48 - Integer divider A control register"]
    pub idiva_ctrl: IDIVA_CTRL,
    #[doc = "0x4c - Integer divider B control register"]
    pub idivb_ctrl: IDIVB_CTRL,
    #[doc = "0x50 - Integer divider C control register"]
    pub idivc_ctrl: IDIVC_CTRL,
    #[doc = "0x54 - Integer divider D control register"]
    pub idivd_ctrl: IDIVD_CTRL,
    #[doc = "0x58 - Integer divider E control register"]
    pub idive_ctrl: IDIVE_CTRL,
    #[doc = "0x5c - Output stage 0 control register for base clock BASE_SAFE_CLK"]
    pub base_safe_clk: BASE_SAFE_CLK,
    #[doc = "0x60 - Output stage 1 control register for base clock BASE_USB0_CLK"]
    pub base_usb0_clk: BASE_USB0_CLK,
    #[doc = "0x64 - Output stage 2 control register for base clock BASE_PERIPH_CLK"]
    pub base_periph_clk: BASE_PERIPH_CLK,
    #[doc = "0x68 - Output stage 3 control register for base clock BASE_USB1_CLK"]
    pub base_usb1_clk: BASE_USB1_CLK,
    #[doc = "0x6c - Output stage BASE_M4_CLK control register"]
    pub base_m4_clk: BASE_M4_CLK,
    #[doc = "0x70 - Output stage BASE_SPIFI_CLK control register"]
    pub base_spifi_clk: BASE_SPIFI_CLK,
    #[doc = "0x74 - Output stage BASE_SPI_CLK control register"]
    pub base_spi_clk: BASE_SPI_CLK,
    #[doc = "0x78 - Output stage BASE_PHY_RX_CLK control register"]
    pub base_phy_rx_clk: BASE_PHY_RX_CLK,
    #[doc = "0x7c - Output stage BASE_PHY_TX_CLK control register"]
    pub base_phy_tx_clk: BASE_PHY_TX_CLK,
    #[doc = "0x80 - Output stage BASE_APB1_CLK control register"]
    pub base_apb1_clk: BASE_APB1_CLK,
    #[doc = "0x84 - Output stage BASE_APB3_CLK control register"]
    pub base_apb3_clk: BASE_APB3_CLK,
    #[doc = "0x88 - Output stage BASE_LCD_CLK control register"]
    pub base_lcd_clk: BASE_LCD_CLK,
    _reserved1: [u8; 4usize],
    #[doc = "0x90 - Output stage BASE_SDIO_CLK control register"]
    pub base_sdio_clk: BASE_SDIO_CLK,
    #[doc = "0x94 - Output stage BASE_SSP0_CLK control register"]
    pub base_ssp0_clk: BASE_SSP0_CLK,
    #[doc = "0x98 - Output stage BASE_SSP1_CLK control register"]
    pub base_ssp1_clk: BASE_SSP1_CLK,
    #[doc = "0x9c - Output stage BASE_UART0_CLK control register"]
    pub base_uart0_clk: BASE_UART0_CLK,
    #[doc = "0xa0 - Output stage BASE_UART1_CLK control register"]
    pub base_uart1_clk: BASE_UART1_CLK,
    #[doc = "0xa4 - Output stage BASE_UART2_CLK control register"]
    pub base_uart2_clk: BASE_UART2_CLK,
    #[doc = "0xa8 - Output stage BASE_UART3_CLK control register"]
    pub base_uart3_clk: BASE_UART3_CLK,
    #[doc = "0xac - Output stage 20 control register for base clock BASE_OUT_CLK"]
    pub base_out_clk: BASE_OUT_CLK,
    _reserved2: [u8; 16usize],
    #[doc = "0xc0 - Output stage 25 control register for base clock BASE_AUDIO_CLK"]
    pub base_audio_clk: BASE_AUDIO_CLK,
    #[doc = "0xc4 - Output stage 25 control register for base clock BASE_CGU_OUT0_CLK"]
    pub base_cgu_out0_clk: BASE_CGU_OUT0_CLK,
    #[doc = "0xc8 - Output stage 25 control register for base clock BASE_CGU_OUT1_CLK"]
    pub base_cgu_out1_clk: BASE_CGU_OUT1_CLK,
}
#[doc = "Frequency monitor register"]
pub struct FREQ_MON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frequency monitor register"]
pub mod freq_mon;
#[doc = "Crystal oscillator control register"]
pub struct XTAL_OSC_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Crystal oscillator control register"]
pub mod xtal_osc_ctrl;
#[doc = "PLL0USB status register"]
pub struct PLL0USB_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL0USB status register"]
pub mod pll0usb_stat;
#[doc = "PLL0USB control register"]
pub struct PLL0USB_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL0USB control register"]
pub mod pll0usb_ctrl;
#[doc = "PLL0USB M-divider register"]
pub struct PLL0USB_MDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL0USB M-divider register"]
pub mod pll0usb_mdiv;
#[doc = "PLL0USB N/P-divider register"]
pub struct PLL0USB_NP_DIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL0USB N/P-divider register"]
pub mod pll0usb_np_div;
#[doc = "PLL0AUDIO status register"]
pub struct PLL0AUDIO_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL0AUDIO status register"]
pub mod pll0audio_stat;
#[doc = "PLL0AUDIO control register"]
pub struct PLL0AUDIO_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL0AUDIO control register"]
pub mod pll0audio_ctrl;
#[doc = "PLL0AUDIO M-divider register"]
pub struct PLL0AUDIO_MDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL0AUDIO M-divider register"]
pub mod pll0audio_mdiv;
#[doc = "PLL0AUDIO N/P-divider register"]
pub struct PLL0AUDIO_NP_DIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL0AUDIO N/P-divider register"]
pub mod pll0audio_np_div;
#[doc = "PLL0AUDIO fractional divider register"]
pub struct PLL0AUDIO_FRAC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL0AUDIO fractional divider register"]
pub mod pll0audio_frac;
#[doc = "PLL1 status register"]
pub struct PLL1_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL1 status register"]
pub mod pll1_stat;
#[doc = "PLL1 control register"]
pub struct PLL1_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL1 control register"]
pub mod pll1_ctrl;
#[doc = "Integer divider A control register"]
pub struct IDIVA_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Integer divider A control register"]
pub mod idiva_ctrl;
#[doc = "Integer divider B control register"]
pub struct IDIVB_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Integer divider B control register"]
pub mod idivb_ctrl;
#[doc = "Integer divider C control register"]
pub struct IDIVC_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Integer divider C control register"]
pub mod idivc_ctrl;
#[doc = "Integer divider D control register"]
pub struct IDIVD_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Integer divider D control register"]
pub mod idivd_ctrl;
#[doc = "Integer divider E control register"]
pub struct IDIVE_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Integer divider E control register"]
pub mod idive_ctrl;
#[doc = "Output stage 0 control register for base clock BASE_SAFE_CLK"]
pub struct BASE_SAFE_CLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output stage 0 control register for base clock BASE_SAFE_CLK"]
pub mod base_safe_clk;
#[doc = "Output stage 1 control register for base clock BASE_USB0_CLK"]
pub struct BASE_USB0_CLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output stage 1 control register for base clock BASE_USB0_CLK"]
pub mod base_usb0_clk;
#[doc = "Output stage 2 control register for base clock BASE_PERIPH_CLK"]
pub struct BASE_PERIPH_CLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output stage 2 control register for base clock BASE_PERIPH_CLK"]
pub mod base_periph_clk;
#[doc = "Output stage 3 control register for base clock BASE_USB1_CLK"]
pub struct BASE_USB1_CLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output stage 3 control register for base clock BASE_USB1_CLK"]
pub mod base_usb1_clk;
#[doc = "Output stage BASE_M4_CLK control register"]
pub struct BASE_M4_CLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output stage BASE_M4_CLK control register"]
pub mod base_m4_clk;
#[doc = "Output stage BASE_SPIFI_CLK control register"]
pub struct BASE_SPIFI_CLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output stage BASE_SPIFI_CLK control register"]
pub mod base_spifi_clk;
#[doc = "Output stage BASE_SPI_CLK control register"]
pub struct BASE_SPI_CLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output stage BASE_SPI_CLK control register"]
pub mod base_spi_clk;
#[doc = "Output stage BASE_PHY_RX_CLK control register"]
pub struct BASE_PHY_RX_CLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output stage BASE_PHY_RX_CLK control register"]
pub mod base_phy_rx_clk;
#[doc = "Output stage BASE_PHY_TX_CLK control register"]
pub struct BASE_PHY_TX_CLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output stage BASE_PHY_TX_CLK control register"]
pub mod base_phy_tx_clk;
#[doc = "Output stage BASE_APB1_CLK control register"]
pub struct BASE_APB1_CLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output stage BASE_APB1_CLK control register"]
pub mod base_apb1_clk;
#[doc = "Output stage BASE_APB3_CLK control register"]
pub struct BASE_APB3_CLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output stage BASE_APB3_CLK control register"]
pub mod base_apb3_clk;
#[doc = "Output stage BASE_LCD_CLK control register"]
pub struct BASE_LCD_CLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output stage BASE_LCD_CLK control register"]
pub mod base_lcd_clk;
#[doc = "Output stage BASE_SDIO_CLK control register"]
pub struct BASE_SDIO_CLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output stage BASE_SDIO_CLK control register"]
pub mod base_sdio_clk;
#[doc = "Output stage BASE_SSP0_CLK control register"]
pub struct BASE_SSP0_CLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output stage BASE_SSP0_CLK control register"]
pub mod base_ssp0_clk;
#[doc = "Output stage BASE_SSP1_CLK control register"]
pub struct BASE_SSP1_CLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output stage BASE_SSP1_CLK control register"]
pub mod base_ssp1_clk;
#[doc = "Output stage BASE_UART0_CLK control register"]
pub struct BASE_UART0_CLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output stage BASE_UART0_CLK control register"]
pub mod base_uart0_clk;
#[doc = "Output stage BASE_UART1_CLK control register"]
pub struct BASE_UART1_CLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output stage BASE_UART1_CLK control register"]
pub mod base_uart1_clk;
#[doc = "Output stage BASE_UART2_CLK control register"]
pub struct BASE_UART2_CLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output stage BASE_UART2_CLK control register"]
pub mod base_uart2_clk;
#[doc = "Output stage BASE_UART3_CLK control register"]
pub struct BASE_UART3_CLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output stage BASE_UART3_CLK control register"]
pub mod base_uart3_clk;
#[doc = "Output stage 20 control register for base clock BASE_OUT_CLK"]
pub struct BASE_OUT_CLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output stage 20 control register for base clock BASE_OUT_CLK"]
pub mod base_out_clk;
#[doc = "Output stage 25 control register for base clock BASE_AUDIO_CLK"]
pub struct BASE_AUDIO_CLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output stage 25 control register for base clock BASE_AUDIO_CLK"]
pub mod base_audio_clk;
#[doc = "Output stage 25 control register for base clock BASE_CGU_OUT0_CLK"]
pub struct BASE_CGU_OUT0_CLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output stage 25 control register for base clock BASE_CGU_OUT0_CLK"]
pub mod base_cgu_out0_clk;
#[doc = "Output stage 25 control register for base clock BASE_CGU_OUT1_CLK"]
pub struct BASE_CGU_OUT1_CLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output stage 25 control register for base clock BASE_CGU_OUT1_CLK"]
pub mod base_cgu_out1_clk;
