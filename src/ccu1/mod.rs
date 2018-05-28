#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CCU1 power mode register"]
    pub pm: PM,
    #[doc = "0x04 - CCU1 base clocks status register"]
    pub base_stat: BASE_STAT,
    _reserved0: [u8; 248usize],
    #[doc = "0x100 - CLK_APB3_BUS clock configuration register"]
    pub clk_apb3_bus_cfg: CLK_APB3_BUS_CFG,
    #[doc = "0x104 - CLK_APB3_BUS clock status register"]
    pub clk_apb3_bus_stat: CLK_APB3_BUS_STAT,
    #[doc = "0x108 - CLK_APB3_I2C1 clock configuration register"]
    pub clk_apb3_i2c1_cfg: CLK_APB3_I2C1_CFG,
    #[doc = "0x10c - CLK_APB3_I2C1 clock status register"]
    pub clk_apb3_i2c1_stat: CLK_APB3_I2C1_STAT,
    #[doc = "0x110 - CLK_APB3_DAC clock configuration register"]
    pub clk_apb3_dac_cfg: CLK_APB3_DAC_CFG,
    #[doc = "0x114 - CLK_APB3_DAC clock status register"]
    pub clk_apb3_dac_stat: CLK_APB3_DAC_STAT,
    #[doc = "0x118 - CLK_APB3_ADC0 clock configuration register"]
    pub clk_apb3_adc0_cfg: CLK_APB3_ADC0_CFG,
    #[doc = "0x11c - CLK_APB3_ADC0 clock status register"]
    pub clk_apb3_adc0_stat: CLK_APB3_ADC0_STAT,
    #[doc = "0x120 - CLK_APB3_ADC1 clock configuration register"]
    pub clk_apb3_adc1_cfg: CLK_APB3_ADC1_CFG,
    #[doc = "0x124 - CLK_APB3_ADC1 clock status register"]
    pub clk_apb3_adc1_stat: CLK_APB3_ADC1_STAT,
    #[doc = "0x128 - CLK_APB3_CAN0 clock configuration register"]
    pub clk_apb3_can0_cfg: CLK_APB3_CAN0_CFG,
    #[doc = "0x12c - CLK_APB3_CAN0 clock status register"]
    pub clk_apb3_can0_stat: CLK_APB3_CAN0_STAT,
    _reserved1: [u8; 208usize],
    #[doc = "0x200 - CLK_APB1_BUS clock configuration register"]
    pub clk_apb1_bus_cfg: CLK_APB1_BUS_CFG,
    #[doc = "0x204 - CLK_APB1_BUS clock status register"]
    pub clk_apb1_bus_stat: CLK_APB1_BUS_STAT,
    #[doc = "0x208 - CLK_APB1_MOTOCONPWM clock configuration register"]
    pub clk_apb1_motoconpwm_cfg: CLK_APB1_MOTOCONPWM_CFG,
    #[doc = "0x20c - CLK_APB1_MOTOCONPWM clock status register"]
    pub clk_apb1_motoconpwm_stat: CLK_APB1_MOTOCONPWM_STAT,
    #[doc = "0x210 - CLK_ABP1_I2C0 clock configuration register"]
    pub clk_apb1_i2c0_cfg: CLK_APB1_I2C0_CFG,
    #[doc = "0x214 - CLK_APB1_I2C0 clock status register"]
    pub clk_apb1_i2c0_stat: CLK_APB1_I2C0_STAT,
    #[doc = "0x218 - CLK_APB1_I2S clock configuration register"]
    pub clk_apb1_i2s_cfg: CLK_APB1_I2S_CFG,
    #[doc = "0x21c - CLK_APB1_I2S clock status register"]
    pub clk_apb1_i2s_stat: CLK_APB1_I2S_STAT,
    #[doc = "0x220 - CLK_APB1_CAN1 clock configuration register"]
    pub clk_apb1_can1_cfg: CLK_APB1_CAN1_CFG,
    #[doc = "0x224 - CLK_APB1_CAN1 clock status register"]
    pub clk_apb1_can1_stat: CLK_APB1_CAN1_STAT,
    _reserved2: [u8; 216usize],
    #[doc = "0x300 - CLK_SPIFI clock configuration register"]
    pub clk_spifi_cfg: CLK_SPIFI_CFG,
    #[doc = "0x304 - CLK_APB1_SPIFI clock status register"]
    pub clk_spifi_stat: CLK_SPIFI_STAT,
    _reserved3: [u8; 248usize],
    #[doc = "0x400 - CLK_M4_BUS clock configuration register"]
    pub clk_m4_bus_cfg: CLK_M4_BUS_CFG,
    #[doc = "0x404 - CLK_M4_BUSclock status register"]
    pub clk_m4_bus_stat: CLK_M4_BUS_STAT,
    #[doc = "0x408 - CLK_M4_SPIFI clock configuration register"]
    pub clk_m4_spifi_cfg: CLK_M4_SPIFI_CFG,
    #[doc = "0x40c - CLK_M4_SPIFI clock status register"]
    pub clk_m4_spifi_stat: CLK_M4_SPIFI_STAT,
    #[doc = "0x410 - CLK_M4_GPIO clock configuration register"]
    pub clk_m4_gpio_cfg: CLK_M4_GPIO_CFG,
    #[doc = "0x414 - CLK_M4_GPIO clock status register"]
    pub clk_m4_gpio_stat: CLK_M4_GPIO_STAT,
    #[doc = "0x418 - CLK_M4_LCD clock configuration register"]
    pub clk_m4_lcd_cfg: CLK_M4_LCD_CFG,
    #[doc = "0x41c - CLK_M4_LCD clock status register"]
    pub clk_m4_lcd_stat: CLK_M4_LCD_STAT,
    #[doc = "0x420 - CLK_M4_ETHERNET clock configuration register"]
    pub clk_m4_ethernet_cfg: CLK_M4_ETHERNET_CFG,
    #[doc = "0x424 - CLK_M4_ETHERNET clock status register"]
    pub clk_m4_ethernet_stat: CLK_M4_ETHERNET_STAT,
    #[doc = "0x428 - CLK_M4_USB0 clock configuration register"]
    pub clk_m4_usb0_cfg: CLK_M4_USB0_CFG,
    #[doc = "0x42c - CLK_M4_USB0 clock status register"]
    pub clk_m4_usb0_stat: CLK_M4_USB0_STAT,
    #[doc = "0x430 - CLK_M4_EMC clock configuration register"]
    pub clk_m4_emc_cfg: CLK_M4_EMC_CFG,
    #[doc = "0x434 - CLK_M4_EMC clock status register"]
    pub clk_m4_emc_stat: CLK_M4_EMC_STAT,
    #[doc = "0x438 - CLK_M4_SDIO clock configuration register"]
    pub clk_m4_sdio_cfg: CLK_M4_SDIO_CFG,
    #[doc = "0x43c - CLK_M4_SDIO clock status register"]
    pub clk_m4_sdio_stat: CLK_M4_SDIO_STAT,
    #[doc = "0x440 - CLK_M4_DMA clock configuration register"]
    pub clk_m4_dma_cfg: CLK_M4_DMA_CFG,
    #[doc = "0x444 - CLK_M4_DMA clock status register"]
    pub clk_m4_dma_stat: CLK_M4_DMA_STAT,
    #[doc = "0x448 - CLK_M4_M4CORE clock configuration register"]
    pub clk_m4_m4core_cfg: CLK_M4_M4CORE_CFG,
    #[doc = "0x44c - CLK_M4_M3CORE clock status register"]
    pub clk_m4_m4core_stat: CLK_M4_M4CORE_STAT,
    _reserved4: [u8; 24usize],
    #[doc = "0x468 - CLK_M4_SCT clock configuration register"]
    pub clk_m4_sct_cfg: CLK_M4_SCT_CFG,
    #[doc = "0x46c - CLK_M4_SCT clock status register"]
    pub clk_m4_sct_stat: CLK_M4_SCT_STAT,
    #[doc = "0x470 - CLK_M4_USB1 clock configuration register"]
    pub clk_m4_usb1_cfg: CLK_M4_USB1_CFG,
    #[doc = "0x474 - CLK_M4_USB1 clock status register"]
    pub clk_m4_usb1_stat: CLK_M4_USB1_STAT,
    #[doc = "0x478 - CLK_M4_EMCDIV clock configuration register"]
    pub clk_m4_emcdiv_cfg: CLK_M4_EMCDIV_CFG,
    #[doc = "0x47c - CLK_M4_EMCDIV clock status register"]
    pub clk_m4_emcdiv_stat: CLK_M4_EMCDIV_STAT,
    #[doc = "0x480 - CLK_M4_FLASHA clock configuration register"]
    pub clk_m4_flasha_cfg: CLK_M4_FLASHA_CFG,
    #[doc = "0x484 - CLK_M4_FLASHA clock status register"]
    pub clk_m4_flasha_stat: CLK_M4_FLASHA_STAT,
    #[doc = "0x488 - CLK_M4_FLASHB clock configuration register"]
    pub clk_m4_flashb_cfg: CLK_M4_FLASHB_CFG,
    #[doc = "0x48c - CLK_M4_FLASHB clock status register"]
    pub clk_m4_flashb_stat: CLK_M4_FLASHB_STAT,
    #[doc = "0x490 - CLK_M0APP_CFG clock configuration register"]
    pub clk_m4_m0app_cfg: CLK_M4_M0APP_CFG,
    #[doc = "0x494 - CLK_M4_MOAPP clock status register"]
    pub clk_m4_m0app_stat: CLK_M4_M0APP_STAT,
    #[doc = "0x498 - CLK_ADCHS_CFG clock configuration register"]
    pub clk_m4_adchs_cfg: CLK_M4_ADCHS_CFG,
    #[doc = "0x49c - CLK_M4_ADCHS clock status register"]
    pub clk_m4_adchs_stat: CLK_M4_ADCHS_STAT,
    #[doc = "0x4a0 - CLK_EEPROM_CFG clock configuration register"]
    pub clk_m4_eeprom_cfg: CLK_M4_EEPROM_CFG,
    #[doc = "0x4a4 - CLK_M4_EEPROM clock status register"]
    pub clk_m4_eeprom_stat: CLK_M4_EEPROM_STAT,
    _reserved5: [u8; 88usize],
    #[doc = "0x500 - CLK_M4_WWDT clock configuration register"]
    pub clk_m4_wwdt_cfg: CLK_M4_WWDT_CFG,
    #[doc = "0x504 - CLK_M4_WWDT clock status register"]
    pub clk_m4_wwdt_stat: CLK_M4_WWDT_STAT,
    #[doc = "0x508 - CLK_M4_USART0 clock configuration register"]
    pub clk_m4_usart0_cfg: CLK_M4_USART0_CFG,
    #[doc = "0x50c - CLK_M4_USART0 clock status register"]
    pub clk_m4_usart0_stat: CLK_M4_USART0_STAT,
    #[doc = "0x510 - CLK_M4_UART1 clock configuration register"]
    pub clk_m4_uart1_cfg: CLK_M4_UART1_CFG,
    #[doc = "0x514 - CLK_M4_UART1 clock status register"]
    pub clk_m4_uart1_stat: CLK_M4_UART1_STAT,
    #[doc = "0x518 - CLK_M4_SSP0 clock configuration register"]
    pub clk_m4_ssp0_cfg: CLK_M4_SSP0_CFG,
    #[doc = "0x51c - CLK_M4_SSP0 clock status register"]
    pub clk_m4_ssp0_stat: CLK_M4_SSP0_STAT,
    #[doc = "0x520 - CLK_M4_TIMER0 clock configuration register"]
    pub clk_m4_timer0_cfg: CLK_M4_TIMER0_CFG,
    #[doc = "0x524 - CLK_M4_TIMER0 clock status register"]
    pub clk_m4_timer0_stat: CLK_M4_TIMER0_STAT,
    #[doc = "0x528 - CLK_M4_TIMER1clock configuration register"]
    pub clk_m4_timer1_cfg: CLK_M4_TIMER1_CFG,
    #[doc = "0x52c - CLK_M4_TIMER1 clock status register"]
    pub clk_m4_timer1_stat: CLK_M4_TIMER1_STAT,
    #[doc = "0x530 - CLK_M4_SCU clock configuration register"]
    pub clk_m4_scu_cfg: CLK_M4_SCU_CFG,
    #[doc = "0x534 - CLK_SCU_XXX clock status register"]
    pub clk_m4_scu_stat: CLK_M4_SCU_STAT,
    #[doc = "0x538 - CLK_M4_CREGclock configuration register"]
    pub clk_m4_creg_cfg: CLK_M4_CREG_CFG,
    #[doc = "0x53c - CLK_M4_CREG clock status register"]
    pub clk_m4_creg_stat: CLK_M4_CREG_STAT,
    _reserved6: [u8; 192usize],
    #[doc = "0x600 - CLK_M4_RITIMER clock configuration register"]
    pub clk_m4_ritimer_cfg: CLK_M4_RITIMER_CFG,
    #[doc = "0x604 - CLK_M4_RITIMER clock status register"]
    pub clk_m4_ritimer_stat: CLK_M4_RITIMER_STAT,
    #[doc = "0x608 - CLK_M4_USART2 clock configuration register"]
    pub clk_m4_usart2_cfg: CLK_M4_USART2_CFG,
    #[doc = "0x60c - CLK_M4_USART2 clock status register"]
    pub clk_m4_usart2_stat: CLK_M4_USART2_STAT,
    #[doc = "0x610 - CLK_M4_USART3 clock configuration register"]
    pub clk_m4_usart3_cfg: CLK_M4_USART3_CFG,
    #[doc = "0x614 - CLK_M4_USART3 clock status register"]
    pub clk_m4_usart3_stat: CLK_M4_USART3_STAT,
    #[doc = "0x618 - CLK_M4_TIMER2 clock configuration register"]
    pub clk_m4_timer2_cfg: CLK_M4_TIMER2_CFG,
    #[doc = "0x61c - CLK_M4_TIMER2 clock status register"]
    pub clk_m4_timer2_stat: CLK_M4_TIMER2_STAT,
    #[doc = "0x620 - CLK_M4_TIMER3 clock configuration register"]
    pub clk_m4_timer3_cfg: CLK_M4_TIMER3_CFG,
    #[doc = "0x624 - CLK_M4_TIMER3 clock status register"]
    pub clk_m4_timer3_stat: CLK_M4_TIMER3_STAT,
    #[doc = "0x628 - CLK_M4_SSP1 clock configuration register"]
    pub clk_m4_ssp1_cfg: CLK_M4_SSP1_CFG,
    #[doc = "0x62c - CLK_M4_SSP1 clock status register"]
    pub clk_m4_ssp1_stat: CLK_M4_SSP1_STAT,
    #[doc = "0x630 - CLK_M4_QEIclock configuration register"]
    pub clk_m4_qei_cfg: CLK_M4_QEI_CFG,
    #[doc = "0x634 - CLK_M4_QEI clock status register"]
    pub clk_m4_qei_stat: CLK_M4_QEI_STAT,
    _reserved7: [u8; 200usize],
    #[doc = "0x700 - CLK_PERIPH_BUS_CFG clock configuration register"]
    pub clk_periph_bus_cfg: CLK_PERIPH_BUS_CFG,
    #[doc = "0x704 - CLK_PERIPH_BUS_STAT clock status register"]
    pub clk_periph_bus_stat: CLK_PERIPH_BUS_STAT,
    _reserved8: [u8; 8usize],
    #[doc = "0x710 - CLK_PERIPH_CORE_CFG clock configuration register"]
    pub clk_periph_core_cfg: CLK_PERIPH_CORE_CFG,
    #[doc = "0x714 - CLK_CORE_BUS_STAT clock status register"]
    pub clk_periph_core_stat: CLK_PERIPH_CORE_STAT,
    #[doc = "0x718 - CLK_PERIPH_SGPIO_CFG clock configuration register"]
    pub clk_periph_sgpio_cfg: CLK_PERIPH_SGPIO_CFG,
    #[doc = "0x71c - CLK_CORE_SGPIO_STAT clock status register"]
    pub clk_periph_sgpio_stat: CLK_PERIPH_SGPIO_STAT,
    _reserved9: [u8; 224usize],
    #[doc = "0x800 - CLK_M4_USB0 clock configuration register"]
    pub clk_usb0_cfg: CLK_USB0_CFG,
    #[doc = "0x804 - CLK_USB0 clock status register"]
    pub clk_usb0_stat: CLK_USB0_STAT,
    _reserved10: [u8; 248usize],
    #[doc = "0x900 - CLK_USB1 clock configuration register"]
    pub clk_usb1_cfg: CLK_USB1_CFG,
    #[doc = "0x904 - CLK_USB1 clock status register"]
    pub clk_usb1_stat: CLK_USB1_STAT,
    _reserved11: [u8; 248usize],
    #[doc = "0xa00 - CLK_SPI clock configuration register"]
    pub clk_spi_cfg: CLK_SPI_CFG,
    #[doc = "0xa04 - CLK_SPI clock status register"]
    pub clk_spi_stat: CLK_SPI_STAT,
    _reserved12: [u8; 248usize],
    #[doc = "0xb00 - CLK_ADCHS clock configuration register"]
    pub clk_adchs_cfg: CLK_ADCHS_CFG,
    #[doc = "0xb04 - CLK_ADCHS clock status register"]
    pub clk_adchs_stat: CLK_ADCHS_STAT,
}
#[doc = "CCU1 power mode register"]
pub struct PM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCU1 power mode register"]
pub mod pm;
#[doc = "CCU1 base clocks status register"]
pub struct BASE_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCU1 base clocks status register"]
pub mod base_stat;
#[doc = "CLK_APB3_BUS clock configuration register"]
pub struct CLK_APB3_BUS_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB3_BUS clock configuration register"]
pub mod clk_apb3_bus_cfg;
#[doc = "CLK_APB3_I2C1 clock configuration register"]
pub struct CLK_APB3_I2C1_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB3_I2C1 clock configuration register"]
pub mod clk_apb3_i2c1_cfg;
#[doc = "CLK_APB3_DAC clock configuration register"]
pub struct CLK_APB3_DAC_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB3_DAC clock configuration register"]
pub mod clk_apb3_dac_cfg;
#[doc = "CLK_APB3_ADC0 clock configuration register"]
pub struct CLK_APB3_ADC0_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB3_ADC0 clock configuration register"]
pub mod clk_apb3_adc0_cfg;
#[doc = "CLK_APB3_ADC1 clock configuration register"]
pub struct CLK_APB3_ADC1_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB3_ADC1 clock configuration register"]
pub mod clk_apb3_adc1_cfg;
#[doc = "CLK_APB3_CAN0 clock configuration register"]
pub struct CLK_APB3_CAN0_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB3_CAN0 clock configuration register"]
pub mod clk_apb3_can0_cfg;
#[doc = "CLK_APB1_BUS clock configuration register"]
pub struct CLK_APB1_BUS_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB1_BUS clock configuration register"]
pub mod clk_apb1_bus_cfg;
#[doc = "CLK_APB1_MOTOCONPWM clock configuration register"]
pub struct CLK_APB1_MOTOCONPWM_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB1_MOTOCONPWM clock configuration register"]
pub mod clk_apb1_motoconpwm_cfg;
#[doc = "CLK_ABP1_I2C0 clock configuration register"]
pub struct CLK_APB1_I2C0_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_ABP1_I2C0 clock configuration register"]
pub mod clk_apb1_i2c0_cfg;
#[doc = "CLK_APB1_I2S clock configuration register"]
pub struct CLK_APB1_I2S_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB1_I2S clock configuration register"]
pub mod clk_apb1_i2s_cfg;
#[doc = "CLK_APB1_CAN1 clock configuration register"]
pub struct CLK_APB1_CAN1_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB1_CAN1 clock configuration register"]
pub mod clk_apb1_can1_cfg;
#[doc = "CLK_SPIFI clock configuration register"]
pub struct CLK_SPIFI_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_SPIFI clock configuration register"]
pub mod clk_spifi_cfg;
#[doc = "CLK_M4_BUS clock configuration register"]
pub struct CLK_M4_BUS_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_BUS clock configuration register"]
pub mod clk_m4_bus_cfg;
#[doc = "CLK_M4_SPIFI clock configuration register"]
pub struct CLK_M4_SPIFI_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_SPIFI clock configuration register"]
pub mod clk_m4_spifi_cfg;
#[doc = "CLK_M4_GPIO clock configuration register"]
pub struct CLK_M4_GPIO_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_GPIO clock configuration register"]
pub mod clk_m4_gpio_cfg;
#[doc = "CLK_M4_LCD clock configuration register"]
pub struct CLK_M4_LCD_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_LCD clock configuration register"]
pub mod clk_m4_lcd_cfg;
#[doc = "CLK_M4_ETHERNET clock configuration register"]
pub struct CLK_M4_ETHERNET_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_ETHERNET clock configuration register"]
pub mod clk_m4_ethernet_cfg;
#[doc = "CLK_M4_USB0 clock configuration register"]
pub struct CLK_M4_USB0_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_USB0 clock configuration register"]
pub mod clk_m4_usb0_cfg;
#[doc = "CLK_M4_EMC clock configuration register"]
pub struct CLK_M4_EMC_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_EMC clock configuration register"]
pub mod clk_m4_emc_cfg;
#[doc = "CLK_M4_SDIO clock configuration register"]
pub struct CLK_M4_SDIO_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_SDIO clock configuration register"]
pub mod clk_m4_sdio_cfg;
#[doc = "CLK_M4_DMA clock configuration register"]
pub struct CLK_M4_DMA_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_DMA clock configuration register"]
pub mod clk_m4_dma_cfg;
#[doc = "CLK_M4_M4CORE clock configuration register"]
pub struct CLK_M4_M4CORE_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_M4CORE clock configuration register"]
pub mod clk_m4_m4core_cfg;
#[doc = "CLK_M4_SCT clock configuration register"]
pub struct CLK_M4_SCT_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_SCT clock configuration register"]
pub mod clk_m4_sct_cfg;
#[doc = "CLK_M4_USB1 clock configuration register"]
pub struct CLK_M4_USB1_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_USB1 clock configuration register"]
pub mod clk_m4_usb1_cfg;
#[doc = "CLK_M4_EMCDIV clock configuration register"]
pub struct CLK_M4_EMCDIV_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_EMCDIV clock configuration register"]
pub mod clk_m4_emcdiv_cfg;
#[doc = "CLK_M4_FLASHA clock configuration register"]
pub struct CLK_M4_FLASHA_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_FLASHA clock configuration register"]
pub mod clk_m4_flasha_cfg;
#[doc = "CLK_M4_FLASHB clock configuration register"]
pub struct CLK_M4_FLASHB_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_FLASHB clock configuration register"]
pub mod clk_m4_flashb_cfg;
#[doc = "CLK_M0APP_CFG clock configuration register"]
pub struct CLK_M4_M0APP_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M0APP_CFG clock configuration register"]
pub mod clk_m4_m0app_cfg;
#[doc = "CLK_ADCHS_CFG clock configuration register"]
pub struct CLK_M4_ADCHS_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_ADCHS_CFG clock configuration register"]
pub mod clk_m4_adchs_cfg;
#[doc = "CLK_EEPROM_CFG clock configuration register"]
pub struct CLK_M4_EEPROM_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_EEPROM_CFG clock configuration register"]
pub mod clk_m4_eeprom_cfg;
#[doc = "CLK_M4_WWDT clock configuration register"]
pub struct CLK_M4_WWDT_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_WWDT clock configuration register"]
pub mod clk_m4_wwdt_cfg;
#[doc = "CLK_M4_USART0 clock configuration register"]
pub struct CLK_M4_USART0_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_USART0 clock configuration register"]
pub mod clk_m4_usart0_cfg;
#[doc = "CLK_M4_UART1 clock configuration register"]
pub struct CLK_M4_UART1_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_UART1 clock configuration register"]
pub mod clk_m4_uart1_cfg;
#[doc = "CLK_M4_SSP0 clock configuration register"]
pub struct CLK_M4_SSP0_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_SSP0 clock configuration register"]
pub mod clk_m4_ssp0_cfg;
#[doc = "CLK_M4_TIMER0 clock configuration register"]
pub struct CLK_M4_TIMER0_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_TIMER0 clock configuration register"]
pub mod clk_m4_timer0_cfg;
#[doc = "CLK_M4_TIMER1clock configuration register"]
pub struct CLK_M4_TIMER1_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_TIMER1clock configuration register"]
pub mod clk_m4_timer1_cfg;
#[doc = "CLK_M4_SCU clock configuration register"]
pub struct CLK_M4_SCU_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_SCU clock configuration register"]
pub mod clk_m4_scu_cfg;
#[doc = "CLK_M4_CREGclock configuration register"]
pub struct CLK_M4_CREG_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_CREGclock configuration register"]
pub mod clk_m4_creg_cfg;
#[doc = "CLK_M4_RITIMER clock configuration register"]
pub struct CLK_M4_RITIMER_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_RITIMER clock configuration register"]
pub mod clk_m4_ritimer_cfg;
#[doc = "CLK_M4_USART2 clock configuration register"]
pub struct CLK_M4_USART2_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_USART2 clock configuration register"]
pub mod clk_m4_usart2_cfg;
#[doc = "CLK_M4_USART3 clock configuration register"]
pub struct CLK_M4_USART3_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_USART3 clock configuration register"]
pub mod clk_m4_usart3_cfg;
#[doc = "CLK_M4_TIMER2 clock configuration register"]
pub struct CLK_M4_TIMER2_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_TIMER2 clock configuration register"]
pub mod clk_m4_timer2_cfg;
#[doc = "CLK_M4_TIMER3 clock configuration register"]
pub struct CLK_M4_TIMER3_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_TIMER3 clock configuration register"]
pub mod clk_m4_timer3_cfg;
#[doc = "CLK_M4_SSP1 clock configuration register"]
pub struct CLK_M4_SSP1_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_SSP1 clock configuration register"]
pub mod clk_m4_ssp1_cfg;
#[doc = "CLK_M4_QEIclock configuration register"]
pub struct CLK_M4_QEI_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_QEIclock configuration register"]
pub mod clk_m4_qei_cfg;
#[doc = "CLK_PERIPH_BUS_CFG clock configuration register"]
pub struct CLK_PERIPH_BUS_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_PERIPH_BUS_CFG clock configuration register"]
pub mod clk_periph_bus_cfg;
#[doc = "CLK_PERIPH_CORE_CFG clock configuration register"]
pub struct CLK_PERIPH_CORE_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_PERIPH_CORE_CFG clock configuration register"]
pub mod clk_periph_core_cfg;
#[doc = "CLK_PERIPH_SGPIO_CFG clock configuration register"]
pub struct CLK_PERIPH_SGPIO_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_PERIPH_SGPIO_CFG clock configuration register"]
pub mod clk_periph_sgpio_cfg;
#[doc = "CLK_M4_USB0 clock configuration register"]
pub struct CLK_USB0_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_USB0 clock configuration register"]
pub mod clk_usb0_cfg;
#[doc = "CLK_USB1 clock configuration register"]
pub struct CLK_USB1_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_USB1 clock configuration register"]
pub mod clk_usb1_cfg;
#[doc = "CLK_SPI clock configuration register"]
pub struct CLK_SPI_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_SPI clock configuration register"]
pub mod clk_spi_cfg;
#[doc = "CLK_ADCHS clock configuration register"]
pub struct CLK_ADCHS_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_ADCHS clock configuration register"]
pub mod clk_adchs_cfg;
#[doc = "CLK_APB3_BUS clock status register"]
pub struct CLK_APB3_BUS_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB3_BUS clock status register"]
pub mod clk_apb3_bus_stat;
#[doc = "CLK_APB3_I2C1 clock status register"]
pub struct CLK_APB3_I2C1_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB3_I2C1 clock status register"]
pub mod clk_apb3_i2c1_stat;
#[doc = "CLK_APB3_DAC clock status register"]
pub struct CLK_APB3_DAC_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB3_DAC clock status register"]
pub mod clk_apb3_dac_stat;
#[doc = "CLK_APB3_ADC0 clock status register"]
pub struct CLK_APB3_ADC0_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB3_ADC0 clock status register"]
pub mod clk_apb3_adc0_stat;
#[doc = "CLK_APB3_ADC1 clock status register"]
pub struct CLK_APB3_ADC1_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB3_ADC1 clock status register"]
pub mod clk_apb3_adc1_stat;
#[doc = "CLK_APB3_CAN0 clock status register"]
pub struct CLK_APB3_CAN0_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB3_CAN0 clock status register"]
pub mod clk_apb3_can0_stat;
#[doc = "CLK_APB1_BUS clock status register"]
pub struct CLK_APB1_BUS_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB1_BUS clock status register"]
pub mod clk_apb1_bus_stat;
#[doc = "CLK_APB1_MOTOCONPWM clock status register"]
pub struct CLK_APB1_MOTOCONPWM_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB1_MOTOCONPWM clock status register"]
pub mod clk_apb1_motoconpwm_stat;
#[doc = "CLK_APB1_I2C0 clock status register"]
pub struct CLK_APB1_I2C0_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB1_I2C0 clock status register"]
pub mod clk_apb1_i2c0_stat;
#[doc = "CLK_APB1_I2S clock status register"]
pub struct CLK_APB1_I2S_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB1_I2S clock status register"]
pub mod clk_apb1_i2s_stat;
#[doc = "CLK_APB1_CAN1 clock status register"]
pub struct CLK_APB1_CAN1_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB1_CAN1 clock status register"]
pub mod clk_apb1_can1_stat;
#[doc = "CLK_APB1_SPIFI clock status register"]
pub struct CLK_SPIFI_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_APB1_SPIFI clock status register"]
pub mod clk_spifi_stat;
#[doc = "CLK_M4_BUSclock status register"]
pub struct CLK_M4_BUS_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_BUSclock status register"]
pub mod clk_m4_bus_stat;
#[doc = "CLK_M4_SPIFI clock status register"]
pub struct CLK_M4_SPIFI_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_SPIFI clock status register"]
pub mod clk_m4_spifi_stat;
#[doc = "CLK_M4_GPIO clock status register"]
pub struct CLK_M4_GPIO_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_GPIO clock status register"]
pub mod clk_m4_gpio_stat;
#[doc = "CLK_M4_LCD clock status register"]
pub struct CLK_M4_LCD_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_LCD clock status register"]
pub mod clk_m4_lcd_stat;
#[doc = "CLK_M4_ETHERNET clock status register"]
pub struct CLK_M4_ETHERNET_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_ETHERNET clock status register"]
pub mod clk_m4_ethernet_stat;
#[doc = "CLK_M4_USB0 clock status register"]
pub struct CLK_M4_USB0_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_USB0 clock status register"]
pub mod clk_m4_usb0_stat;
#[doc = "CLK_M4_EMC clock status register"]
pub struct CLK_M4_EMC_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_EMC clock status register"]
pub mod clk_m4_emc_stat;
#[doc = "CLK_M4_SDIO clock status register"]
pub struct CLK_M4_SDIO_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_SDIO clock status register"]
pub mod clk_m4_sdio_stat;
#[doc = "CLK_M4_DMA clock status register"]
pub struct CLK_M4_DMA_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_DMA clock status register"]
pub mod clk_m4_dma_stat;
#[doc = "CLK_M4_M3CORE clock status register"]
pub struct CLK_M4_M4CORE_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_M3CORE clock status register"]
pub mod clk_m4_m4core_stat;
#[doc = "CLK_M4_SCT clock status register"]
pub struct CLK_M4_SCT_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_SCT clock status register"]
pub mod clk_m4_sct_stat;
#[doc = "CLK_M4_USB1 clock status register"]
pub struct CLK_M4_USB1_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_USB1 clock status register"]
pub mod clk_m4_usb1_stat;
#[doc = "CLK_M4_EMCDIV clock status register"]
pub struct CLK_M4_EMCDIV_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_EMCDIV clock status register"]
pub mod clk_m4_emcdiv_stat;
#[doc = "CLK_M4_FLASHA clock status register"]
pub struct CLK_M4_FLASHA_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_FLASHA clock status register"]
pub mod clk_m4_flasha_stat;
#[doc = "CLK_M4_FLASHB clock status register"]
pub struct CLK_M4_FLASHB_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_FLASHB clock status register"]
pub mod clk_m4_flashb_stat;
#[doc = "CLK_M4_MOAPP clock status register"]
pub struct CLK_M4_M0APP_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_MOAPP clock status register"]
pub mod clk_m4_m0app_stat;
#[doc = "CLK_M4_ADCHS clock status register"]
pub struct CLK_M4_ADCHS_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_ADCHS clock status register"]
pub mod clk_m4_adchs_stat;
#[doc = "CLK_M4_EEPROM clock status register"]
pub struct CLK_M4_EEPROM_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_EEPROM clock status register"]
pub mod clk_m4_eeprom_stat;
#[doc = "CLK_M4_WWDT clock status register"]
pub struct CLK_M4_WWDT_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_WWDT clock status register"]
pub mod clk_m4_wwdt_stat;
#[doc = "CLK_M4_USART0 clock status register"]
pub struct CLK_M4_USART0_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_USART0 clock status register"]
pub mod clk_m4_usart0_stat;
#[doc = "CLK_M4_UART1 clock status register"]
pub struct CLK_M4_UART1_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_UART1 clock status register"]
pub mod clk_m4_uart1_stat;
#[doc = "CLK_M4_SSP0 clock status register"]
pub struct CLK_M4_SSP0_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_SSP0 clock status register"]
pub mod clk_m4_ssp0_stat;
#[doc = "CLK_M4_TIMER0 clock status register"]
pub struct CLK_M4_TIMER0_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_TIMER0 clock status register"]
pub mod clk_m4_timer0_stat;
#[doc = "CLK_M4_TIMER1 clock status register"]
pub struct CLK_M4_TIMER1_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_TIMER1 clock status register"]
pub mod clk_m4_timer1_stat;
#[doc = "CLK_SCU_XXX clock status register"]
pub struct CLK_M4_SCU_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_SCU_XXX clock status register"]
pub mod clk_m4_scu_stat;
#[doc = "CLK_M4_CREG clock status register"]
pub struct CLK_M4_CREG_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_CREG clock status register"]
pub mod clk_m4_creg_stat;
#[doc = "CLK_M4_RITIMER clock status register"]
pub struct CLK_M4_RITIMER_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_RITIMER clock status register"]
pub mod clk_m4_ritimer_stat;
#[doc = "CLK_M4_USART2 clock status register"]
pub struct CLK_M4_USART2_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_USART2 clock status register"]
pub mod clk_m4_usart2_stat;
#[doc = "CLK_M4_USART3 clock status register"]
pub struct CLK_M4_USART3_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_USART3 clock status register"]
pub mod clk_m4_usart3_stat;
#[doc = "CLK_M4_TIMER2 clock status register"]
pub struct CLK_M4_TIMER2_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_TIMER2 clock status register"]
pub mod clk_m4_timer2_stat;
#[doc = "CLK_M4_TIMER3 clock status register"]
pub struct CLK_M4_TIMER3_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_TIMER3 clock status register"]
pub mod clk_m4_timer3_stat;
#[doc = "CLK_M4_SSP1 clock status register"]
pub struct CLK_M4_SSP1_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_SSP1 clock status register"]
pub mod clk_m4_ssp1_stat;
#[doc = "CLK_M4_QEI clock status register"]
pub struct CLK_M4_QEI_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_M4_QEI clock status register"]
pub mod clk_m4_qei_stat;
#[doc = "CLK_PERIPH_BUS_STAT clock status register"]
pub struct CLK_PERIPH_BUS_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_PERIPH_BUS_STAT clock status register"]
pub mod clk_periph_bus_stat;
#[doc = "CLK_CORE_BUS_STAT clock status register"]
pub struct CLK_PERIPH_CORE_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_CORE_BUS_STAT clock status register"]
pub mod clk_periph_core_stat;
#[doc = "CLK_CORE_SGPIO_STAT clock status register"]
pub struct CLK_PERIPH_SGPIO_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_CORE_SGPIO_STAT clock status register"]
pub mod clk_periph_sgpio_stat;
#[doc = "CLK_USB0 clock status register"]
pub struct CLK_USB0_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_USB0 clock status register"]
pub mod clk_usb0_stat;
#[doc = "CLK_USB1 clock status register"]
pub struct CLK_USB1_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_USB1 clock status register"]
pub mod clk_usb1_stat;
#[doc = "CLK_SPI clock status register"]
pub struct CLK_SPI_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_SPI clock status register"]
pub mod clk_spi_stat;
#[doc = "CLK_ADCHS clock status register"]
pub struct CLK_ADCHS_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_ADCHS clock status register"]
pub mod clk_adchs_stat;
